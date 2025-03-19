// This file is @generated by prost-build.
/// Please see following for JWT authentication flow:
///
/// * `JSON Web Token (JWT) <<https://tools.ietf.org/html/rfc7519>`\_>
/// * `The OAuth 2.0 Authorization Framework <<https://tools.ietf.org/html/rfc6749>`\_>
/// * `OpenID Connect <<http://openid.net/connect>`\_>
///
/// A JwtProvider message specifies how a JSON Web Token (JWT) can be verified. It specifies:
///
/// * issuer: the principal that issues the JWT. If specified, it has to match the `iss` field in JWT.
/// * allowed audiences: the ones in the token have to be listed here.
/// * how to fetch public key JWKS to verify the token signature.
/// * how to extract the JWT in the request.
/// * how to pass successfully verified token payload.
///
/// Example:
///
/// .. code-block:: yaml
///
/// ```text
/// issuer: <https://example.com>
/// audiences:
/// - bookstore_android.apps.googleusercontent.com
/// - bookstore_web.apps.googleusercontent.com
/// remote_jwks:
///    http_uri:
///      uri: <https://example.com/.well-known/jwks.json>
///      cluster: example_jwks_cluster
///      timeout: 1s
///    cache_duration:
///      seconds: 300
/// ```
///
/// \[\#next-free-field: 22\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtProvider {
    /// Specify the `principal <<https://tools.ietf.org/html/rfc7519#section-4.1.1>`\_> that issued
    /// the JWT, usually a URL or an email address.
    ///
    /// It is optional. If specified, it has to match the `iss` field in JWT,
    /// otherwise the JWT `iss` field is not checked.
    ///
    /// Note: `JwtRequirement` :ref:`allow_missing <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.JwtRequirement.allow_missing>`
    /// and :ref:`allow_missing_or_failed <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.JwtRequirement.allow_missing_or_failed>`
    /// are implemented differently than other `JwtRequirements`. Hence the usage of this field
    /// is different as follows if `allow_missing` or `allow_missing_or_failed` is used:
    ///
    /// * If a JWT has `iss` field, it needs to be specified by this field in one of `JwtProviders`.
    /// * If a JWT doesn't have `iss` field, one of `JwtProviders` should fill this field empty.
    /// * Multiple `JwtProviders` should not have same value in this field.
    ///
    /// Example: <https://securetoken.google.com>
    /// Example: 1234567-compute@developer.gserviceaccount.com
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    /// The list of JWT `audiences <<https://tools.ietf.org/html/rfc7519#section-4.1.3>`\_> are
    /// allowed to access. A JWT containing any of these audiences will be accepted. If not specified,
    /// will not check audiences in the token.
    ///
    /// Example:
    ///
    /// .. code-block:: yaml
    ///
    /// ```text
    /// audiences:
    /// - bookstore_android.apps.googleusercontent.com
    /// - bookstore_web.apps.googleusercontent.com
    /// ```
    #[prost(string, repeated, tag = "2")]
    pub audiences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Restrict the `subjects <<https://tools.ietf.org/html/rfc7519#section-4.1.2>`\_>
    /// that the JwtProvider can assert. For instance, this could implement JWT-SVID
    /// `subject restrictions <<https://github.com/spiffe/spiffe/blob/main/standards/JWT-SVID.md#31-subject>`\_.>
    /// If not specified, will not check subjects in the token.
    ///
    /// Example:
    ///
    /// .. code-block:: yaml
    ///
    /// ```text
    /// subjects:
    ///    prefix: spiffe://spiffe.example.com/
    /// ```
    #[prost(message, optional, tag = "19")]
    pub subjects: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    /// Requires that the credential contains an `expiration <<https://tools.ietf.org/html/rfc7519#section-4.1.4>`*.>
    /// For instance, this could implement JWT-SVID
    /// `expiration restrictions <<https://github.com/spiffe/spiffe/blob/main/standards/JWT-SVID.md#33-expiration-time>`*.>
    /// Unlike `max_lifetime`, this only requires that expiration is present, where `max_lifetime` also checks the value.
    ///
    /// Example:
    ///
    /// .. code-block:: yaml
    ///
    /// ```text
    /// require_expiration: true
    /// ```
    #[prost(bool, tag = "20")]
    pub require_expiration: bool,
    /// Restrict the maximum remaining lifetime of a credential from the JwtProvider. Credential lifetime
    /// is the difference between the current time and the expiration of the credential. For instance,
    /// the following example will reject credentials that have a lifetime longer than 24 hours. If not set,
    /// expiration checking still occurs, but there is no limit on credential lifetime. If set, takes precedence
    /// over `require_expiration`.
    ///
    /// Example:
    ///
    /// .. code-block:: yaml
    ///
    /// ```text
    /// max_lifetime:
    ///    seconds: 86400
    /// ```
    #[prost(message, optional, tag = "21")]
    pub max_lifetime: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// If false, the JWT is removed in the request after a success verification. If true, the JWT is
    /// not removed in the request. Default value is false.
    /// caveat: only works for from_header/from_params & has no effect for JWTs extracted through from_cookies.
    #[prost(bool, tag = "5")]
    pub forward: bool,
    /// Two fields below define where to extract the JWT from an HTTP request.
    ///
    /// If no explicit location is specified, the following default locations are tried in order:
    ///
    /// 1. The Authorization header using the `Bearer schema  <<https://tools.ietf.org/html/rfc6750#section-2.1>`\_.> Example::
    ///
    ///    Authorization: Bearer <token>.
    ///
    /// 1. `access_token <<https://tools.ietf.org/html/rfc6750#section-2.3>`\_> query parameter.
    ///
    /// Multiple JWTs can be verified for a request. Each JWT has to be extracted from the locations
    /// its provider specified or from the default locations.
    ///
    /// Specify the HTTP headers to extract the JWT. For examples, following config:
    ///
    /// .. code-block:: yaml
    ///
    /// from_headers:
    ///
    /// * name: x-goog-iap-jwt-assertion
    ///
    /// can be used to extract token from header::
    ///
    /// `x-goog-iap-jwt-assertion: <JWT>`.
    #[prost(message, repeated, tag = "6")]
    pub from_headers: ::prost::alloc::vec::Vec<JwtHeader>,
    /// JWT is sent in a query parameter. `jwt_params` represents the query parameter names.
    ///
    /// For example, if config is:
    ///
    /// .. code-block:: yaml
    ///
    /// from_params:
    ///
    /// * jwt_token
    ///
    /// The JWT format in query parameter is::
    ///
    /// ```text
    /// /path?jwt_token=<JWT>
    /// ```
    #[prost(string, repeated, tag = "7")]
    pub from_params: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// JWT is sent in a cookie. `from_cookies` represents the cookie names to extract from.
    ///
    /// For example, if config is:
    ///
    /// .. code-block:: yaml
    ///
    /// from_cookies:
    ///
    /// * auth-token
    ///
    /// Then JWT will be extracted from `auth-token` cookie in the request.
    #[prost(string, repeated, tag = "13")]
    pub from_cookies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This field specifies the header name to forward a successfully verified JWT payload to the
    /// backend. The forwarded data is::
    ///
    /// ```text
    /// base64url_encoded(jwt_payload_in_JSON)
    /// ```
    ///
    /// If it is not specified, the payload will not be forwarded.
    #[prost(string, tag = "8")]
    pub forward_payload_header: ::prost::alloc::string::String,
    /// When :ref:`forward_payload_header <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.JwtProvider.forward_payload_header>`
    /// is specified, the base64 encoded payload will be added to the headers.
    /// Normally JWT based64 encode doesn't add padding. If this field is true,
    /// the header will be padded.
    ///
    /// This field is only relevant if :ref:`forward_payload_header <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.JwtProvider.forward_payload_header>`
    /// is specified.
    #[prost(bool, tag = "11")]
    pub pad_forward_payload_header: bool,
    /// If non empty, successfully verified JWT payloads will be written to StreamInfo DynamicMetadata
    /// in the format as: `namespace` is the jwt_authn filter name as `envoy.filters.http.jwt_authn`
    /// The value is the `protobuf::Struct`. The value of this field will be the key for its `fields`
    /// and the value is the `protobuf::Struct` converted from JWT JSON payload.
    ///
    /// For example, if payload_in_metadata is `my_payload`:
    ///
    /// .. code-block:: yaml
    ///
    /// envoy.filters.http.jwt_authn:
    /// my_payload:
    /// iss: <https://example.com>
    /// sub: test@example.com
    /// aud: <https://example.com>
    /// exp: 1501281058
    #[prost(string, tag = "9")]
    pub payload_in_metadata: ::prost::alloc::string::String,
    /// Normalizes the payload representation in the request metadata.
    #[prost(message, optional, tag = "18")]
    pub normalize_payload_in_metadata: ::core::option::Option<
        jwt_provider::NormalizePayload,
    >,
    /// If not empty, similar to :ref:`payload_in_metadata <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.JwtProvider.payload_in_metadata>`,
    /// a successfully verified JWT header will be written to :ref:`Dynamic State <arch_overview_data_sharing_between_filters>`
    /// as an entry (`protobuf::Struct`) in `envoy.filters.http.jwt_authn` `namespace` with the
    /// value of this field as the key.
    ///
    /// For example, if `header_in_metadata` is `my_header`:
    ///
    /// .. code-block:: yaml
    ///
    /// envoy.filters.http.jwt_authn:
    /// my_header:
    /// alg: JWT
    /// kid: EF71iSaosbC5C4tC6Syq1Gm647M
    /// alg: PS256
    ///
    ///
    /// When the metadata has `envoy.filters.http.jwt_authn` entry already (for example if
    /// : ref:`payload_in_metadata <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.JwtProvider.payload_in_metadata>`
    ///   is not empty), it will be inserted as a new entry in the same `namespace` as shown below:
    ///
    ///
    /// .. code-block:: yaml
    ///
    /// envoy.filters.http.jwt_authn:
    /// my_payload:
    /// iss: <https://example.com>
    /// sub: test@example.com
    /// aud: <https://example.com>
    /// exp: 1501281058
    /// my_header:
    /// alg: JWT
    /// kid: EF71iSaosbC5C4tC6Syq1Gm647M
    /// alg: PS256
    ///
    /// .. warning::
    /// Using the same key name for :ref:`header_in_metadata <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.JwtProvider.payload_in_metadata>`
    /// and :ref:`payload_in_metadata <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.JwtProvider.payload_in_metadata>`
    /// is not suggested due to potential override of existing entry, while it is not enforced during
    /// config validation.
    #[prost(string, tag = "14")]
    pub header_in_metadata: ::prost::alloc::string::String,
    /// If non empty, the failure status `::google::jwt_verify::Status` for a non verified JWT will be written to StreamInfo DynamicMetadata
    /// in the format as: `namespace` is the jwt_authn filter name as `envoy.filters.http.jwt_authn`
    /// The value is the `protobuf::Struct`. The values of this field will be `code` and `message`
    /// and they will contain the JWT authentication failure status code and a message describing the failure.
    ///
    /// For example, if failed_status_in_metadata is `my_auth_failure_status`:
    ///
    /// .. code-block:: yaml
    ///
    /// envoy.filters.http.jwt_authn:
    /// my_auth_failure_status:
    /// code: 3
    /// message: Jwt expired
    #[prost(string, tag = "16")]
    pub failed_status_in_metadata: ::prost::alloc::string::String,
    /// Specify the clock skew in seconds when verifying JWT time constraint,
    /// such as `exp`, and `nbf`. If not specified, default is 60 seconds.
    #[prost(uint32, tag = "10")]
    pub clock_skew_seconds: u32,
    /// Enables JWT cache, its size is specified by `jwt_cache_size`.
    /// Only valid JWTs are cached.
    #[prost(message, optional, tag = "12")]
    pub jwt_cache_config: ::core::option::Option<JwtCacheConfig>,
    /// Add JWT claim to HTTP Header
    /// Specify the claim name you want to copy in which HTTP header. For examples, following config:
    /// The claim must be of type; string, int, double, bool. Array type claims are not supported
    ///
    /// .. literalinclude:: /\_configs/repo/jwt_authn.yaml
    /// :language: yaml
    /// :lines: 44-48
    /// :linenos:
    /// :lineno-start: 44
    /// :caption: :download:`jwt_authn.yaml </_configs/repo/jwt_authn.yaml>`
    ///
    /// This header is only reserved for jwt claim; any other value will be overwritten.
    #[prost(message, repeated, tag = "15")]
    pub claim_to_headers: ::prost::alloc::vec::Vec<JwtClaimToHeader>,
    /// Clears route cache in order to allow the JWT to correctly affect
    /// routing decisions. Filter clears all cached routes when:
    ///
    /// 1. The field is set to `true`.
    ///
    /// 1. At least one `claim_to_headers` header is added to the request OR
    ///    if `payload_in_metadata` is set.
    #[prost(bool, tag = "17")]
    pub clear_route_cache: bool,
    /// `JSON Web Key Set (JWKS) <<https://tools.ietf.org/html/rfc7517#appendix-A>`\_> is needed to
    /// validate signature of a JWT. This field specifies where to fetch JWKS.
    #[prost(oneof = "jwt_provider::JwksSourceSpecifier", tags = "3, 4")]
    pub jwks_source_specifier: ::core::option::Option<jwt_provider::JwksSourceSpecifier>,
}
/// Nested message and enum types in `JwtProvider`.
pub mod jwt_provider {
    /// Alters the payload representation in the request dynamic metadata to facilitate its use in matching.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NormalizePayload {
        /// Each claim in this list will be interpreted as a space-delimited string
        /// and converted to a list of strings based on the delimited values.
        /// Example: a token with a claim `scope: "email profile"` is translated
        /// to dynamic metadata  `scope: \["email", "profile"\]` if this field is
        /// set value `\["scope"\]`. This special handling of `scope` is
        /// recommended by `RFC8693  <<https://datatracker.ietf.org/doc/html/rfc8693#name-scope-scopes-claim>`\_.>
        #[prost(string, repeated, tag = "1")]
        pub space_delimited_claims: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// `JSON Web Key Set (JWKS) <<https://tools.ietf.org/html/rfc7517#appendix-A>`\_> is needed to
    /// validate signature of a JWT. This field specifies where to fetch JWKS.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum JwksSourceSpecifier {
        /// JWKS can be fetched from remote server via HTTP/HTTPS. This field specifies the remote HTTP
        /// URI and how the fetched JWKS should be cached.
        ///
        /// Example:
        ///
        /// .. code-block:: yaml
        ///
        /// ```text
        /// remote_jwks:
        ///   http_uri:
        ///     uri: <https://www.googleapis.com/oauth2/v1/certs>
        ///     cluster: jwt.www.googleapis.com|443
        ///     timeout: 1s
        ///   cache_duration:
        ///     seconds: 300
        /// ```
        #[prost(message, tag = "3")]
        RemoteJwks(super::RemoteJwks),
        /// JWKS is in local data source. It could be either in a local file or embedded in the
        /// inline_string.
        ///
        /// Example: local file
        ///
        /// .. code-block:: yaml
        ///
        /// ```text
        /// local_jwks:
        ///   filename: /etc/envoy/jwks/jwks1.txt
        /// ```
        ///
        /// Example: inline_string
        ///
        /// .. code-block:: yaml
        ///
        /// ```text
        /// local_jwks:
        ///   inline_string: ACADADADADA
        /// ```
        #[prost(message, tag = "4")]
        LocalJwks(
            super::super::super::super::super::super::config::core::v3::DataSource,
        ),
    }
}
/// This message specifies JWT Cache configuration.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct JwtCacheConfig {
    /// The unit is number of JWTs, default to 100.
    #[prost(uint32, tag = "1")]
    pub jwt_cache_size: u32,
    /// The maximum size of a single cached token in bytes.
    /// If this field is not set or is set to 0, then the default value 4096 bytes is used.
    /// The maximum value for a token is inclusive.
    #[prost(uint32, tag = "2")]
    pub jwt_max_token_size: u32,
}
/// This message specifies how to fetch JWKS from remote and how to cache it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteJwks {
    /// The HTTP URI to fetch the JWKS. For example:
    ///
    /// .. code-block:: yaml
    ///
    /// ```text
    /// http_uri:
    ///   uri: <https://www.googleapis.com/oauth2/v1/certs>
    ///   cluster: jwt.www.googleapis.com|443
    ///   timeout: 1s
    /// ```
    #[prost(message, optional, tag = "1")]
    pub http_uri: ::core::option::Option<
        super::super::super::super::super::config::core::v3::HttpUri,
    >,
    /// Duration after which the cached JWKS should be expired. If not specified, default cache
    /// duration is 10 minutes.
    #[prost(message, optional, tag = "2")]
    pub cache_duration: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Fetch Jwks asynchronously in the main thread before the listener is activated.
    /// Fetched Jwks can be used by all worker threads.
    ///
    /// If this feature is not enabled:
    ///
    /// * The Jwks is fetched on-demand when the requests come. During the fetching, first
    ///   few requests are paused until the Jwks is fetched.
    /// * Each worker thread fetches its own Jwks since Jwks cache is per worker thread.
    ///
    /// If this feature is enabled:
    ///
    /// * Fetched Jwks is done in the main thread before the listener is activated. Its fetched
    ///   Jwks can be used by all worker threads. Each worker thread doesn't need to fetch its own.
    /// * Jwks is ready when the requests come, not need to wait for the Jwks fetching.
    #[prost(message, optional, tag = "3")]
    pub async_fetch: ::core::option::Option<JwksAsyncFetch>,
    /// Retry policy for fetching Jwks. optional. turned off by default.
    ///
    /// For example:
    ///
    /// .. code-block:: yaml
    ///
    /// retry_policy:
    /// retry_back_off:
    /// base_interval: 0.01s
    /// max_interval: 20s
    /// num_retries: 10
    ///
    /// will yield a randomized truncated exponential backoff policy with an initial delay of 10ms
    /// 10 maximum attempts spaced at most 20s seconds.
    ///
    /// .. code-block:: yaml
    ///
    /// retry_policy:
    /// num_retries:1
    ///
    /// uses the default :ref:`retry backoff strategy <envoy_v3_api_msg_config.core.v3.BackoffStrategy>`.
    /// with the default base interval is 1000 milliseconds. and the default maximum interval of 10 times the base interval.
    ///
    /// if num_retries is omitted, the default is to allow only one retry.
    ///
    /// If enabled, the retry policy will apply to all Jwks fetching approaches, e.g. on demand or asynchronously in background.
    #[prost(message, optional, tag = "4")]
    pub retry_policy: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RetryPolicy,
    >,
}
/// Fetch Jwks asynchronously in the main thread when the filter config is parsed.
/// The listener is activated only after the Jwks is fetched.
/// When the Jwks is expired in the cache, it is fetched again in the main thread.
/// The fetched Jwks from the main thread can be used by all worker threads.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct JwksAsyncFetch {
    /// If false, the listener is activated after the initial fetch is completed.
    /// The initial fetch result can be either successful or failed.
    /// If true, it is activated without waiting for the initial fetch to complete.
    /// Default is false.
    #[prost(bool, tag = "1")]
    pub fast_listener: bool,
    /// The duration to refetch after a failed fetch. If not specified, default is 1 second.
    #[prost(message, optional, tag = "2")]
    pub failed_refetch_duration: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
}
/// This message specifies a header location to extract the JWT.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtHeader {
    /// The HTTP header name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The value prefix. The value format is "value_prefix<token>"
    /// For example, for "Authorization: Bearer <token>", value_prefix="Bearer " with a space at the
    /// end.
    #[prost(string, tag = "2")]
    pub value_prefix: ::prost::alloc::string::String,
}
/// Specify a required provider with audiences.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProviderWithAudiences {
    /// Specify a required provider name.
    #[prost(string, tag = "1")]
    pub provider_name: ::prost::alloc::string::String,
    /// This field overrides the one specified in the JwtProvider.
    #[prost(string, repeated, tag = "2")]
    pub audiences: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This message specifies a Jwt requirement. An empty message means JWT verification is not
