// This file is @generated by prost-build.
/// ExpressionFilter is an access logging filter that evaluates configured
/// symbolic Common Expression Language expressions to inform the decision
/// to generate an access log.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpressionFilter {
    /// Expression that, when evaluated, will be used to filter access logs.
    /// Expressions are based on the set of Envoy :ref:`attributes <arch_overview_attributes>`.
    /// The provided expression must evaluate to true for logging (expression errors are considered false).
    /// Examples:
    ///
    /// * `response.code >= 400`
    /// * `(connection.mtls && request.headers\['x-log-mtls'\] == 'true') || request.url_path.contains('v1beta3')`
    #[prost(string, tag = "1")]
    pub expression: ::prost::alloc::string::String,
}
