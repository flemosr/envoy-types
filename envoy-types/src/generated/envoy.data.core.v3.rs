/// [#next-free-field: 10]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckEvent {
    #[prost(enumeration = "HealthCheckerType", tag = "1")]
    pub health_checker_type: i32,
    #[prost(message, optional, tag = "2")]
    pub host: ::core::option::Option<super::super::super::config::core::v3::Address>,
    #[prost(string, tag = "3")]
    pub cluster_name: ::prost::alloc::string::String,
    /// Timestamp for event.
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<
        super::super::super::super::google::protobuf::Timestamp,
    >,
    #[prost(oneof = "health_check_event::Event", tags = "4, 5, 7, 8, 9")]
    pub event: ::core::option::Option<health_check_event::Event>,
}
/// Nested message and enum types in `HealthCheckEvent`.
pub mod health_check_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// Host ejection.
        #[prost(message, tag = "4")]
        EjectUnhealthyEvent(super::HealthCheckEjectUnhealthy),
        /// Host addition.
        #[prost(message, tag = "5")]
        AddHealthyEvent(super::HealthCheckAddHealthy),
        /// Host failure.
        #[prost(message, tag = "7")]
        HealthCheckFailureEvent(super::HealthCheckFailure),
        /// Healthy host became degraded.
        #[prost(message, tag = "8")]
        DegradedHealthyHost(super::DegradedHealthyHost),
        /// A degraded host returned to being healthy.
        #[prost(message, tag = "9")]
        NoLongerDegradedHost(super::NoLongerDegradedHost),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckEjectUnhealthy {
    /// The type of failure that caused this ejection.
    #[prost(enumeration = "HealthCheckFailureType", tag = "1")]
    pub failure_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckAddHealthy {
    /// Whether this addition is the result of the first ever health check on a host, in which case
    /// the configured :ref:`healthy threshold <envoy_v3_api_field_config.core.v3.HealthCheck.healthy_threshold>`
    /// is bypassed and the host is immediately added.
    #[prost(bool, tag = "1")]
    pub first_check: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckFailure {
    /// The type of failure that caused this event.
    #[prost(enumeration = "HealthCheckFailureType", tag = "1")]
    pub failure_type: i32,
    /// Whether this event is the result of the first ever health check on a host.
    #[prost(bool, tag = "2")]
    pub first_check: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DegradedHealthyHost {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoLongerDegradedHost {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HealthCheckFailureType {
    Active = 0,
    Passive = 1,
    Network = 2,
    NetworkTimeout = 3,
}
impl HealthCheckFailureType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HealthCheckFailureType::Active => "ACTIVE",
            HealthCheckFailureType::Passive => "PASSIVE",
            HealthCheckFailureType::Network => "NETWORK",
            HealthCheckFailureType::NetworkTimeout => "NETWORK_TIMEOUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACTIVE" => Some(Self::Active),
            "PASSIVE" => Some(Self::Passive),
            "NETWORK" => Some(Self::Network),
            "NETWORK_TIMEOUT" => Some(Self::NetworkTimeout),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HealthCheckerType {
    Http = 0,
    Tcp = 1,
    Grpc = 2,
    Redis = 3,
    Thrift = 4,
}
impl HealthCheckerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HealthCheckerType::Http => "HTTP",
            HealthCheckerType::Tcp => "TCP",
            HealthCheckerType::Grpc => "GRPC",
            HealthCheckerType::Redis => "REDIS",
            HealthCheckerType::Thrift => "THRIFT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HTTP" => Some(Self::Http),
            "TCP" => Some(Self::Tcp),
            "GRPC" => Some(Self::Grpc),
            "REDIS" => Some(Self::Redis),
            "THRIFT" => Some(Self::Thrift),
            _ => None,
        }
    }
}
