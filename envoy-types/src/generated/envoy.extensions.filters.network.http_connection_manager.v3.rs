/// \[\#next-free-field: 59\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpConnectionManager {
    /// Supplies the type of codec that the connection manager should use.
    #[prost(enumeration = "http_connection_manager::CodecType", tag = "1")]
    pub codec_type: i32,
    /// The human readable prefix to use when emitting statistics for the
    /// connection manager. See the :ref:`statistics documentation <config_http_conn_man_stats>` for
    /// more information.
    #[prost(string, tag = "2")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// A list of individual HTTP filters that make up the filter chain for
    /// requests made to the connection manager. :ref:`Order matters <arch_overview_http_filters_ordering>`
    /// as the filters are processed sequentially as request events happen.
    #[prost(message, repeated, tag = "5")]
    pub http_filters: ::prost::alloc::vec::Vec<HttpFilter>,
    /// Whether the connection manager manipulates the :ref:`config_http_conn_man_headers_user-agent`
    /// and :ref:`config_http_conn_man_headers_downstream-service-cluster` headers. See the linked
    /// documentation for more information. Defaults to false.
    #[prost(message, optional, tag = "6")]
    pub add_user_agent: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Presence of the object defines whether the connection manager
    /// emits :ref:`tracing <arch_overview_tracing>` data to the :ref:`configured tracing provider <envoy_v3_api_msg_config.trace.v3.Tracing>`.
    #[prost(message, optional, tag = "7")]
    pub tracing: ::core::option::Option<http_connection_manager::Tracing>,
    /// Additional settings for HTTP requests handled by the connection manager. These will be
    /// applicable to both HTTP1 and HTTP2 requests.
    #[prost(message, optional, tag = "35")]
    pub common_http_protocol_options: ::core::option::Option<
        super::super::super::super::super::config::core::v3::HttpProtocolOptions,
    >,
    /// If set to true, Envoy will not start a drain timer for downstream HTTP1 connections after
    /// :ref:`common_http_protocol_options.max_connection_duration <envoy_v3_api_field_config.core.v3.HttpProtocolOptions.max_connection_duration>` passes.
    /// Instead, Envoy will wait for the next downstream request, add connection:close to the response
    /// headers, then close the connection after the stream ends.
    ///
    /// This behavior is compliant with `RFC 9112 section 9.6 <<https://www.rfc-editor.org/rfc/rfc9112#name-tear-down>`\_>
    ///
    /// If set to false, `max_connection_duration` will cause Envoy to enter the normal drain
    /// sequence for HTTP1 with Envoy eventually closing the connection (once there are no active
    /// streams).
    ///
    /// Has no effect if `max_connection_duration` is unset. Defaults to false.
    #[prost(bool, tag = "58")]
    pub http1_safe_max_connection_duration: bool,
    /// Additional HTTP/1 settings that are passed to the HTTP/1 codec.
    /// \[\#comment:TODO: The following fields are ignored when the
    /// :ref:`header validation configuration <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.typed_header_validation_config>`
    /// is present:
    ///
    /// 1. :ref:`allow_chunked_length <envoy_v3_api_field_config.core.v3.Http1ProtocolOptions.allow_chunked_length>`\]
    #[prost(message, optional, tag = "8")]
    pub http_protocol_options: ::core::option::Option<
        super::super::super::super::super::config::core::v3::Http1ProtocolOptions,
    >,
    /// Additional HTTP/2 settings that are passed directly to the HTTP/2 codec.
    #[prost(message, optional, tag = "9")]
    pub http2_protocol_options: ::core::option::Option<
        super::super::super::super::super::config::core::v3::Http2ProtocolOptions,
    >,
    /// Additional HTTP/3 settings that are passed directly to the HTTP/3 codec.
    /// \[\#not-implemented-hide:\]
    #[prost(message, optional, tag = "44")]
    pub http3_protocol_options: ::core::option::Option<
        super::super::super::super::super::config::core::v3::Http3ProtocolOptions,
    >,
    /// An optional override that the connection manager will write to the server
    /// header in responses. If not set, the default is `envoy`.
    #[prost(string, tag = "10")]
    pub server_name: ::prost::alloc::string::String,
    /// Defines the action to be applied to the Server header on the response path.
    /// By default, Envoy will overwrite the header with the value specified in
    /// server_name.
    #[prost(
        enumeration = "http_connection_manager::ServerHeaderTransformation",
        tag = "34"
    )]
    pub server_header_transformation: i32,
    /// Allows for explicit transformation of the :scheme header on the request path.
    /// If not set, Envoy's default :ref:`scheme  <config_http_conn_man_headers_scheme>`
    /// handling applies.
    #[prost(message, optional, tag = "48")]
    pub scheme_header_transformation: ::core::option::Option<
        super::super::super::super::super::config::core::v3::SchemeHeaderTransformation,
    >,
    /// The maximum request headers size for incoming connections.
    /// If unconfigured, the default max request headers allowed is 60 KiB.
    /// Requests that exceed this limit will receive a 431 response.
    #[prost(message, optional, tag = "29")]
    pub max_request_headers_kb: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// The stream idle timeout for connections managed by the connection manager.
    /// If not specified, this defaults to 5 minutes. The default value was selected
    /// so as not to interfere with any smaller configured timeouts that may have
    /// existed in configurations prior to the introduction of this feature, while
    /// introducing robustness to TCP connections that terminate without a FIN.
    ///
    /// This idle timeout applies to new streams and is overridable by the
    /// :ref:`route-level idle_timeout <envoy_v3_api_field_config.route.v3.RouteAction.idle_timeout>`. Even on a stream in
    /// which the override applies, prior to receipt of the initial request
    /// headers, the :ref:`stream_idle_timeout <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.stream_idle_timeout>`
    /// applies. Each time an encode/decode event for headers or data is processed
    /// for the stream, the timer will be reset. If the timeout fires, the stream
    /// is terminated with a 408 Request Timeout error code if no upstream response
    /// header has been received, otherwise a stream reset occurs.
    ///
    /// This timeout also specifies the amount of time that Envoy will wait for the peer to open enough
    /// window to write any remaining stream data once the entirety of stream data (local end stream is
    /// true) has been buffered pending available window. In other words, this timeout defends against
    /// a peer that does not release enough window to completely write the stream, even though all
    /// data has been proxied within available flow control windows. If the timeout is hit in this
    /// case, the :ref:`tx_flush_timeout <config_http_conn_man_stats_per_codec>` counter will be
    /// incremented. Note that :ref:`max_stream_duration <envoy_v3_api_field_config.core.v3.HttpProtocolOptions.max_stream_duration>` does not apply to
    /// this corner case.
    ///
    /// If the :ref:`overload action <config_overload_manager_overload_actions>` "envoy.overload_actions.reduce_timeouts"
    /// is configured, this timeout is scaled according to the value for
    /// :ref:`HTTP_DOWNSTREAM_STREAM_IDLE <envoy_v3_api_enum_value_config.overload.v3.ScaleTimersOverloadActionConfig.TimerType.HTTP_DOWNSTREAM_STREAM_IDLE>`.
    ///
    /// Note that it is possible to idle timeout even if the wire traffic for a stream is non-idle, due
    /// to the granularity of events presented to the connection manager. For example, while receiving
    /// very large request headers, it may be the case that there is traffic regularly arriving on the
    /// wire while the connection manage is only able to observe the end-of-headers event, hence the
    /// stream may still idle timeout.
    ///
    /// A value of 0 will completely disable the connection manager stream idle
    /// timeout, although per-route idle timeout overrides will continue to apply.
    #[prost(message, optional, tag = "24")]
    pub stream_idle_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The amount of time that Envoy will wait for the entire request to be received.
    /// The timer is activated when the request is initiated, and is disarmed when the last byte of the
    /// request is sent upstream (i.e. all decoding filters have processed the request), OR when the
    /// response is initiated. If not specified or set to 0, this timeout is disabled.
    #[prost(message, optional, tag = "28")]
    pub request_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The amount of time that Envoy will wait for the request headers to be received. The timer is
    /// activated when the first byte of the headers is received, and is disarmed when the last byte of
    /// the headers has been received. If not specified or set to 0, this timeout is disabled.
    #[prost(message, optional, tag = "41")]
    pub request_headers_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The time that Envoy will wait between sending an HTTP/2 “shutdown
    /// notification” (GOAWAY frame with max stream ID) and a final GOAWAY frame.
    /// This is used so that Envoy provides a grace period for new streams that
    /// race with the final GOAWAY frame. During this grace period, Envoy will
    /// continue to accept new streams. After the grace period, a final GOAWAY
    /// frame is sent and Envoy will start refusing new streams. Draining occurs
    /// either when a connection hits the idle timeout, when :ref:`max_connection_duration <envoy_v3_api_field_config.core.v3.HttpProtocolOptions.max_connection_duration>`
    /// is reached, or during general server draining. The default grace period is
    /// 5000 milliseconds (5 seconds) if this option is not specified.
    #[prost(message, optional, tag = "12")]
    pub drain_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The delayed close timeout is for downstream connections managed by the HTTP connection manager.
    /// It is defined as a grace period after connection close processing has been locally initiated
    /// during which Envoy will wait for the peer to close (i.e., a TCP FIN/RST is received by Envoy
    /// from the downstream connection) prior to Envoy closing the socket associated with that
    /// connection.
    /// NOTE: This timeout is enforced even when the socket associated with the downstream connection
    /// is pending a flush of the write buffer. However, any progress made writing data to the socket
    /// will restart the timer associated with this timeout. This means that the total grace period for
    /// a socket in this state will be
    /// \<total_time_waiting_for_write_buffer_flushes>+\<delayed_close_timeout>.
    ///
    /// Delaying Envoy's connection close and giving the peer the opportunity to initiate the close
    /// sequence mitigates a race condition that exists when downstream clients do not drain/process
    /// data in a connection's receive buffer after a remote close has been detected via a socket
    /// write(). This race leads to such clients failing to process the response code sent by Envoy,
    /// which could result in erroneous downstream processing.
    ///
    /// If the timeout triggers, Envoy will close the connection's socket.
    ///
    /// The default timeout is 1000 ms if this option is not specified.
    ///
    /// .. NOTE::
    /// To be useful in avoiding the race condition described above, this timeout must be set
    /// to *at least* <max round trip time expected between clients and Envoy>+\<100ms to account for
    /// a reasonable "worst" case processing time for a full iteration of Envoy's event loop>.
    ///
    /// .. WARNING::
    /// A value of 0 will completely disable delayed close processing. When disabled, the downstream
    /// connection's socket will be closed immediately after the write flush is completed or will
    /// never close if the write flush does not complete.
    #[prost(message, optional, tag = "26")]
    pub delayed_close_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Configuration for :ref:`HTTP access logs <arch_overview_access_logs>`
    /// emitted by the connection manager.
    #[prost(message, repeated, tag = "13")]
    pub access_log: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::accesslog::v3::AccessLog,
    >,
    /// The interval to flush the above access logs.
    ///
    /// .. attention::
    ///
    /// This field is deprecated in favor of
    /// :ref:`access_log_flush_interval <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.HcmAccessLogOptions.access_log_flush_interval>`.
    /// Note that if both this field and :ref:`access_log_flush_interval <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.HcmAccessLogOptions.access_log_flush_interval>`
    /// are specified, the former (deprecated field) is ignored.
    #[deprecated]
    #[prost(message, optional, tag = "54")]
    pub access_log_flush_interval: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// If set to true, HCM will flush an access log once when a new HTTP request is received, after the request
    /// headers have been evaluated, and before iterating through the HTTP filter chain.
    ///
    /// .. attention::
    ///
    /// This field is deprecated in favor of
    /// :ref:`flush_access_log_on_new_request <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.HcmAccessLogOptions.flush_access_log_on_new_request>`.
    /// Note that if both this field and :ref:`flush_access_log_on_new_request <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.HcmAccessLogOptions.flush_access_log_on_new_request>`
    /// are specified, the former (deprecated field) is ignored.
    #[deprecated]
    #[prost(bool, tag = "55")]
    pub flush_access_log_on_new_request: bool,
    /// Additional access log options for HTTP connection manager.
    #[prost(message, optional, tag = "56")]
    pub access_log_options: ::core::option::Option<
        http_connection_manager::HcmAccessLogOptions,
    >,
    /// If set to true, the connection manager will use the real remote address
    /// of the client connection when determining internal versus external origin and manipulating
    /// various headers. If set to false or absent, the connection manager will use the
    /// :ref:`config_http_conn_man_headers_x-forwarded-for` HTTP header. See the documentation for
    /// :ref:`config_http_conn_man_headers_x-forwarded-for`,
    /// :ref:`config_http_conn_man_headers_x-envoy-internal`, and
    /// :ref:`config_http_conn_man_headers_x-envoy-external-address` for more information.
    #[prost(message, optional, tag = "14")]
    pub use_remote_address: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// The number of additional ingress proxy hops from the right side of the
    /// :ref:`config_http_conn_man_headers_x-forwarded-for` HTTP header to trust when
    /// determining the origin client's IP address. The default is zero if this option
    /// is not specified. See the documentation for
    /// :ref:`config_http_conn_man_headers_x-forwarded-for` for more information.
    #[prost(uint32, tag = "19")]
    pub xff_num_trusted_hops: u32,
    /// The configuration for the original IP detection extensions.
    ///
    /// When configured the extensions will be called along with the request headers
    /// and information about the downstream connection, such as the directly connected address.
    /// Each extension will then use these parameters to decide the request's effective remote address.
    /// If an extension fails to detect the original IP address and isn't configured to reject
    /// the request, the HCM will try the remaining extensions until one succeeds or rejects
    /// the request. If the request isn't rejected nor any extension succeeds, the HCM will
    /// fallback to using the remote address.
    ///
    /// .. WARNING::
    /// Extensions cannot be used in conjunction with :ref:`use_remote_address <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.use_remote_address>`
    /// nor :ref:`xff_num_trusted_hops <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.xff_num_trusted_hops>`.
    ///
    /// \[\#extension-category: envoy.http.original_ip_detection\]
    #[prost(message, repeated, tag = "46")]
    pub original_ip_detection_extensions: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// The configuration for the early header mutation extensions.
    ///
    /// When configured the extensions will be called before any routing, tracing, or any filter processing.
    /// Each extension will be applied in the order they are configured.
    /// If the same header is mutated by multiple extensions, then the last extension will win.
    ///
    /// \[\#extension-category: envoy.http.early_header_mutation\]
    #[prost(message, repeated, tag = "52")]
    pub early_header_mutation_extensions: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// Configures what network addresses are considered internal for stats and header sanitation
    /// purposes. If unspecified, only RFC1918 IP addresses will be considered internal.
    /// See the documentation for :ref:`config_http_conn_man_headers_x-envoy-internal` for more
    /// information about internal/external addresses.
    #[prost(message, optional, tag = "25")]
    pub internal_address_config: ::core::option::Option<
        http_connection_manager::InternalAddressConfig,
    >,
    /// If set, Envoy will not append the remote address to the
    /// :ref:`config_http_conn_man_headers_x-forwarded-for` HTTP header. This may be used in
    /// conjunction with HTTP filters that explicitly manipulate XFF after the HTTP connection manager
    /// has mutated the request headers. While :ref:`use_remote_address <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.use_remote_address>`
    /// will also suppress XFF addition, it has consequences for logging and other
    /// Envoy uses of the remote address, so `skip_xff_append` should be used
    /// when only an elision of XFF addition is intended.
    #[prost(bool, tag = "21")]
    pub skip_xff_append: bool,
    /// Via header value to append to request and response headers. If this is
    /// empty, no via header will be appended.
    #[prost(string, tag = "22")]
    pub via: ::prost::alloc::string::String,
    /// Whether the connection manager will generate the :ref:`x-request-id <config_http_conn_man_headers_x-request-id>` header if it does not exist. This defaults to
    /// true. Generating a random UUID4 is expensive so in high throughput scenarios where this feature
    /// is not desired it can be disabled.
    #[prost(message, optional, tag = "15")]
    pub generate_request_id: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Whether the connection manager will keep the :ref:`x-request-id <config_http_conn_man_headers_x-request-id>` header if passed for a request that is edge
    /// (Edge request is the request from external clients to front Envoy) and not reset it, which
    /// is the current Envoy behaviour. This defaults to false.
    #[prost(bool, tag = "32")]
    pub preserve_external_request_id: bool,
    /// If set, Envoy will always set :ref:`x-request-id <config_http_conn_man_headers_x-request-id>` header in response.
    /// If this is false or not set, the request ID is returned in responses only if tracing is forced using
    /// :ref:`x-envoy-force-trace <config_http_conn_man_headers_x-envoy-force-trace>` header.
    #[prost(bool, tag = "37")]
    pub always_set_request_id_in_response: bool,
    /// How to handle the :ref:`config_http_conn_man_headers_x-forwarded-client-cert` (XFCC) HTTP
    /// header.
    #[prost(
        enumeration = "http_connection_manager::ForwardClientCertDetails",
        tag = "16"
    )]
    pub forward_client_cert_details: i32,
    /// This field is valid only when :ref:`forward_client_cert_details <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.forward_client_cert_details>`
    /// is APPEND_FORWARD or SANITIZE_SET and the client connection is mTLS. It specifies the fields in
    /// the client certificate to be forwarded. Note that in the
    /// :ref:`config_http_conn_man_headers_x-forwarded-client-cert` header, `Hash` is always set, and
    /// `By` is always set when the client certificate presents the URI type Subject Alternative Name
    /// value.
    #[prost(message, optional, tag = "17")]
    pub set_current_client_cert_details: ::core::option::Option<
        http_connection_manager::SetCurrentClientCertDetails,
    >,
    /// If proxy_100_continue is true, Envoy will proxy incoming "Expect:
    /// 100-continue" headers upstream, and forward "100 Continue" responses
    /// downstream. If this is false or not set, Envoy will instead strip the
    /// "Expect: 100-continue" header, and send a "100 Continue" response itself.
    #[prost(bool, tag = "18")]
    pub proxy_100_continue: bool,
    /// If
    /// :ref:`use_remote_address <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.use_remote_address>`
    /// is true and represent_ipv4_remote_address_as_ipv4_mapped_ipv6 is true and the remote address is
    /// an IPv4 address, the address will be mapped to IPv6 before it is appended to `x-forwarded-for`.
    /// This is useful for testing compatibility of upstream services that parse the header value. For
    /// example, 50.0.0.1 is represented as ::FFFF:50.0.0.1. See `IPv4-Mapped IPv6 Addresses <<https://tools.ietf.org/html/rfc4291#section-2.5.5.2>`\_> for details. This will also affect the
    /// :ref:`config_http_conn_man_headers_x-envoy-external-address` header. See
    /// :ref:`http_connection_manager.represent_ipv4_remote_address_as_ipv4_mapped_ipv6 <config_http_conn_man_runtime_represent_ipv4_remote_address_as_ipv4_mapped_ipv6>` for runtime
    /// control.
    /// \[\#not-implemented-hide:\]
    #[prost(bool, tag = "20")]
    pub represent_ipv4_remote_address_as_ipv4_mapped_ipv6: bool,
    #[prost(message, repeated, tag = "23")]
    pub upgrade_configs: ::prost::alloc::vec::Vec<
        http_connection_manager::UpgradeConfig,
    >,
    /// Should paths be normalized according to RFC 3986 before any processing of
    /// requests by HTTP filters or routing? This affects the upstream `:path` header
    /// as well. For paths that fail this check, Envoy will respond with 400 to
    /// paths that are malformed. This defaults to false currently but will default
    /// true in the future. When not specified, this value may be overridden by the
    /// runtime variable
    /// :ref:`http_connection_manager.normalize_path<config_http_conn_man_runtime_normalize_path>`.
    /// See `Normalization and Comparison <<https://tools.ietf.org/html/rfc3986#section-6>`\_>
    /// for details of normalization.
    /// Note that Envoy does not perform
    /// `case normalization <<https://tools.ietf.org/html/rfc3986#section-6.2.2.1>`\_>
    /// \[\#comment:TODO: This field is ignored when the
    /// :ref:`header validation configuration <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.typed_header_validation_config>`
    /// is present.\]
    #[prost(message, optional, tag = "30")]
    pub normalize_path: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Determines if adjacent slashes in the path are merged into one before any processing of
    /// requests by HTTP filters or routing. This affects the upstream `:path` header as well. Without
    /// setting this option, incoming requests with path `//dir///file` will not match against route
    /// with `prefix` match set to `/dir`. Defaults to `false`. Note that slash merging is not part of
    /// `HTTP spec <<https://tools.ietf.org/html/rfc3986>`\_> and is provided for convenience.
    /// \[\#comment:TODO: This field is ignored when the
    /// :ref:`header validation configuration <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.typed_header_validation_config>`
    /// is present.\]
    #[prost(bool, tag = "33")]
    pub merge_slashes: bool,
    /// Action to take when request URL path contains escaped slash sequences (%2F, %2f, %5C and %5c).
    /// The default value can be overridden by the :ref:`http_connection_manager.path_with_escaped_slashes_action<config_http_conn_man_runtime_path_with_escaped_slashes_action>`
    /// runtime variable.
    /// The :ref:`http_connection_manager.path_with_escaped_slashes_action_sampling<config_http_conn_man_runtime_path_with_escaped_slashes_action_enabled>` runtime
    /// variable can be used to apply the action to a portion of all requests.
    /// \[\#comment:TODO: This field is ignored when the
    /// :ref:`header validation configuration <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.typed_header_validation_config>`
    /// is present.\]
    #[prost(
        enumeration = "http_connection_manager::PathWithEscapedSlashesAction",
        tag = "45"
    )]
    pub path_with_escaped_slashes_action: i32,
    /// The configuration of the request ID extension. This includes operations such as
    /// generation, validation, and associated tracing operations. If empty, the
    /// :ref:`UuidRequestIdConfig <envoy_v3_api_msg_extensions.request_id.uuid.v3.UuidRequestIdConfig>`
    /// default extension is used with default parameters. See the documentation for that extension
    /// for details on what it does. Customizing the configuration for the default extension can be
    /// achieved by configuring it explicitly here. For example, to disable trace reason packing,
    /// the following configuration can be used:
    ///
    /// .. validated-code-block:: yaml
    /// :type-name: envoy.extensions.filters.network.http_connection_manager.v3.RequestIDExtension
    ///
    /// typed_config:
    /// "@type": type.googleapis.com/envoy.extensions.request_id.uuid.v3.UuidRequestIdConfig
    /// pack_trace_reason: false
    ///
    /// \[\#extension-category: envoy.request_id\]
    #[prost(message, optional, tag = "36")]
    pub request_id_extension: ::core::option::Option<RequestIdExtension>,
    /// The configuration to customize local reply returned by Envoy. It can customize status code,
    /// body text and response content type. If not specified, status code and text body are hard
    /// coded in Envoy, the response content type is plain text.
    #[prost(message, optional, tag = "38")]
    pub local_reply_config: ::core::option::Option<LocalReplyConfig>,
    /// Determines if the port part should be removed from host/authority header before any processing
    /// of request by HTTP filters or routing. The port would be removed only if it is equal to the :ref:`listener's<envoy_v3_api_field_config.listener.v3.Listener.address>`
    /// local port. This affects the upstream host header unless the method is
    /// CONNECT in which case if no filter adds a port the original port will be restored before headers are
    /// sent upstream.
    /// Without setting this option, incoming requests with host `example:443` will not match against
    /// route with :ref:`domains<envoy_v3_api_field_config.route.v3.VirtualHost.domains>` match set to `example`. Defaults to `false`. Note that port removal is not part
    /// of `HTTP spec <<https://tools.ietf.org/html/rfc3986>`\_> and is provided for convenience.
    /// Only one of `strip_matching_host_port` or `strip_any_host_port` can be set.
    #[prost(bool, tag = "39")]
    pub strip_matching_host_port: bool,
    /// Governs Envoy's behavior when receiving invalid HTTP from downstream.
    /// If this option is false (default), Envoy will err on the conservative side handling HTTP
    /// errors, terminating both HTTP/1.1 and HTTP/2 connections when receiving an invalid request.
    /// If this option is set to true, Envoy will be more permissive, only resetting the invalid
    /// stream in the case of HTTP/2 and leaving the connection open where possible (if the entire
    /// request is read for HTTP/1.1)
    /// In general this should be true for deployments receiving trusted traffic (L2 Envoys,
    /// company-internal mesh) and false when receiving untrusted traffic (edge deployments).
    ///
    /// If different behaviors for invalid_http_message for HTTP/1 and HTTP/2 are
    /// desired, one should use the new HTTP/1 option :ref:`override_stream_error_on_invalid_http_message <envoy_v3_api_field_config.core.v3.Http1ProtocolOptions.override_stream_error_on_invalid_http_message>` or the new HTTP/2 option
    /// :ref:`override_stream_error_on_invalid_http_message <envoy_v3_api_field_config.core.v3.Http2ProtocolOptions.override_stream_error_on_invalid_http_message>`
    /// `not` the deprecated but similarly named :ref:`stream_error_on_invalid_http_messaging <envoy_v3_api_field_config.core.v3.Http2ProtocolOptions.stream_error_on_invalid_http_messaging>`
    #[prost(message, optional, tag = "40")]
    pub stream_error_on_invalid_http_message: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// \[\#not-implemented-hide:\] Path normalization configuration. This includes
    /// configurations for transformations (e.g. RFC 3986 normalization or merge
    /// adjacent slashes) and the policy to apply them. The policy determines
    /// whether transformations affect the forwarded `:path` header. RFC 3986 path
    /// normalization is enabled by default and the default policy is that the
    /// normalized header will be forwarded. See :ref:`PathNormalizationOptions <envoy_v3_api_msg_extensions.filters.network.http_connection_manager.v3.PathNormalizationOptions>`
    /// for details.
    #[prost(message, optional, tag = "43")]
    pub path_normalization_options: ::core::option::Option<
        http_connection_manager::PathNormalizationOptions,
    >,
    /// Determines if trailing dot of the host should be removed from host/authority header before any
    /// processing of request by HTTP filters or routing.
    /// This affects the upstream host header.
    /// Without setting this option, incoming requests with host `example.com.` will not match against
    /// route with :ref:`domains<envoy_v3_api_field_config.route.v3.VirtualHost.domains>` match set to `example.com`. Defaults to `false`.
    /// When the incoming request contains a host/authority header that includes a port number,
    /// setting this option will strip a trailing dot, if present, from the host section,
    /// leaving the port as is (e.g. host value `example.com.:443` will be updated to `example.com:443`).
    #[prost(bool, tag = "47")]
    pub strip_trailing_host_dot: bool,
    /// Proxy-Status HTTP response header configuration.
    /// If this config is set, the Proxy-Status HTTP response header field is
    /// populated. By default, it is not.
    #[prost(message, optional, tag = "49")]
    pub proxy_status_config: ::core::option::Option<
        http_connection_manager::ProxyStatusConfig,
    >,
    /// Configuration options for Header Validation (UHV).
    /// UHV is an extensible mechanism for checking validity of HTTP requests as well as providing
    /// normalization for request attributes, such as URI path.
    /// If the typed_header_validation_config is present it overrides the following options:
    /// `normalize_path`, `merge_slashes`, `path_with_escaped_slashes_action`
    /// `http_protocol_options.allow_chunked_length`, `common_http_protocol_options.headers_with_underscores_action`.
    ///
    /// The default UHV checks the following:
    ///
    /// \#. HTTP/1 header map validity according to `RFC 7230 section 3.2<<https://datatracker.ietf.org/doc/html/rfc7230#section-3.2>`\_>
    /// \#. Syntax of HTTP/1 request target URI and response status
    /// \#. HTTP/2 header map validity according to `RFC 7540 section 8.1.2<<https://datatracker.ietf.org/doc/html/rfc7540#section-8.1.2`\_>
    /// \#. Syntax of HTTP/2 pseudo headers
    /// \#. HTTP/3 header map validity according to `RFC 9114 section 4.3 <<https://www.rfc-editor.org/rfc/rfc9114.html>`\_>
    /// \#. Syntax of HTTP/3 pseudo headers
    /// \#. Syntax of `Content-Length` and `Transfer-Encoding`
    /// \#. Validation of HTTP/1 requests with both `Content-Length` and `Transfer-Encoding` headers
    /// \#. Normalization of the URI path according to `Normalization and Comparison <<https://datatracker.ietf.org/doc/html/rfc3986#section-6>`\_>
    /// without `case normalization <<https://datatracker.ietf.org/doc/html/rfc3986#section-6.2.2.1>`\_>
    ///
    /// \[\#not-implemented-hide:\]
    /// \[\#extension-category: envoy.http.header_validators\]
    #[prost(message, optional, tag = "50")]
    pub typed_header_validation_config: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// Append the `x-forwarded-port` header with the port value client used to connect to Envoy. It
    /// will be ignored if the `x-forwarded-port` header has been set by any trusted proxy in front of Envoy.
    #[prost(bool, tag = "51")]
    pub append_x_forwarded_port: bool,
    /// Append the :ref:`config_http_conn_man_headers_x-envoy-local-overloaded` HTTP header in the scenario where
    /// the Overload Manager has been triggered.
    #[prost(bool, tag = "57")]
    pub append_local_overload: bool,
    /// Whether the HCM will add ProxyProtocolFilterState to the Connection lifetime filter state. Defaults to `true`.
    /// This should be set to `false` in cases where Envoy's view of the downstream address may not correspond to the
    /// actual client address, for example, if there's another proxy in front of the Envoy.
    #[prost(message, optional, tag = "53")]
    pub add_proxy_protocol_connection_state: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    #[prost(oneof = "http_connection_manager::RouteSpecifier", tags = "3, 4, 31")]
    pub route_specifier: ::core::option::Option<http_connection_manager::RouteSpecifier>,
    #[prost(oneof = "http_connection_manager::StripPortMode", tags = "42")]
    pub strip_port_mode: ::core::option::Option<http_connection_manager::StripPortMode>,
}
/// Nested message and enum types in `HttpConnectionManager`.
pub mod http_connection_manager {
    /// \[\#next-free-field: 11\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Tracing {
        /// Target percentage of requests managed by this HTTP connection manager that will be force
        /// traced if the :ref:`x-client-trace-id <config_http_conn_man_headers_x-client-trace-id>`
        /// header is set. This field is a direct analog for the runtime variable
        /// 'tracing.client_enabled' in the :ref:`HTTP Connection Manager <config_http_conn_man_runtime>`.
        /// Default: 100%
        #[prost(message, optional, tag = "3")]
        pub client_sampling: ::core::option::Option<
            super::super::super::super::super::super::r#type::v3::Percent,
        >,
        /// Target percentage of requests managed by this HTTP connection manager that will be randomly
        /// selected for trace generation, if not requested by the client or not forced. This field is
        /// a direct analog for the runtime variable 'tracing.random_sampling' in the
        /// :ref:`HTTP Connection Manager <config_http_conn_man_runtime>`.
        /// Default: 100%
        #[prost(message, optional, tag = "4")]
        pub random_sampling: ::core::option::Option<
            super::super::super::super::super::super::r#type::v3::Percent,
        >,
        /// Target percentage of requests managed by this HTTP connection manager that will be traced
        /// after all other sampling checks have been applied (client-directed, force tracing, random
        /// sampling). This field functions as an upper limit on the total configured sampling rate. For
        /// instance, setting client_sampling to 100% but overall_sampling to 1% will result in only 1%
        /// of client requests with the appropriate headers to be force traced. This field is a direct
        /// analog for the runtime variable 'tracing.global_enabled' in the
        /// :ref:`HTTP Connection Manager <config_http_conn_man_runtime>`.
        /// Default: 100%
        #[prost(message, optional, tag = "5")]
        pub overall_sampling: ::core::option::Option<
            super::super::super::super::super::super::r#type::v3::Percent,
        >,
        /// Whether to annotate spans with additional data. If true, spans will include logs for stream
        /// events.
        #[prost(bool, tag = "6")]
        pub verbose: bool,
        /// Maximum length of the request path to extract and include in the HttpUrl tag. Used to
        /// truncate lengthy request paths to meet the needs of a tracing backend.
        /// Default: 256
        #[prost(message, optional, tag = "7")]
        pub max_path_tag_length: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// A list of custom tags with unique tag name to create tags for the active span.
        #[prost(message, repeated, tag = "8")]
        pub custom_tags: ::prost::alloc::vec::Vec<
            super::super::super::super::super::super::r#type::tracing::v3::CustomTag,
        >,
        /// Configuration for an external tracing provider.
        /// If not specified, no tracing will be performed.
        ///
        /// .. attention::
        /// Please be aware that `envoy.tracers.opencensus` provider can only be configured once
        /// in Envoy lifetime.
        /// Any attempts to reconfigure it or to use different configurations for different HCM filters
        /// will be rejected.
        /// Such a constraint is inherent to OpenCensus itself. It cannot be overcome without changes
        /// on OpenCensus side.
        #[prost(message, optional, tag = "9")]
        pub provider: ::core::option::Option<
            super::super::super::super::super::super::config::trace::v3::tracing::Http,
        >,
        /// Create separate tracing span for each upstream request if true. And if this flag is set to true,
        /// the tracing provider will assume that Envoy will be independent hop in the trace chain and may
        /// set span type to client or server based on this flag.
        /// This will deprecate the
        /// :ref:`start_child_span <envoy_v3_api_field_extensions.filters.http.router.v3.Router.start_child_span>`
        /// in the router.
        ///
        /// Users should set appropriate value based on their tracing provider and actual scenario:
        ///
        /// * If Envoy is used as sidecar and users want to make the sidecar and its application as only one
        ///   hop in the trace chain, this flag should be set to false. And please also make sure the
        ///   :ref:`start_child_span <envoy_v3_api_field_extensions.filters.http.router.v3.Router.start_child_span>`
        ///   in the router is not set to true.
        /// * If Envoy is used as gateway or independent proxy, or users want to make the sidecar and its
        ///   application as different hops in the trace chain, this flag should be set to true.
        /// * If tracing provider that has explicit requirements on span creation (like SkyWalking),
        ///   this flag should be set to true.
        ///
        /// The default value is false for now for backward compatibility.
        #[prost(message, optional, tag = "10")]
        pub spawn_upstream_span: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::BoolValue,
        >,
    }
    /// Nested message and enum types in `Tracing`.
    pub mod tracing {
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
        pub enum OperationName {
            /// The HTTP listener is used for ingress/incoming requests.
            Ingress = 0,
            /// The HTTP listener is used for egress/outgoing requests.
            Egress = 1,
        }
        impl OperationName {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    OperationName::Ingress => "INGRESS",
                    OperationName::Egress => "EGRESS",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "INGRESS" => Some(Self::Ingress),
                    "EGRESS" => Some(Self::Egress),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InternalAddressConfig {
        /// Whether unix socket addresses should be considered internal.
        #[prost(bool, tag = "1")]
        pub unix_sockets: bool,
        /// List of CIDR ranges that are treated as internal. If unset, then RFC1918 / RFC4193
        /// IP addresses will be considered internal.
        #[prost(message, repeated, tag = "2")]
        pub cidr_ranges: ::prost::alloc::vec::Vec<
            super::super::super::super::super::super::config::core::v3::CidrRange,
        >,
    }
    /// \[\#next-free-field: 7\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetCurrentClientCertDetails {
        /// Whether to forward the subject of the client cert. Defaults to false.
        #[prost(message, optional, tag = "1")]
        pub subject: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::BoolValue,
        >,
        /// Whether to forward the entire client cert in URL encoded PEM format. This will appear in the
        /// XFCC header comma separated from other values with the value Cert="PEM".
        /// Defaults to false.
        #[prost(bool, tag = "3")]
        pub cert: bool,
        /// Whether to forward the entire client cert chain (including the leaf cert) in URL encoded PEM
        /// format. This will appear in the XFCC header comma separated from other values with the value
        /// Chain="PEM".
        /// Defaults to false.
        #[prost(bool, tag = "6")]
        pub chain: bool,
        /// Whether to forward the DNS type Subject Alternative Names of the client cert.
        /// Defaults to false.
        #[prost(bool, tag = "4")]
        pub dns: bool,
        /// Whether to forward the URI type Subject Alternative Name of the client cert. Defaults to
        /// false.
        #[prost(bool, tag = "5")]
        pub uri: bool,
    }
    /// The configuration for HTTP upgrades.
    /// For each upgrade type desired, an UpgradeConfig must be added.
    ///
    /// .. warning::
    ///
    /// ```text
    /// The current implementation of upgrade headers does not handle
    /// multi-valued upgrade headers. Support for multi-valued headers may be
    /// added in the future if needed.
    /// ```
    ///
    /// .. warning::
    /// The current implementation of upgrade headers does not work with HTTP/2
    /// upstreams.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpgradeConfig {
        /// The case-insensitive name of this upgrade, e.g. "websocket".
        /// For each upgrade type present in upgrade_configs, requests with
        /// Upgrade: \[upgrade_type\]
        /// will be proxied upstream.
        #[prost(string, tag = "1")]
        pub upgrade_type: ::prost::alloc::string::String,
        /// If present, this represents the filter chain which will be created for
        /// this type of upgrade. If no filters are present, the filter chain for
        /// HTTP connections will be used for this upgrade type.
        #[prost(message, repeated, tag = "2")]
        pub filters: ::prost::alloc::vec::Vec<super::HttpFilter>,
        /// Determines if upgrades are enabled or disabled by default. Defaults to true.
        /// This can be overridden on a per-route basis with :ref:`cluster <envoy_v3_api_field_config.route.v3.RouteAction.upgrade_configs>` as documented in the
        /// :ref:`upgrade documentation <arch_overview_upgrades>`.
        #[prost(message, optional, tag = "3")]
        pub enabled: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::BoolValue,
        >,
    }
    /// \[\#not-implemented-hide:\] Transformations that apply to path headers. Transformations are applied
    /// before any processing of requests by HTTP filters, routing, and matching. Only the normalized
    /// path will be visible internally if a transformation is enabled. Any path rewrites that the
    /// router performs (e.g. :ref:`regex_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.regex_rewrite>` or :ref:`prefix_rewrite <envoy_v3_api_field_config.route.v3.RouteAction.prefix_rewrite>`) will apply to the `:path` header
    /// destined for the upstream.
    ///
    /// Note: access logging and tracing will show the original `:path` header.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PathNormalizationOptions {
        /// \[\#not-implemented-hide:\] Normalization applies internally before any processing of requests by
        /// HTTP filters, routing, and matching *and* will affect the forwarded `:path` header. Defaults
        /// to :ref:`NormalizePathRFC3986 <envoy_v3_api_msg_type.http.v3.PathTransformation.Operation.NormalizePathRFC3986>`. When not
        /// specified, this value may be overridden by the runtime variable
        /// :ref:`http_connection_manager.normalize_path<config_http_conn_man_runtime_normalize_path>`.
        /// Envoy will respond with 400 to paths that are malformed (e.g. for paths that fail RFC 3986
        /// normalization due to disallowed characters.)
        #[prost(message, optional, tag = "1")]
        pub forwarding_transformation: ::core::option::Option<
            super::super::super::super::super::super::r#type::http::v3::PathTransformation,
        >,
        /// \[\#not-implemented-hide:\] Normalization only applies internally before any processing of
        /// requests by HTTP filters, routing, and matching. These will be applied after full
        /// transformation is applied. The `:path` header before this transformation will be restored in
        /// the router filter and sent upstream unless it was mutated by a filter. Defaults to no
        /// transformations.
        /// Multiple actions can be applied in the same Transformation, forming a sequential
        /// pipeline. The transformations will be performed in the order that they appear. Envoy will
        /// respond with 400 to paths that are malformed (e.g. for paths that fail RFC 3986
        /// normalization due to disallowed characters.)
        #[prost(message, optional, tag = "2")]
        pub http_filter_transformation: ::core::option::Option<
            super::super::super::super::super::super::r#type::http::v3::PathTransformation,
        >,
    }
    /// Configures the manner in which the Proxy-Status HTTP response header is
    /// populated.
    ///
    /// See the [Proxy-Status
    /// RFC](<https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-proxy-status-08>).
    /// \[\#comment:TODO: Update this with the non-draft URL when finalized.\]
    ///
    /// The Proxy-Status header is a string of the form:
    ///
    /// "\<server_name>; error=\<error_type>; details=<details>"
    /// \[\#next-free-field: 7\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProxyStatusConfig {
        /// If true, the details field of the Proxy-Status header is not populated with stream_info.response_code_details.
        /// This value defaults to `false`, i.e. the `details` field is populated by default.
        #[prost(bool, tag = "1")]
        pub remove_details: bool,
        /// If true, the details field of the Proxy-Status header will not contain
        /// connection termination details. This value defaults to `false`, i.e. the
        /// `details` field will contain connection termination details by default.
        #[prost(bool, tag = "2")]
        pub remove_connection_termination_details: bool,
        /// If true, the details field of the Proxy-Status header will not contain an
        /// enumeration of the Envoy ResponseFlags. This value defaults to `false`,
        /// i.e. the `details` field will contain a list of ResponseFlags by default.
        #[prost(bool, tag = "3")]
        pub remove_response_flags: bool,
        /// If true, overwrites the existing Status header with the response code
        /// recommended by the Proxy-Status spec.
        /// This value defaults to `false`, i.e. the HTTP response code is not
        /// overwritten.
        #[prost(bool, tag = "4")]
        pub set_recommended_response_code: bool,
        /// The name of the proxy as it appears at the start of the Proxy-Status
        /// header.
        ///
        /// If neither of these values are set, this value defaults to `server_name`,
        /// which itself defaults to "envoy".
        #[prost(oneof = "proxy_status_config::ProxyName", tags = "5, 6")]
        pub proxy_name: ::core::option::Option<proxy_status_config::ProxyName>,
    }
    /// Nested message and enum types in `ProxyStatusConfig`.
    pub mod proxy_status_config {
        /// The name of the proxy as it appears at the start of the Proxy-Status
        /// header.
        ///
        /// If neither of these values are set, this value defaults to `server_name`,
        /// which itself defaults to "envoy".
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ProxyName {
            /// If `use_node_id` is set, Proxy-Status headers will use the Envoy's node
            /// ID as the name of the proxy.
            #[prost(bool, tag = "5")]
            UseNodeId(bool),
            /// If `literal_proxy_name` is set, Proxy-Status headers will use this
            /// value as the name of the proxy.
            #[prost(string, tag = "6")]
            LiteralProxyName(::prost::alloc::string::String),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HcmAccessLogOptions {
        /// The interval to flush the above access logs. By default, the HCM will flush exactly one access log
        /// on stream close, when the HTTP request is complete. If this field is set, the HCM will flush access
        /// logs periodically at the specified interval. This is especially useful in the case of long-lived
        /// requests, such as CONNECT and Websockets. Final access logs can be detected via the
        /// `requestComplete()` method of `StreamInfo` in access log filters, or through the `%DURATION%` substitution
        /// string.
        /// The interval must be at least 1 millisecond.
        #[prost(message, optional, tag = "1")]
        pub access_log_flush_interval: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
        /// If set to true, HCM will flush an access log when a new HTTP request is received, after request
        /// headers have been evaluated, before iterating through the HTTP filter chain.
        /// This log record, if enabled, does not depend on periodic log records or request completion log.
        /// Details related to upstream cluster, such as upstream host, will not be available for this log.
        #[prost(bool, tag = "2")]
        pub flush_access_log_on_new_request: bool,
        /// If true, the HCM will flush an access log when a tunnel is successfully established. For example,
        /// this could be when an upstream has successfully returned 101 Switching Protocols, or when the proxy
        /// has returned 200 to a CONNECT request.
        #[prost(bool, tag = "3")]
        pub flush_log_on_tunnel_successfully_established: bool,
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
    pub enum CodecType {
        /// For every new connection, the connection manager will determine which
        /// codec to use. This mode supports both ALPN for TLS listeners as well as
        /// protocol inference for plaintext listeners. If ALPN data is available, it
        /// is preferred, otherwise protocol inference is used. In almost all cases,
        /// this is the right option to choose for this setting.
        Auto = 0,
        /// The connection manager will assume that the client is speaking HTTP/1.1.
        Http1 = 1,
        /// The connection manager will assume that the client is speaking HTTP/2
        /// (Envoy does not require HTTP/2 to take place over TLS or to use ALPN.
        /// Prior knowledge is allowed).
        Http2 = 2,
        /// \[\#not-implemented-hide:\] QUIC implementation is not production ready yet. Use this enum with
        /// caution to prevent accidental execution of QUIC code. I.e. `!= HTTP2` is no longer sufficient
        /// to distinguish HTTP1 and HTTP2 traffic.
        Http3 = 3,
    }
    impl CodecType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CodecType::Auto => "AUTO",
                CodecType::Http1 => "HTTP1",
                CodecType::Http2 => "HTTP2",
                CodecType::Http3 => "HTTP3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AUTO" => Some(Self::Auto),
                "HTTP1" => Some(Self::Http1),
                "HTTP2" => Some(Self::Http2),
                "HTTP3" => Some(Self::Http3),
                _ => None,
            }
        }
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
    pub enum ServerHeaderTransformation {
        /// Overwrite any Server header with the contents of server_name.
        Overwrite = 0,
        /// If no Server header is present, append Server server_name
        /// If a Server header is present, pass it through.
        AppendIfAbsent = 1,
        /// Pass through the value of the server header, and do not append a header
        /// if none is present.
        PassThrough = 2,
    }
    impl ServerHeaderTransformation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ServerHeaderTransformation::Overwrite => "OVERWRITE",
                ServerHeaderTransformation::AppendIfAbsent => "APPEND_IF_ABSENT",
                ServerHeaderTransformation::PassThrough => "PASS_THROUGH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OVERWRITE" => Some(Self::Overwrite),
                "APPEND_IF_ABSENT" => Some(Self::AppendIfAbsent),
                "PASS_THROUGH" => Some(Self::PassThrough),
                _ => None,
            }
        }
    }
    /// How to handle the :ref:`config_http_conn_man_headers_x-forwarded-client-cert` (XFCC) HTTP
    /// header.
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
    pub enum ForwardClientCertDetails {
        /// Do not send the XFCC header to the next hop. This is the default value.
        Sanitize = 0,
        /// When the client connection is mTLS (Mutual TLS), forward the XFCC header
        /// in the request.
        ForwardOnly = 1,
        /// When the client connection is mTLS, append the client certificate
        /// information to the request’s XFCC header and forward it.
        AppendForward = 2,
        /// When the client connection is mTLS, reset the XFCC header with the client
        /// certificate information and send it to the next hop.
        SanitizeSet = 3,
        /// Always forward the XFCC header in the request, regardless of whether the
        /// client connection is mTLS.
        AlwaysForwardOnly = 4,
    }
    impl ForwardClientCertDetails {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ForwardClientCertDetails::Sanitize => "SANITIZE",
                ForwardClientCertDetails::ForwardOnly => "FORWARD_ONLY",
                ForwardClientCertDetails::AppendForward => "APPEND_FORWARD",
                ForwardClientCertDetails::SanitizeSet => "SANITIZE_SET",
                ForwardClientCertDetails::AlwaysForwardOnly => "ALWAYS_FORWARD_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SANITIZE" => Some(Self::Sanitize),
                "FORWARD_ONLY" => Some(Self::ForwardOnly),
                "APPEND_FORWARD" => Some(Self::AppendForward),
                "SANITIZE_SET" => Some(Self::SanitizeSet),
                "ALWAYS_FORWARD_ONLY" => Some(Self::AlwaysForwardOnly),
                _ => None,
            }
        }
    }
    /// Determines the action for request that contain %2F, %2f, %5C or %5c sequences in the URI path.
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
        /// Envoy, by default, takes the KEEP_UNCHANGED action.
        /// NOTE: the implementation may change the default behavior at-will.
        ImplementationSpecificDefault = 0,
        /// Keep escaped slashes.
        KeepUnchanged = 1,
        /// Reject client request with the 400 status. gRPC requests will be rejected with the INTERNAL (13) error code.
        /// The "httpN.downstream_rq_failed_path_normalization" counter is incremented for each rejected request.
        RejectRequest = 2,
        /// Unescape %2F and %5C sequences and redirect request to the new path if these sequences were present.
        /// Redirect occurs after path normalization and merge slashes transformations if they were configured.
        /// NOTE: gRPC requests will be rejected with the INTERNAL (13) error code.
        /// This option minimizes possibility of path confusion exploits by forcing request with unescaped slashes to
        /// traverse all parties: downstream client, intermediate proxies, Envoy and upstream server.
        /// The "httpN.downstream_rq_redirected_with_normalized_path" counter is incremented for each
        /// redirected request.
        UnescapeAndRedirect = 3,
        /// Unescape %2F and %5C sequences.
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
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RouteSpecifier {
        /// The connection manager’s route table will be dynamically loaded via the RDS API.
        #[prost(message, tag = "3")]
        Rds(super::Rds),
        /// The route table for the connection manager is static and is specified in this property.
        #[prost(message, tag = "4")]
        RouteConfig(
            super::super::super::super::super::super::config::route::v3::RouteConfiguration,
        ),
        /// A route table will be dynamically assigned to each request based on request attributes
        /// (e.g., the value of a header). The "routing scopes" (i.e., route tables) and "scope keys" are
        /// specified in this message.
        #[prost(message, tag = "31")]
        ScopedRoutes(super::ScopedRoutes),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StripPortMode {
        /// Determines if the port part should be removed from host/authority header before any processing
        /// of request by HTTP filters or routing.
        /// This affects the upstream host header unless the method is CONNECT in
        /// which case if no filter adds a port the original port will be restored before headers are sent upstream.
        /// Without setting this option, incoming requests with host `example:443` will not match against
        /// route with :ref:`domains<envoy_v3_api_field_config.route.v3.VirtualHost.domains>` match set to `example`. Defaults to `false`. Note that port removal is not part
        /// of `HTTP spec <<https://tools.ietf.org/html/rfc3986>`\_> and is provided for convenience.
        /// Only one of `strip_matching_host_port` or `strip_any_host_port` can be set.
        #[prost(bool, tag = "42")]
        StripAnyHostPort(bool),
    }
}
/// The configuration to customize local reply returned by Envoy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalReplyConfig {
    /// Configuration of list of mappers which allows to filter and change local response.
    /// The mappers will be checked by the specified order until one is matched.
    #[prost(message, repeated, tag = "1")]
    pub mappers: ::prost::alloc::vec::Vec<ResponseMapper>,
    /// The configuration to form response body from the :ref:`command operators <config_access_log_command_operators>`
    /// and to specify response content type as one of: plain/text or application/json.
    ///
    /// Example one: "plain/text" `body_format`.
    ///
    /// .. validated-code-block:: yaml
    /// :type-name: envoy.config.core.v3.SubstitutionFormatString
    ///
    /// text_format: "%LOCAL_REPLY_BODY%:%RESPONSE_CODE%:path=%REQ(:path)%\n"
    ///
    /// The following response body in "plain/text" format will be generated for a request with
    /// local reply body of "upstream connection error", response_code=503 and path=/foo.
    ///
    /// .. code-block:: text
    ///
    /// upstream connect error:503:path=/foo
    ///
    /// Example two: "application/json" `body_format`.
    ///
    /// .. validated-code-block:: yaml
    /// :type-name: envoy.config.core.v3.SubstitutionFormatString
    ///
    /// json_format:
    /// status: "%RESPONSE_CODE%"
    /// message: "%LOCAL_REPLY_BODY%"
    /// path: "%REQ(:path)%"
    ///
    /// The following response body in "application/json" format would be generated for a request with
    /// local reply body of "upstream connection error", response_code=503 and path=/foo.
    ///
    /// .. code-block:: json
    ///
    /// {
    /// "status": 503,
    /// "message": "upstream connection error",
    /// "path": "/foo"
    /// }
    #[prost(message, optional, tag = "2")]
    pub body_format: ::core::option::Option<
        super::super::super::super::super::config::core::v3::SubstitutionFormatString,
    >,
}
/// The configuration to filter and change local response.
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMapper {
    /// Filter to determine if this mapper should apply.
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<
        super::super::super::super::super::config::accesslog::v3::AccessLogFilter,
    >,
    /// The new response status code if specified.
    #[prost(message, optional, tag = "2")]
    pub status_code: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// The new local reply body text if specified. It will be used in the `%LOCAL_REPLY_BODY%`
    /// command operator in the `body_format`.
    #[prost(message, optional, tag = "3")]
    pub body: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// A per mapper `body_format` to override the :ref:`body_format <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.LocalReplyConfig.body_format>`.
    /// It will be used when this mapper is matched.
    #[prost(message, optional, tag = "4")]
    pub body_format_override: ::core::option::Option<
        super::super::super::super::super::config::core::v3::SubstitutionFormatString,
    >,
    /// HTTP headers to add to a local reply. This allows the response mapper to append, to add
    /// or to override headers of any local reply before it is sent to a downstream client.
    #[prost(message, repeated, tag = "5")]
    pub headers_to_add: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::HeaderValueOption,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rds {
    /// Configuration source specifier for RDS.
    #[prost(message, optional, tag = "1")]
    pub config_source: ::core::option::Option<
        super::super::super::super::super::config::core::v3::ConfigSource,
    >,
    /// The name of the route configuration. This name will be passed to the RDS
    /// API. This allows an Envoy configuration with multiple HTTP listeners (and
    /// associated HTTP connection manager filters) to use different route
    /// configurations.
    #[prost(string, tag = "2")]
    pub route_config_name: ::prost::alloc::string::String,
}
/// This message is used to work around the limitations with 'oneof' and repeated fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopedRouteConfigurationsList {
    #[prost(message, repeated, tag = "1")]
    pub scoped_route_configurations: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::route::v3::ScopedRouteConfiguration,
    >,
}
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopedRoutes {
    /// The name assigned to the scoped routing configuration.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The algorithm to use for constructing a scope key for each request.
    #[prost(message, optional, tag = "2")]
    pub scope_key_builder: ::core::option::Option<scoped_routes::ScopeKeyBuilder>,
    /// Configuration source specifier for RDS.
    /// This config source is used to subscribe to RouteConfiguration resources specified in
    /// ScopedRouteConfiguration messages.
    #[prost(message, optional, tag = "3")]
    pub rds_config_source: ::core::option::Option<
        super::super::super::super::super::config::core::v3::ConfigSource,
    >,
    #[prost(oneof = "scoped_routes::ConfigSpecifier", tags = "4, 5")]
    pub config_specifier: ::core::option::Option<scoped_routes::ConfigSpecifier>,
}
/// Nested message and enum types in `ScopedRoutes`.
pub mod scoped_routes {
    /// Specifies the mechanism for constructing "scope keys" based on HTTP request attributes. These
    /// keys are matched against a set of :ref:`Key<envoy_v3_api_msg_config.route.v3.ScopedRouteConfiguration.Key>`
    /// objects assembled from :ref:`ScopedRouteConfiguration<envoy_v3_api_msg_config.route.v3.ScopedRouteConfiguration>`
    /// messages distributed via SRDS (the Scoped Route Discovery Service) or assigned statically via
    /// :ref:`scoped_route_configurations_list<envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.ScopedRoutes.scoped_route_configurations_list>`.
    ///
    /// Upon receiving a request's headers, the Router will build a key using the algorithm specified
    /// by this message. This key will be used to look up the routing table (i.e., the
    /// :ref:`RouteConfiguration<envoy_v3_api_msg_config.route.v3.RouteConfiguration>`) to use for the request.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ScopeKeyBuilder {
        /// The final(built) scope key consists of the ordered union of these fragments, which are compared in order with the
        /// fragments of a :ref:`ScopedRouteConfiguration<envoy_v3_api_msg_config.route.v3.ScopedRouteConfiguration>`.
        /// A missing fragment during comparison will make the key invalid, i.e., the computed key doesn't match any key.
        #[prost(message, repeated, tag = "1")]
        pub fragments: ::prost::alloc::vec::Vec<scope_key_builder::FragmentBuilder>,
    }
    /// Nested message and enum types in `ScopeKeyBuilder`.
    pub mod scope_key_builder {
        /// Specifies the mechanism for constructing key fragments which are composed into scope keys.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FragmentBuilder {
            #[prost(oneof = "fragment_builder::Type", tags = "1")]
            pub r#type: ::core::option::Option<fragment_builder::Type>,
        }
        /// Nested message and enum types in `FragmentBuilder`.
        pub mod fragment_builder {
            /// Specifies how the value of a header should be extracted.
            /// The following example maps the structure of a header to the fields in this message.
            ///
            /// .. code::
            ///
            /// ```text
            ///           <0> <1>   <-- index
            /// X-Header: a=b;c=d
            /// |         || |
            /// |         || \----> <element_separator>
            /// |         ||
            /// |         |\----> <element.separator>
            /// |         |
            /// |         \----> <element.key>
            /// |
            /// \----> <name>
            ///
            /// Each 'a=b' key-value pair constitutes an 'element' of the header field.
            /// ```
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct HeaderValueExtractor {
                /// The name of the header field to extract the value from.
                ///
                /// .. note::
                ///
                /// If the header appears multiple times only the first value is used.
                #[prost(string, tag = "1")]
                pub name: ::prost::alloc::string::String,
                /// The element separator (e.g., ';' separates 'a;b;c;d').
                /// Default: empty string. This causes the entirety of the header field to be extracted.
                /// If this field is set to an empty string and 'index' is used in the oneof below, 'index'
                /// must be set to 0.
                #[prost(string, tag = "2")]
                pub element_separator: ::prost::alloc::string::String,
                #[prost(oneof = "header_value_extractor::ExtractType", tags = "3, 4")]
                pub extract_type: ::core::option::Option<
                    header_value_extractor::ExtractType,
                >,
            }
            /// Nested message and enum types in `HeaderValueExtractor`.
            pub mod header_value_extractor {
                /// Specifies a header field's key value pair to match on.
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct KvElement {
                    /// The separator between key and value (e.g., '=' separates 'k=v;...').
                    /// If an element is an empty string, the element is ignored.
                    /// If an element contains no separator, the whole element is parsed as key and the
                    /// fragment value is an empty string.
                    /// If there are multiple values for a matched key, the first value is returned.
                    #[prost(string, tag = "1")]
                    pub separator: ::prost::alloc::string::String,
                    /// The key to match on.
                    #[prost(string, tag = "2")]
                    pub key: ::prost::alloc::string::String,
                }
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum ExtractType {
                    /// Specifies the zero based index of the element to extract.
                    /// Note Envoy concatenates multiple values of the same header key into a comma separated
                    /// string, the splitting always happens after the concatenation.
                    #[prost(uint32, tag = "3")]
                    Index(u32),
                    /// Specifies the key value pair to extract the value from.
                    #[prost(message, tag = "4")]
                    Element(KvElement),
                }
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Type {
                /// Specifies how a header field's value should be extracted.
                #[prost(message, tag = "1")]
                HeaderValueExtractor(HeaderValueExtractor),
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigSpecifier {
        /// The set of routing scopes corresponding to the HCM. A scope is assigned to a request by
        /// matching a key constructed from the request's attributes according to the algorithm specified
        /// by the
        /// :ref:`ScopeKeyBuilder<envoy_v3_api_msg_extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder>`
        /// in this message.
        #[prost(message, tag = "4")]
        ScopedRouteConfigurationsList(super::ScopedRouteConfigurationsList),
        /// The set of routing scopes associated with the HCM will be dynamically loaded via the SRDS
        /// API. A scope is assigned to a request by matching a key constructed from the request's
        /// attributes according to the algorithm specified by the
        /// :ref:`ScopeKeyBuilder<envoy_v3_api_msg_extensions.filters.network.http_connection_manager.v3.ScopedRoutes.ScopeKeyBuilder>`
        /// in this message.
        #[prost(message, tag = "5")]
        ScopedRds(super::ScopedRds),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopedRds {
    /// Configuration source specifier for scoped RDS.
    #[prost(message, optional, tag = "1")]
    pub scoped_rds_config_source: ::core::option::Option<
        super::super::super::super::super::config::core::v3::ConfigSource,
    >,
    /// xdstp:// resource locator for scoped RDS collection.
    /// \[\#not-implemented-hide:\]
    #[prost(string, tag = "2")]
    pub srds_resources_locator: ::prost::alloc::string::String,
}
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpFilter {
    /// The name of the filter configuration. It also serves as a resource name in ExtensionConfigDS.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If true, clients that do not support this filter may ignore the
    /// filter but otherwise accept the config.
    /// Otherwise, clients that do not support this filter must reject the config.
    #[prost(bool, tag = "6")]
    pub is_optional: bool,
    /// If true, the filter is disabled by default and must be explicitly enabled by setting
    /// per filter configuration in the route configuration.
    /// See :ref:`route based filter chain <arch_overview_http_filters_route_based_filter_chain>`
    /// for more details.
    ///
    /// Terminal filters (e.g. `envoy.filters.http.router`) cannot be marked as disabled.
    #[prost(bool, tag = "7")]
    pub disabled: bool,
    #[prost(oneof = "http_filter::ConfigType", tags = "4, 5")]
    pub config_type: ::core::option::Option<http_filter::ConfigType>,
}
/// Nested message and enum types in `HttpFilter`.
pub mod http_filter {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        /// Filter specific configuration which depends on the filter being instantiated. See the supported
        /// filters for further documentation.
        ///
        /// To support configuring a :ref:`match tree <arch_overview_matching_api>`, use an
        /// :ref:`ExtensionWithMatcher <envoy_v3_api_msg_extensions.common.matching.v3.ExtensionWithMatcher>`
        /// with the desired HTTP filter.
        /// \[\#extension-category: envoy.filters.http\]
        #[prost(message, tag = "4")]
        TypedConfig(
            super::super::super::super::super::super::super::google::protobuf::Any,
        ),
        /// Configuration source specifier for an extension configuration discovery service.
        /// In case of a failure and without the default configuration, the HTTP listener responds with code 500.
        /// Extension configs delivered through this mechanism are not expected to require warming (see <https://github.com/envoyproxy/envoy/issues/12061>).
        ///
        /// To support configuring a :ref:`match tree <arch_overview_matching_api>`, use an
        /// :ref:`ExtensionWithMatcher <envoy_v3_api_msg_extensions.common.matching.v3.ExtensionWithMatcher>`
        /// with the desired HTTP filter. This works for both the default filter configuration as well
        /// as for filters provided via the API.
        #[prost(message, tag = "5")]
        ConfigDiscovery(
            super::super::super::super::super::super::config::core::v3::ExtensionConfigSource,
        ),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestIdExtension {
    /// Request ID extension specific configuration.
    #[prost(message, optional, tag = "1")]
    pub typed_config: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Any,
    >,
}
/// \[\#protodoc-title: Envoy Mobile HTTP connection manager\]
/// HTTP connection manager for use in Envoy mobile.
/// \[\#extension: envoy.filters.network.envoy_mobile_http_connection_manager\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvoyMobileHttpConnectionManager {
    /// The configuration for the underlying HttpConnectionManager which will be
    /// instantiated for Envoy mobile.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<HttpConnectionManager>,
}
