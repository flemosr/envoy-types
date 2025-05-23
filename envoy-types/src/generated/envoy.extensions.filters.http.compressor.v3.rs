// This file is @generated by prost-build.
/// \[\#next-free-field: 10\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Compressor {
    /// Minimum response length, in bytes, which will trigger compression. The default value is 30.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub content_length: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Set of strings that allows specifying which mime-types yield compression; e.g.,
    /// application/json, text/html, etc. When this field is not defined, compression will be applied
    /// to the following mime-types: "application/javascript", "application/json",
    /// "application/xhtml+xml", "image/svg+xml", "text/css", "text/html", "text/plain", "text/xml"
    /// and their synonyms.
    #[deprecated]
    #[prost(string, repeated, tag = "2")]
    pub content_type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If true, disables compression when the response contains an etag header. When it is false, the
    /// filter will preserve weak etags and remove the ones that require strong validation.
    #[deprecated]
    #[prost(bool, tag = "3")]
    pub disable_on_etag_header: bool,
    /// If true, removes accept-encoding from the request headers before dispatching it to the upstream
    /// so that responses do not get compressed before reaching the filter.
    ///
    /// .. attention::
    ///
    /// ```text
    /// To avoid interfering with other compression filters in the same chain use this option in
    /// the filter closest to the upstream.
    /// ```
    #[deprecated]
    #[prost(bool, tag = "4")]
    pub remove_accept_encoding_header: bool,
    /// Runtime flag that controls whether the filter is enabled or not. If set to false, the
    /// filter will operate as a pass-through filter, unless overridden by
    /// CompressorPerRoute. If not specified, defaults to enabled.
    #[deprecated]
    #[prost(message, optional, tag = "5")]
    pub runtime_enabled: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFeatureFlag,
    >,
    ///
    /// A compressor library to use for compression. Currently only
    /// : ref:`envoy.compression.gzip.compressor<envoy_v3_api_msg_extensions.compression.gzip.compressor.v3.Gzip>`
    ///   is included in Envoy.
    ///   \[\#extension-category: envoy.compression.compressor\]
    #[prost(message, optional, tag = "6")]
    pub compressor_library: ::core::option::Option<
        super::super::super::super::super::config::core::v3::TypedExtensionConfig,
    >,
    /// Configuration for request compression. Compression is disabled by default if left empty.
    #[prost(message, optional, tag = "7")]
    pub request_direction_config: ::core::option::Option<
        compressor::RequestDirectionConfig,
    >,
    /// Configuration for response compression. Compression is enabled by default if left empty.
    ///
    /// .. attention::
    ///
    /// ```text
    /// If the field is not empty then the duplicate deprecated fields of the ``Compressor`` message,
    /// such as ``content_length``, ``content_type``, ``disable_on_etag_header``,
    /// ``remove_accept_encoding_header`` and ``runtime_enabled``, are ignored.
    ///
    /// Also all the statistics related to response compression will be rooted in
    /// ``<stat_prefix>.compressor.<compressor_library.name>.<compressor_library_stat_prefix>.response.*``
    /// instead of
    /// ``<stat_prefix>.compressor.<compressor_library.name>.<compressor_library_stat_prefix>.*``.
    /// ```
    #[prost(message, optional, tag = "8")]
    pub response_direction_config: ::core::option::Option<
        compressor::ResponseDirectionConfig,
    >,
    /// If true, chooses this compressor first to do compression when the q-values in `Accept-Encoding` are same.
    /// The last compressor which enables choose_first will be chosen if multiple compressor filters in the chain have choose_first as true.
    #[prost(bool, tag = "9")]
    pub choose_first: bool,
}
/// Nested message and enum types in `Compressor`.
pub mod compressor {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommonDirectionConfig {
        /// Runtime flag that controls whether compression is enabled or not for the direction this
        /// common config is put in. If set to false, the filter will operate as a pass-through filter
        /// in the chosen direction, unless overridden by CompressorPerRoute.
        /// If the field is omitted, the filter will be enabled.
        #[prost(message, optional, tag = "1")]
        pub enabled: ::core::option::Option<
            super::super::super::super::super::super::config::core::v3::RuntimeFeatureFlag,
        >,
        /// Minimum value of Content-Length header of request or response messages (depending on the direction
        /// this common config is put in), in bytes, which will trigger compression. The default value is 30.
        #[prost(message, optional, tag = "2")]
        pub min_content_length: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// Set of strings that allows specifying which mime-types yield compression; e.g.,
        /// application/json, text/html, etc. When this field is not defined, compression will be applied
        /// to the following mime-types: "application/javascript", "application/json",
        /// "application/xhtml+xml", "image/svg+xml", "text/css", "text/html", "text/plain", "text/xml"
        /// and their synonyms.
        #[prost(string, repeated, tag = "3")]
        pub content_type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Configuration for filter behavior on the request direction.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestDirectionConfig {
        #[prost(message, optional, tag = "1")]
        pub common_config: ::core::option::Option<CommonDirectionConfig>,
    }
    /// Configuration for filter behavior on the response direction.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResponseDirectionConfig {
        #[prost(message, optional, tag = "1")]
        pub common_config: ::core::option::Option<CommonDirectionConfig>,
        /// If true, disables compression when the response contains an etag header. When it is false, the
        /// filter will preserve weak etags and remove the ones that require strong validation.
        #[prost(bool, tag = "2")]
        pub disable_on_etag_header: bool,
        /// If true, removes accept-encoding from the request headers before dispatching it to the upstream
        /// so that responses do not get compressed before reaching the filter.
        ///
        /// .. attention::
        ///
        /// ```text
        /// To avoid interfering with other compression filters in the same chain use this option in
        /// the filter closest to the upstream.
        /// ```
        #[prost(bool, tag = "3")]
        pub remove_accept_encoding_header: bool,
        /// Set of response codes for which compression is disabled, e.g. 206 Partial Content should not
        /// be compressed.
        #[prost(uint32, repeated, packed = "false", tag = "4")]
        pub uncompressible_response_codes: ::prost::alloc::vec::Vec<u32>,
    }
}
/// Per-route overrides of `ResponseDirectionConfig`. Anything added here should be optional,
/// to allow overriding arbitrary subsets of configuration. Omitted fields must have no effect.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ResponseDirectionOverrides {
    ///
    /// If set, overrides the filter-level
    /// : ref:`remove_accept_encoding_header<envoy_v3_api_field_extensions.filters.http.compressor.v3.Compressor.ResponseDirectionConfig.remove_accept_encoding_header>`.
    #[prost(message, optional, tag = "1")]
    pub remove_accept_encoding_header: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
/// Per-route overrides. As per-route overrides are needed, they should be
/// added here, mirroring the structure of `Compressor`. All fields should be
/// optional, to allow overriding arbitrary subsets of configuration.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CompressorOverrides {
    /// If present, response compression is enabled.
    #[prost(message, optional, tag = "1")]
    pub response_direction_config: ::core::option::Option<ResponseDirectionOverrides>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CompressorPerRoute {
    #[prost(oneof = "compressor_per_route::Override", tags = "1, 2")]
    pub r#override: ::core::option::Option<compressor_per_route::Override>,
}
/// Nested message and enum types in `CompressorPerRoute`.
pub mod compressor_per_route {
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum Override {
        /// If set, the filter will operate as a pass-through filter.
        /// Overrides Compressor.runtime_enabled and CommonDirectionConfig.enabled.
        #[prost(bool, tag = "1")]
        Disabled(bool),
        /// Per-route overrides. Fields set here will override corresponding fields in `Compressor`.
        #[prost(message, tag = "2")]
        Overrides(super::CompressorOverrides),
    }
}
