/// Configuration for apple DNS resolver.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppleDnsResolverConfig {
    /// The resolver will avoid the system's heuristics to only return
    /// IPv4 or IPv6 addresses that it considers to be "routable", instead
    /// returning all possible IPv4 or IPv6 addresses. This setting is
    /// ignored if the DNS lookup family is set to v4-only or v6-only.
    /// This should remain false in the vast majority of cases, but may be
    /// useful when performing custom filtering of addresses, such as with
    /// Happy Eyeballs.
    #[prost(bool, tag = "1")]
    pub include_unroutable_families: bool,
}
