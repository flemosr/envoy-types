/// Basic HTTP authentication.
///
/// Example:
///
/// .. code-block:: yaml
///
/// users:
/// inline_string: |-
/// user1:{SHA}hashed_user1_password
/// user2:{SHA}hashed_user2_password
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicAuth {
    /// Username-password pairs used to verify user credentials in the "Authorization" header.
    /// The value needs to be the htpasswd format.
    /// Reference to <https://httpd.apache.org/docs/2.4/programs/htpasswd.html>
    #[prost(message, optional, tag = "1")]
    pub users: ::core::option::Option<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// This field specifies the header name to forward a successfully authenticated user to
    /// the backend. The header will be added to the request with the username as the value.
    ///
    /// If it is not specified, the username will not be forwarded.
    #[prost(string, tag = "2")]
    pub forward_username_header: ::prost::alloc::string::String,
}
