/// An internal redirect predicate that accepts only explicitly allowed target routes.
/// [#extension: envoy.internal_redirect_predicates.allow_listed_routes]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowListedRoutesConfig {
    /// The list of routes that's allowed as redirect target by this predicate,
    /// identified by the route's :ref:`name <envoy_v3_api_field_config.route.v3.Route.route>`.
    /// Empty route names are not allowed.
    #[prost(string, repeated, tag = "1")]
    pub allowed_route_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
