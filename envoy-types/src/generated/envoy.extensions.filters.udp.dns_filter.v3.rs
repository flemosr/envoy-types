/// Configuration for the DNS filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsFilterConfig {
    /// The stat prefix used when emitting DNS filter statistics
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// Server context configuration contains the data that the filter uses to respond
    /// to DNS requests.
    #[prost(message, optional, tag = "2")]
    pub server_config: ::core::option::Option<dns_filter_config::ServerContextConfig>,
    /// Client context configuration controls Envoy's behavior when it must use external
    /// resolvers to answer a query. This object is optional and if omitted instructs
    /// the filter to resolve queries from the data in the server_config
    #[prost(message, optional, tag = "3")]
    pub client_config: ::core::option::Option<dns_filter_config::ClientContextConfig>,
}
/// Nested message and enum types in `DnsFilterConfig`.
pub mod dns_filter_config {
    /// This message contains the configuration for the DNS Filter operating
    /// in a server context. This message will contain the virtual hosts and
    /// associated addresses with which Envoy will respond to queries
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerContextConfig {
        #[prost(oneof = "server_context_config::ConfigSource", tags = "1, 2")]
        pub config_source: ::core::option::Option<server_context_config::ConfigSource>,
    }
    /// Nested message and enum types in `ServerContextConfig`.
    pub mod server_context_config {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigSource {
            /// Load the configuration specified from the control plane
            #[prost(message, tag = "1")]
            InlineDnsTable(
                super::super::super::super::super::super::super::data::dns::v3::DnsTable,
            ),
            /// Seed the filter configuration from an external path. This source
            /// is a yaml formatted file that contains the DnsTable driving Envoy's
            /// responses to DNS queries
            #[prost(message, tag = "2")]
            ExternalDnsTable(
                super::super::super::super::super::super::super::config::core::v3::DataSource,
            ),
        }
    }
    /// This message contains the configuration for the DNS Filter operating
    /// in a client context. This message will contain the timeouts, retry,
    /// and forwarding configuration for Envoy to make DNS requests to other
    /// resolvers
    ///
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClientContextConfig {
        /// Sets the maximum time we will wait for the upstream query to complete
        /// We allow 5s for the upstream resolution to complete, so the minimum
        /// value here is 1. Note that the total latency for a failed query is the
        /// number of retries multiplied by the resolver_timeout.
        #[prost(message, optional, tag = "1")]
        pub resolver_timeout: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
        /// This field was used for `dns_resolution_config` in Envoy 1.19.0 and
        /// 1.19.1.
        /// Control planes that need to set this field for Envoy 1.19.0 and
        /// 1.19.1 clients should fork the protobufs and change the field type
        /// to `DnsResolutionConfig`.
        /// Control planes that need to simultaneously support Envoy 1.18.x and
        /// Envoy 1.19.x should avoid Envoy 1.19.0 and 1.19.1.
        ///
        /// \\[\#not-implemented-hide:\\]
        #[deprecated]
        #[prost(message, repeated, tag = "2")]
        pub upstream_resolvers: ::prost::alloc::vec::Vec<
            super::super::super::super::super::super::config::core::v3::Address,
        >,
        /// DNS resolution configuration which includes the underlying dns resolver addresses and options.
        /// This field is deprecated in favor of
        /// :ref:`typed_dns_resolver_config <envoy_v3_api_field_extensions.filters.udp.dns_filter.v3.DnsFilterConfig.ClientContextConfig.typed_dns_resolver_config>`.
        #[deprecated]
        #[prost(message, optional, tag = "5")]
        pub dns_resolution_config: ::core::option::Option<
            super::super::super::super::super::super::config::core::v3::DnsResolutionConfig,
        >,
        /// DNS resolver type configuration extension. This extension can be used to configure c-ares, apple,
        /// or any other DNS resolver types and the related parameters.
        /// For example, an object of
        /// :ref:`CaresDnsResolverConfig <envoy_v3_api_msg_extensions.network.dns_resolver.cares.v3.CaresDnsResolverConfig>`
        /// can be packed into this `typed_dns_resolver_config`. This configuration replaces the
        /// :ref:`dns_resolution_config <envoy_v3_api_field_extensions.filters.udp.dns_filter.v3.DnsFilterConfig.ClientContextConfig.dns_resolution_config>`
        /// configuration.
        /// During the transition period when both `dns_resolution_config` and `typed_dns_resolver_config` exists,
        /// when `typed_dns_resolver_config` is in place, Envoy will use it and ignore `dns_resolution_config`.
        /// When `typed_dns_resolver_config` is missing, the default behavior is in place.
        /// \[\#extension-category: envoy.network.dns_resolver\]
        #[prost(message, optional, tag = "4")]
        pub typed_dns_resolver_config: ::core::option::Option<
            super::super::super::super::super::super::config::core::v3::TypedExtensionConfig,
        >,
        /// Controls how many outstanding external lookup contexts the filter tracks.
        /// The context structure allows the filter to respond to every query even if the external
        /// resolution times out or is otherwise unsuccessful
        #[prost(uint64, tag = "3")]
        pub max_pending_lookups: u64,
    }
}
