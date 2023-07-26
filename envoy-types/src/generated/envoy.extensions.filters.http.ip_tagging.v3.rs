#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpTagging {
    /// The type of request the filter should apply to.
    #[prost(enumeration = "ip_tagging::RequestType", tag = "1")]
    pub request_type: i32,
    /// [#comment:TODO(ccaraman): Extend functionality to load IP tags from file system.
    /// Tracked by issue <https://github.com/envoyproxy/envoy/issues/2695]>
    /// The set of IP tags for the filter.
    #[prost(message, repeated, tag = "4")]
    pub ip_tags: ::prost::alloc::vec::Vec<ip_tagging::IpTag>,
}
/// Nested message and enum types in `IPTagging`.
pub mod ip_tagging {
    /// Supplies the IP tag name and the IP address subnets.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IpTag {
        /// Specifies the IP tag name to apply.
        #[prost(string, tag = "1")]
        pub ip_tag_name: ::prost::alloc::string::String,
        /// A list of IP address subnets that will be tagged with
        /// ip_tag_name. Both IPv4 and IPv6 are supported.
        #[prost(message, repeated, tag = "2")]
        pub ip_list: ::prost::alloc::vec::Vec<
            super::super::super::super::super::super::config::core::v3::CidrRange,
        >,
    }
    /// The type of requests the filter should apply to. The supported types
    /// are internal, external or both. The
    /// :ref:`x-forwarded-for<config_http_conn_man_headers_x-forwarded-for_internal_origin>` header is
    /// used to determine if a request is internal and will result in
    /// :ref:`x-envoy-internal<config_http_conn_man_headers_x-envoy-internal>`
    /// being set. The filter defaults to both, and it will apply to all request types.
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
    pub enum RequestType {
        /// Both external and internal requests will be tagged. This is the default value.
        Both = 0,
        /// Only internal requests will be tagged.
        Internal = 1,
        /// Only external requests will be tagged.
        External = 2,
    }
    impl RequestType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RequestType::Both => "BOTH",
                RequestType::Internal => "INTERNAL",
                RequestType::External => "EXTERNAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BOTH" => Some(Self::Both),
                "INTERNAL" => Some(Self::Internal),
                "EXTERNAL" => Some(Self::External),
                _ => None,
            }
        }
    }
}
