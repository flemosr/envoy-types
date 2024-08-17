/// Bootstrap :ref:`configuration overview <config_overview_bootstrap>`.
/// \[\#next-free-field: 42\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bootstrap {
    /// Node identity to present to the management server and for instance
    /// identification purposes (e.g. in generated headers).
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<super::super::core::v3::Node>,
    /// A list of :ref:`Node <envoy_v3_api_msg_config.core.v3.Node>` field names
    /// that will be included in the context parameters of the effective
    /// xdstp:// URL that is sent in a discovery request when resource
    /// locators are used for LDS/CDS. Any non-string field will have its JSON
    /// encoding set as the context parameter value, with the exception of
    /// metadata, which will be flattened (see example below). The supported field
    /// names are:
    ///
    /// * "cluster"
    /// * "id"
    /// * "locality.region"
    /// * "locality.sub_zone"
    /// * "locality.zone"
    /// * "metadata"
    /// * "user_agent_build_version.metadata"
    /// * "user_agent_build_version.version"
    /// * "user_agent_name"
    /// * "user_agent_version"
    ///
    /// The node context parameters act as a base layer dictionary for the context
    /// parameters (i.e. more specific resource specific context parameters will
    /// override). Field names will be prefixed with “udpa.node.” when included in
    /// context parameters.
    ///
    /// For example, if node_context_params is `\["user_agent_name", "metadata"\]`,
    /// the implied context parameters might be::
    ///
    /// node.user_agent_name: "envoy"
    /// node.metadata.foo: "{"bar": "baz"}"
    /// node.metadata.some: "42"
    /// node.metadata.thing: ""thing""
    ///
    /// \[\#not-implemented-hide:\]
    #[prost(string, repeated, tag = "26")]
    pub node_context_params: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Statically specified resources.
    #[prost(message, optional, tag = "2")]
    pub static_resources: ::core::option::Option<bootstrap::StaticResources>,
    /// xDS configuration sources.
    #[prost(message, optional, tag = "3")]
    pub dynamic_resources: ::core::option::Option<bootstrap::DynamicResources>,
    /// Configuration for the cluster manager which owns all upstream clusters
    /// within the server.
    #[prost(message, optional, tag = "4")]
    pub cluster_manager: ::core::option::Option<ClusterManager>,
    /// Health discovery service config option.
    /// (:ref:`core.ApiConfigSource <envoy_v3_api_msg_config.core.v3.ApiConfigSource>`)
    #[prost(message, optional, tag = "14")]
    pub hds_config: ::core::option::Option<super::super::core::v3::ApiConfigSource>,
    /// Optional file system path to search for startup flag files.
    #[prost(string, tag = "5")]
    pub flags_path: ::prost::alloc::string::String,
    /// Optional set of stats sinks.
    #[prost(message, repeated, tag = "6")]
    pub stats_sinks: ::prost::alloc::vec::Vec<super::super::metrics::v3::StatsSink>,
    /// Options to control behaviors of deferred creation compatible stats.
    #[prost(message, optional, tag = "39")]
    pub deferred_stat_options: ::core::option::Option<bootstrap::DeferredStatOptions>,
    /// Configuration for internal processing of stats.
    #[prost(message, optional, tag = "13")]
    pub stats_config: ::core::option::Option<super::super::metrics::v3::StatsConfig>,
    /// Optional duration between flushes to configured stats sinks. For
    /// performance reasons Envoy latches counters and only flushes counters and
    /// gauges at a periodic interval. If not specified the default is 5000ms (5
    /// seconds). Only one of `stats_flush_interval` or `stats_flush_on_admin`
    /// can be set.
    /// Duration must be at least 1ms and at most 5 min.
    #[prost(message, optional, tag = "7")]
    pub stats_flush_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Optional watchdog configuration.
    /// This is for a single watchdog configuration for the entire system.
    /// Deprecated in favor of `watchdogs` which has finer granularity.
    #[deprecated]
    #[prost(message, optional, tag = "8")]
    pub watchdog: ::core::option::Option<Watchdog>,
    /// Optional watchdogs configuration.
    /// This is used for specifying different watchdogs for the different subsystems.
    /// \[\#extension-category: envoy.guarddog_actions\]
    #[prost(message, optional, tag = "27")]
    pub watchdogs: ::core::option::Option<Watchdogs>,
    /// Configuration for an external tracing provider.
    ///
    /// .. attention::
    /// This field has been deprecated in favor of :ref:`HttpConnectionManager.Tracing.provider <envoy_v3_api_field_extensions.filters.network.http_connection_manager.v3.HttpConnectionManager.Tracing.provider>`.
    #[deprecated]
    #[prost(message, optional, tag = "9")]
    pub tracing: ::core::option::Option<super::super::trace::v3::Tracing>,
    /// Configuration for the runtime configuration provider. If not
    /// specified, a “null” provider will be used which will result in all defaults
    /// being used.
    #[prost(message, optional, tag = "17")]
    pub layered_runtime: ::core::option::Option<LayeredRuntime>,
    /// Configuration for the local administration HTTP server.
    #[prost(message, optional, tag = "12")]
    pub admin: ::core::option::Option<Admin>,
    /// Optional overload manager configuration.
    #[prost(message, optional, tag = "15")]
    pub overload_manager: ::core::option::Option<
        super::super::overload::v3::OverloadManager,
    >,
    /// Enable :ref:`stats for event dispatcher <operations_performance>`, defaults to false.
    /// Note that this records a value for each iteration of the event loop on every thread. This
    /// should normally be minimal overhead, but when using
    /// :ref:`statsd <envoy_v3_api_msg_config.metrics.v3.StatsdSink>`, it will send each observed value
    /// over the wire individually because the statsd protocol doesn't have any way to represent a
    /// histogram summary. Be aware that this can be a very large volume of data.
    #[prost(bool, tag = "16")]
    pub enable_dispatcher_stats: bool,
    /// Optional string which will be used in lieu of x-envoy in prefixing headers.
    ///
    /// For example, if this string is present and set to X-Foo, then x-envoy-retry-on will be
    /// transformed into x-foo-retry-on etc.
    ///
    /// Note this applies to the headers Envoy will generate, the headers Envoy will sanitize, and the
    /// headers Envoy will trust for core code and core extensions only. Be VERY careful making
    /// changes to this string, especially in multi-layer Envoy deployments or deployments using
    /// extensions which are not upstream.
    #[prost(string, tag = "18")]
    pub header_prefix: ::prost::alloc::string::String,
    /// Optional proxy version which will be used to set the value of :ref:`server.version statistic <server_statistics>` if specified. Envoy will not process this value, it will be sent as is to
    /// :ref:`stats sinks <envoy_v3_api_msg_config.metrics.v3.StatsSink>`.
    #[prost(message, optional, tag = "19")]
    pub stats_server_version_override: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// Always use TCP queries instead of UDP queries for DNS lookups.
    /// This may be overridden on a per-cluster basis in cds_config,
    /// when :ref:`dns_resolvers <envoy_v3_api_field_config.cluster.v3.Cluster.dns_resolvers>` and
    /// :ref:`use_tcp_for_dns_lookups <envoy_v3_api_field_config.cluster.v3.Cluster.use_tcp_for_dns_lookups>` are
    /// specified.
    /// This field is deprecated in favor of `dns_resolution_config`
    /// which aggregates all of the DNS resolver configuration in a single message.
    #[deprecated]
    #[prost(bool, tag = "20")]
    pub use_tcp_for_dns_lookups: bool,
    /// DNS resolution configuration which includes the underlying dns resolver addresses and options.
    /// This may be overridden on a per-cluster basis in cds_config, when
    /// :ref:`dns_resolution_config <envoy_v3_api_field_config.cluster.v3.Cluster.dns_resolution_config>`
    /// is specified.
    /// This field is deprecated in favor of
    /// :ref:`typed_dns_resolver_config <envoy_v3_api_field_config.bootstrap.v3.Bootstrap.typed_dns_resolver_config>`.
    #[deprecated]
    #[prost(message, optional, tag = "30")]
    pub dns_resolution_config: ::core::option::Option<
        super::super::core::v3::DnsResolutionConfig,
    >,
    /// DNS resolver type configuration extension. This extension can be used to configure c-ares, apple,
    /// or any other DNS resolver types and the related parameters.
    /// For example, an object of
    /// :ref:`CaresDnsResolverConfig <envoy_v3_api_msg_extensions.network.dns_resolver.cares.v3.CaresDnsResolverConfig>`
    /// can be packed into this `typed_dns_resolver_config`. This configuration replaces the
    /// :ref:`dns_resolution_config <envoy_v3_api_field_config.bootstrap.v3.Bootstrap.dns_resolution_config>`
    /// configuration.
    /// During the transition period when both `dns_resolution_config` and `typed_dns_resolver_config` exists,
    /// when `typed_dns_resolver_config` is in place, Envoy will use it and ignore `dns_resolution_config`.
    /// When `typed_dns_resolver_config` is missing, the default behavior is in place.
    /// \[\#extension-category: envoy.network.dns_resolver\]
    #[prost(message, optional, tag = "31")]
    pub typed_dns_resolver_config: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// Specifies optional bootstrap extensions to be instantiated at startup time.
    /// Each item contains extension specific configuration.
    /// \[\#extension-category: envoy.bootstrap\]
    #[prost(message, repeated, tag = "21")]
    pub bootstrap_extensions: ::prost::alloc::vec::Vec<
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// Specifies optional extensions instantiated at startup time and
    /// invoked during crash time on the request that caused the crash.
    #[prost(message, repeated, tag = "28")]
    pub fatal_actions: ::prost::alloc::vec::Vec<FatalAction>,
    /// Configuration sources that will participate in
    /// xdstp:// URL authority resolution. The algorithm is as
    /// follows:
    ///
    /// 1. The authority field is taken from the xdstp:// URL, call
    ///    this `resource_authority`.
    /// 1. `resource_authority` is compared against the authorities in any peer
    ///    `ConfigSource`. The peer `ConfigSource` is the configuration source
    ///    message which would have been used unconditionally for resolution
    ///    with opaque resource names. If there is a match with an authority, the
    ///    peer `ConfigSource` message is used.
    /// 1. `resource_authority` is compared sequentially with the authorities in
    ///    each configuration source in `config_sources`. The first `ConfigSource`
    ///    to match wins.
    /// 1. As a fallback, if no configuration source matches, then
    ///    `default_config_source` is used.
    /// 1. If `default_config_source` is not specified, resolution fails.
    ///    \[\#not-implemented-hide:\]
    #[prost(message, repeated, tag = "22")]
    pub config_sources: ::prost::alloc::vec::Vec<super::super::core::v3::ConfigSource>,
    /// Default configuration source for xdstp:// URLs if all
    /// other resolution fails.
    /// \[\#not-implemented-hide:\]
    #[prost(message, optional, tag = "23")]
    pub default_config_source: ::core::option::Option<
        super::super::core::v3::ConfigSource,
    >,
    /// Optional overriding of default socket interface. The value must be the name of one of the
    /// socket interface factories initialized through a bootstrap extension
    #[prost(string, tag = "24")]
    pub default_socket_interface: ::prost::alloc::string::String,
    /// Global map of CertificateProvider instances. These instances are referred to by name in the
    /// :ref:`CommonTlsContext.CertificateProviderInstance.instance_name <envoy_v3_api_field_extensions.transport_sockets.tls.v3.CommonTlsContext.CertificateProviderInstance.instance_name>`
    /// field.
    /// \[\#not-implemented-hide:\]
    #[prost(map = "string, message", tag = "25")]
    pub certificate_provider_instances: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// Specifies a set of headers that need to be registered as inline header. This configuration
    /// allows users to customize the inline headers on-demand at Envoy startup without modifying
    /// Envoy's source code.
    ///
    /// Note that the 'set-cookie' header cannot be registered as inline header.
    #[prost(message, repeated, tag = "32")]
    pub inline_headers: ::prost::alloc::vec::Vec<CustomInlineHeader>,
    /// Optional path to a file with performance tracing data created by "Perfetto" SDK in binary
    /// ProtoBuf format. The default value is "envoy.pftrace".
    #[prost(string, tag = "33")]
    pub perf_tracing_file_path: ::prost::alloc::string::String,
    /// Optional overriding of default regex engine.
    /// If the value is not specified, Google RE2 will be used by default.
    /// \[\#extension-category: envoy.regex_engines\]
    #[prost(message, optional, tag = "34")]
    pub default_regex_engine: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// Optional XdsResourcesDelegate configuration, which allows plugging custom logic into both
    /// fetch and load events during xDS processing.
    /// If a value is not specified, no XdsResourcesDelegate will be used.
    /// TODO(abeyad): Add public-facing documentation.
    /// \[\#not-implemented-hide:\]
    #[prost(message, optional, tag = "35")]
    pub xds_delegate_extension: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// Optional XdsConfigTracker configuration, which allows tracking xDS responses in external components,
    /// e.g., external tracer or monitor. It provides the process point when receive, ingest, or fail to
    /// process xDS resources and messages. If a value is not specified, no XdsConfigTracker will be used.
    ///
    /// .. note::
    ///
    /// ```text
    /// There are no in-repo extensions currently, and the :repo:`XdsConfigTracker <envoy/config/xds_config_tracker.h>`
    /// interface should be implemented before using.
    /// See :repo:`xds_config_tracker_integration_test <test/integration/xds_config_tracker_integration_test.cc>`
    /// for an example usage of the interface.
    /// ```
    #[prost(message, optional, tag = "36")]
    pub xds_config_tracker_extension: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// \[\#not-implemented-hide:\]
    /// This controls the type of listener manager configured for Envoy. Currently
    /// Envoy only supports ListenerManager for this field and Envoy Mobile
    /// supports ApiListenerManager.
    #[prost(message, optional, tag = "37")]
    pub listener_manager: ::core::option::Option<
        super::super::core::v3::TypedExtensionConfig,
    >,
    /// Optional application log configuration.
    #[prost(message, optional, tag = "38")]
    pub application_log_config: ::core::option::Option<bootstrap::ApplicationLogConfig>,
    /// Optional gRPC async manager config.
    #[prost(message, optional, tag = "40")]
    pub grpc_async_client_manager_config: ::core::option::Option<
        bootstrap::GrpcAsyncClientManagerConfig,
    >,
    /// Optional configuration for memory allocation manager.
    /// Memory releasing is only supported for `tcmalloc allocator <<https://github.com/google/tcmalloc>`\_.>
    #[prost(message, optional, tag = "41")]
    pub memory_allocator_manager: ::core::option::Option<MemoryAllocatorManager>,
    #[prost(oneof = "bootstrap::StatsFlush", tags = "29")]
    pub stats_flush: ::core::option::Option<bootstrap::StatsFlush>,
}
/// Nested message and enum types in `Bootstrap`.
pub mod bootstrap {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StaticResources {
        /// Static :ref:`Listeners <envoy_v3_api_msg_config.listener.v3.Listener>`. These listeners are
        /// available regardless of LDS configuration.
        #[prost(message, repeated, tag = "1")]
        pub listeners: ::prost::alloc::vec::Vec<
            super::super::super::listener::v3::Listener,
        >,
        /// If a network based configuration source is specified for :ref:`cds_config <envoy_v3_api_field_config.bootstrap.v3.Bootstrap.DynamicResources.cds_config>`, it's necessary
        /// to have some initial cluster definitions available to allow Envoy to know
        /// how to speak to the management server. These cluster definitions may not
        /// use :ref:`EDS <arch_overview_dynamic_config_eds>` (i.e. they should be static
        /// IP or DNS-based).
        #[prost(message, repeated, tag = "2")]
        pub clusters: ::prost::alloc::vec::Vec<
            super::super::super::cluster::v3::Cluster,
        >,
        /// These static secrets can be used by :ref:`SdsSecretConfig <envoy_v3_api_msg_extensions.transport_sockets.tls.v3.SdsSecretConfig>`
        #[prost(message, repeated, tag = "3")]
        pub secrets: ::prost::alloc::vec::Vec<
            super::super::super::super::extensions::transport_sockets::tls::v3::Secret,
        >,
    }
    /// \[\#next-free-field: 7\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicResources {
        /// All :ref:`Listeners <envoy_v3_api_msg_config.listener.v3.Listener>` are provided by a single
        /// :ref:`LDS <arch_overview_dynamic_config_lds>` configuration source.
        #[prost(message, optional, tag = "1")]
        pub lds_config: ::core::option::Option<
            super::super::super::core::v3::ConfigSource,
        >,
        /// xdstp:// resource locator for listener collection.
        /// \[\#not-implemented-hide:\]
        #[prost(string, tag = "5")]
        pub lds_resources_locator: ::prost::alloc::string::String,
        /// All post-bootstrap :ref:`Cluster <envoy_v3_api_msg_config.cluster.v3.Cluster>` definitions are
        /// provided by a single :ref:`CDS <arch_overview_dynamic_config_cds>`
        /// configuration source.
        #[prost(message, optional, tag = "2")]
        pub cds_config: ::core::option::Option<
            super::super::super::core::v3::ConfigSource,
        >,
        /// xdstp:// resource locator for cluster collection.
        /// \[\#not-implemented-hide:\]
        #[prost(string, tag = "6")]
        pub cds_resources_locator: ::prost::alloc::string::String,
        /// A single :ref:`ADS <config_overview_ads>` source may be optionally
        /// specified. This must have :ref:`api_type <envoy_v3_api_field_config.core.v3.ApiConfigSource.api_type>` :ref:`GRPC <envoy_v3_api_enum_value_config.core.v3.ApiConfigSource.ApiType.GRPC>`. Only
        /// :ref:`ConfigSources <envoy_v3_api_msg_config.core.v3.ConfigSource>` that have
        /// the :ref:`ads <envoy_v3_api_field_config.core.v3.ConfigSource.ads>` field set will be
        /// streamed on the ADS channel.
        #[prost(message, optional, tag = "3")]
        pub ads_config: ::core::option::Option<
            super::super::super::core::v3::ApiConfigSource,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApplicationLogConfig {
        /// Optional field to set the application logs format. If this field is set, it will override
        /// the default log format. Setting both this field and :option:`--log-format` command line
        /// option is not allowed, and will cause a bootstrap error.
        #[prost(message, optional, tag = "1")]
        pub log_format: ::core::option::Option<application_log_config::LogFormat>,
    }
    /// Nested message and enum types in `ApplicationLogConfig`.
    pub mod application_log_config {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LogFormat {
            #[prost(oneof = "log_format::LogFormat", tags = "1, 2")]
            pub log_format: ::core::option::Option<log_format::LogFormat>,
        }
        /// Nested message and enum types in `LogFormat`.
        pub mod log_format {
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum LogFormat {
                /// Flush application logs in JSON format. The configured JSON struct can
                /// support all the format flags specified in the :option:`--log-format`
                /// command line options section, except for the `%v` and `%_` flags.
                #[prost(message, tag = "1")]
                JsonFormat(
                    super::super::super::super::super::super::super::google::protobuf::Struct,
                ),
                /// Flush application log in a format defined by a string. The text format
                /// can support all the format flags specified in the :option:`--log-format`
                /// command line option section.
                #[prost(string, tag = "2")]
                TextFormat(::prost::alloc::string::String),
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeferredStatOptions {
        /// When the flag is enabled, Envoy will lazily initialize a subset of the stats (see below).
        /// This will save memory and CPU cycles when creating the objects that own these stats, if those
        /// stats are never referenced throughout the lifetime of the process. However, it will incur additional
        /// memory overhead for these objects, and a small increase of CPU usage when a at least one of the stats
        /// is updated for the first time.
        /// Groups of stats that will be lazily initialized:
        ///
        /// * Cluster traffic stats: a subgroup of the :ref:`cluster statistics <config_cluster_manager_cluster_stats>`
        ///   that are used when requests are routed to the cluster.
        #[prost(bool, tag = "1")]
        pub enable_deferred_creation_stats: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GrpcAsyncClientManagerConfig {
        /// Optional field to set the expiration time for the cached gRPC client object.
        /// The minimal value is 5s and the default is 50s.
        #[prost(message, optional, tag = "1")]
        pub max_cached_entry_idle_duration: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StatsFlush {
        /// Flush stats to sinks only when queried for on the admin interface. If set,
        /// a flush timer is not created. Only one of `stats_flush_on_admin` or
        /// `stats_flush_interval` can be set.
        #[prost(bool, tag = "29")]
        StatsFlushOnAdmin(bool),
    }
}
/// Administration interface :ref:`operations documentation <operations_admin_interface>`.
/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Admin {
    /// Configuration for :ref:`access logs <arch_overview_access_logs>`
    /// emitted by the administration server.
    #[prost(message, repeated, tag = "5")]
    pub access_log: ::prost::alloc::vec::Vec<super::super::accesslog::v3::AccessLog>,
    /// The path to write the access log for the administration server. If no
    /// access log is desired specify ‘/dev/null’. This is only required if
    /// :ref:`address <envoy_v3_api_field_config.bootstrap.v3.Admin.address>` is set.
    /// Deprecated in favor of `access_log` which offers more options.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub access_log_path: ::prost::alloc::string::String,
    /// The cpu profiler output path for the administration server. If no profile
    /// path is specified, the default is ‘/var/log/envoy/envoy.prof’.
    #[prost(string, tag = "2")]
    pub profile_path: ::prost::alloc::string::String,
    /// The TCP address that the administration server will listen on.
    /// If not specified, Envoy will not start an administration server.
    #[prost(message, optional, tag = "3")]
    pub address: ::core::option::Option<super::super::core::v3::Address>,
    /// Additional socket options that may not be present in Envoy source code or
    /// precompiled binaries.
    #[prost(message, repeated, tag = "4")]
    pub socket_options: ::prost::alloc::vec::Vec<super::super::core::v3::SocketOption>,
    /// Indicates whether :ref:`global_downstream_max_connections <config_overload_manager_limiting_connections>`
    /// should apply to the admin interface or not.
    #[prost(bool, tag = "6")]
    pub ignore_global_conn_limit: bool,
}
/// Cluster manager :ref:`architecture overview <arch_overview_cluster_manager>`.
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterManager {
    /// Name of the local cluster (i.e., the cluster that owns the Envoy running
    /// this configuration). In order to enable :ref:`zone aware routing <arch_overview_load_balancing_zone_aware_routing>` this option must be set.
    /// If `local_cluster_name` is defined then :ref:`clusters <envoy_v3_api_msg_config.cluster.v3.Cluster>` must be defined in the :ref:`Bootstrap static cluster resources <envoy_v3_api_field_config.bootstrap.v3.Bootstrap.StaticResources.clusters>`. This is unrelated to
    /// the :option:`--service-cluster` option which does not `affect zone aware routing <<https://github.com/envoyproxy/envoy/issues/774>`\_.>
    #[prost(string, tag = "1")]
    pub local_cluster_name: ::prost::alloc::string::String,
    /// Optional global configuration for outlier detection.
    #[prost(message, optional, tag = "2")]
    pub outlier_detection: ::core::option::Option<cluster_manager::OutlierDetection>,
    /// Optional configuration used to bind newly established upstream connections.
    /// This may be overridden on a per-cluster basis by upstream_bind_config in the cds_config.
    #[prost(message, optional, tag = "3")]
    pub upstream_bind_config: ::core::option::Option<super::super::core::v3::BindConfig>,
    /// A management server endpoint to stream load stats to via
    /// `StreamLoadStats`. This must have :ref:`api_type <envoy_v3_api_field_config.core.v3.ApiConfigSource.api_type>` :ref:`GRPC <envoy_v3_api_enum_value_config.core.v3.ApiConfigSource.ApiType.GRPC>`.
    #[prost(message, optional, tag = "4")]
    pub load_stats_config: ::core::option::Option<
        super::super::core::v3::ApiConfigSource,
    >,
    /// Whether the ClusterManager will create clusters on the worker threads
    /// inline during requests. This will save memory and CPU cycles in cases where
    /// there are lots of inactive clusters and > 1 worker thread.
    #[prost(bool, tag = "5")]
    pub enable_deferred_cluster_creation: bool,
}
/// Nested message and enum types in `ClusterManager`.
pub mod cluster_manager {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutlierDetection {
        /// Specifies the path to the outlier event log.
        #[prost(string, tag = "1")]
        pub event_log_path: ::prost::alloc::string::String,
        /// \[\#not-implemented-hide:\]
        /// The gRPC service for the outlier detection event service.
        /// If empty, outlier detection events won't be sent to a remote endpoint.
        #[prost(message, optional, tag = "2")]
        pub event_service: ::core::option::Option<
            super::super::super::core::v3::EventServiceConfig,
        >,
    }
}
/// Allows you to specify different watchdog configs for different subsystems.
/// This allows finer tuned policies for the watchdog. If a subsystem is omitted
/// the default values for that system will be used.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Watchdogs {
    /// Watchdog for the main thread.
    #[prost(message, optional, tag = "1")]
    pub main_thread_watchdog: ::core::option::Option<Watchdog>,
    /// Watchdog for the worker threads.
    #[prost(message, optional, tag = "2")]
    pub worker_watchdog: ::core::option::Option<Watchdog>,
}
/// Envoy process watchdog configuration. When configured, this monitors for
/// nonresponsive threads and kills the process after the configured thresholds.
/// See the :ref:`watchdog documentation <operations_performance_watchdog>` for more information.
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Watchdog {
    /// Register actions that will fire on given WatchDog events.
    /// See `WatchDogAction` for priority of events.
    #[prost(message, repeated, tag = "7")]
    pub actions: ::prost::alloc::vec::Vec<watchdog::WatchdogAction>,
    /// The duration after which Envoy counts a nonresponsive thread in the
    /// `watchdog_miss` statistic. If not specified the default is 200ms.
    #[prost(message, optional, tag = "1")]
    pub miss_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// The duration after which Envoy counts a nonresponsive thread in the
    /// `watchdog_mega_miss` statistic. If not specified the default is
    /// 1000ms.
    #[prost(message, optional, tag = "2")]
    pub megamiss_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// If a watched thread has been nonresponsive for this duration, assume a
    /// programming error and kill the entire Envoy process. Set to 0 to disable
    /// kill behavior. If not specified the default is 0 (disabled).
    #[prost(message, optional, tag = "3")]
    pub kill_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Defines the maximum jitter used to adjust the `kill_timeout` if `kill_timeout` is
    /// enabled. Enabling this feature would help to reduce risk of synchronized
    /// watchdog kill events across proxies due to external triggers. Set to 0 to
    /// disable. If not specified the default is 0 (disabled).
    #[prost(message, optional, tag = "6")]
    pub max_kill_timeout_jitter: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// If `max(2, ceil(registered_threads * Fraction(*multikill_threshold*)))`
    /// threads have been nonresponsive for at least this duration kill the entire
    /// Envoy process. Set to 0 to disable this behavior. If not specified the
    /// default is 0 (disabled).
    #[prost(message, optional, tag = "4")]
    pub multikill_timeout: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
    /// Sets the threshold for `multikill_timeout` in terms of the percentage of
    /// nonresponsive threads required for the `multikill_timeout`.
    /// If not specified the default is 0.
    #[prost(message, optional, tag = "5")]
    pub multikill_threshold: ::core::option::Option<
        super::super::super::r#type::v3::Percent,
    >,
}
/// Nested message and enum types in `Watchdog`.
pub mod watchdog {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WatchdogAction {
        /// Extension specific configuration for the action.
        #[prost(message, optional, tag = "1")]
        pub config: ::core::option::Option<
            super::super::super::core::v3::TypedExtensionConfig,
        >,
        #[prost(enumeration = "watchdog_action::WatchdogEvent", tag = "2")]
        pub event: i32,
    }
    /// Nested message and enum types in `WatchdogAction`.
    pub mod watchdog_action {
        /// The events are fired in this order: KILL, MULTIKILL, MEGAMISS, MISS.
        /// Within an event type, actions execute in the order they are configured.
        /// For KILL/MULTIKILL there is a default PANIC that will run after the
        /// registered actions and kills the process if it wasn't already killed.
        /// It might be useful to specify several debug actions, and possibly an
        /// alternate FATAL action.
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
        pub enum WatchdogEvent {
            Unknown = 0,
            Kill = 1,
            Multikill = 2,
            Megamiss = 3,
            Miss = 4,
        }
        impl WatchdogEvent {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    WatchdogEvent::Unknown => "UNKNOWN",
                    WatchdogEvent::Kill => "KILL",
                    WatchdogEvent::Multikill => "MULTIKILL",
                    WatchdogEvent::Megamiss => "MEGAMISS",
                    WatchdogEvent::Miss => "MISS",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNKNOWN" => Some(Self::Unknown),
                    "KILL" => Some(Self::Kill),
                    "MULTIKILL" => Some(Self::Multikill),
                    "MEGAMISS" => Some(Self::Megamiss),
                    "MISS" => Some(Self::Miss),
                    _ => None,
                }
            }
        }
    }
}
/// Fatal actions to run while crashing. Actions can be safe (meaning they are
/// async-signal safe) or unsafe. We run all safe actions before we run unsafe actions.
/// If using an unsafe action that could get stuck or deadlock, it important to
/// have an out of band system to terminate the process.
///
/// The interface for the extension is `Envoy::Server::Configuration::FatalAction`.
/// `FatalAction` extensions live in the `envoy.extensions.fatal_actions` API
/// namespace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FatalAction {
    /// Extension specific configuration for the action. It's expected to conform
    /// to the `Envoy::Server::Configuration::FatalAction` interface.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<super::super::core::v3::TypedExtensionConfig>,
}
/// Runtime :ref:`configuration overview <config_runtime>` (deprecated).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Runtime {
    /// The implementation assumes that the file system tree is accessed via a
    /// symbolic link. An atomic link swap is used when a new tree should be
    /// switched to. This parameter specifies the path to the symbolic link. Envoy
    /// will watch the location for changes and reload the file system tree when
    /// they happen. If this parameter is not set, there will be no disk based
    /// runtime.
    #[prost(string, tag = "1")]
    pub symlink_root: ::prost::alloc::string::String,
    /// Specifies the subdirectory to load within the root directory. This is
    /// useful if multiple systems share the same delivery mechanism. Envoy
    /// configuration elements can be contained in a dedicated subdirectory.
    #[prost(string, tag = "2")]
    pub subdirectory: ::prost::alloc::string::String,
    /// Specifies an optional subdirectory to load within the root directory. If
    /// specified and the directory exists, configuration values within this
    /// directory will override those found in the primary subdirectory. This is
    /// useful when Envoy is deployed across many different types of servers.
    /// Sometimes it is useful to have a per service cluster directory for runtime
    /// configuration. See below for exactly how the override directory is used.
    #[prost(string, tag = "3")]
    pub override_subdirectory: ::prost::alloc::string::String,
    /// Static base runtime. This will be :ref:`overridden <config_runtime_layering>` by other runtime layers, e.g.
    /// disk or admin. This follows the :ref:`runtime protobuf JSON representation encoding <config_runtime_proto_json>`.
    #[prost(message, optional, tag = "4")]
    pub base: ::core::option::Option<
        super::super::super::super::google::protobuf::Struct,
    >,
}
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeLayer {
    /// Descriptive name for the runtime layer. This is only used for the runtime
    /// :http:get:`/runtime` output.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "runtime_layer::LayerSpecifier", tags = "2, 3, 4, 5")]
    pub layer_specifier: ::core::option::Option<runtime_layer::LayerSpecifier>,
}
/// Nested message and enum types in `RuntimeLayer`.
pub mod runtime_layer {
    /// :ref:`Disk runtime <config_runtime_local_disk>` layer.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DiskLayer {
        /// The implementation assumes that the file system tree is accessed via a
        /// symbolic link. An atomic link swap is used when a new tree should be
        /// switched to. This parameter specifies the path to the symbolic link.
        /// Envoy will watch the location for changes and reload the file system tree
        /// when they happen. See documentation on runtime :ref:`atomicity <config_runtime_atomicity>` for further details on how reloads are
        /// treated.
        #[prost(string, tag = "1")]
        pub symlink_root: ::prost::alloc::string::String,
        /// Specifies the subdirectory to load within the root directory. This is
        /// useful if multiple systems share the same delivery mechanism. Envoy
        /// configuration elements can be contained in a dedicated subdirectory.
        #[prost(string, tag = "3")]
        pub subdirectory: ::prost::alloc::string::String,
        /// :ref:`Append <config_runtime_local_disk_service_cluster_subdirs>` the
        /// service cluster to the path under symlink root.
        #[prost(bool, tag = "2")]
        pub append_service_cluster: bool,
    }
    /// :ref:`Admin console runtime <config_runtime_admin>` layer.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AdminLayer {}
    /// :ref:`Runtime Discovery Service (RTDS) <config_runtime_rtds>` layer.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RtdsLayer {
        /// Resource to subscribe to at `rtds_config` for the RTDS layer.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// RTDS configuration source.
        #[prost(message, optional, tag = "2")]
        pub rtds_config: ::core::option::Option<
            super::super::super::core::v3::ConfigSource,
        >,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LayerSpecifier {
        /// :ref:`Static runtime <config_runtime_bootstrap>` layer.
        /// This follows the :ref:`runtime protobuf JSON representation encoding <config_runtime_proto_json>`. Unlike static xDS resources, this static
        /// layer is overridable by later layers in the runtime virtual filesystem.
        #[prost(message, tag = "2")]
        StaticLayer(super::super::super::super::super::google::protobuf::Struct),
        #[prost(message, tag = "3")]
        DiskLayer(DiskLayer),
        #[prost(message, tag = "4")]
        AdminLayer(AdminLayer),
        #[prost(message, tag = "5")]
        RtdsLayer(RtdsLayer),
    }
}
/// Runtime :ref:`configuration overview <config_runtime>`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LayeredRuntime {
    /// The :ref:`layers <config_runtime_layering>` of the runtime. This is ordered
    /// such that later layers in the list overlay earlier entries.
    #[prost(message, repeated, tag = "1")]
    pub layers: ::prost::alloc::vec::Vec<RuntimeLayer>,
}
/// Used to specify the header that needs to be registered as an inline header.
///
/// If request or response contain multiple headers with the same name and the header
/// name is registered as an inline header. Then multiple headers will be folded
/// into one, and multiple header values will be concatenated by a suitable delimiter.
/// The delimiter is generally a comma.
///
/// For example, if 'foo' is registered as an inline header, and the headers contains
/// the following two headers:
///
/// .. code-block:: text
///
/// foo: bar
/// foo: eep
///
/// Then they will eventually be folded into:
///
/// .. code-block:: text
///
/// foo: bar, eep
///
/// Inline headers provide O(1) search performance, but each inline header imposes
/// an additional memory overhead on all instances of the corresponding type of
/// HeaderMap or TrailerMap.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInlineHeader {
    /// The name of the header that is expected to be set as the inline header.
    #[prost(string, tag = "1")]
    pub inline_header_name: ::prost::alloc::string::String,
    /// The type of the header that is expected to be set as the inline header.
    #[prost(enumeration = "custom_inline_header::InlineHeaderType", tag = "2")]
    pub inline_header_type: i32,
}
/// Nested message and enum types in `CustomInlineHeader`.
pub mod custom_inline_header {
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
    pub enum InlineHeaderType {
        RequestHeader = 0,
        RequestTrailer = 1,
        ResponseHeader = 2,
        ResponseTrailer = 3,
    }
    impl InlineHeaderType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InlineHeaderType::RequestHeader => "REQUEST_HEADER",
                InlineHeaderType::RequestTrailer => "REQUEST_TRAILER",
                InlineHeaderType::ResponseHeader => "RESPONSE_HEADER",
                InlineHeaderType::ResponseTrailer => "RESPONSE_TRAILER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REQUEST_HEADER" => Some(Self::RequestHeader),
                "REQUEST_TRAILER" => Some(Self::RequestTrailer),
                "RESPONSE_HEADER" => Some(Self::ResponseHeader),
                "RESPONSE_TRAILER" => Some(Self::ResponseTrailer),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemoryAllocatorManager {
    /// Configures tcmalloc to perform background release of free memory in amount of bytes per `memory_release_interval` interval.
    /// If equals to `0`, no memory release will occur. Defaults to `0`.
    #[prost(uint64, tag = "1")]
    pub bytes_to_release: u64,
    /// Interval in milliseconds for memory releasing. If specified, during every
    /// interval Envoy will try to release `bytes_to_release` of free memory back to operating system for reuse.
    /// Defaults to 1000 milliseconds.
    #[prost(message, optional, tag = "2")]
    pub memory_release_interval: ::core::option::Option<
        super::super::super::super::google::protobuf::Duration,
    >,
}
