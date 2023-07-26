/// An internal redirect predicate that checks the scheme between the
/// downstream url and the redirect target url and allows a) same scheme
/// redirect and b) safe cross scheme redirect, which means if the downstream
/// scheme is HTTPS, both HTTPS and HTTP redirect targets are allowed, but if the
/// downstream scheme is HTTP, only HTTP redirect targets are allowed.
/// [#extension: envoy.internal_redirect_predicates.safe_cross_scheme]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SafeCrossSchemeConfig {}
