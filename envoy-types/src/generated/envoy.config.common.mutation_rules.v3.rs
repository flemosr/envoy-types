/// The HeaderMutationRules structure specifies what headers may be
/// manipulated by a processing filter. This set of rules makes it
/// possible to control which modifications a filter may make.
///
/// By default, an external processing server may add, modify, or remove
/// any header except for an "Envoy internal" header (which is typically
/// denoted by an x-envoy prefix) or specific headers that may affect
/// further filter processing:
///
/// * ``host``
/// * ``:authority``
/// * ``:scheme``
/// * ``:method``
///
/// Every attempt to add, change, append, or remove a header will be
/// tested against the rules here. Disallowed header mutations will be
/// ignored unless ``disallow_is_error`` is set to true.
///
/// Attempts to remove headers are further constrained -- regardless of the
/// settings, system-defined headers (that start with ``:``) and the ``host``
/// header may never be removed.
///
/// In addition, a counter will be incremented whenever a mutation is
/// rejected. In the ext_proc filter, that counter is named
/// ``rejected_header_mutations``.
/// [#next-free-field: 8]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutationRules {
    /// By default, certain headers that could affect processing of subsequent
    /// filters or request routing cannot be modified. These headers are
    /// ``host``, ``:authority``, ``:scheme``, and ``:method``. Setting this parameter
    /// to true allows these headers to be modified as well.
    #[prost(message, optional, tag = "1")]
    pub allow_all_routing: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// If true, allow modification of envoy internal headers. By default, these
    /// start with ``x-envoy`` but this may be overridden in the ``Bootstrap``
    /// configuration using the
    /// :ref:`header_prefix <envoy_v3_api_field_config.bootstrap.v3.Bootstrap.header_prefix>`
    /// field. Default is false.
    #[prost(message, optional, tag = "2")]
    pub allow_envoy: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// If true, prevent modification of any system header, defined as a header
    /// that starts with a ``:`` character, regardless of any other settings.
    /// A processing server may still override the ``:status`` of an HTTP response
    /// using an ``ImmediateResponse`` message. Default is false.
    #[prost(message, optional, tag = "3")]
    pub disallow_system: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// If true, prevent modifications of all header values, regardless of any
    /// other settings. A processing server may still override the ``:status``
    /// of an HTTP response using an ``ImmediateResponse`` message. Default is false.
    #[prost(message, optional, tag = "4")]
    pub disallow_all: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// If set, specifically allow any header that matches this regular
    /// expression. This overrides all other settings except for
    /// ``disallow_expression``.
    #[prost(message, optional, tag = "5")]
    pub allow_expression: ::core::option::Option<
        super::super::super::super::r#type::matcher::v3::RegexMatcher,
    >,
    /// If set, specifically disallow any header that matches this regular
    /// expression regardless of any other settings.
    #[prost(message, optional, tag = "6")]
    pub disallow_expression: ::core::option::Option<
        super::super::super::super::r#type::matcher::v3::RegexMatcher,
    >,
    /// If true, and if the rules in this list cause a header mutation to be
    /// disallowed, then the filter using this configuration will terminate the
    /// request with a 500 error. In addition, regardless of the setting of this
    /// parameter, any attempt to set, add, or modify a disallowed header will
    /// cause the ``rejected_header_mutations`` counter to be incremented.
    /// Default is false.
    #[prost(message, optional, tag = "7")]
    pub disallow_is_error: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
/// The HeaderMutation structure specifies an action that may be taken on HTTP
/// headers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderMutation {
    #[prost(oneof = "header_mutation::Action", tags = "1, 2")]
    pub action: ::core::option::Option<header_mutation::Action>,
}
/// Nested message and enum types in `HeaderMutation`.
pub mod header_mutation {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// Remove the specified header if it exists.
        #[prost(string, tag = "1")]
        Remove(::prost::alloc::string::String),
        /// Append new header by the specified HeaderValueOption.
        #[prost(message, tag = "2")]
        Append(super::super::super::super::core::v3::HeaderValueOption),
    }
}
