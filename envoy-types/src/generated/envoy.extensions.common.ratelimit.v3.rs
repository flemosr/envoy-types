/// A RateLimitDescriptor is a list of hierarchical entries that are used by the service to
/// determine the final rate limit key and overall allowed limit. Here are some examples of how
/// they might be used for the domain "envoy".
///
/// .. code-block:: cpp
///
/// \["authenticated": "false"\], \["remote_address": "10.0.0.1"\]
///
/// What it does: Limits all unauthenticated traffic for the IP address 10.0.0.1. The
/// configuration supplies a default limit for the *remote_address* key. If there is a desire to
/// raise the limit for 10.0.0.1 or block it entirely it can be specified directly in the
/// configuration.
///
/// .. code-block:: cpp
///
/// \["authenticated": "false"\], \["path": "/foo/bar"\]
///
/// What it does: Limits all unauthenticated traffic globally for a specific path (or prefix if
/// configured that way in the service).
///
/// .. code-block:: cpp
///
/// \["authenticated": "false"\], \["path": "/foo/bar"\], \["remote_address": "10.0.0.1"\]
///
/// What it does: Limits unauthenticated traffic to a specific path for a specific IP address.
/// Like (1) we can raise/block specific IP addresses if we want with an override configuration.
///
/// .. code-block:: cpp
///
/// \["authenticated": "true"\], \["client_id": "foo"\]
///
/// What it does: Limits all traffic for an authenticated client "foo"
///
/// .. code-block:: cpp
///
/// \["authenticated": "true"\], \["client_id": "foo"\], \["path": "/foo/bar"\]
///
/// What it does: Limits traffic to a specific path for an authenticated client "foo"
///
/// The idea behind the API is that (1)/(2)/(3) and (4)/(5) can be sent in 1 request if desired.
/// This enables building complex application scenarios with a generic backend.
///
/// Optionally the descriptor can contain a limit override under a "limit" key, that specifies
/// the number of requests per unit to use instead of the number configured in the
/// rate limiting service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitDescriptor {
    /// Descriptor entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<rate_limit_descriptor::Entry>,
    /// Optional rate limit override to supply to the ratelimit service.
    #[prost(message, optional, tag = "2")]
    pub limit: ::core::option::Option<rate_limit_descriptor::RateLimitOverride>,
}
/// Nested message and enum types in `RateLimitDescriptor`.
pub mod rate_limit_descriptor {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        /// Descriptor key.
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// Descriptor value.
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
    }
    /// Override rate limit to apply to this descriptor instead of the limit
    /// configured in the rate limit service. See :ref:`rate limit override <config_http_filters_rate_limit_rate_limit_override>` for more information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RateLimitOverride {
        /// The number of requests per unit of time.
        #[prost(uint32, tag = "1")]
        pub requests_per_unit: u32,
        /// The unit of time.
        #[prost(
            enumeration = "super::super::super::super::super::r#type::v3::RateLimitUnit",
            tag = "2"
        )]
        pub unit: i32,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalRateLimitDescriptor {
    /// Descriptor entries.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<rate_limit_descriptor::Entry>,
    /// Token Bucket algorithm for local ratelimiting.
    #[prost(message, optional, tag = "2")]
    pub token_bucket: ::core::option::Option<
        super::super::super::super::r#type::v3::TokenBucket,
    >,
}
/// Defines the version of the standard to use for X-RateLimit headers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum XRateLimitHeadersRfcVersion {
    /// X-RateLimit headers disabled.
    Off = 0,
    /// Use `draft RFC Version 03 <<https://tools.ietf.org/id/draft-polli-ratelimit-headers-03.html>`\_> where 3 headers will be added:
    ///
    /// * `X-RateLimit-Limit` - indicates the request-quota associated to the
    ///   client in the current time-window followed by the description of the
    ///   quota policy. The value is returned by the maximum tokens of the token bucket.
    /// * `X-RateLimit-Remaining` - indicates the remaining requests in the
    ///   current time-window. The value is returned by the remaining tokens in the token bucket.
    /// * `X-RateLimit-Reset` - indicates the number of seconds until reset of
    ///   the current time-window. The value is returned by the remaining fill interval of the token bucket.
    DraftVersion03 = 1,
}
impl XRateLimitHeadersRfcVersion {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            XRateLimitHeadersRfcVersion::Off => "OFF",
            XRateLimitHeadersRfcVersion::DraftVersion03 => "DRAFT_VERSION_03",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFF" => Some(Self::Off),
            "DRAFT_VERSION_03" => Some(Self::DraftVersion03),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VhRateLimitsOptions {
    /// Use the virtual host rate limits unless the route has a rate limit policy.
    Override = 0,
    /// Use the virtual host rate limits even if the route has a rate limit policy.
    Include = 1,
    /// Ignore the virtual host rate limits even if the route does not have a rate limit policy.
    Ignore = 2,
}
impl VhRateLimitsOptions {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VhRateLimitsOptions::Override => "OVERRIDE",
            VhRateLimitsOptions::Include => "INCLUDE",
            VhRateLimitsOptions::Ignore => "IGNORE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OVERRIDE" => Some(Self::Override),
            "INCLUDE" => Some(Self::Include),
            "IGNORE" => Some(Self::Ignore),
            _ => None,
        }
    }
}
