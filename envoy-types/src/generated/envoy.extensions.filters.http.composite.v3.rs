/// :ref:`Composite filter <config_http_filters_composite>` config. The composite filter config
/// allows delegating filter handling to another filter as determined by matching on the request
/// headers. This makes it possible to use different filters or filter configurations based on the
/// incoming request.
///
/// This is intended to be used with
/// :ref:`ExtensionWithMatcher <envoy_v3_api_msg_extensions.common.matching.v3.ExtensionWithMatcher>`
/// where a match tree is specified that indicates (via
/// :ref:`ExecuteFilterAction <envoy_v3_api_msg_extensions.filters.http.composite.v3.ExecuteFilterAction>`)
/// which filter configuration to create and delegate to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Composite {}
/// Composite match action (see :ref:`matching docs <arch_overview_matching_api>` for more info on match actions).
/// This specifies the filter configuration of the filter that the composite filter should delegate filter interactions to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteFilterAction {
    #[prost(message, optional, tag = "1")]
    pub typed_config: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
}
