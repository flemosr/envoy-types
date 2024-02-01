#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// Indicates the upstream cluster to which the request should be routed.
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
}
/// Configuration for the UDP proxy filter.
/// \[\#next-free-field: 14\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UdpProxyConfig {
    /// The stat prefix used when emitting UDP proxy filter stats.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// The idle timeout for sessions. Idle is defined as no datagrams between received or sent by
    /// the session. The default if not specified is 1 minute.
    #[prost(message, optional, tag = "3")]
    pub idle_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Use the remote downstream IP address as the sender IP address when sending packets to upstream hosts.
    /// This option requires Envoy to be run with the `CAP_NET_ADMIN` capability on Linux.
    /// And the IPv6 stack must be enabled on Linux kernel.
    /// This option does not preserve the remote downstream port.
    /// If this option is enabled, the IP address of sent datagrams will be changed to the remote downstream IP address.
    /// This means that Envoy will not receive packets that are sent by upstream hosts because the upstream hosts
    /// will send the packets with the remote downstream IP address as the destination. All packets will be routed
    /// to the remote downstream directly if there are route rules on the upstream host side.
    /// There are two options to return the packets back to the remote downstream.
    /// The first one is to use DSR (Direct Server Return).
    /// The other one is to configure routing rules on the upstream hosts to forward
    /// all packets back to Envoy and configure iptables rules on the host running Envoy to
    /// forward all packets from upstream hosts to the Envoy process so that Envoy can forward the packets to the downstream.
    /// If the platform does not support this option, Envoy will raise a configuration error.
    #[prost(bool, tag = "4")]
    pub use_original_src_ip: bool,
    /// Optional configuration for UDP proxy hash policies. If hash_policies is not set, the hash-based
    /// load balancing algorithms will select a host randomly. Currently the number of hash policies is
    /// limited to 1.
    #[prost(message, repeated, tag = "5")]
    pub hash_policies: ::prost::alloc::vec::Vec<udp_proxy_config::HashPolicy>,
    /// UDP socket configuration for upstream sockets. The default for
    /// :ref:`prefer_gro <envoy_v3_api_field_config.core.v3.UdpSocketConfig.prefer_gro>` is true for upstream
    /// sockets as the assumption is datagrams will be received from a single source.
    #[prost(message, optional, tag = "6")]
    pub upstream_socket_config: ::core::option::Option<
        super::super::super::super::super::config::core::v3::UdpSocketConfig,
    >,
    /// Perform per packet load balancing (upstream host selection) on each received data chunk.
    /// The default if not specified is false, that means each data chunk is forwarded
    /// to upstream host selected on first chunk receival for that "session" (identified by source IP/port and local IP/port).
    /// Only one of use_per_packet_load_balancing or session_filters can be used.
    #[prost(bool, tag = "7")]
    pub use_per_packet_load_balancing: bool,
    /// Configuration for session access logs emitted by the UDP proxy. Note that certain UDP specific data is emitted as :ref:`Dynamic Metadata <config_access_log_format_dynamic_metadata>`.
    #[prost(message, repeated, tag = "8")]
    pub access_log: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::accesslog::v3::AccessLog,
    >,
    /// Configuration for proxy access logs emitted by the UDP proxy. Note that certain UDP specific data is emitted as :ref:`Dynamic Metadata <config_access_log_format_dynamic_metadata>`.
    #[prost(message, repeated, tag = "10")]
    pub proxy_access_log: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::accesslog::v3::AccessLog,
    >,
    /// Optional session filters that will run for each UDP session.
    /// Only one of use_per_packet_load_balancing or session_filters can be used.
    /// \[\#extension-category: envoy.filters.udp.session\]
    #[prost(message, repeated, tag = "11")]
    pub session_filters: ::prost::alloc::vec::Vec<udp_proxy_config::SessionFilter>,
    /// If set, this configures UDP tunneling. See `Proxying UDP in HTTP <<https://www.rfc-editor.org/rfc/rfc9298.html>`\_.>
    /// More information can be found in the UDP Proxy and HTTP upgrade documentation.
    #[prost(message, optional, tag = "12")]
    pub tunneling_config: ::core::option::Option<udp_proxy_config::UdpTunnelingConfig>,
    /// Additional access log options for UDP Proxy.
    #[prost(message, optional, tag = "13")]
    pub access_log_options: ::core::option::Option<
        udp_proxy_config::UdpAccessLogOptions,
    >,
    #[prost(oneof = "udp_proxy_config::RouteSpecifier", tags = "2, 9")]
    pub route_specifier: ::core::option::Option<udp_proxy_config::RouteSpecifier>,
}
/// Nested message and enum types in `UdpProxyConfig`.
pub mod udp_proxy_config {
    /// Specifies the UDP hash policy.
    /// The packets can be routed by hash policy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HashPolicy {
        #[prost(oneof = "hash_policy::PolicySpecifier", tags = "1, 2")]
        pub policy_specifier: ::core::option::Option<hash_policy::PolicySpecifier>,
    }
    /// Nested message and enum types in `HashPolicy`.
    pub mod hash_policy {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PolicySpecifier {
            /// The source IP will be used to compute the hash used by hash-based load balancing algorithms.
            #[prost(bool, tag = "1")]
            SourceIp(bool),
            /// A given key will be used to compute the hash used by hash-based load balancing algorithms.
            /// In certain cases there is a need to direct different UDP streams jointly towards the selected set of endpoints.
            /// A possible use-case is VoIP telephony, where media (RTP) and its corresponding control (RTCP) belong to the same logical session,
            /// although they travel in separate streams. To ensure that these pair of streams are load-balanced on session level
            /// (instead of individual stream level), dynamically created listeners can use the same hash key for each stream in the session.
            #[prost(string, tag = "2")]
            Key(::prost::alloc::string::String),
        }
    }
    /// Configuration for UDP session filters.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SessionFilter {
        /// The name of the filter configuration.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(oneof = "session_filter::ConfigType", tags = "2")]
        pub config_type: ::core::option::Option<session_filter::ConfigType>,
    }
    /// Nested message and enum types in `SessionFilter`.
    pub mod session_filter {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigType {
            /// Filter specific configuration which depends on the filter being
            /// instantiated. See the supported filters for further documentation.
            #[prost(message, tag = "2")]
            TypedConfig(
                super::super::super::super::super::super::super::super::google::protobuf::Any,
            ),
        }
    }
    /// Configuration for tunneling UDP over other transports or application layers.
    /// Tunneling is currently supported over HTTP/2.
    /// \[\#next-free-field: 12\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UdpTunnelingConfig {
        /// The hostname to send in the synthesized CONNECT headers to the upstream proxy.
        /// This field evaluates command operators if set, otherwise returns hostname as is.
        ///
        /// Example: dynamically set hostname using filter state
        ///
        /// .. code-block:: yaml
        ///
        /// ```text
        /// tunneling_config:
        ///   proxy_host: "%FILTER_STATE(proxy.host.key:PLAIN)%"
        /// ```
        #[prost(string, tag = "1")]
        pub proxy_host: ::prost::alloc::string::String,
        /// Optional port value to add to the HTTP request URI.
        /// This value can be overridden per-session by setting the required port value for
        /// the filter state key `udp.connect.proxy_port`.
        #[prost(message, optional, tag = "2")]
        pub proxy_port: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// The target host to send in the synthesized CONNECT headers to the upstream proxy.
        /// This field evaluates command operators if set, otherwise returns hostname as is.
        ///
        /// Example: dynamically set target host using filter state
        ///
        /// .. code-block:: yaml
        ///
        /// ```text
        /// tunneling_config:
        ///   target_host: "%FILTER_STATE(target.host.key:PLAIN)%"
        /// ```
        #[prost(string, tag = "3")]
        pub target_host: ::prost::alloc::string::String,
        /// The default target port to send in the CONNECT headers to the upstream proxy.
        /// This value can be overridden per-session by setting the required port value for
        /// the filter state key `udp.connect.target_port`.
        #[prost(uint32, tag = "4")]
        pub default_target_port: u32,
        /// Use POST method instead of CONNECT method to tunnel the UDP stream.
        ///
        /// .. note::
        /// If use_post is set, the upstream stream does not comply with the connect-udp RFC, and
        /// instead it will be a POST request. the path used in the headers will be set from the
        /// post_path field, and the headers will not contain the target host and target port, as
        /// required by the connect-udp protocol. This flag should be used carefully.
        #[prost(bool, tag = "5")]
        pub use_post: bool,
        /// The path used with POST method. Default path is `/`. If post path is specified and
        /// use_post field isn't true, it will be rejected.
        #[prost(string, tag = "6")]
        pub post_path: ::prost::alloc::string::String,
        /// Optional retry options, in case connecting to the upstream failed.
        #[prost(message, optional, tag = "7")]
        pub retry_options: ::core::option::Option<udp_tunneling_config::RetryOptions>,
        /// Additional request headers to upstream proxy. Neither `:-prefixed` pseudo-headers
        /// nor the Host: header can be overridden. Values of the added headers evaluates command
        /// operators if they are set in the value template.
        ///
        /// Example: dynamically set a header with the local port
        ///
        /// .. code-block:: yaml
        ///
        /// ```text
        /// headers_to_add:
        /// - header:
        ///     key: original_dst_port
        ///     value: "%DOWNSTREAM_LOCAL_PORT%"
        /// ```
        #[prost(message, repeated, tag = "8")]
        pub headers_to_add: ::prost::alloc::vec::Vec<
            super::super::super::super::super::super::config::core::v3::HeaderValueOption,
        >,
        /// If configured, the filter will buffer datagrams in case that it is waiting for the upstream to be
        /// ready, whether if it is during the connection process or due to upstream buffer watermarks.
        /// If this field is not configured, there will be no buffering and downstream datagrams that arrive
        /// while the upstream is not ready will be dropped. In case this field is set but the options
        /// are not configured, the default values will be applied as described in the `BufferOptions`.
        #[prost(message, optional, tag = "9")]
        pub buffer_options: ::core::option::Option<udp_tunneling_config::BufferOptions>,
        /// Save the response headers to the downstream info filter state for consumption
        /// by the session filters. The filter state key is `envoy.udp_proxy.propagate_response_headers`.
        #[prost(bool, tag = "10")]
        pub propagate_response_headers: bool,
        /// Save the response trailers to the downstream info filter state for consumption
        /// by the session filters. The filter state key is `envoy.udp_proxy.propagate_response_trailers`.
        #[prost(bool, tag = "11")]
        pub propagate_response_trailers: bool,
    }
    /// Nested message and enum types in `UdpTunnelingConfig`.
    pub mod udp_tunneling_config {
        /// Configuration for UDP datagrams buffering.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BufferOptions {
            /// If set, the filter will only buffer datagrams up to the requested limit, and will drop
            /// new UDP datagrams if the buffer contains the max_buffered_datagrams value at the time
            /// of a new datagram arrival. If not set, the default value is 1024 datagrams.
            #[prost(message, optional, tag = "1")]
            pub max_buffered_datagrams: ::core::option::Option<
                super::super::super::super::super::super::super::super::google::protobuf::UInt32Value,
            >,
            /// If set, the filter will only buffer datagrams up to the requested total buffered bytes limit,
            /// and will drop new UDP datagrams if the buffer contains the max_buffered_datagrams value
            /// at the time of a new datagram arrival. If not set, the default value is 16,384 (16KB).
            #[prost(message, optional, tag = "2")]
            pub max_buffered_bytes: ::core::option::Option<
                super::super::super::super::super::super::super::super::google::protobuf::UInt64Value,
            >,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RetryOptions {
            /// The maximum number of unsuccessful connection attempts that will be made before giving up.
            /// If the parameter is not specified, 1 connection attempt will be made.
            #[prost(message, optional, tag = "1")]
            pub max_connect_attempts: ::core::option::Option<
                super::super::super::super::super::super::super::super::google::protobuf::UInt32Value,
            >,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UdpAccessLogOptions {
        /// The interval to flush access log. The UDP proxy will flush only one access log when the session
        /// is ended by default. If this field is set, the UDP proxy will flush access log periodically with
        /// the specified interval.
        /// This field does not require on-tunnel-connected access logging enabled, and the other way around.
        /// The interval must be at least 1ms.
        #[prost(message, optional, tag = "1")]
        pub access_log_flush_interval: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
        /// If set to true and UDP tunneling is configured, access log will be flushed when the UDP proxy has successfully
        /// established a connection tunnel with the upstream. If the connection failed, the access log will not be flushed.
        #[prost(bool, tag = "2")]
        pub flush_access_log_on_tunnel_connected: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RouteSpecifier {
        /// The upstream cluster to connect to.
        /// This field is deprecated in favor of
        /// :ref:`matcher <envoy_v3_api_field_extensions.filters.udp.udp_proxy.v3.UdpProxyConfig.matcher>`.
        #[prost(string, tag = "2")]
        Cluster(::prost::alloc::string::String),
        /// The match tree to use when resolving route actions for incoming requests.
        /// See :ref:`Routing <config_udp_listener_filters_udp_proxy_routing>` for more information.
        #[prost(message, tag = "9")]
        Matcher(
            super::super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
        ),
    }
}
