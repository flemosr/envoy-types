/// Top level configuration for the AWS request signing filter.
/// \[\#next-free-field: 6\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsRequestSigning {
    /// The `service namespace <<https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces>`\_>
    /// of the HTTP endpoint.
    ///
    /// Example: s3
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// The `region <<https://docs.aws.amazon.com/general/latest/gr/rande.html>`\_> hosting the HTTP
    /// endpoint.
    ///
    /// Example: us-west-2
    #[prost(string, tag = "2")]
    pub region: ::prost::alloc::string::String,
    /// Indicates that before signing headers, the host header will be swapped with
    /// this value. If not set or empty, the original host header value
    /// will be used and no rewrite will happen.
    ///
    /// Note: this rewrite affects both signing and host header forwarding. However, this
    /// option shouldn't be used with
    /// :ref:`HCM host rewrite <envoy_v3_api_field_config.route.v3.RouteAction.host_rewrite_literal>` given that the
    /// value set here would be used for signing whereas the value set in the HCM would be used
    /// for host header forwarding which is not the desired outcome.
    #[prost(string, tag = "3")]
    pub host_rewrite: ::prost::alloc::string::String,
    /// Instead of buffering the request to calculate the payload hash, use the literal string `UNSIGNED-PAYLOAD`
    /// to calculate the payload hash. Not all services support this option. See the `S3 <<https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html>`\_> policy for details.
    #[prost(bool, tag = "4")]
    pub use_unsigned_payload: bool,
    /// A list of request header string matchers that will be excluded from signing. The excluded header can be matched by
    /// any patterns defined in the StringMatcher proto (e.g. exact string, prefix, regex, etc).
    ///
    /// Example:
    /// match_excluded_headers:
    ///
    /// * prefix: x-envoy
    /// * exact: foo
    /// * exact: bar
    ///   When applied, all headers that start with "x-envoy" and headers "foo" and "bar" will not be signed.
    #[prost(message, repeated, tag = "5")]
    pub match_excluded_headers: ::prost::alloc::vec::Vec<
        super::super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsRequestSigningPerRoute {
    /// Override the global configuration of the filter with this new config.
    /// This overrides the entire message of AwsRequestSigning and not at field level.
    #[prost(message, optional, tag = "1")]
    pub aws_request_signing: ::core::option::Option<AwsRequestSigning>,
    /// The human readable prefix to use when emitting stats.
    #[prost(string, tag = "2")]
    pub stat_prefix: ::prost::alloc::string::String,
}
