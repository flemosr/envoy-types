#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mutations {
    /// The request mutations are applied before the request is forwarded to the upstream cluster.
    #[prost(message, repeated, tag = "1")]
    pub request_mutations: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::common::mutation_rules::v3::HeaderMutation,
    >,
    /// The response mutations are applied before the response is sent to the downstream client.
    #[prost(message, repeated, tag = "2")]
    pub response_mutations: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::common::mutation_rules::v3::HeaderMutation,
    >,
}
/// Per route configuration for the header mutation filter. If this is configured at multiple levels
/// (route level, virtual host level, and route table level), only the most specific one will be used.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutationPerRoute {
    #[prost(message, optional, tag = "1")]
    pub mutations: ::core::option::Option<Mutations>,
}
/// Configuration for the header mutation filter. The mutation rules in the filter configuration will
/// always be applied first and then the per-route mutation rules, if both are specified.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutation {
    #[prost(message, optional, tag = "1")]
    pub mutations: ::core::option::Option<Mutations>,
}
