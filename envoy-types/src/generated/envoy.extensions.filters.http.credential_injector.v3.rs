/// Credential Injector injects credentials into outgoing HTTP requests. The filter configuration is used to retrieve the credentials, or
/// they can be requested through the OAuth2 client credential grant. The credentials obtained are then injected into the Authorization header
/// of the proxied HTTP requests, utilizing either the Basic or Bearer scheme.
///
/// If the credential is not present, the request will fail with 401 Unauthorized if fail_if_not_present is set to true.
///
/// Notice: This filter is intended to be used for workload authentication, which means that the identity associated with the inserted credential
/// is considered as the identity of the workload behind the envoy proxy(in this case, envoy is typically deployed as a sidecar alongside that
/// workload). Please note that this filter does not handle end user authentication. Its purpose is solely to authenticate the workload itself.
///
/// Here is an example of CredentialInjector configuration with Generic credential, which injects an HTTP Basic Auth credential into the proxied requests.
///
/// .. code-block:: yaml
///
/// overwrite: true
/// fail_if_not_present: true
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
/// It can also be configured to inject a Bearer token into the proxied requests.
/// credential.yaml for Bearer Token:
/// .. code-block:: yaml
///
/// resources:
///
/// * "@type": "type.googleapis.com/envoy.extensions.transport_sockets.tls.v3.Secret"
///   name: credential
///   generic_secret:
///   secret:
///   inline_string: "Bearer myToken"
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CredentialInjector {
    /// Whether to overwrite the value or not if the injected headers already exist.
    /// Value defaults to false.
    #[prost(bool, tag = "1")]
    pub overwrite: bool,
    /// Whether to fail the request if the credential is not present.
    /// Value defaults to false.
    /// If set to true, the request will fail with 401 Unauthorized if the credential is not present.
    #[prost(bool, tag = "2")]
    pub fail_if_not_present: bool,
    /// The credential to inject into the proxied requests
    /// TODO add extension-category
    #[prost(message, optional, tag = "3")]
    pub credential: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
}