/// required. Here are some config examples:
///
/// .. code-block:: yaml
///
/// # Example 1: not required with an empty message
///
/// # Example 2: require A
///
/// provider_name: provider-A
///
/// # Example 3: require A or B
///
/// requires_any:
/// requirements:
/// - provider_name: provider-A
/// - provider_name: provider-B
///
/// # Example 4: require A and B
///
/// requires_all:
/// requirements:
/// - provider_name: provider-A
/// - provider_name: provider-B
///
/// # Example 5: require A and (B or C)
///
/// requires_all:
/// requirements:
/// - provider_name: provider-A
/// - requires_any:
/// requirements:
/// - provider_name: provider-B
/// - provider_name: provider-C
///
/// # Example 6: require A or (B and C)
///
/// requires_any:
/// requirements:
/// - provider_name: provider-A
/// - requires_all:
/// requirements:
/// - provider_name: provider-B
/// - provider_name: provider-C
///
/// # Example 7: A is optional (if token from A is provided, it must be valid, but also allows
///
/// missing token.)
/// requires_any:
/// requirements:
/// - provider_name: provider-A
/// - allow_missing: {}
///
/// # Example 8: A is optional and B is required.
///
/// requires_all:
/// requirements:
/// - requires_any:
/// requirements:
/// - provider_name: provider-A
/// - allow_missing: {}
/// - provider_name: provider-B
///
/// \[\#next-free-field: 7\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtRequirement {
    #[prost(oneof = "jwt_requirement::RequiresType", tags = "1, 2, 3, 4, 5, 6")]
    pub requires_type: ::core::option::Option<jwt_requirement::RequiresType>,
}
/// Nested message and enum types in `JwtRequirement`.
pub mod jwt_requirement {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequiresType {
        /// Specify a required provider name.
        #[prost(string, tag = "1")]
        ProviderName(::prost::alloc::string::String),
        /// Specify a required provider with audiences.
        #[prost(message, tag = "2")]
        ProviderAndAudiences(super::ProviderWithAudiences),
        /// Specify list of JwtRequirement. Their results are OR-ed.
        /// If any one of them passes, the result is passed.
        #[prost(message, tag = "3")]
        RequiresAny(super::JwtRequirementOrList),
        /// Specify list of JwtRequirement. Their results are AND-ed.
        /// All of them must pass, if one of them fails or missing, it fails.
        #[prost(message, tag = "4")]
        RequiresAll(super::JwtRequirementAndList),
        /// The requirement is always satisfied even if JWT is missing or the JWT
        /// verification fails. A typical usage is: this filter is used to only verify
        /// JWTs and pass the verified JWT payloads to another filter, the other filter
        /// will make decision. In this mode, all JWTs will be verified.
        #[prost(message, tag = "5")]
        AllowMissingOrFailed(
            super::super::super::super::super::super::super::google::protobuf::Empty,
        ),
        /// The requirement is satisfied if JWT is missing, but failed if JWT is
        /// presented but invalid. Similar to allow_missing_or_failed, this is used
        /// to only verify JWTs and pass the verified payload to another filter. The
        /// different is this mode will reject requests with invalid tokens.
        #[prost(message, tag = "6")]
        AllowMissing(
            super::super::super::super::super::super::super::google::protobuf::Empty,
        ),
    }
}
/// This message specifies a list of RequiredProvider.
/// Their results are OR-ed; if any one of them passes, the result is passed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtRequirementOrList {
    /// Specify a list of JwtRequirement.
    #[prost(message, repeated, tag = "1")]
    pub requirements: ::prost::alloc::vec::Vec<JwtRequirement>,
}
/// This message specifies a list of RequiredProvider.
/// Their results are AND-ed; all of them must pass, if one of them fails or missing, it fails.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtRequirementAndList {
    /// Specify a list of JwtRequirement.
    #[prost(message, repeated, tag = "1")]
    pub requirements: ::prost::alloc::vec::Vec<JwtRequirement>,
}
/// This message specifies a Jwt requirement for a specific Route condition.
/// Example 1:
///
/// .. code-block:: yaml
///
/// ```text
/// - match:
///     prefix: /healthz
/// ```
///
/// In above example, "requires" field is empty for /healthz prefix match,
/// it means that requests matching the path prefix don't require JWT authentication.
///
/// Example 2:
///
/// .. code-block:: yaml
///
/// ```text
/// - match:
///     prefix: /
///   requires: { provider_name: provider-A }
/// ```
///
/// In above example, all requests matched the path prefix require jwt authentication
/// from "provider-A".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequirementRule {
    /// The route matching parameter. Only when the match is satisfied, the "requires" field will
    /// apply.
    ///
    /// For example: following match will match all requests.
    ///
    /// .. code-block:: yaml
    ///
    /// ```text
    /// match:
    ///   prefix: /
    /// ```
    #[prost(message, optional, tag = "1")]
    pub r#match: ::core::option::Option<
        super::super::super::super::super::config::route::v3::RouteMatch,
    >,
    /// Specify a Jwt requirement.
    /// If not specified, Jwt verification is disabled.
    #[prost(oneof = "requirement_rule::RequirementType", tags = "2, 3")]
    pub requirement_type: ::core::option::Option<requirement_rule::RequirementType>,
}
/// Nested message and enum types in `RequirementRule`.
pub mod requirement_rule {
    /// Specify a Jwt requirement.
    /// If not specified, Jwt verification is disabled.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequirementType {
        /// Specify a Jwt requirement. Please see detail comment in message JwtRequirement.
        #[prost(message, tag = "2")]
        Requires(super::JwtRequirement),
        ///
        /// Use requirement_name to specify a Jwt requirement.
        /// This requirement_name MUST be specified at the
        /// : ref:`requirement_map <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.JwtAuthentication.requirement_map>`
        ///   in `JwtAuthentication`.
        #[prost(string, tag = "3")]
        RequirementName(::prost::alloc::string::String),
    }
}
/// This message specifies Jwt requirements based on stream_info.filterState.
/// This FilterState should use `Router::StringAccessor` object to set a string value.
/// Other HTTP filters can use it to specify Jwt requirements dynamically.
///
/// Example:
///
/// .. code-block:: yaml
///
/// ```text
/// name: jwt_selector
/// requires:
///   issuer_1:
///     provider_name: issuer1
///   issuer_2:
///     provider_name: issuer2
/// ```
///
/// If a filter set "jwt_selector" with "issuer_1" to FilterState for a request,
/// jwt_authn filter will use JwtRequirement{"provider_name": "issuer1"} to verify.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterStateRule {
    /// The filter state name to retrieve the `Router::StringAccessor` object.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A map of string keys to requirements. The string key is the string value
    /// in the FilterState with the name specified in the `name` field above.
    #[prost(map = "string, message", tag = "3")]
    pub requires: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        JwtRequirement,
    >,
}
/// This is the Envoy HTTP filter config for JWT authentication.
///
/// For example:
///
/// .. code-block:: yaml
///
/// providers:
/// provider1:
/// issuer: issuer1
/// audiences:
/// - audience1
/// - audience2
/// remote_jwks:
/// http_uri:
/// uri: <https://example.com/.well-known/jwks.json>
/// cluster: example_jwks_cluster
/// timeout: 1s
/// provider2:
/// issuer: issuer2
/// local_jwks:
/// inline_string: jwks_string
///
/// rules:
/// \# Not jwt verification is required for /health path
/// - match:
/// prefix: /health
///
/// ```text
///   # Jwt verification for provider1 is required for path prefixed with "prefix"
///   - match:
///       prefix: /prefix
///     requires:
///       provider_name: provider1
///
///   # Jwt verification for either provider1 or provider2 is required for all other requests.
///   - match:
///       prefix: /
///     requires:
///       requires_any:
///         requirements:
///           - provider_name: provider1
///           - provider_name: provider2
/// ```
///
/// \[\#next-free-field: 7\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtAuthentication {
    /// Map of provider names to JwtProviders.
    ///
    /// .. code-block:: yaml
    ///
    /// providers:
    /// provider1:
    /// issuer: issuer1
    /// audiences:
    /// - audience1
    /// - audience2
    /// remote_jwks:
    /// http_uri:
    /// uri: <https://example.com/.well-known/jwks.json>
    /// cluster: example_jwks_cluster
    /// timeout: 1s
    /// provider2:
    /// issuer: provider2
    /// local_jwks:
    /// inline_string: jwks_string
    #[prost(map = "string, message", tag = "1")]
    pub providers: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        JwtProvider,
    >,
    /// Specifies requirements based on the route matches. The first matched requirement will be
    /// applied. If there are overlapped match conditions, please put the most specific match first.
    ///
    /// Examples
    ///
    /// .. code-block:: yaml
    ///
    /// rules:
    /// - match:
    /// prefix: /healthz
    /// - match:
    /// prefix: /baz
    /// requires:
    /// provider_name: provider1
    /// - match:
    /// prefix: /foo
    /// requires:
    /// requires_any:
    /// requirements:
    /// - provider_name: provider1
    /// - provider_name: provider2
    /// - match:
    /// prefix: /bar
    /// requires:
    /// requires_all:
    /// requirements:
    /// - provider_name: provider1
    /// - provider_name: provider2
    #[prost(message, repeated, tag = "2")]
    pub rules: ::prost::alloc::vec::Vec<RequirementRule>,
    /// This message specifies Jwt requirements based on stream_info.filterState.
    /// Other HTTP filters can use it to specify Jwt requirements dynamically.
    /// The `rules` field above is checked first, if it could not find any matches,
    /// check this one.
    #[prost(message, optional, tag = "3")]
    pub filter_state_rules: ::core::option::Option<FilterStateRule>,
    /// When set to true, bypass the `CORS preflight request  <<http://www.w3.org/TR/cors/#cross-origin-request-with-preflight>`\_> regardless of JWT
    /// requirements specified in the rules.
    #[prost(bool, tag = "4")]
    pub bypass_cors_preflight: bool,
    ///
    /// A map of unique requirement_names to JwtRequirements.
    /// : ref:`requirement_name <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.PerRouteConfig.requirement_name>`
    ///   in `PerRouteConfig` uses this map to specify a JwtRequirement.
    #[prost(map = "string, message", tag = "5")]
    pub requirement_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        JwtRequirement,
    >,
    /// A request failing the verification process will receive a 401 downstream with the failure response details
    /// in the body along with WWWAuthenticate header value set with "invalid token". If this value is set to true,
    /// the response details will be stripped and only a 401 response code will be returned. Default value is false
    #[prost(bool, tag = "6")]
    pub strip_failure_response: bool,
}
/// Specify per-route config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerRouteConfig {
    #[prost(oneof = "per_route_config::RequirementSpecifier", tags = "1, 2")]
    pub requirement_specifier: ::core::option::Option<
        per_route_config::RequirementSpecifier,
    >,
}
/// Nested message and enum types in `PerRouteConfig`.
pub mod per_route_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequirementSpecifier {
        /// Disable Jwt Authentication for this route.
        #[prost(bool, tag = "1")]
        Disabled(bool),
        ///
        /// Use requirement_name to specify a JwtRequirement.
        /// This requirement_name MUST be specified at the
        /// : ref:`requirement_map <envoy_v3_api_field_extensions.filters.http.jwt_authn.v3.JwtAuthentication.requirement_map>`
        ///   in `JwtAuthentication`. If no, the requests using this route will be rejected with 403.
        #[prost(string, tag = "2")]
        RequirementName(::prost::alloc::string::String),
    }
}
/// This message specifies a combination of header name and claim name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwtClaimToHeader {
    /// The HTTP header name to copy the claim to.
    /// The header name will be sanitized and replaced.
    #[prost(string, tag = "1")]
    pub header_name: ::prost::alloc::string::String,
    /// The field name for the JWT Claim : it can be a nested claim of type (eg. "claim.nested.key", "sub")
    /// String separated with "." in case of nested claims. The nested claim name must use dot "." to separate
    /// the JSON name path.
    #[prost(string, tag = "2")]
    pub claim_name: ::prost::alloc::string::String,
}
