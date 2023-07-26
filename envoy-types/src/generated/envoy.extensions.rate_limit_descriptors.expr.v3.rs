/// The following descriptor entry is appended with a value computed
/// from a symbolic Common Expression Language expression.
/// See :ref:`attributes <arch_overview_attributes>` for the set of
/// available attributes.
///
/// .. code-block:: cpp
///
///    ("<descriptor_key>", "<expression_value>")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Descriptor {
    /// The key to use in the descriptor entry.
    #[prost(string, tag = "1")]
    pub descriptor_key: ::prost::alloc::string::String,
    /// If set to true, Envoy skips the descriptor if the expression evaluates to an error.
    /// By default, the rate limit is not applied when an expression produces an error.
    #[prost(bool, tag = "2")]
    pub skip_if_error: bool,
    #[prost(oneof = "descriptor::ExprSpecifier", tags = "3, 4")]
    pub expr_specifier: ::core::option::Option<descriptor::ExprSpecifier>,
}
/// Nested message and enum types in `Descriptor`.
pub mod descriptor {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExprSpecifier {
        /// Expression in a text form, e.g. "connection.requested_server_name".
        #[prost(string, tag = "3")]
        Text(::prost::alloc::string::String),
        /// Parsed expression in AST form.
        #[prost(message, tag = "4")]
        Parsed(
            super::super::super::super::super::super::google::api::expr::v1alpha1::Expr,
        ),
    }
}
