/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BandwidthLimit {
    /// The human readable prefix to use when emitting stats.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// The enable mode for the bandwidth limit filter.
    /// Default is Disabled.
    #[prost(enumeration = "bandwidth_limit::EnableMode", tag = "2")]
    pub enable_mode: i32,
    /// The limit supplied in KiB/s.
    ///
    /// .. note::
    /// It's fine for the limit to be unset for the global configuration since the bandwidth limit
    /// can be applied at a the virtual host or route level. Thus, the limit must be set for the
    /// per route configuration otherwise the config will be rejected.
    ///
    /// .. note::
    /// When using per route configuration, the limit becomes unique to that route.
    #[prost(message, optional, tag = "3")]
    pub limit_kbps: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// Optional Fill interval in milliseconds for the token refills. Defaults to 50ms.
    /// It must be at least 20ms to avoid too aggressive refills.
    #[prost(message, optional, tag = "4")]
    pub fill_interval: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Runtime flag that controls whether the filter is enabled or not. If not specified, defaults
    /// to enabled.
    #[prost(message, optional, tag = "5")]
    pub runtime_enabled: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFeatureFlag,
    >,
    /// Enable response trailers.
    ///
    /// .. note::
    ///
    /// If set true, the following 4 trailers will be added, prefixed by `response_trailer_prefix`:
    ///
    /// * bandwidth-request-delay-ms: delay time in milliseconds it took for the request stream transfer including request body transfer time and the time added by the filter.
    /// * bandwidth-response-delay-ms: delay time in milliseconds it took for the response stream transfer including response body transfer time and the time added by the filter.
    /// * bandwidth-request-filter-delay-ms: delay time in milliseconds in request stream transfer added by the filter.
    /// * bandwidth-response-filter-delay-ms: delay time in milliseconds that added by the filter.
    ///   If :ref:`enable_mode <envoy_v3_api_field_extensions.filters.http.bandwidth_limit.v3.BandwidthLimit.enable_mode>` is `DISABLED` or `REQUEST`, the trailers will not be set.
    ///   If both the request and response delay time is 0, the trailers will not be set.
    #[prost(bool, tag = "6")]
    pub enable_response_trailers: bool,
    /// Optional The prefix for the response trailers.
    #[prost(string, tag = "7")]
    pub response_trailer_prefix: ::prost::alloc::string::String,
}
/// Nested message and enum types in `BandwidthLimit`.
pub mod bandwidth_limit {
    /// Defines the mode for the bandwidth limit filter.
    /// Values represent bitmask.
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
    pub enum EnableMode {
        /// Filter is disabled.
        Disabled = 0,
        /// Filter enabled only for incoming traffic.
        Request = 1,
        /// Filter enabled only for outgoing traffic.
        Response = 2,
        /// Filter enabled for both incoming and outgoing traffic.
        RequestAndResponse = 3,
    }
    impl EnableMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EnableMode::Disabled => "DISABLED",
                EnableMode::Request => "REQUEST",
                EnableMode::Response => "RESPONSE",
                EnableMode::RequestAndResponse => "REQUEST_AND_RESPONSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DISABLED" => Some(Self::Disabled),
                "REQUEST" => Some(Self::Request),
                "RESPONSE" => Some(Self::Response),
                "REQUEST_AND_RESPONSE" => Some(Self::RequestAndResponse),
                _ => None,
            }
        }
    }
}
