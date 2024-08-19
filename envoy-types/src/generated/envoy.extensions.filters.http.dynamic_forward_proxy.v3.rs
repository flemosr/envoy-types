// This file is @generated by prost-build.
/// Configuration for the dynamic forward proxy HTTP filter. See the :ref:`architecture overview <arch_overview_http_dynamic_forward_proxy>` for more information.
/// \[\#extension: envoy.filters.http.dynamic_forward_proxy\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfig {
    /// When this flag is set, the filter will add the resolved upstream address in the filter
    /// state. The state should be saved with key
    /// `envoy.stream.upstream_address` (See
    /// :repo:`upstream_address.h<source/common/stream_info/upstream_address.h>`).
    #[prost(bool, tag = "2")]
    pub save_upstream_address: bool,
    #[prost(oneof = "filter_config::ImplementationSpecifier", tags = "1, 3")]
    pub implementation_specifier: ::core::option::Option<
        filter_config::ImplementationSpecifier,
    >,
}
/// Nested message and enum types in `FilterConfig`.
pub mod filter_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ImplementationSpecifier {
        /// The DNS cache configuration that the filter will attach to. Note this configuration must
        /// match that of associated :ref:`dynamic forward proxy cluster configuration <envoy_v3_api_field_extensions.clusters.dynamic_forward_proxy.v3.ClusterConfig.dns_cache_config>`.
        #[prost(message, tag = "1")]
        DnsCacheConfig(
            super::super::super::super::super::common::dynamic_forward_proxy::v3::DnsCacheConfig,
        ),
        /// The configuration that the filter will use, when the related dynamic forward proxy cluster enabled
        /// sub clusters.
        #[prost(message, tag = "3")]
        SubClusterConfig(super::SubClusterConfig),
    }
}
/// Per route Configuration for the dynamic forward proxy HTTP filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerRouteConfig {
    #[prost(oneof = "per_route_config::HostRewriteSpecifier", tags = "1, 2")]
    pub host_rewrite_specifier: ::core::option::Option<
        per_route_config::HostRewriteSpecifier,
    >,
}
/// Nested message and enum types in `PerRouteConfig`.
pub mod per_route_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HostRewriteSpecifier {
        /// Indicates that before DNS lookup, the host header will be swapped with
        /// this value. If not set or empty, the original host header value
        /// will be used and no rewrite will happen.
        ///
        /// Note: this rewrite affects both DNS lookup and host header forwarding. However, this
        /// option shouldn't be used with
        /// :ref:`HCM host rewrite <envoy_v3_api_field_config.route.v3.RouteAction.host_rewrite_literal>` given that the
        /// value set here would be used for DNS lookups whereas the value set in the HCM would be used
        /// for host header forwarding which is not the desired outcome.
        #[prost(string, tag = "1")]
        HostRewriteLiteral(::prost::alloc::string::String),
        /// Indicates that before DNS lookup, the host header will be swapped with
        /// the value of this header. If not set or empty, the original host header
        /// value will be used and no rewrite will happen.
        ///
        /// Note: this rewrite affects both DNS lookup and host header forwarding. However, this
        /// option shouldn't be used with
        /// :ref:`HCM host rewrite header <envoy_v3_api_field_config.route.v3.RouteAction.auto_host_rewrite>`
        /// given that the value set here would be used for DNS lookups whereas the value set in the HCM
        /// would be used for host header forwarding which is not the desired outcome.
        ///
        /// .. note::
        ///
        /// If the header appears multiple times only the first value is used.
        #[prost(string, tag = "2")]
        HostRewriteHeader(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SubClusterConfig {
    /// The timeout used for sub cluster initialization. Defaults to 5s if not set.
    #[prost(message, optional, tag = "3")]
    pub cluster_init_timeout: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
}
