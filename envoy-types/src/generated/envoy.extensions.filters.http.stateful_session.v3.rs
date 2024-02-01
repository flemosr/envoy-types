#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSession {
    /// Specific implementation of session state. This session state will be used to store and
    /// get address of the upstream host to which the session is assigned.
    ///
    /// \[\#extension-category: envoy.http.stateful_session\]
    #[prost(message, optional, tag = "1")]
    pub session_state: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// If set to True, the HTTP request must be routed to the requested destination.
    /// If the requested destination is not available, Envoy returns 503. Defaults to False.
    #[prost(bool, tag = "2")]
    pub strict: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSessionPerRoute {
    #[prost(oneof = "stateful_session_per_route::Override", tags = "1, 2")]
    pub r#override: ::core::option::Option<stateful_session_per_route::Override>,
}
/// Nested message and enum types in `StatefulSessionPerRoute`.
pub mod stateful_session_per_route {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Override {
        /// Disable the stateful session filter for this particular vhost or route. If disabled is
        /// specified in multiple per-filter-configs, the most specific one will be used.
        #[prost(bool, tag = "1")]
        Disabled(bool),
        /// Per-route stateful session configuration that can be served by RDS or static route table.
        #[prost(message, tag = "2")]
        StatefulSession(super::StatefulSession),
    }
}
