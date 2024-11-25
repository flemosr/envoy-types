// This file is @generated by prost-build.
///
/// Configuration for the SNI-based dynamic forward proxy filter. See the
/// : ref:`architecture overview <arch_overview_http_dynamic_forward_proxy>` for
/// more information. Note this filter must be configured along with
/// : ref:`TLS inspector listener filter <config_listener_filters_tls_inspector>`
/// to work.
/// \[\#extension: envoy.filters.network.sni_dynamic_forward_proxy\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterConfig {
    /// The DNS cache configuration that the filter will attach to. Note this
    /// configuration must match that of associated :ref:`dynamic forward proxy  cluster configuration  <envoy_v3_api_field_extensions.clusters.dynamic_forward_proxy.v3.ClusterConfig.dns_cache_config>`.
    #[prost(message, optional, tag = "1")]
    pub dns_cache_config: ::core::option::Option<
        super::super::super::super::common::dynamic_forward_proxy::v3::DnsCacheConfig,
    >,
    ///
    /// When this flag is set, the filter will add the resolved upstream address in the filter
    /// state. The state should be saved with key
    /// `envoy.stream.upstream_address` (See
    /// : repo:`upstream_address.h<source/common/stream_info/upstream_address.h>`).
    #[prost(bool, tag = "3")]
    pub save_upstream_address: bool,
    #[prost(oneof = "filter_config::PortSpecifier", tags = "2")]
    pub port_specifier: ::core::option::Option<filter_config::PortSpecifier>,
}
/// Nested message and enum types in `FilterConfig`.
pub mod filter_config {
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum PortSpecifier {
        /// The port number to connect to the upstream.
        #[prost(uint32, tag = "2")]
        PortValue(u32),
    }
}
