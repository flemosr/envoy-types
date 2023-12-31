/// An internal redirect predicate that rejects redirect targets that are pointing
/// to a route that has been followed by a previous redirect from the current route.
/// \[\#extension: envoy.internal_redirect_predicates.previous_routes\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviousRoutesConfig {}
