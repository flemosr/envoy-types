/// [#extension: envoy.filters.http.cache]
/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheConfig {
    /// Config specific to the cache storage implementation. Required unless ``disabled``
    /// is true.
    /// [#extension-category: envoy.http.cache]
    #[prost(message, optional, tag = "1")]
    pub typed_config: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Any,
    >,
    /// When true, the cache filter is a no-op filter.
    ///
    /// Possible use-cases for this include:
    /// - Turning a filter on and off with :ref:`ECDS <envoy_v3_api_file_envoy/service/extension/v3/config_discovery.proto>`.
    /// [#comment: once route-specific overrides are implemented, they are the more likely use-case.]
    #[prost(message, optional, tag = "5")]
    pub disabled: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// List of matching rules that defines allowed ``Vary`` headers.
    ///
    /// The ``vary`` response header holds a list of header names that affect the
    /// contents of a response, as described by
    /// <https://httpwg.org/specs/rfc7234.html#caching.negotiated.responses.>
    ///
    /// During insertion, ``allowed_vary_headers`` acts as a allowlist: if a
    /// response's ``vary`` header mentions any header names that aren't matched by any rules in
    /// ``allowed_vary_headers``, that response will not be cached.
    ///
    /// During lookup, ``allowed_vary_headers`` controls what request headers will be
    /// sent to the cache storage implementation.
    #[prost(message, repeated, tag = "2")]
    pub allowed_vary_headers: ::prost::alloc::vec::Vec<
        super::super::super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    /// \[#not-implemented-hide:\]
    /// <TODO(toddmgreer) implement key customization>
    ///
    /// Modifies cache key creation by restricting which parts of the URL are included.
    #[prost(message, optional, tag = "3")]
    pub key_creator_params: ::core::option::Option<cache_config::KeyCreatorParams>,
    /// \[#not-implemented-hide:\]
    /// <TODO(toddmgreer) implement size limit>
    ///
    /// Max body size the cache filter will insert into a cache. 0 means unlimited (though the cache
    /// storage implementation may have its own limit beyond which it will reject insertions).
    #[prost(uint32, tag = "4")]
    pub max_body_bytes: u32,
}
/// Nested message and enum types in `CacheConfig`.
pub mod cache_config {
    /// \[#not-implemented-hide:\]
    /// Modifies cache key creation by restricting which parts of the URL are included.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyCreatorParams {
        /// If true, exclude the URL scheme from the cache key. Set to true if your origins always
        /// produce the same response for http and https requests.
        #[prost(bool, tag = "1")]
        pub exclude_scheme: bool,
        /// If true, exclude the host from the cache key. Set to true if your origins' responses don't
        /// ever depend on host.
        #[prost(bool, tag = "2")]
        pub exclude_host: bool,
        /// If ``query_parameters_included`` is nonempty, only query parameters matched
        /// by one or more of its matchers are included in the cache key. Any other
        /// query params will not affect cache lookup.
        #[prost(message, repeated, tag = "3")]
        pub query_parameters_included: ::prost::alloc::vec::Vec<
            super::super::super::super::super::super::config::route::v3::QueryParameterMatcher,
        >,
        /// If ``query_parameters_excluded`` is nonempty, query parameters matched by one
        /// or more of its matchers are excluded from the cache key (even if also
        /// matched by ``query_parameters_included``), and will not affect cache lookup.
        #[prost(message, repeated, tag = "4")]
        pub query_parameters_excluded: ::prost::alloc::vec::Vec<
            super::super::super::super::super::super::config::route::v3::QueryParameterMatcher,
        >,
    }
}
