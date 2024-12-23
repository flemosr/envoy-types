// This file is @generated by prost-build.
/// API Key HTTP authentication.
///
/// For example, the following configuration configures the filter to authenticate the clients using
/// the API key from the header `X-API-KEY`. And only the clients with the key `real-key` are
/// considered as authenticated.
///
/// .. code-block:: yaml
///
/// ```text
/// credentials:
/// - key: real-key
///   client: user
/// key_sources:
/// - header: "X-API-KEY"
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiKeyAuth {
    /// The credentials that are used to authenticate the clients.
    #[prost(message, repeated, tag = "1")]
    pub credentials: ::prost::alloc::vec::Vec<Credential>,
    /// The key sources to fetch the key from the coming request.
    #[prost(message, repeated, tag = "2")]
    pub key_sources: ::prost::alloc::vec::Vec<KeySource>,
}
/// API key auth configuration of per route or per virtual host or per route configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiKeyAuthPerRoute {
    /// The credentials that are used to authenticate the clients. If this field is non-empty, then the
    /// credentials in the filter level configuration will be ignored and the credentials in this
    /// configuration will be used.
    #[prost(message, repeated, tag = "1")]
    pub credentials: ::prost::alloc::vec::Vec<Credential>,
    /// The key sources to fetch the key from the coming request. If this field is non-empty, then the
    /// key sources in the filter level configuration will be ignored and the key sources in this
    /// configuration will be used.
    #[prost(message, repeated, tag = "2")]
    pub key_sources: ::prost::alloc::vec::Vec<KeySource>,
    /// A list of clients that are allowed to access the route or vhost. The clients listed here
    /// should be subset of the clients listed in the `credentials` to provide authorization control
    /// after the authentication is successful. If the list is empty, then all authenticated clients
    /// are allowed. This provides very limited but simple authorization. If more complex authorization
    /// is required, then use the :ref:`HTTP RBAC filter <config_http_filters_rbac>` instead.
    ///
    /// .. note::
    /// Setting this field and `credentials` at the same configuration entry is not an error but
    /// also makes no much sense because they provide similar functionality. Please only use
    /// one of them at same configuration entry except for the case that you want to share the same
    /// credentials list across multiple routes but still use different allowed clients for each
    /// route.
    #[prost(string, repeated, tag = "3")]
    pub allowed_clients: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Single credential entry that contains the API key and the related client id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Credential {
    /// The value of the unique API key.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The unique id or identity that used to identify the client or consumer.
    #[prost(string, tag = "2")]
    pub client: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeySource {
    /// The header name to fetch the key. If multiple header values are present, the first one will be
    /// used. If the header value starts with 'Bearer ', this prefix will be stripped to get the
    /// key value.
    ///
    /// If set, takes precedence over `query` and `cookie`.
    #[prost(string, tag = "1")]
    pub header: ::prost::alloc::string::String,
    /// The query parameter name to fetch the key. If multiple query values are present, the first one
    /// will be used.
    ///
    /// The field will be used if `header` is not set. If set, takes precedence over `cookie`.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// The cookie name to fetch the key.
    ///
    /// The field will be used if the `header` and `query` are not set.
    #[prost(string, tag = "3")]
    pub cookie: ::prost::alloc::string::String,
}
