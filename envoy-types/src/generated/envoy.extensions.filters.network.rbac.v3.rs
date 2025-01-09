// This file is @generated by prost-build.
/// RBAC network filter config.
///
/// Header should not be used in rules/shadow_rules in RBAC network filter as
/// this information is only available in :ref:`RBAC http filter <config_http_filters_rbac>`.
/// \[\#next-free-field: 9\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rbac {
    /// Specify the RBAC rules to be applied globally.
    /// If absent, no enforcing RBAC policy will be applied.
    /// If present and empty, DENY.
    /// If both rules and matcher are configured, rules will be ignored.
    #[prost(message, optional, tag = "1")]
    pub rules: ::core::option::Option<
        super::super::super::super::super::config::rbac::v3::Rbac,
    >,
    /// The match tree to use when resolving RBAC action for incoming connections. Connections do
    /// not match any matcher will be denied.
    /// If absent, no enforcing RBAC matcher will be applied.
    /// If present and empty, deny all connections.
    #[prost(message, optional, tag = "6")]
    pub matcher: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    /// Shadow rules are not enforced by the filter but will emit stats and logs
    /// and can be used for rule testing.
    /// If absent, no shadow RBAC policy will be applied.
    /// If both shadow rules and shadow matcher are configured, shadow rules will be ignored.
    #[prost(message, optional, tag = "2")]
    pub shadow_rules: ::core::option::Option<
        super::super::super::super::super::config::rbac::v3::Rbac,
    >,
    /// The match tree to use for emitting stats and logs which can be used for rule testing for
    /// incoming connections.
    /// If absent, no shadow matcher will be applied.
    #[prost(message, optional, tag = "7")]
    pub shadow_matcher: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    /// If specified, shadow rules will emit stats with the given prefix.
    /// This is useful to distinguish the stat when there are more than 1 RBAC filter configured with
    /// shadow rules.
    #[prost(string, tag = "5")]
    pub shadow_rules_stat_prefix: ::prost::alloc::string::String,
    /// The prefix to use when emitting statistics.
    #[prost(string, tag = "3")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// RBAC enforcement strategy. By default RBAC will be enforced only once
    /// when the first byte of data arrives from the downstream. When used in
    /// conjunction with filters that emit dynamic metadata after decoding
    /// every payload (e.g., Mongo, MySQL, Kafka) set the enforcement type to
    /// CONTINUOUS to enforce RBAC policies on every message boundary.
    #[prost(enumeration = "rbac::EnforcementType", tag = "4")]
    pub enforcement_type: i32,
    /// Delay the specified duration before closing the connection when the policy evaluation
    /// result is `DENY`. If this is not present, the connection will be closed immediately.
    /// This is useful to provide a better protection for Envoy against clients that retries
    /// aggressively when the connection is rejected by the RBAC filter.
    #[prost(message, optional, tag = "8")]
    pub delay_deny: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
}
/// Nested message and enum types in `RBAC`.
pub mod rbac {
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
    pub enum EnforcementType {
        /// Apply RBAC policies when the first byte of data arrives on the connection.
        OneTimeOnFirstByte = 0,
        /// Continuously apply RBAC policies as data arrives. Use this mode when
        /// using RBAC with message oriented protocols such as Mongo, MySQL, Kafka,
        /// etc. when the protocol decoders emit dynamic metadata such as the
        /// resources being accessed and the operations on the resources.
        Continuous = 1,
    }
    impl EnforcementType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::OneTimeOnFirstByte => "ONE_TIME_ON_FIRST_BYTE",
                Self::Continuous => "CONTINUOUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ONE_TIME_ON_FIRST_BYTE" => Some(Self::OneTimeOnFirstByte),
                "CONTINUOUS" => Some(Self::Continuous),
                _ => None,
            }
        }
    }
}
