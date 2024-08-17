#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualHost {
    /// The name of the virtual host.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of hosts that will be matched to this virtual host. Wildcard hosts are supported in
    /// the suffix or prefix form.
    ///
    /// Host search order:
    ///
    /// 1. Exact names: `www.foo.com`.
    /// 1. Suffix wildcards: `*.foo.com` or `*-bar.foo.com`.
    /// 1. Prefix wildcards: `foo.*` or `foo-*`.
    /// 1. Special wildcard `*` matching any host and will be the default virtual host.
    ///
    /// .. note::
    ///
    /// The wildcard will not match the empty string.
    /// e.g. `*-bar.foo.com` will match `baz-bar.foo.com` but not `-bar.foo.com`.
    /// The longest wildcards match first.
    /// Only a single virtual host in the entire route configuration can match on `*`. A domain
    /// must be unique across all virtual hosts or the config will fail to load.
    #[prost(string, repeated, tag = "2")]
    pub hosts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The match tree to use when resolving route actions for incoming requests.
    #[prost(message, optional, tag = "3")]
    pub routes: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
}
/// The generic proxy makes use of the xDS matching API for routing configurations.
///
/// In the below example, we combine a top level tree matcher with a linear matcher to match
/// the incoming requests, and send the matching requests to v1 of the upstream service.
///
/// .. code-block:: yaml
///
/// name: example
/// routes:
/// matcher_tree:
/// input:
/// name: request-service
/// typed_config:
/// "@type": type.googleapis.com/envoy.extensions.filters.network.generic_proxy.matcher.v3.ServiceMatchInput
/// exact_match_map:
/// map:
/// service_name_0:
/// matcher:
/// matcher_list:
/// matchers:
/// - predicate:
/// and_matcher:
/// predicate:
/// - single_predicate:
/// input:
/// name: request-properties
/// typed_config:
/// "@type": type.googleapis.com/envoy.extensions.filters.network.generic_proxy.matcher.v3.PropertyMatchInput
/// property_name: version
/// value_match:
/// exact: v1
/// - single_predicate:
/// input:
/// name: request-properties
/// typed_config:
/// "@type": type.googleapis.com/envoy.extensions.filters.network.generic_proxy.matcher.v3.PropertyMatchInput
/// property_name: user
/// value_match:
/// exact: john
/// on_match:
/// action:
/// name: route
/// typed_config:
/// "@type": type.googleapis.com/envoy.extensions.filters.network.generic_proxy.action.v3.routeAction
/// cluster: cluster_0
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteConfiguration {
    /// The name of the route configuration. For example, it might match route_config_name in
    /// envoy.extensions.filters.network.generic_proxy.v3.Rds.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The match tree to use when resolving route actions for incoming requests.
    /// If no any virtual host is configured in the `virtual_hosts` field or no special wildcard
    /// virtual host is configured, the `routes` field will be used as the default route table.
    /// If both the wildcard virtual host and `routes` are configured, the configuration will fail
    /// to load.
    #[prost(message, optional, tag = "2")]
    pub routes: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    /// An array of virtual hosts that make up the route table.
    #[prost(message, repeated, tag = "3")]
    pub virtual_hosts: ::prost::alloc::vec::Vec<VirtualHost>,
}
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericProxy {
    /// The human readable prefix to use when emitting statistics.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// The codec which encodes and decodes the application protocol.
    /// \[\#extension-category: envoy.generic_proxy.codecs\]
    #[prost(message, optional, tag = "2")]
    pub codec_config: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// A list of individual Layer-7 filters that make up the filter chain for requests made to the
    /// proxy. Order matters as the filters are processed sequentially as request events
    /// happen.
    /// \[\#extension-category: envoy.generic_proxy.filters\]
    #[prost(message, repeated, tag = "5")]
    pub filters: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// Tracing configuration for the generic proxy.
    #[prost(message, optional, tag = "6")]
    pub tracing: ::core::option::Option<
        super::super::http_connection_manager::v3::http_connection_manager::Tracing,
    >,
    /// Configuration for :ref:`access logs <arch_overview_access_logs>` emitted by generic proxy.
    #[prost(message, repeated, tag = "7")]
    pub access_log: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::accesslog::v3::AccessLog,
    >,
    #[prost(oneof = "generic_proxy::RouteSpecifier", tags = "3, 4")]
    pub route_specifier: ::core::option::Option<generic_proxy::RouteSpecifier>,
}
/// Nested message and enum types in `GenericProxy`.
pub mod generic_proxy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RouteSpecifier {
        /// The generic proxies route table will be dynamically loaded via the meta RDS API.
        #[prost(message, tag = "3")]
        GenericRds(super::GenericRds),
        /// The route table for the generic proxy is static and is specified in this property.
        #[prost(message, tag = "4")]
        RouteConfig(super::RouteConfiguration),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericRds {
    /// Configuration source specifier for RDS.
    #[prost(message, optional, tag = "1")]
    pub config_source: ::core::option::Option<
        super::super::super::super::super::config::core::v3::ConfigSource,
    >,
    /// The name of the route configuration. This name will be passed to the RDS API. This allows an
    /// Envoy configuration with multiple generic proxies to use different route configurations.
    #[prost(string, tag = "2")]
    pub route_config_name: ::prost::alloc::string::String,
}
