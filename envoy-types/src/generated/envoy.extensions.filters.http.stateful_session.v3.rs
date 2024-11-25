// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSession {
    /// Specifies the implementation of session state. This session state is used to store and retrieve the address of the
    /// upstream host assigned to the session.
    ///
    /// \[\#extension-category: envoy.http.stateful_session\]
    #[prost(message, optional, tag = "1")]
    pub session_state: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// Determines whether the HTTP request must be strictly routed to the requested destination. When set to `true`,
    /// if the requested destination is unavailable, Envoy will return a 503 status code. The default value is `false`,
    /// which allows Envoy to fall back to its load balancing mechanism. In this case, if the requested destination is not
    /// found, the request will be routed according to the load balancing algorithm.
    #[prost(bool, tag = "2")]
    pub strict: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSessionPerRoute {
    #[prost(oneof = "stateful_session_per_route::Override", tags = "1, 2")]
    pub r#override: ::core::option::Option<stateful_session_per_route::Override>,
}
/// Nested message and enum types in `StatefulSessionPerRoute`.
pub mod stateful_session_per_route {
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
