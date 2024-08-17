#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuth2Credentials {
    /// The client_id to be used in the authorize calls. This value will be URL encoded when sent to the OAuth server.
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// The secret used to retrieve the access token. This value will be URL encoded when sent to the OAuth server.
    #[prost(message, optional, tag = "2")]
    pub token_secret: ::core::option::Option<
        super::super::super::super::transport_sockets::tls::v3::SdsSecretConfig,
    >,
    /// The cookie names used in OAuth filters flow.
    #[prost(message, optional, tag = "4")]
    pub cookie_names: ::core::option::Option<o_auth2_credentials::CookieNames>,
    /// Configures how the secret token should be created.
    #[prost(oneof = "o_auth2_credentials::TokenFormation", tags = "3")]
    pub token_formation: ::core::option::Option<o_auth2_credentials::TokenFormation>,
}
/// Nested message and enum types in `OAuth2Credentials`.
pub mod o_auth2_credentials {
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CookieNames {
        /// Cookie name to hold OAuth bearer token value. When the authentication server validates the
        /// client and returns an authorization token back to the OAuth filter, no matter what format
        /// that token is, if :ref:`forward_bearer_token <envoy_v3_api_field_extensions.filters.http.oauth2.v3.OAuth2Config.forward_bearer_token>`
        /// is set to true the filter will send over the bearer token as a cookie with this name to the
        /// upstream. Defaults to `BearerToken`.
        #[prost(string, tag = "1")]
        pub bearer_token: ::prost::alloc::string::String,
        /// Cookie name to hold OAuth HMAC value. Defaults to `OauthHMAC`.
        #[prost(string, tag = "2")]
        pub oauth_hmac: ::prost::alloc::string::String,
        /// Cookie name to hold OAuth expiry value. Defaults to `OauthExpires`.
        #[prost(string, tag = "3")]
        pub oauth_expires: ::prost::alloc::string::String,
        /// Cookie name to hold the id token. Defaults to `IdToken`.
        #[prost(string, tag = "4")]
        pub id_token: ::prost::alloc::string::String,
        /// Cookie name to hold the refresh token. Defaults to `RefreshToken`.
        #[prost(string, tag = "5")]
        pub refresh_token: ::prost::alloc::string::String,
    }
    /// Configures how the secret token should be created.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TokenFormation {
        /// If present, the secret token will be a HMAC using the provided secret.
        #[prost(message, tag = "3")]
        HmacSecret(
            super::super::super::super::super::transport_sockets::tls::v3::SdsSecretConfig,
        ),
    }
}
/// OAuth config
///
/// \[\#next-free-field: 18\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuth2Config {
    /// Endpoint on the authorization server to retrieve the access token from.
    #[prost(message, optional, tag = "1")]
    pub token_endpoint: ::core::option::Option<
        super::super::super::super::super::config::core::v3::HttpUri,
    >,
    /// The endpoint redirect to for authorization in response to unauthorized requests.
    #[prost(string, tag = "2")]
    pub authorization_endpoint: ::prost::alloc::string::String,
    /// Credentials used for OAuth.
    #[prost(message, optional, tag = "3")]
    pub credentials: ::core::option::Option<OAuth2Credentials>,
    /// The redirect URI passed to the authorization endpoint. Supports header formatting
    /// tokens. For more information, including details on header value syntax, see the
    /// documentation on :ref:`custom request headers <config_http_conn_man_headers_custom_request_headers>`.
    ///
    /// This URI should not contain any query parameters.
    #[prost(string, tag = "4")]
    pub redirect_uri: ::prost::alloc::string::String,
    /// Matching criteria used to determine whether a path appears to be the result of a redirect from the authorization server.
    #[prost(message, optional, tag = "5")]
    pub redirect_path_matcher: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::PathMatcher,
    >,
    /// The path to sign a user out, clearing their credential cookies.
    #[prost(message, optional, tag = "6")]
    pub signout_path: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::PathMatcher,
    >,
    /// Forward the OAuth token as a Bearer to upstream web service.
    #[prost(bool, tag = "7")]
    pub forward_bearer_token: bool,
    /// If set to true, preserve the existing authorization header.
    /// By default Envoy strips the existing authorization header before forwarding upstream.
    /// Can not be set to true if forward_bearer_token is already set to true.
    /// Default value is false.
    #[prost(bool, tag = "16")]
    pub preserve_authorization_header: bool,
    /// Any request that matches any of the provided matchers will be passed through without OAuth validation.
    #[prost(message, repeated, tag = "8")]
    pub pass_through_matcher: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::route::v3::HeaderMatcher,
    >,
    /// Optional list of OAuth scopes to be claimed in the authorization request. If not specified,
    /// defaults to "user" scope.
    /// OAuth RFC <https://tools.ietf.org/html/rfc6749#section-3.3>
    #[prost(string, repeated, tag = "9")]
    pub auth_scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional resource parameter for authorization request
    /// RFC: <https://tools.ietf.org/html/rfc8707>
    #[prost(string, repeated, tag = "10")]
    pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Defines how `client_id` and `client_secret` are sent in OAuth client to OAuth server requests.
    /// RFC <https://datatracker.ietf.org/doc/html/rfc6749#section-2.3.1>
    #[prost(enumeration = "o_auth2_config::AuthType", tag = "11")]
    pub auth_type: i32,
    /// If set to true, allows automatic access token refresh using the associated refresh token (see
    /// `RFC 6749 section 6 <<https://datatracker.ietf.org/doc/html/rfc6749#section-6>`\_>), provided that the OAuth server supports that.
    /// Default value is false.
    #[prost(message, optional, tag = "12")]
    pub use_refresh_token: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// The default lifetime in seconds of the access token, if omitted by the authorization server.
    ///
    /// If this value is not set, it will default to `0s`. In this case, the expiry must be set by
    /// the authorization server or the OAuth flow will fail.
    #[prost(message, optional, tag = "13")]
    pub default_expires_in: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Any request that matches any of the provided matchers won't be redirected to OAuth server when tokens are not valid.
    /// Automatic access token refresh will be performed for these requests, if enabled.
    /// This behavior can be useful for AJAX requests.
    #[prost(message, repeated, tag = "14")]
    pub deny_redirect_matcher: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::route::v3::HeaderMatcher,
    >,
    /// The default lifetime in seconds of the refresh token, if the exp (expiration time) claim is omitted in the refresh token or the refresh token is not JWT.
    ///
    /// If this value is not set, it will default to `604800s`. In this case, the cookie with the refresh token will be expired
    /// in a week.
    /// This setting is only considered if `use_refresh_token` is set to true, otherwise the authorization server expiration or `default_expires_in` is used.
    #[prost(message, optional, tag = "15")]
    pub default_refresh_token_expires_in: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// If set to true, Envoy will not set a cookie for ID Token even if one is received from the Identity Provider. This may be useful in cases where the ID
    /// Token is too large for HTTP cookies (longer than 4096 characters). Enabling this option will only disable setting the cookie response header, the filter
    /// will still process incoming ID Tokens as part of the HMAC if they are there. This is to ensure compatibility while switching this setting on. Future
    /// sessions would not set the IdToken cookie header.
    #[prost(bool, tag = "17")]
    pub disable_id_token_set_cookie: bool,
}
/// Nested message and enum types in `OAuth2Config`.
pub mod o_auth2_config {
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
        /// The `client_id` and `client_secret` will be sent in the URL encoded request body.
        /// This type should only be used when Auth server does not support Basic authentication.
        UrlEncodedBody = 0,
        /// The `client_id` and `client_secret` will be sent using HTTP Basic authentication scheme.
        BasicAuth = 1,
    }
    impl AuthType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AuthType::UrlEncodedBody => "URL_ENCODED_BODY",
                AuthType::BasicAuth => "BASIC_AUTH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "URL_ENCODED_BODY" => Some(Self::UrlEncodedBody),
                "BASIC_AUTH" => Some(Self::BasicAuth),
                _ => None,
            }
        }
    }
}
/// Filter config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuth2 {
    /// Leave this empty to disable OAuth2 for a specific route, using per filter config.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<OAuth2Config>,
}
