/// Configuration for DataSourceServerPreferredAddressConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSourceServerPreferredAddressConfig {
    /// The IPv4 address to advertise to clients for Server Preferred Address.
    #[prost(message, optional, tag = "1")]
    pub ipv4_config: ::core::option::Option<
        data_source_server_preferred_address_config::AddressFamilyConfig,
    >,
    /// The IPv6 address to advertise to clients for Server Preferred Address.
    #[prost(message, optional, tag = "2")]
    pub ipv6_config: ::core::option::Option<
        data_source_server_preferred_address_config::AddressFamilyConfig,
    >,
}
/// Nested message and enum types in `DataSourceServerPreferredAddressConfig`.
pub mod data_source_server_preferred_address_config {
    /// Addresses for server preferred address for a single address family (IPv4 or IPv6).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddressFamilyConfig {
        /// The server preferred address sent to clients. The data must contain an IP address string.
        #[prost(message, optional, tag = "1")]
        pub address: ::core::option::Option<
            super::super::super::super::super::config::core::v3::DataSource,
        >,
        /// The server preferred address port sent to clients. The data must contain a integer port value.
        ///
        /// If this is not specified, the listener's port is used.
        ///
        /// Note: Envoy currently must receive all packets for a QUIC connection on the same port, so unless
        /// :ref:`dnat_address <envoy_v3_api_field_extensions.quic.server_preferred_address.v3.DataSourceServerPreferredAddressConfig.AddressFamilyConfig.dnat_address>`
        /// is configured, this must be left unset.
        #[prost(message, optional, tag = "2")]
        pub port: ::core::option::Option<
            super::super::super::super::super::config::core::v3::DataSource,
        >,
        /// If there is a DNAT between the client and Envoy, the address that Envoy will observe
        /// server preferred address packets being sent to. If this is not specified, it is assumed
        /// there is no DNAT and the server preferred address packets will be sent to the address advertised
        /// to clients for server preferred address.
        #[prost(message, optional, tag = "3")]
        pub dnat_address: ::core::option::Option<
            super::super::super::super::super::config::core::v3::DataSource,
        >,
    }
}
/// Configuration for FixedServerPreferredAddressConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedServerPreferredAddressConfig {
    /// String representation of IPv4 address, i.e. "127.0.0.2".
    /// If not specified, none will be configured.
    #[prost(string, tag = "1")]
    pub ipv4_address: ::prost::alloc::string::String,
    /// The IPv4 address to advertise to clients for Server Preferred Address.
    /// This field takes precedence over
    /// :ref:`ipv4_address <envoy_v3_api_field_extensions.quic.server_preferred_address.v3.FixedServerPreferredAddressConfig.ipv4_address>`.
    #[prost(message, optional, tag = "3")]
    pub ipv4_config: ::core::option::Option<
        fixed_server_preferred_address_config::AddressFamilyConfig,
    >,
    /// String representation of IPv6 address, i.e. "::1".
    /// If not specified, none will be configured.
    #[prost(string, tag = "2")]
    pub ipv6_address: ::prost::alloc::string::String,
    /// The IPv6 address to advertise to clients for Server Preferred Address.
    /// This field takes precedence over
    /// :ref:`ipv6_address <envoy_v3_api_field_extensions.quic.server_preferred_address.v3.FixedServerPreferredAddressConfig.ipv6_address>`.
    #[prost(message, optional, tag = "4")]
    pub ipv6_config: ::core::option::Option<
        fixed_server_preferred_address_config::AddressFamilyConfig,
    >,
}
/// Nested message and enum types in `FixedServerPreferredAddressConfig`.
pub mod fixed_server_preferred_address_config {
    /// Addresses for server preferred address for a single address family (IPv4 or IPv6).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddressFamilyConfig {
        /// The server preferred address sent to clients.
        ///
        /// Note: Envoy currently must receive all packets for a QUIC connection on the same port, so unless
        /// :ref:`dnat_address <envoy_v3_api_field_extensions.quic.server_preferred_address.v3.FixedServerPreferredAddressConfig.AddressFamilyConfig.dnat_address>`
        /// is configured, the port for this address must be zero, and the listener's
        /// port will be used instead.
        #[prost(message, optional, tag = "1")]
        pub address: ::core::option::Option<
            super::super::super::super::super::config::core::v3::SocketAddress,
        >,
        /// If there is a DNAT between the client and Envoy, the address that Envoy will observe
        /// server preferred address packets being sent to. If this is not specified, it is assumed
        /// there is no DNAT and the server preferred address packets will be sent to the address advertised
        /// to clients for server preferred address.
        ///
        /// Note: Envoy currently must receive all packets for a QUIC connection on the same port, so the
        /// port for this address must be zero, and the listener's port will be used instead.
        #[prost(message, optional, tag = "2")]
        pub dnat_address: ::core::option::Option<
            super::super::super::super::super::config::core::v3::SocketAddress,
        >,
    }
}
