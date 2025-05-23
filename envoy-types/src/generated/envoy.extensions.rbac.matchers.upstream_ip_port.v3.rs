// This file is @generated by prost-build.
///
/// This is configuration for matching upstream ip and port.
/// Note that although both fields are optional, at least one of IP or port must be supplied. If only
/// one is supplied the other is a wildcard match.
/// This matcher requires a filter in the chain to have saved the upstream address in the
/// filter state before the matcher is executed by RBAC filter. The state should be saved with key
/// `envoy.stream.upstream_address` (See
/// : repo:`upstream_address.h<source/common/stream_info/upstream_address.h>`).
///   Also, See :repo:`proxy_filter.cc<source/extensions/filters/http/dynamic_forward_proxy/proxy_filter.cc>`
///   for an example of a filter which populates the FilterState.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamIpPortMatcher {
    /// A CIDR block that will be used to match the upstream IP.
    /// Both Ipv4 and Ipv6 ranges can be matched.
    #[prost(message, optional, tag = "1")]
    pub upstream_ip: ::core::option::Option<
        super::super::super::super::super::config::core::v3::CidrRange,
    >,
    /// A port range that will be used to match the upstream port.
    #[prost(message, optional, tag = "2")]
    pub upstream_port_range: ::core::option::Option<
        super::super::super::super::super::r#type::v3::Int64Range,
    >,
}
