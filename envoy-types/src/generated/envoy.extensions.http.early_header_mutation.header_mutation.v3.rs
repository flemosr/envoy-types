/// This extension allows for early header mutation by the substitution formatter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutation {
    #[prost(message, repeated, tag = "1")]
    pub mutations: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::common::mutation_rules::v3::HeaderMutation,
    >,
}
