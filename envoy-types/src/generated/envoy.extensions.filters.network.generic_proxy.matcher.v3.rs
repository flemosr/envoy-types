/// Used to match request service of the downstream request. Only applicable if a service provided
/// by the application protocol.
/// This is deprecated and should be replaced by HostMatchInput. This is kept for backward compatibility.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceMatchInput {}
/// Used to match request host of the generic downstream request. Only applicable if a host provided
/// by the application protocol.
/// This is same with the ServiceMatchInput and this should be preferred over ServiceMatchInput.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostMatchInput {}
/// Used to match request path of the generic downstream request. Only applicable if a path provided
/// by the application protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PathMatchInput {}
/// Used to match request method of the generic downstream request. Only applicable if a method provided
/// by the application protocol.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MethodMatchInput {}
/// Used to match an arbitrary property of the generic downstream request.
/// These properties are populated by the codecs of application protocols.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyMatchInput {
    /// The property name to match on.
    #[prost(string, tag = "1")]
    pub property_name: ::prost::alloc::string::String,
}
/// Used to match an whole generic downstream request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMatchInput {}
/// Used to match an arbitrary key-value pair for headers, trailers or properties.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValueMatchEntry {
    /// The key name to match on.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The key value pattern.
    #[prost(message, optional, tag = "2")]
    pub string_match: ::core::option::Option<
        super::super::super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
}
/// Custom matcher to match on the generic downstream request. This is used to match
/// multiple fields of the downstream request and avoid complex combinations of
/// HostMatchInput, PathMatchInput, MethodMatchInput and PropertyMatchInput.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMatcher {
    /// Optional host pattern to match on. If not specified, any host will match.
    #[prost(message, optional, tag = "1")]
    pub host: ::core::option::Option<
        super::super::super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    /// Optional path pattern to match on. If not specified, any path will match.
    #[prost(message, optional, tag = "2")]
    pub path: ::core::option::Option<
        super::super::super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    /// Optional method pattern to match on. If not specified, any method will match.
    #[prost(message, optional, tag = "3")]
    pub method: ::core::option::Option<
        super::super::super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    /// Optional arbitrary properties to match on. If not specified, any properties
    /// will match. The key is the property name and the value is the property value
    /// to match on.
    #[prost(message, repeated, tag = "4")]
    pub properties: ::prost::alloc::vec::Vec<KeyValueMatchEntry>,
}
