#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Geoip {
    /// If set, the :ref:`xff_num_trusted_hops <envoy_v3_api_field_extensions.filters.http.geoip.v3.Geoip.XffConfig.xff_num_trusted_hops>` field will be used to determine
    /// trusted client address from `x-forwarded-for` header.
    /// Otherwise, the immediate downstream connection source address will be used.
    /// \[\#next-free-field: 2\]
    #[prost(message, optional, tag = "1")]
    pub xff_config: ::core::option::Option<geoip::XffConfig>,
    /// Configuration for geolocation headers to add to request.
    #[prost(message, optional, tag = "2")]
    pub geo_headers_to_add: ::core::option::Option<geoip::GeolocationHeadersToAdd>,
    /// Geolocation provider specific configuration.
    #[prost(message, optional, tag = "3")]
    pub provider: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
}
/// Nested message and enum types in `Geoip`.
pub mod geoip {
    /// The set of geolocation headers to add to request. If any of the configured headers is present
    /// in the incoming request, it will be overridden by Geoip filter.
    /// \[\#next-free-field: 10\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GeolocationHeadersToAdd {
        /// If set, the header will be used to populate the country ISO code associated with the IP address.
        #[prost(string, tag = "1")]
        pub country: ::prost::alloc::string::String,
        /// If set, the header will be used to populate the city associated with the IP address.
        #[prost(string, tag = "2")]
        pub city: ::prost::alloc::string::String,
        /// If set, the header will be used to populate the region ISO code associated with the IP address.
        #[prost(string, tag = "3")]
        pub region: ::prost::alloc::string::String,
        /// If set, the header will be used to populate the ASN associated with the IP address.
        #[prost(string, tag = "4")]
        pub asn: ::prost::alloc::string::String,
        /// If set, the IP address will be checked if it belongs to any type of anonymization network (e.g. VPN, public proxy etc)
        /// and header will be populated with the check result. Header value will be set to either "true" or "false" depending on the check result.
        #[prost(string, tag = "5")]
        pub is_anon: ::prost::alloc::string::String,
        /// If set, the IP address will be checked if it belongs to a VPN and header will be populated with the check result.
        /// Header value will be set to either "true" or "false" depending on the check result.
        #[prost(string, tag = "6")]
        pub anon_vpn: ::prost::alloc::string::String,
        /// If set, the IP address will be checked if it belongs to a hosting provider and header will be populated with the check result.
        /// Header value will be set to either "true" or "false" depending on the check result.
        #[prost(string, tag = "7")]
        pub anon_hosting: ::prost::alloc::string::String,
        /// If set, the IP address will be checked if it belongs to a TOR exit node and header will be populated with the check result.
        /// Header value will be set to either "true" or "false" depending on the check result.
        #[prost(string, tag = "8")]
        pub anon_tor: ::prost::alloc::string::String,
        /// If set, the IP address will be checked if it belongs to a public proxy and header will be populated with the check result.
        /// Header value will be set to either "true" or "false" depending on the check result.
        #[prost(string, tag = "9")]
        pub anon_proxy: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct XffConfig {
        /// The number of additional ingress proxy hops from the right side of the
        /// :ref:`config_http_conn_man_headers_x-forwarded-for` HTTP header to trust when
        /// determining the origin client's IP address. The default is zero if this option
        /// is not specified. See the documentation for
        /// :ref:`config_http_conn_man_headers_x-forwarded-for` for more information.
        #[prost(uint32, tag = "1")]
        pub xff_num_trusted_hops: u32,
    }
}
