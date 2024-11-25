// This file is @generated by prost-build.
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
/// Per route configuration for the header mutation filter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutationPerRoute {
    #[prost(message, optional, tag = "1")]
    pub mutations: ::core::option::Option<Mutations>,
}
/// Configuration for the header mutation filter. The mutation rules in the filter configuration will
/// always be applied first and then the per-route mutation rules, if both are specified.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutation {
    #[prost(message, optional, tag = "1")]
    pub mutations: ::core::option::Option<Mutations>,
    /// If per route HeaderMutationPerRoute config is configured at multiple route levels, header mutations
    /// at all specified levels are evaluated. By default, the order is from most specific (i.e. route entry level)
    /// to least specific (i.e. route configuration level). Later header mutations may override earlier mutations.
    ///
    /// This order can be reversed by setting this field to true. In other words, most specific level mutation
    /// is evaluated last.
    #[prost(bool, tag = "2")]
    pub most_specific_header_mutations_wins: bool,
}
