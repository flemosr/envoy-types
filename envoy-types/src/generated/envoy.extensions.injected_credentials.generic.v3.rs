/// Generic extension can be used to inject HTTP Basic Auth, Bearer Token, or any arbitrary credential
/// into the proxied requests.
/// The credential will be injected into the specified HTTP request header.
/// Example:
///
/// .. code-block:: yaml
///
/// credential:
/// name: generic_credential
/// typed_config:
/// "@type": type.googleapis.com/envoy.extensions.injected_credentials.generic.v3.Generic
/// credential:
/// name: credential
/// sds_config:
/// path_config_source:
/// path: credential.yaml
/// header: Authorization
///
/// credential.yaml for Basic Auth:
///
/// .. code-block:: yaml
///
/// resources:
///
/// * "@type": "type.googleapis.com/envoy.extensions.transport_sockets.tls.v3.Secret"
///   name: credential
///   generic_secret:
///   secret:
///   inline_string: "Basic base64EncodedUsernamePassword"
///
/// Refer to [RFC 7617: The 'Basic' HTTP Authentication Scheme](<https://www.rfc-editor.org/rfc/rfc7617>) for details.
///
/// credential.yaml for Bearer Token:
///
/// .. code-block:: yaml
/// resources:
///
/// * "@type": "type.googleapis.com/envoy.extensions.transport_sockets.tls.v3.Secret"
///   name: credential
///   generic_secret:
///   secret:
///   inline_string: "Bearer myToken"
///
/// Refer to [RFC 6750: The OAuth 2.0 Authorization Framework: Bearer Token Usage](<https://www.rfc-editor.org/rfc/rfc6750>) for details.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Generic {
    /// The SDS configuration for the credential that will be injected to the specified HTTP request header.
    /// It must be a generic secret.
    #[prost(message, optional, tag = "1")]
    pub credential: ::core::option::Option<
        super::super::super::transport_sockets::tls::v3::SdsSecretConfig,
    >,
    /// The header that will be injected to the HTTP request with the provided credential.
    /// If not set, filter will default to: `Authorization`
    #[prost(string, tag = "2")]
    pub header: ::prost::alloc::string::String,
}
