// This file is @generated by prost-build.
/// Delay specification is used to inject latency into the
/// HTTP/Mongo operation.
/// \[\#next-free-field: 6\]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct FaultDelay {
    /// The percentage of operations/connections/requests on which the delay will be injected.
    #[prost(message, optional, tag = "4")]
    pub percentage: ::core::option::Option<
        super::super::super::super::super::r#type::v3::FractionalPercent,
    >,
    #[prost(oneof = "fault_delay::FaultDelaySecifier", tags = "3, 5")]
    pub fault_delay_secifier: ::core::option::Option<fault_delay::FaultDelaySecifier>,
}
/// Nested message and enum types in `FaultDelay`.
pub mod fault_delay {
    ///
    /// Fault delays are controlled via an HTTP header (if applicable). See the
    /// : ref:`HTTP fault filter <config_http_filters_fault_injection_http_header>` documentation for
    ///   more information.
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct HeaderDelay {}
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
    pub enum FaultDelayType {
        /// Unused and deprecated.
        Fixed = 0,
    }
    impl FaultDelayType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Fixed => "FIXED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FIXED" => Some(Self::Fixed),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum FaultDelaySecifier {
        /// Add a fixed delay before forwarding the operation upstream. See
        /// <https://developers.google.com/protocol-buffers/docs/proto3#json> for
        /// the JSON/YAML Duration mapping. For HTTP/Mongo, the specified
        /// delay will be injected before a new request/operation.
        /// This is required if type is FIXED.
        #[prost(message, tag = "3")]
        FixedDelay(
            super::super::super::super::super::super::super::google::protobuf::Duration,
        ),
        /// Fault delays are controlled via an HTTP header (if applicable).
        #[prost(message, tag = "5")]
        HeaderDelay(HeaderDelay),
    }
}
/// Describes a rate limit to be applied.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct FaultRateLimit {
    /// The percentage of operations/connections/requests on which the rate limit will be injected.
    #[prost(message, optional, tag = "2")]
    pub percentage: ::core::option::Option<
        super::super::super::super::super::r#type::v3::FractionalPercent,
    >,
    #[prost(oneof = "fault_rate_limit::LimitType", tags = "1, 3")]
    pub limit_type: ::core::option::Option<fault_rate_limit::LimitType>,
}
/// Nested message and enum types in `FaultRateLimit`.
pub mod fault_rate_limit {
    /// Describes a fixed/constant rate limit.
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct FixedLimit {
        /// The limit supplied in KiB/s.
        #[prost(uint64, tag = "1")]
        pub limit_kbps: u64,
    }
    ///
    /// Rate limits are controlled via an HTTP header (if applicable). See the
    /// : ref:`HTTP fault filter <config_http_filters_fault_injection_http_header>` documentation for
    ///   more information.
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct HeaderLimit {}
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum LimitType {
        /// A fixed rate limit.
        #[prost(message, tag = "1")]
        FixedLimit(FixedLimit),
        /// Rate limits are controlled via an HTTP header (if applicable).
        #[prost(message, tag = "3")]
        HeaderLimit(HeaderLimit),
    }
}
