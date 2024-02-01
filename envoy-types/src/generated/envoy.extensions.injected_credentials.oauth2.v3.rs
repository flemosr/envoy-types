/// OAuth2 extension can be used to retrieve an OAuth2 access token from an authorization server and inject it into the
/// proxied requests.
/// Currently, only the Client Credentials Grant flow is supported.
/// The access token will be injected into the request headers using the `Authorization` header as a bearer token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuth2 {
    /// Endpoint on the authorization server to retrieve the access token from.
    /// Refer to [RFC 6749: The OAuth 2.0 Authorization Framework](<https://www.rfc-editor.org/rfc/rfc6749#section-3.2>) for details.
    #[prost(message, optional, tag = "1")]
    pub token_endpoint: ::core::option::Option<
        super::super::super::super::config::core::v3::HttpUri,
    >,
    /// Optional list of OAuth scopes to be claimed in the authorization request.
    /// Refer to [RFC 6749: The OAuth 2.0 Authorization Framework](<https://www.rfc-editor.org/rfc/rfc6749#section-4.4.2>) for details.
    #[prost(string, repeated, tag = "2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(oneof = "o_auth2::FlowType", tags = "3")]
    pub flow_type: ::core::option::Option<o_auth2::FlowType>,
}
/// Nested message and enum types in `OAuth2`.
pub mod o_auth2 {
    /// Credentials to authenticate client to the authorization server.
    /// Refer to [RFC 6749: The OAuth 2.0 Authorization Framework](<https://www.rfc-editor.org/rfc/rfc6749#section-2.3>) for details.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClientCredentials {
        /// Client ID.
        /// Refer to [RFC 6749: The OAuth 2.0 Authorization Framework](<https://www.rfc-editor.org/rfc/rfc6749#section-2.3.1>) for details.
        #[prost(string, tag = "1")]
        pub client_id: ::prost::alloc::string::String,
        /// Client secret.
        /// Refer to [RFC 6749: The OAuth 2.0 Authorization Framework](<https://www.rfc-editor.org/rfc/rfc6749#section-2.3.1>) for details.
        #[prost(message, optional, tag = "2")]
        pub client_secret: ::core::option::Option<
            super::super::super::super::transport_sockets::tls::v3::SdsSecretConfig,
        >,
        /// The method to use when sending credentials to the authorization server.
        /// Refer to [RFC 6749: The OAuth 2.0 Authorization Framework](<https://www.rfc-editor.org/rfc/rfc6749#section-2.3.1>) for details.
        #[prost(enumeration = "AuthType", tag = "3")]
        pub auth_type: i32,
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
    pub enum AuthType {
        /// The `client_id` and `client_secret` will be sent using HTTP Basic authentication scheme.
        BasicAuth = 0,
        /// The `client_id` and `client_secret` will be sent in the URL encoded request body.
        /// This type should only be used when Auth server does not support Basic authentication.
        UrlEncodedBody = 1,
    }
    impl AuthType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AuthType::BasicAuth => "BASIC_AUTH",
                AuthType::UrlEncodedBody => "URL_ENCODED_BODY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BASIC_AUTH" => Some(Self::BasicAuth),
                "URL_ENCODED_BODY" => Some(Self::UrlEncodedBody),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FlowType {
        /// Client Credentials Grant.
        /// Refer to [RFC 6749: The OAuth 2.0 Authorization Framework](<https://www.rfc-editor.org/rfc/rfc6749#section-4.4>) for details.
        #[prost(message, tag = "3")]
        ClientCredentials(ClientCredentials),
    }
}
