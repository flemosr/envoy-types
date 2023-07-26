#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpAccessLogEntry {
    /// Common properties shared by all Envoy access logs.
    #[prost(message, optional, tag = "1")]
    pub common_properties: ::core::option::Option<AccessLogCommon>,
    /// Properties of the TCP connection.
    #[prost(message, optional, tag = "2")]
    pub connection_properties: ::core::option::Option<ConnectionProperties>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpAccessLogEntry {
    /// Common properties shared by all Envoy access logs.
    #[prost(message, optional, tag = "1")]
    pub common_properties: ::core::option::Option<AccessLogCommon>,
    #[prost(enumeration = "http_access_log_entry::HttpVersion", tag = "2")]
    pub protocol_version: i32,
    /// Description of the incoming HTTP request.
    #[prost(message, optional, tag = "3")]
    pub request: ::core::option::Option<HttpRequestProperties>,
    /// Description of the outgoing HTTP response.
    #[prost(message, optional, tag = "4")]
    pub response: ::core::option::Option<HttpResponseProperties>,
}
/// Nested message and enum types in `HTTPAccessLogEntry`.
pub mod http_access_log_entry {
    /// HTTP version
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
    pub enum HttpVersion {
        ProtocolUnspecified = 0,
        Http10 = 1,
        Http11 = 2,
        Http2 = 3,
        Http3 = 4,
    }
    impl HttpVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HttpVersion::ProtocolUnspecified => "PROTOCOL_UNSPECIFIED",
                HttpVersion::Http10 => "HTTP10",
                HttpVersion::Http11 => "HTTP11",
                HttpVersion::Http2 => "HTTP2",
                HttpVersion::Http3 => "HTTP3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROTOCOL_UNSPECIFIED" => Some(Self::ProtocolUnspecified),
                "HTTP10" => Some(Self::Http10),
                "HTTP11" => Some(Self::Http11),
                "HTTP2" => Some(Self::Http2),
                "HTTP3" => Some(Self::Http3),
                _ => None,
            }
        }
    }
}
/// Defines fields for a connection
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionProperties {
    /// Number of bytes received from downstream.
    #[prost(uint64, tag = "1")]
    pub received_bytes: u64,
    /// Number of bytes sent to downstream.
    #[prost(uint64, tag = "2")]
    pub sent_bytes: u64,
}
/// Defines fields that are shared by all Envoy access logs.
/// [#next-free-field: 34]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessLogCommon {
    /// \[#not-implemented-hide:\]
    /// This field indicates the rate at which this log entry was sampled.
    /// Valid range is (0.0, 1.0].
    #[prost(double, tag = "1")]
    pub sample_rate: f64,
    /// This field is the remote/origin address on which the request from the user was received.
    /// Note: This may not be the physical peer. E.g, if the remote address is inferred from for
    /// example the x-forwarder-for header, proxy protocol, etc.
    #[prost(message, optional, tag = "2")]
    pub downstream_remote_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    /// This field is the local/destination address on which the request from the user was received.
    #[prost(message, optional, tag = "3")]
    pub downstream_local_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    /// If the connection is secure,S this field will contain TLS properties.
    #[prost(message, optional, tag = "4")]
    pub tls_properties: ::core::option::Option<TlsProperties>,
    /// The time that Envoy started servicing this request. This is effectively the time that the first
    /// downstream byte is received.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<
        super::super::super::super::google::protobuf::Timestamp,
    >,
    /// Interval between the first downstream byte received and the last
    /// downstream byte received (i.e. time it takes to receive a request).
    #[prost(message, optional, tag = "6")]
    pub time_to_last_rx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Interval between the first downstream byte received and the first upstream byte sent. There may
    /// by considerable delta between ``time_to_last_rx_byte`` and this value due to filters.
    /// Additionally, the same caveats apply as documented in ``time_to_last_downstream_tx_byte`` about
    /// not accounting for kernel socket buffer time, etc.
    #[prost(message, optional, tag = "7")]
    pub time_to_first_upstream_tx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Interval between the first downstream byte received and the last upstream byte sent. There may
    /// by considerable delta between ``time_to_last_rx_byte`` and this value due to filters.
    /// Additionally, the same caveats apply as documented in ``time_to_last_downstream_tx_byte`` about
    /// not accounting for kernel socket buffer time, etc.
    #[prost(message, optional, tag = "8")]
    pub time_to_last_upstream_tx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Interval between the first downstream byte received and the first upstream
    /// byte received (i.e. time it takes to start receiving a response).
    #[prost(message, optional, tag = "9")]
    pub time_to_first_upstream_rx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Interval between the first downstream byte received and the last upstream
    /// byte received (i.e. time it takes to receive a complete response).
    #[prost(message, optional, tag = "10")]
    pub time_to_last_upstream_rx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Interval between the first downstream byte received and the first downstream byte sent.
    /// There may be a considerable delta between the ``time_to_first_upstream_rx_byte`` and this field
    /// due to filters. Additionally, the same caveats apply as documented in
    /// ``time_to_last_downstream_tx_byte`` about not accounting for kernel socket buffer time, etc.
    #[prost(message, optional, tag = "11")]
    pub time_to_first_downstream_tx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Interval between the first downstream byte received and the last downstream byte sent.
    /// Depending on protocol, buffering, windowing, filters, etc. there may be a considerable delta
    /// between ``time_to_last_upstream_rx_byte`` and this field. Note also that this is an approximate
    /// time. In the current implementation it does not include kernel socket buffer time. In the
    /// current implementation it also does not include send window buffering inside the HTTP/2 codec.
    /// In the future it is likely that work will be done to make this duration more accurate.
    #[prost(message, optional, tag = "12")]
    pub time_to_last_downstream_tx_byte: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// The upstream remote/destination address that handles this exchange. This does not include
    /// retries.
    #[prost(message, optional, tag = "13")]
    pub upstream_remote_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    /// The upstream local/origin address that handles this exchange. This does not include retries.
    #[prost(message, optional, tag = "14")]
    pub upstream_local_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    /// The upstream cluster that ``upstream_remote_address`` belongs to.
    #[prost(string, tag = "15")]
    pub upstream_cluster: ::prost::alloc::string::String,
    /// Flags indicating occurrences during request/response processing.
    #[prost(message, optional, tag = "16")]
    pub response_flags: ::core::option::Option<ResponseFlags>,
    /// All metadata encountered during request processing, including endpoint
    /// selection.
    ///
    /// This can be used to associate IDs attached to the various configurations
    /// used to process this request with the access log entry. For example, a
    /// route created from a higher level forwarding rule with some ID can place
    /// that ID in this field and cross reference later. It can also be used to
    /// determine if a canary endpoint was used or not.
    #[prost(message, optional, tag = "17")]
    pub metadata: ::core::option::Option<
        super::super::super::config::core::v3::Metadata,
    >,
    /// If upstream connection failed due to transport socket (e.g. TLS handshake), provides the
    /// failure reason from the transport socket. The format of this field depends on the configured
    /// upstream transport socket. Common TLS failures are in
    /// :ref:`TLS trouble shooting <arch_overview_ssl_trouble_shooting>`.
    #[prost(string, tag = "18")]
    pub upstream_transport_failure_reason: ::prost::alloc::string::String,
    /// The name of the route
    #[prost(string, tag = "19")]
    pub route_name: ::prost::alloc::string::String,
    /// This field is the downstream direct remote address on which the request from the user was
    /// received. Note: This is always the physical peer, even if the remote address is inferred from
    /// for example the x-forwarder-for header, proxy protocol, etc.
    #[prost(message, optional, tag = "20")]
    pub downstream_direct_remote_address: ::core::option::Option<
        super::super::super::config::core::v3::Address,
    >,
    /// Map of filter state in stream info that have been configured to be logged. If the filter
    /// state serialized to any message other than ``google.protobuf.Any`` it will be packed into
    /// ``google.protobuf.Any``.
    #[prost(map = "string, message", tag = "21")]
    pub filter_state_objects: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::super::google::protobuf::Any,
    >,
    /// A list of custom tags, which annotate logs with additional information.
    /// To configure this value, users should configure
    /// :ref:`custom_tags <envoy_v3_api_field_extensions.access_loggers.grpc.v3.CommonGrpcAccessLogConfig.custom_tags>`.
    #[prost(map = "string, string", tag = "22")]
    pub custom_tags: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// For HTTP: Total duration in milliseconds of the request from the start time to the last byte out.
    /// For TCP: Total duration in milliseconds of the downstream connection.
    /// This is the total duration of the request (i.e., when the request's ActiveStream is destroyed)
    /// and may be longer than ``time_to_last_downstream_tx_byte``.
    #[prost(message, optional, tag = "23")]
    pub duration: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// For HTTP: Number of times the request is attempted upstream. Note that the field is omitted when the request was never attempted upstream.
    /// For TCP: Number of times the connection request is attempted upstream. Note that the field is omitted when the connect request was never attempted upstream.
    #[prost(uint32, tag = "24")]
    pub upstream_request_attempt_count: u32,
    /// Connection termination details may provide additional information about why the connection was terminated by Envoy for L4 reasons.
    #[prost(string, tag = "25")]
    pub connection_termination_details: ::prost::alloc::string::String,
    /// Optional unique id of stream (TCP connection, long-live HTTP2 stream, HTTP request) for logging and tracing.
    /// This could be any format string that could be used to identify one stream.
    #[prost(string, tag = "26")]
    pub stream_id: ::prost::alloc::string::String,
    /// If this log entry is final log entry that flushed after the stream completed or
    /// intermediate log entry that flushed periodically during the stream.
    /// There may be multiple intermediate log entries and only one final log entry for each
    /// long-live stream (TCP connection, long-live HTTP2 stream).
    /// And if it is necessary, unique ID or identifier can be added to the log entry
    /// :ref:`stream_id <envoy_v3_api_field_data.accesslog.v3.AccessLogCommon.stream_id>` to
    /// correlate all these intermediate log entries and final log entry.
    ///
    /// .. attention::
    ///
    ///    This field is deprecated in favor of ``access_log_type`` for better indication of the
    ///    type of the access log record.
    #[deprecated]
    #[prost(bool, tag = "27")]
    pub intermediate_log_entry: bool,
    /// If downstream connection in listener failed due to transport socket (e.g. TLS handshake), provides the
    /// failure reason from the transport socket. The format of this field depends on the configured downstream
    /// transport socket. Common TLS failures are in :ref:`TLS trouble shooting <arch_overview_ssl_trouble_shooting>`.
    #[prost(string, tag = "28")]
    pub downstream_transport_failure_reason: ::prost::alloc::string::String,
    /// For HTTP: Total number of bytes sent to the downstream by the http stream.
    /// For TCP: Total number of bytes sent to the downstream by the tcp proxy.
    #[prost(uint64, tag = "29")]
    pub downstream_wire_bytes_sent: u64,
    /// For HTTP: Total number of bytes received from the downstream by the http stream. Envoy over counts sizes of received HTTP/1.1 pipelined requests by adding up bytes of requests in the pipeline to the one currently being processed.
    /// For TCP: Total number of bytes received from the downstream by the tcp proxy.
    #[prost(uint64, tag = "30")]
    pub downstream_wire_bytes_received: u64,
    /// For HTTP: Total number of bytes sent to the upstream by the http stream. This value accumulates during upstream retries.
    /// For TCP: Total number of bytes sent to the upstream by the tcp proxy.
    #[prost(uint64, tag = "31")]
    pub upstream_wire_bytes_sent: u64,
    /// For HTTP: Total number of bytes received from the upstream by the http stream.
    /// For TCP: Total number of bytes sent to the upstream by the tcp proxy.
    #[prost(uint64, tag = "32")]
    pub upstream_wire_bytes_received: u64,
    /// The type of the access log, which indicates when the log was recorded.
    /// See :ref:`ACCESS_LOG_TYPE <config_access_log_format_access_log_type>` for the available values.
    /// In case the access log was recorded by a flow which does not correspond to one of the supported
    /// values, then the default value will be ``NotSet``.
    /// For more information about how access log behaves and when it is being recorded,
    /// please refer to :ref:`access logging <arch_overview_access_logs>`.
    #[prost(enumeration = "AccessLogType", tag = "33")]
    pub access_log_type: i32,
}
/// Flags indicating occurrences during request/response processing.
/// [#next-free-field: 28]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseFlags {
    /// Indicates local server healthcheck failed.
    #[prost(bool, tag = "1")]
    pub failed_local_healthcheck: bool,
    /// Indicates there was no healthy upstream.
    #[prost(bool, tag = "2")]
    pub no_healthy_upstream: bool,
    /// Indicates an there was an upstream request timeout.
    #[prost(bool, tag = "3")]
    pub upstream_request_timeout: bool,
    /// Indicates local codec level reset was sent on the stream.
    #[prost(bool, tag = "4")]
    pub local_reset: bool,
    /// Indicates remote codec level reset was received on the stream.
    #[prost(bool, tag = "5")]
    pub upstream_remote_reset: bool,
    /// Indicates there was a local reset by a connection pool due to an initial connection failure.
    #[prost(bool, tag = "6")]
    pub upstream_connection_failure: bool,
    /// Indicates the stream was reset due to an upstream connection termination.
    #[prost(bool, tag = "7")]
    pub upstream_connection_termination: bool,
    /// Indicates the stream was reset because of a resource overflow.
    #[prost(bool, tag = "8")]
    pub upstream_overflow: bool,
    /// Indicates no route was found for the request.
    #[prost(bool, tag = "9")]
    pub no_route_found: bool,
    /// Indicates that the request was delayed before proxying.
    #[prost(bool, tag = "10")]
    pub delay_injected: bool,
    /// Indicates that the request was aborted with an injected error code.
    #[prost(bool, tag = "11")]
    pub fault_injected: bool,
    /// Indicates that the request was rate-limited locally.
    #[prost(bool, tag = "12")]
    pub rate_limited: bool,
    /// Indicates if the request was deemed unauthorized and the reason for it.
    #[prost(message, optional, tag = "13")]
    pub unauthorized_details: ::core::option::Option<response_flags::Unauthorized>,
    /// Indicates that the request was rejected because there was an error in rate limit service.
    #[prost(bool, tag = "14")]
    pub rate_limit_service_error: bool,
    /// Indicates the stream was reset due to a downstream connection termination.
    #[prost(bool, tag = "15")]
    pub downstream_connection_termination: bool,
    /// Indicates that the upstream retry limit was exceeded, resulting in a downstream error.
    #[prost(bool, tag = "16")]
    pub upstream_retry_limit_exceeded: bool,
    /// Indicates that the stream idle timeout was hit, resulting in a downstream 408.
    #[prost(bool, tag = "17")]
    pub stream_idle_timeout: bool,
    /// Indicates that the request was rejected because an envoy request header failed strict
    /// validation.
    #[prost(bool, tag = "18")]
    pub invalid_envoy_request_headers: bool,
    /// Indicates there was an HTTP protocol error on the downstream request.
    #[prost(bool, tag = "19")]
    pub downstream_protocol_error: bool,
    /// Indicates there was a max stream duration reached on the upstream request.
    #[prost(bool, tag = "20")]
    pub upstream_max_stream_duration_reached: bool,
    /// Indicates the response was served from a cache filter.
    #[prost(bool, tag = "21")]
    pub response_from_cache_filter: bool,
    /// Indicates that a filter configuration is not available.
    #[prost(bool, tag = "22")]
    pub no_filter_config_found: bool,
    /// Indicates that request or connection exceeded the downstream connection duration.
    #[prost(bool, tag = "23")]
    pub duration_timeout: bool,
    /// Indicates there was an HTTP protocol error in the upstream response.
    #[prost(bool, tag = "24")]
    pub upstream_protocol_error: bool,
    /// Indicates no cluster was found for the request.
    #[prost(bool, tag = "25")]
    pub no_cluster_found: bool,
    /// Indicates overload manager terminated the request.
    #[prost(bool, tag = "26")]
    pub overload_manager: bool,
    /// Indicates a DNS resolution failed.
    #[prost(bool, tag = "27")]
    pub dns_resolution_failure: bool,
}
/// Nested message and enum types in `ResponseFlags`.
pub mod response_flags {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Unauthorized {
        #[prost(enumeration = "unauthorized::Reason", tag = "1")]
        pub reason: i32,
    }
    /// Nested message and enum types in `Unauthorized`.
    pub mod unauthorized {
        /// Reasons why the request was unauthorized
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
        pub enum Reason {
            Unspecified = 0,
            /// The request was denied by the external authorization service.
            ExternalService = 1,
        }
        impl Reason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Reason::Unspecified => "REASON_UNSPECIFIED",
                    Reason::ExternalService => "EXTERNAL_SERVICE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                    "EXTERNAL_SERVICE" => Some(Self::ExternalService),
                    _ => None,
                }
            }
        }
    }
}
/// Properties of a negotiated TLS connection.
/// [#next-free-field: 8]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsProperties {
    /// Version of TLS that was negotiated.
    #[prost(enumeration = "tls_properties::TlsVersion", tag = "1")]
    pub tls_version: i32,
    /// TLS cipher suite negotiated during handshake. The value is a
    /// four-digit hex code defined by the IANA TLS Cipher Suite Registry
    /// (e.g. ``009C`` for ``TLS_RSA_WITH_AES_128_GCM_SHA256``).
    ///
    /// Here it is expressed as an integer.
    #[prost(message, optional, tag = "2")]
    pub tls_cipher_suite: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// SNI hostname from handshake.
    #[prost(string, tag = "3")]
    pub tls_sni_hostname: ::prost::alloc::string::String,
    /// Properties of the local certificate used to negotiate TLS.
    #[prost(message, optional, tag = "4")]
    pub local_certificate_properties: ::core::option::Option<
        tls_properties::CertificateProperties,
    >,
    /// Properties of the peer certificate used to negotiate TLS.
    #[prost(message, optional, tag = "5")]
    pub peer_certificate_properties: ::core::option::Option<
        tls_properties::CertificateProperties,
    >,
    /// The TLS session ID.
    #[prost(string, tag = "6")]
    pub tls_session_id: ::prost::alloc::string::String,
    /// The ``JA3`` fingerprint when ``JA3`` fingerprinting is enabled.
    #[prost(string, tag = "7")]
    pub ja3_fingerprint: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TLSProperties`.
pub mod tls_properties {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CertificateProperties {
        /// SANs present in the certificate.
        #[prost(message, repeated, tag = "1")]
        pub subject_alt_name: ::prost::alloc::vec::Vec<
            certificate_properties::SubjectAltName,
        >,
        /// The subject field of the certificate.
        #[prost(string, tag = "2")]
        pub subject: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `CertificateProperties`.
    pub mod certificate_properties {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SubjectAltName {
            #[prost(oneof = "subject_alt_name::San", tags = "1, 2")]
            pub san: ::core::option::Option<subject_alt_name::San>,
        }
        /// Nested message and enum types in `SubjectAltName`.
        pub mod subject_alt_name {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum San {
                #[prost(string, tag = "1")]
                Uri(::prost::alloc::string::String),
                /// \[#not-implemented-hide:\]
                #[prost(string, tag = "2")]
                Dns(::prost::alloc::string::String),
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
    pub enum TlsVersion {
        VersionUnspecified = 0,
        TlSv1 = 1,
        TlSv11 = 2,
        TlSv12 = 3,
        TlSv13 = 4,
    }
    impl TlsVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TlsVersion::VersionUnspecified => "VERSION_UNSPECIFIED",
                TlsVersion::TlSv1 => "TLSv1",
                TlsVersion::TlSv11 => "TLSv1_1",
                TlsVersion::TlSv12 => "TLSv1_2",
                TlsVersion::TlSv13 => "TLSv1_3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VERSION_UNSPECIFIED" => Some(Self::VersionUnspecified),
                "TLSv1" => Some(Self::TlSv1),
                "TLSv1_1" => Some(Self::TlSv11),
                "TLSv1_2" => Some(Self::TlSv12),
                "TLSv1_3" => Some(Self::TlSv13),
                _ => None,
            }
        }
    }
}
/// [#next-free-field: 16]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequestProperties {
    /// The request method (RFC 7231/2616).
    #[prost(
        enumeration = "super::super::super::config::core::v3::RequestMethod",
        tag = "1"
    )]
    pub request_method: i32,
    /// The scheme portion of the incoming request URI.
    #[prost(string, tag = "2")]
    pub scheme: ::prost::alloc::string::String,
    /// HTTP/2 ``:authority`` or HTTP/1.1 ``Host`` header value.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
    /// The port of the incoming request URI
    /// (unused currently, as port is composed onto authority).
    #[prost(message, optional, tag = "4")]
    pub port: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// The path portion from the incoming request URI.
    #[prost(string, tag = "5")]
    pub path: ::prost::alloc::string::String,
    /// Value of the ``User-Agent`` request header.
    #[prost(string, tag = "6")]
    pub user_agent: ::prost::alloc::string::String,
    /// Value of the ``Referer`` request header.
    #[prost(string, tag = "7")]
    pub referer: ::prost::alloc::string::String,
    /// Value of the ``X-Forwarded-For`` request header.
    #[prost(string, tag = "8")]
    pub forwarded_for: ::prost::alloc::string::String,
    /// Value of the ``X-Request-Id`` request header
    ///
    /// This header is used by Envoy to uniquely identify a request.
    /// It will be generated for all external requests and internal requests that
    /// do not already have a request ID.
    #[prost(string, tag = "9")]
    pub request_id: ::prost::alloc::string::String,
    /// Value of the ``X-Envoy-Original-Path`` request header.
    #[prost(string, tag = "10")]
    pub original_path: ::prost::alloc::string::String,
    /// Size of the HTTP request headers in bytes.
    ///
    /// This value is captured from the OSI layer 7 perspective, i.e. it does not
    /// include overhead from framing or encoding at other networking layers.
    #[prost(uint64, tag = "11")]
    pub request_headers_bytes: u64,
    /// Size of the HTTP request body in bytes.
    ///
    /// This value is captured from the OSI layer 7 perspective, i.e. it does not
    /// include overhead from framing or encoding at other networking layers.
    #[prost(uint64, tag = "12")]
    pub request_body_bytes: u64,
    /// Map of additional headers that have been configured to be logged.
    #[prost(map = "string, string", tag = "13")]
    pub request_headers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Number of header bytes sent to the upstream by the http stream, including protocol overhead.
    ///
    /// This value accumulates during upstream retries.
    #[prost(uint64, tag = "14")]
    pub upstream_header_bytes_sent: u64,
    /// Number of header bytes received from the downstream by the http stream, including protocol overhead.
    #[prost(uint64, tag = "15")]
    pub downstream_header_bytes_received: u64,
}
/// [#next-free-field: 9]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpResponseProperties {
    /// The HTTP response code returned by Envoy.
    #[prost(message, optional, tag = "1")]
    pub response_code: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Size of the HTTP response headers in bytes.
    ///
    /// This value is captured from the OSI layer 7 perspective, i.e. it does not
    /// include protocol overhead or overhead from framing or encoding at other networking layers.
    #[prost(uint64, tag = "2")]
    pub response_headers_bytes: u64,
    /// Size of the HTTP response body in bytes.
    ///
    /// This value is captured from the OSI layer 7 perspective, i.e. it does not
    /// include overhead from framing or encoding at other networking layers.
    #[prost(uint64, tag = "3")]
    pub response_body_bytes: u64,
    /// Map of additional headers configured to be logged.
    #[prost(map = "string, string", tag = "4")]
    pub response_headers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Map of trailers configured to be logged.
    #[prost(map = "string, string", tag = "5")]
    pub response_trailers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The HTTP response code details.
    #[prost(string, tag = "6")]
    pub response_code_details: ::prost::alloc::string::String,
    /// Number of header bytes received from the upstream by the http stream, including protocol overhead.
    #[prost(uint64, tag = "7")]
    pub upstream_header_bytes_received: u64,
    /// Number of header bytes sent to the downstream by the http stream, including protocol overhead.
    #[prost(uint64, tag = "8")]
    pub downstream_header_bytes_sent: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessLogType {
    NotSet = 0,
    TcpUpstreamConnected = 1,
    TcpPeriodic = 2,
    TcpConnectionEnd = 3,
    DownstreamStart = 4,
    DownstreamPeriodic = 5,
    DownstreamEnd = 6,
    UpstreamPoolReady = 7,
    UpstreamPeriodic = 8,
    UpstreamEnd = 9,
    DownstreamTunnelSuccessfullyEstablished = 10,
}
impl AccessLogType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessLogType::NotSet => "NotSet",
            AccessLogType::TcpUpstreamConnected => "TcpUpstreamConnected",
            AccessLogType::TcpPeriodic => "TcpPeriodic",
            AccessLogType::TcpConnectionEnd => "TcpConnectionEnd",
            AccessLogType::DownstreamStart => "DownstreamStart",
            AccessLogType::DownstreamPeriodic => "DownstreamPeriodic",
            AccessLogType::DownstreamEnd => "DownstreamEnd",
            AccessLogType::UpstreamPoolReady => "UpstreamPoolReady",
            AccessLogType::UpstreamPeriodic => "UpstreamPeriodic",
            AccessLogType::UpstreamEnd => "UpstreamEnd",
            AccessLogType::DownstreamTunnelSuccessfullyEstablished => {
                "DownstreamTunnelSuccessfullyEstablished"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NotSet" => Some(Self::NotSet),
            "TcpUpstreamConnected" => Some(Self::TcpUpstreamConnected),
            "TcpPeriodic" => Some(Self::TcpPeriodic),
            "TcpConnectionEnd" => Some(Self::TcpConnectionEnd),
            "DownstreamStart" => Some(Self::DownstreamStart),
            "DownstreamPeriodic" => Some(Self::DownstreamPeriodic),
            "DownstreamEnd" => Some(Self::DownstreamEnd),
            "UpstreamPoolReady" => Some(Self::UpstreamPoolReady),
            "UpstreamPeriodic" => Some(Self::UpstreamPeriodic),
            "UpstreamEnd" => Some(Self::UpstreamEnd),
            "DownstreamTunnelSuccessfullyEstablished" => {
                Some(Self::DownstreamTunnelSuccessfullyEstablished)
            }
            _ => None,
        }
    }
}
