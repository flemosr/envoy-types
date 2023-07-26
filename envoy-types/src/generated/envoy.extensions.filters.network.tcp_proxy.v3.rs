/// [#next-free-field: 18]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcpProxy {
    /// The prefix to use when emitting :ref:`statistics
    /// <config_network_filters_tcp_proxy_stats>`.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// The on demand policy for the upstream cluster.
    /// It applies to both
    /// :ref:`TcpProxy.cluster <envoy_v3_api_field_extensions.filters.network.tcp_proxy.v3.TcpProxy.cluster>`
    /// and
    /// :ref:`TcpProxy.weighted_clusters <envoy_v3_api_field_extensions.filters.network.tcp_proxy.v3.TcpProxy.weighted_clusters>`.
    #[prost(message, optional, tag = "14")]
    pub on_demand: ::core::option::Option<tcp_proxy::OnDemand>,
    /// Optional endpoint metadata match criteria. Only endpoints in the upstream
    /// cluster with metadata matching that set in metadata_match will be
    /// considered. The filter name should be specified as ``envoy.lb``.
    #[prost(message, optional, tag = "9")]
    pub metadata_match: ::core::option::Option<
        super::super::super::super::super::config::core::v3::Metadata,
    >,
    /// The idle timeout for connections managed by the TCP proxy filter. The idle timeout
    /// is defined as the period in which there are no bytes sent or received on either
    /// the upstream or downstream connection. If not set, the default idle timeout is 1 hour. If set
    /// to 0s, the timeout will be disabled.
    ///
    /// .. warning::
    ///    Disabling this timeout has a highly likelihood of yielding connection leaks due to lost TCP
    ///    FIN packets, etc.
    #[prost(message, optional, tag = "8")]
    pub idle_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// \[#not-implemented-hide:\] The idle timeout for connections managed by the TCP proxy
    /// filter. The idle timeout is defined as the period in which there is no
    /// active traffic. If not set, there is no idle timeout. When the idle timeout
    /// is reached the connection will be closed. The distinction between
    /// downstream_idle_timeout/upstream_idle_timeout provides a means to set
    /// timeout based on the last byte sent on the downstream/upstream connection.
    #[prost(message, optional, tag = "3")]
    pub downstream_idle_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// \[#not-implemented-hide:\]
    #[prost(message, optional, tag = "4")]
    pub upstream_idle_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Configuration for :ref:`access logs <arch_overview_access_logs>`
    /// emitted by the this tcp_proxy.
    #[prost(message, repeated, tag = "5")]
    pub access_log: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::accesslog::v3::AccessLog,
    >,
    /// The maximum number of unsuccessful connection attempts that will be made before
    /// giving up. If the parameter is not specified, 1 connection attempt will be made.
    #[prost(message, optional, tag = "7")]
    pub max_connect_attempts: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Optional configuration for TCP proxy hash policy. If hash_policy is not set, the hash-based
    /// load balancing algorithms will select a host randomly. Currently the number of hash policies is
    /// limited to 1.
    #[prost(message, repeated, tag = "11")]
    pub hash_policy: ::prost::alloc::vec::Vec<
        super::super::super::super::super::r#type::v3::HashPolicy,
    >,
    /// If set, this configures tunneling, e.g. configuration options to tunnel TCP payload over
    /// HTTP CONNECT. If this message is absent, the payload will be proxied upstream as per usual.
    /// It is possible to dynamically override this configuration and disable tunneling per connection,
    /// by setting a per-connection filter state object for the key ``envoy.tcp_proxy.disable_tunneling``.
    #[prost(message, optional, tag = "12")]
    pub tunneling_config: ::core::option::Option<tcp_proxy::TunnelingConfig>,
    /// The maximum duration of a connection. The duration is defined as the period since a connection
    /// was established. If not set, there is no max duration. When max_downstream_connection_duration
    /// is reached the connection will be closed. Duration must be at least 1ms.
    #[prost(message, optional, tag = "13")]
    pub max_downstream_connection_duration: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// .. attention::
    /// This field is deprecated in favor of
    /// :ref:`access_log_flush_interval
    /// <envoy_v3_api_field_extensions.filters.network.tcp_proxy.v3.TcpProxy.TcpAccessLogOptions.access_log_flush_interval>`.
    /// Note that if both this field and :ref:`access_log_flush_interval
    /// <envoy_v3_api_field_extensions.filters.network.tcp_proxy.v3.TcpProxy.TcpAccessLogOptions.access_log_flush_interval>`
    /// are specified, the former (deprecated field) is ignored.
    #[deprecated]
    #[prost(message, optional, tag = "15")]
    pub access_log_flush_interval: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// .. attention::
    /// This field is deprecated in favor of
    /// :ref:`flush_access_log_on_connected
    /// <envoy_v3_api_field_extensions.filters.network.tcp_proxy.v3.TcpProxy.TcpAccessLogOptions.flush_access_log_on_connected>`.
    /// Note that if both this field and :ref:`flush_access_log_on_connected
    /// <envoy_v3_api_field_extensions.filters.network.tcp_proxy.v3.TcpProxy.TcpAccessLogOptions.flush_access_log_on_connected>`
    /// are specified, the former (deprecated field) is ignored.
    #[deprecated]
    #[prost(bool, tag = "16")]
    pub flush_access_log_on_connected: bool,
    /// Additional access log options for TCP Proxy.
    #[prost(message, optional, tag = "17")]
    pub access_log_options: ::core::option::Option<tcp_proxy::TcpAccessLogOptions>,
    #[prost(oneof = "tcp_proxy::ClusterSpecifier", tags = "2, 10")]
    pub cluster_specifier: ::core::option::Option<tcp_proxy::ClusterSpecifier>,
}
/// Nested message and enum types in `TcpProxy`.
pub mod tcp_proxy {
    /// Allows for specification of multiple upstream clusters along with weights
    /// that indicate the percentage of traffic to be forwarded to each cluster.
    /// The router selects an upstream cluster based on these weights.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WeightedCluster {
        /// Specifies one or more upstream clusters associated with the route.
        #[prost(message, repeated, tag = "1")]
        pub clusters: ::prost::alloc::vec::Vec<weighted_cluster::ClusterWeight>,
    }
    /// Nested message and enum types in `WeightedCluster`.
    pub mod weighted_cluster {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ClusterWeight {
            /// Name of the upstream cluster.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            /// When a request matches the route, the choice of an upstream cluster is
            /// determined by its weight. The sum of weights across all entries in the
            /// clusters array determines the total weight.
            #[prost(uint32, tag = "2")]
            pub weight: u32,
            /// Optional endpoint metadata match criteria used by the subset load balancer. Only endpoints
            /// in the upstream cluster with metadata matching what is set in this field will be considered
            /// for load balancing. Note that this will be merged with what's provided in
            /// :ref:`TcpProxy.metadata_match
            /// <envoy_v3_api_field_extensions.filters.network.tcp_proxy.v3.TcpProxy.metadata_match>`, with values
            /// here taking precedence. The filter name should be specified as ``envoy.lb``.
            #[prost(message, optional, tag = "3")]
            pub metadata_match: ::core::option::Option<
                super::super::super::super::super::super::super::config::core::v3::Metadata,
            >,
        }
    }
    /// Configuration for tunneling TCP over other transports or application layers.
    /// Tunneling is supported over both HTTP/1.1 and HTTP/2. Upstream protocol is
    /// determined by the cluster configuration.
    /// [#next-free-field: 7]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TunnelingConfig {
        /// The hostname to send in the synthesized CONNECT headers to the upstream proxy.
        /// This field evaluates command operators if set, otherwise returns hostname as is.
        ///
        /// Example: dynamically set hostname using downstream SNI
        ///
        /// .. code-block:: yaml
        ///
        ///     tunneling_config:
        ///       hostname: "%REQUESTED_SERVER_NAME%:443"
        ///
        /// Example: dynamically set hostname using dynamic metadata
        ///
        /// .. code-block:: yaml
        ///
        ///     tunneling_config:
        ///       hostname: "%DYNAMIC_METADATA(tunnel:address)%"
        ///
        #[prost(string, tag = "1")]
        pub hostname: ::prost::alloc::string::String,
        /// Use POST method instead of CONNECT method to tunnel the TCP stream.
        /// The 'protocol: bytestream' header is also NOT set for HTTP/2 to comply with the spec.
        ///
        /// The upstream proxy is expected to convert POST payload as raw TCP.
        #[prost(bool, tag = "2")]
        pub use_post: bool,
        /// Additional request headers to upstream proxy. This is mainly used to
        /// trigger upstream to convert POST requests back to CONNECT requests.
        ///
        /// Neither ``:-prefixed`` pseudo-headers nor the Host: header can be overridden.
        #[prost(message, repeated, tag = "3")]
        pub headers_to_add: ::prost::alloc::vec::Vec<
            super::super::super::super::super::super::config::core::v3::HeaderValueOption,
        >,
        /// Save the response headers to the downstream info filter state for consumption
        /// by the network filters. The filter state key is ``envoy.tcp_proxy.propagate_response_headers``.
        #[prost(bool, tag = "4")]
        pub propagate_response_headers: bool,
        /// The path used with POST method. Default path is ``/``. If post path is specified and
        /// :ref:`use_post field <envoy_v3_api_field_extensions.filters.network.tcp_proxy.v3.TcpProxy.TunnelingConfig.use_post>`
        /// isn't true, it will be rejected.
        #[prost(string, tag = "5")]
        pub post_path: ::prost::alloc::string::String,
        /// Save the response trailers to the downstream info filter state for consumption
        /// by the network filters. The filter state key is ``envoy.tcp_proxy.propagate_response_trailers``.
        #[prost(bool, tag = "6")]
        pub propagate_response_trailers: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OnDemand {
        /// An optional configuration for on-demand cluster discovery
        /// service. If not specified, the on-demand cluster discovery will
        /// be disabled. When it's specified, the filter will pause a request
        /// to an unknown cluster and will begin a cluster discovery
        /// process. When the discovery is finished (successfully or not),
        /// the request will be resumed.
        #[prost(message, optional, tag = "1")]
        pub odcds_config: ::core::option::Option<
            super::super::super::super::super::super::config::core::v3::ConfigSource,
        >,
        /// xdstp:// resource locator for on-demand cluster collection.
        /// \[#not-implemented-hide:\]
        #[prost(string, tag = "2")]
        pub resources_locator: ::prost::alloc::string::String,
        /// The timeout for on demand cluster lookup. If the CDS cannot return the required cluster,
        /// the downstream request will be closed with the error code detail NO_CLUSTER_FOUND.
        /// \[#not-implemented-hide:\]
        #[prost(message, optional, tag = "3")]
        pub timeout: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TcpAccessLogOptions {
        /// The interval to flush access log. The TCP proxy will flush only one access log when the connection
        /// is closed by default. If this field is set, the TCP proxy will flush access log periodically with
        /// the specified interval.
        /// The interval must be at least 1ms.
        #[prost(message, optional, tag = "1")]
        pub access_log_flush_interval: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
        /// If set to true, access log will be flushed when the TCP proxy has successfully established a
        /// connection with the upstream. If the connection failed, the access log will not be flushed.
        #[prost(bool, tag = "2")]
        pub flush_access_log_on_connected: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterSpecifier {
        /// The upstream cluster to connect to.
        #[prost(string, tag = "2")]
        Cluster(::prost::alloc::string::String),
        /// Multiple upstream clusters can be specified for a given route. The
        /// request is routed to one of the upstream clusters based on weights
        /// assigned to each cluster.
        #[prost(message, tag = "10")]
        WeightedClusters(WeightedCluster),
    }
}
