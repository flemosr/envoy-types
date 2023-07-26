#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Route {
    /// Indicates the upstream cluster to which the request should be routed.
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
}
/// Configuration for the UDP proxy filter.
/// \[\#next-free-field: 11\]
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
