// This file is @generated by prost-build.
/// AWS Lambda filter config
/// \[\#next-free-field: 7\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// The ARN of the AWS Lambda to invoke when the filter is engaged
    /// Must be in the following format:
    /// arn:<partition>:lambda:<region>:<account-number>:function:<function-name>
    #[prost(string, tag = "1")]
    pub arn: ::prost::alloc::string::String,
    /// Whether to transform the request (headers and body) to a JSON payload or pass it as is.
    #[prost(bool, tag = "2")]
    pub payload_passthrough: bool,
    /// Determines the way to invoke the Lambda function.
    #[prost(enumeration = "config::InvocationMode", tag = "3")]
    pub invocation_mode: i32,
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
    ///   Changing the value of the host header can result in a different route to be selected
    ///   if an HTTP filter after AWS lambda re-evaluates the route (clears route cache).
    #[prost(string, tag = "4")]
    pub host_rewrite: ::prost::alloc::string::String,
    /// Specifies the credentials profile to be used from the AWS credentials file.
    /// This parameter is optional. If set, it will override the value set in the AWS_PROFILE env variable and
    /// the provider chain is limited to the AWS credentials file Provider.
    /// If credentials configuration is provided, this configuration will be ignored.
    /// If this field is provided, then the default providers chain specified in the documentation will be ignored.
    /// (See :ref:`default credentials providers <config_http_filters_aws_lambda_credentials>`).
    #[prost(string, tag = "5")]
    pub credentials_profile: ::prost::alloc::string::String,
    /// Specifies the credentials to be used. This parameter is optional and if it is set,
    /// it will override other providers and will take precedence over credentials_profile.
    /// The provider chain is limited to the configuration credentials provider.
    /// If this field is provided, then the default providers chain specified in the documentation will be ignored.
    /// (See :ref:`default credentials providers <config_http_filters_aws_lambda_credentials>`).
    ///
    /// .. warning::
    /// Distributing the AWS credentials via this configuration should not be done in production.
    #[prost(message, optional, tag = "6")]
    pub credentials: ::core::option::Option<Credentials>,
}
/// Nested message and enum types in `Config`.
pub mod config {
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
    pub enum InvocationMode {
        /// This is the more common mode of invocation, in which Lambda responds after it has completed the function. In
        /// this mode the output of the Lambda function becomes the response of the HTTP request.
        Synchronous = 0,
        /// In this mode Lambda responds immediately but continues to process the function asynchronously. This mode can be
        /// used to signal events for example. In this mode, Lambda responds with an acknowledgment that it received the
        /// call which is translated to an HTTP 200 OK by the filter.
        Asynchronous = 1,
    }
    impl InvocationMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Synchronous => "SYNCHRONOUS",
                Self::Asynchronous => "ASYNCHRONOUS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SYNCHRONOUS" => Some(Self::Synchronous),
                "ASYNCHRONOUS" => Some(Self::Asynchronous),
                _ => None,
            }
        }
    }
}
/// AWS Lambda Credentials config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Credentials {
    /// AWS access key id.
    #[prost(string, tag = "1")]
    pub access_key_id: ::prost::alloc::string::String,
    /// AWS secret access key.
    #[prost(string, tag = "2")]
    pub secret_access_key: ::prost::alloc::string::String,
    /// AWS session token.
    /// This parameter is optional. If it is set to empty string it will not be consider in the request.
    /// It is required if temporary security credentials retrieved directly from AWS STS operations are used.
    #[prost(string, tag = "3")]
    pub session_token: ::prost::alloc::string::String,
}
/// Per-route configuration for AWS Lambda. This can be useful when invoking a different Lambda function or a different
/// version of the same Lambda depending on the route.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerRouteConfig {
    #[prost(message, optional, tag = "1")]
    pub invoke_config: ::core::option::Option<Config>,
}
