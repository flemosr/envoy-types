// This file is @generated by prost-build.
/// Top level configuration for the AWS request signing filter.
/// \[\#next-free-field: 9\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsRequestSigning {
    /// The `service namespace  <<https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces>`\_>
    /// of the HTTP endpoint.
    ///
    /// Example: s3
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// Optional region string. If region is not provided, the region will be retrieved from the environment
    /// or AWS configuration files. See :ref:`config_http_filters_aws_request_signing_region` for more details.
    ///
    /// When signing_algorithm is set to `AWS_SIGV4` the region is a standard AWS `region <<https://docs.aws.amazon.com/general/latest/gr/rande.html>`\_> string for the service
    /// hosting the HTTP endpoint.
    ///
    /// Example: us-west-2
    ///
    /// When signing_algorithm is set to `AWS_SIGV4A` the region is used as a region set.
    ///
    /// A region set is a comma separated list of AWS regions, such as `us-east-1,us-east-2` or wildcard `*`
    /// or even region strings containing wildcards such as `us-east-*`
    ///
    /// Example: '\*'
    ///
    /// By configuring a region set, a SigV4A signed request can be sent to multiple regions, rather than being
    /// valid for only a single region destination.
    #[prost(string, tag = "2")]
    pub region: ::prost::alloc::string::String,
    /// Indicates that before signing headers, the host header will be swapped with
    /// this value. If not set or empty, the original host header value
    /// will be used and no rewrite will happen.
    ///
    ///
    /// Note: this rewrite affects both signing and host header forwarding. However, this
    /// option shouldn't be used with
    /// : ref:`HCM host rewrite <envoy_v3_api_field_config.route.v3.RouteAction.host_rewrite_literal>` given that the
    ///   value set here would be used for signing whereas the value set in the HCM would be used
    ///   for host header forwarding which is not the desired outcome.
    #[prost(string, tag = "3")]
    pub host_rewrite: ::prost::alloc::string::String,
    /// Instead of buffering the request to calculate the payload hash, use the literal string `UNSIGNED-PAYLOAD`
    /// to calculate the payload hash. Not all services support this option. See the `S3  <<https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html>`\_> policy for details.
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
    /// Optional Signing algorithm specifier, either `AWS_SIGV4` or `AWS_SIGV4A`, defaulting to `AWS_SIGV4`.
    #[prost(enumeration = "aws_request_signing::SigningAlgorithm", tag = "6")]
    pub signing_algorithm: i32,
    /// If set, use the query string to store output of SigV4 or SigV4A calculation, rather than HTTP headers. The `Authorization` header will not be modified if `query_string`
    /// is configured.
    ///
    /// Example:
    /// query_string: {}
    #[prost(message, optional, tag = "7")]
    pub query_string: ::core::option::Option<aws_request_signing::QueryString>,
    /// The credential provider for signing the request. This is optional and if not set,
    /// it will be retrieved from the procedure described in :ref:`config_http_filters_aws_request_signing`.
    #[prost(message, optional, tag = "8")]
    pub credential_provider: ::core::option::Option<
        super::super::super::super::common::aws::v3::AwsCredentialProvider,
    >,
}
/// Nested message and enum types in `AwsRequestSigning`.
pub mod aws_request_signing {
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct QueryString {
        /// Optional expiration time for the query string parameters. As query string parameter based requests are replayable, in effect representing
        /// an API call that has already been authenticated, it is recommended to keep this expiration time as short as feasible.
        /// This value will default to 5 seconds and has a maximum value of 3600 seconds (1 hour).
        #[prost(message, optional, tag = "1")]
        pub expiration_time: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SigningAlgorithm {
        /// Use SigV4 for signing
        AwsSigv4 = 0,
        /// Use SigV4A for signing
        AwsSigv4a = 1,
    }
    impl SigningAlgorithm {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::AwsSigv4 => "AWS_SIGV4",
                Self::AwsSigv4a => "AWS_SIGV4A",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "AWS_SIGV4" => Some(Self::AwsSigv4),
                "AWS_SIGV4A" => Some(Self::AwsSigv4a),
                _ => None,
            }
        }
    }
}
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
