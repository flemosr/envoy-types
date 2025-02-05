// This file is @generated by prost-build.
/// RBAC filter config.
/// \[\#next-free-field: 8\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rbac {
    /// The primary RBAC policy which will be applied globally, to all the incoming requests.
    ///
    /// * If absent, no RBAC enforcement occurs.
    /// * If set but empty, all requests are denied.
    ///
    /// .. note::
    ///
    /// When both `rules` and `matcher` are configured, `rules` will be ignored.
    #[prost(message, optional, tag = "1")]
    pub rules: ::core::option::Option<
        super::super::super::super::super::config::rbac::v3::Rbac,
    >,
    /// If specified, rules will emit stats with the given prefix.
    /// This is useful for distinguishing metrics when multiple RBAC filters are configured.
    #[prost(string, tag = "6")]
    pub rules_stat_prefix: ::prost::alloc::string::String,
    /// Match tree for evaluating RBAC actions on incoming requests. Requests not matching any matcher will be denied.
    ///
    /// * If absent, no RBAC enforcement occurs.
    /// * If set but empty, all requests are denied.
    #[prost(message, optional, tag = "4")]
    pub matcher: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    /// Shadow policy for testing RBAC rules without enforcing them. These rules generate stats and logs but do not deny
    /// requests. If absent, no shadow RBAC policy will be applied.
    ///
    /// .. note::
    ///
    /// When both `shadow_rules` and `shadow_matcher` are configured, `shadow_rules` will be ignored.
    #[prost(message, optional, tag = "2")]
    pub shadow_rules: ::core::option::Option<
        super::super::super::super::super::config::rbac::v3::Rbac,
    >,
    /// If absent, no shadow matcher will be applied.
    /// Match tree for testing RBAC rules through stats and logs without enforcing them.
    /// If absent, no shadow matching occurs.
    #[prost(message, optional, tag = "5")]
    pub shadow_matcher: ::core::option::Option<
        super::super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    /// If specified, shadow rules will emit stats with the given prefix.
    /// This is useful for distinguishing metrics when multiple RBAC filters use shadow rules.
    #[prost(string, tag = "3")]
    pub shadow_rules_stat_prefix: ::prost::alloc::string::String,
    /// If `track_per_rule_stats` is `true`, counters will be published for each rule and shadow rule.
    #[prost(bool, tag = "7")]
    pub track_per_rule_stats: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RbacPerRoute {
    /// Per-route specific RBAC configuration that overrides the global RBAC configuration.
    /// If absent, RBAC policy will be disabled for this route.
    #[prost(message, optional, tag = "2")]
    pub rbac: ::core::option::Option<Rbac>,
}
