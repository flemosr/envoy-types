/// Configuration for FixedServerPreferredAddressConfig.
///
/// \[\#comment:TODO(danzh2010): discuss with API shepherds before removing WiP status.\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedServerPreferredAddressConfig {
    #[prost(oneof = "fixed_server_preferred_address_config::Ipv4Type", tags = "1")]
    pub ipv4_type: ::core::option::Option<
        fixed_server_preferred_address_config::Ipv4Type,
    >,
    #[prost(oneof = "fixed_server_preferred_address_config::Ipv6Type", tags = "2")]
    pub ipv6_type: ::core::option::Option<
        fixed_server_preferred_address_config::Ipv6Type,
    >,
}
/// Nested message and enum types in `FixedServerPreferredAddressConfig`.
pub mod fixed_server_preferred_address_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ipv4Type {
        /// String representation of IPv4 address, i.e. "127.0.0.2".
        /// If not specified, none will be configured.
        #[prost(string, tag = "1")]
        Ipv4Address(::prost::alloc::string::String),
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ipv6Type {
        /// String representation of IPv6 address, i.e. "::1".
        /// If not specified, none will be configured.
        #[prost(string, tag = "2")]
        Ipv6Address(::prost::alloc::string::String),
    }
}
