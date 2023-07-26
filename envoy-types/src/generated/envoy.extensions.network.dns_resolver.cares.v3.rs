/// Configuration for c-ares DNS resolver.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaresDnsResolverConfig {
    /// A list of dns resolver addresses.
    /// :ref:`use_resolvers_as_fallback<envoy_v3_api_field_extensions.network.dns_resolver.cares.v3.CaresDnsResolverConfig.use_resolvers_as_fallback>`
    /// below dictates if the DNS client should override system defaults or only use the provided
    /// resolvers if the system defaults are not available, i.e., as a fallback.
    #[prost(message, repeated, tag = "1")]
    pub resolvers: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::Address,
    >,
    /// If true use the resolvers listed in the
    /// :ref:`resolvers<envoy_v3_api_field_extensions.network.dns_resolver.cares.v3.CaresDnsResolverConfig.resolvers>`
    /// field only if c-ares is unable to obtain a
    /// nameserver from the system (e.g., /etc/resolv.conf).
    /// Otherwise, the resolvers listed in the resolvers list will override the default system
    /// resolvers. Defaults to false.
    #[prost(bool, tag = "3")]
    pub use_resolvers_as_fallback: bool,
    /// The resolver will query available network interfaces and determine if there are no available
    /// interfaces for a given IP family. It will then filter these addresses from the results it
    /// presents. e.g., if there are no available IPv4 network interfaces, the resolver will not
    /// provide IPv4 addresses.
    #[prost(bool, tag = "4")]
    pub filter_unroutable_families: bool,
    /// Configuration of DNS resolver option flags which control the behavior of the DNS resolver.
    #[prost(message, optional, tag = "2")]
    pub dns_resolver_options: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DnsResolverOptions,
    >,
}
