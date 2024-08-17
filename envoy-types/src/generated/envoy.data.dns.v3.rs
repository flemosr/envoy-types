/// This message contains the configuration for the DNS Filter if populated
/// from the control plane
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsTable {
    /// Control how many times Envoy makes an attempt to forward a query to an external DNS server
    #[prost(uint32, tag = "1")]
    pub external_retry_count: u32,
    /// Fully qualified domain names for which Envoy will respond to DNS queries. By leaving this
    /// list empty, Envoy will forward all queries to external resolvers
    #[prost(message, repeated, tag = "2")]
    pub virtual_domains: ::prost::alloc::vec::Vec<dns_table::DnsVirtualDomain>,
    /// This field is deprecated and no longer used in Envoy. The filter's behavior has changed
    /// internally to use a different data structure allowing the filter to determine whether a
    /// query is for known domain without the use of this field.
    ///
    /// This field serves to help Envoy determine whether it can authoritatively answer a query
    /// for a name matching a suffix in this list. If the query name does not match a suffix in
    /// this list, Envoy will forward the query to an upstream DNS server
    #[deprecated]
    #[prost(message, repeated, tag = "3")]
    pub known_suffixes: ::prost::alloc::vec::Vec<
        super::super::super::r#type::matcher::v3::StringMatcher,
    >,
}
/// Nested message and enum types in `DnsTable`.
pub mod dns_table {
    /// This message contains a list of IP addresses returned for a query for a known name
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddressList {
        /// This field contains a well formed IP address that is returned in the answer for a
        /// name query. The address field can be an IPv4 or IPv6 address. Address family
        /// detection is done automatically when Envoy parses the string. Since this field is
        /// repeated, Envoy will return as many entries from this list in the DNS response while
        /// keeping the response under 512 bytes
        #[prost(string, repeated, tag = "1")]
        pub address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Specify the service protocol using a numeric or string value
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DnsServiceProtocol {
        #[prost(oneof = "dns_service_protocol::ProtocolConfig", tags = "1, 2")]
        pub protocol_config: ::core::option::Option<
            dns_service_protocol::ProtocolConfig,
        >,
    }
    /// Nested message and enum types in `DnsServiceProtocol`.
    pub mod dns_service_protocol {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ProtocolConfig {
            /// Specify the protocol number for the service. Envoy will try to resolve the number to
            /// the protocol name. For example, 6 will resolve to "tcp". Refer to:
            /// <https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml>
            /// for protocol names and numbers
            #[prost(uint32, tag = "1")]
            Number(u32),
            /// Specify the protocol name for the service.
            #[prost(string, tag = "2")]
            Name(::prost::alloc::string::String),
        }
    }
    /// Specify the target for a given DNS service
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DnsServiceTarget {
        /// The priority of the service record target
        #[prost(uint32, tag = "3")]
        pub priority: u32,
        /// The weight of the service record target
        #[prost(uint32, tag = "4")]
        pub weight: u32,
        /// The port to which the service is bound. This value is optional if the target is a
        /// cluster. Setting port to zero in this case makes the filter use the port value
        /// from the cluster host
        #[prost(uint32, tag = "5")]
        pub port: u32,
        /// Specify the name of the endpoint for the Service. The name is a hostname or a cluster
        #[prost(oneof = "dns_service_target::EndpointType", tags = "1, 2")]
        pub endpoint_type: ::core::option::Option<dns_service_target::EndpointType>,
    }
    /// Nested message and enum types in `DnsServiceTarget`.
    pub mod dns_service_target {
        /// Specify the name of the endpoint for the Service. The name is a hostname or a cluster
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum EndpointType {
            /// Use a resolvable hostname as the endpoint for a service.
            #[prost(string, tag = "1")]
            HostName(::prost::alloc::string::String),
            /// Use a cluster name as the endpoint for a service.
            #[prost(string, tag = "2")]
            ClusterName(::prost::alloc::string::String),
        }
    }
    /// This message defines a service selection record returned for a service query in a domain
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DnsService {
        /// The name of the service without the protocol or domain name
        #[prost(string, tag = "1")]
        pub service_name: ::prost::alloc::string::String,
        /// The service protocol. This can be specified as a string or the numeric value of the protocol
        #[prost(message, optional, tag = "2")]
        pub protocol: ::core::option::Option<DnsServiceProtocol>,
        /// The service entry time to live. This is independent from the DNS Answer record TTL
        #[prost(message, optional, tag = "3")]
        pub ttl: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
        /// The list of targets hosting the service
        #[prost(message, repeated, tag = "4")]
        pub targets: ::prost::alloc::vec::Vec<DnsServiceTarget>,
    }
    /// Define a list of service records for a given service
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DnsServiceList {
        #[prost(message, repeated, tag = "1")]
        pub services: ::prost::alloc::vec::Vec<DnsService>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DnsEndpoint {
        #[prost(oneof = "dns_endpoint::EndpointConfig", tags = "1, 2, 3")]
        pub endpoint_config: ::core::option::Option<dns_endpoint::EndpointConfig>,
    }
    /// Nested message and enum types in `DnsEndpoint`.
    pub mod dns_endpoint {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum EndpointConfig {
            /// Define a list of addresses to return for the specified endpoint
            #[prost(message, tag = "1")]
            AddressList(super::AddressList),
            /// Define a cluster whose addresses are returned for the specified endpoint
            #[prost(string, tag = "2")]
            ClusterName(::prost::alloc::string::String),
            /// Define a DNS Service List for the specified endpoint
            #[prost(message, tag = "3")]
            ServiceList(super::DnsServiceList),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DnsVirtualDomain {
        /// A domain name for which Envoy will respond to query requests.
        /// Wildcard records are supported on the first label only, e.g. `*.example.com` or `*.subdomain.example.com`.
        /// Names such as `*example.com`, `subdomain.*.example.com`, `*subdomain.example.com`, etc
        /// are not valid wildcard names and asterisk will be interpreted as a literal `*` character.
        /// Wildcard records match subdomains on any levels, e.g. `*.example.com` will match
        /// `foo.example.com`, `bar.foo.example.com`, `baz.bar.foo.example.com`, etc. In case there are multiple
        /// wildcard records, the longest wildcard match will be used, e.g. if there are wildcard records for
        /// `*.example.com` and `*.foo.example.com` and the query is for `bar.foo.example.com`, the latter will be used.
        /// Specific records will always take precedence over wildcard records.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The configuration containing the method to determine the address of this endpoint
        #[prost(message, optional, tag = "2")]
        pub endpoint: ::core::option::Option<DnsEndpoint>,
        /// Sets the TTL in DNS answers from Envoy returned to the client. The default TTL is 300s
        #[prost(message, optional, tag = "3")]
        pub answer_ttl: ::core::option::Option<
            super::super::super::super::super::google::protobuf::Duration,
        >,
    }
}
