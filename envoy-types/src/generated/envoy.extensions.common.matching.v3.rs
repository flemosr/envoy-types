/// Wrapper around an existing extension that provides an associated matcher. This allows
/// decorating an existing extension with a matcher, which can be used to match against
/// relevant protocol data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionWithMatcher {
    /// The associated matcher. This is deprecated in favor of xds_matcher.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub matcher: ::core::option::Option<
        super::super::super::super::config::common::matcher::v3::Matcher,
    >,
    /// The associated matcher.
    #[prost(message, optional, tag = "3")]
    pub xds_matcher: ::core::option::Option<
        super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
    /// The underlying extension config.
    #[prost(message, optional, tag = "2")]
    pub extension_config: ::core::option::Option<
        super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
}
/// Extra settings on a per virtualhost/route/weighted-cluster level.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionWithMatcherPerRoute {
    /// Matcher override.
    #[prost(message, optional, tag = "1")]
    pub xds_matcher: ::core::option::Option<
        super::super::super::super::super::xds::r#type::matcher::v3::Matcher,
    >,
}
