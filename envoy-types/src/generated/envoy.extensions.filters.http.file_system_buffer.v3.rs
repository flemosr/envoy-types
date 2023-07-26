/// The behavior of the filter for a stream.
/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BufferBehavior {
    #[prost(oneof = "buffer_behavior::Behavior", tags = "1, 2, 3, 4, 5")]
    pub behavior: ::core::option::Option<buffer_behavior::Behavior>,
}
/// Nested message and enum types in `BufferBehavior`.
pub mod buffer_behavior {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StreamWhenPossible {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Bypass {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InjectContentLengthIfNecessary {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FullyBufferAndAlwaysInjectContentLength {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FullyBuffer {}
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Behavior {
        /// Don't inject ``content-length`` header.
        /// Output immediately, buffer only if output is slower than input.
        #[prost(message, tag = "1")]
        StreamWhenPossible(StreamWhenPossible),
        /// Never buffer, do nothing.
        #[prost(message, tag = "2")]
        Bypass(Bypass),
        /// If ``content-length`` is not present, buffer the entire input,
        /// inject ``content-length`` header, then output.
        /// If ``content-length`` is already present, act like ``stream_when_possible``.
        #[prost(message, tag = "3")]
        InjectContentLengthIfNecessary(InjectContentLengthIfNecessary),
        /// Always buffer the entire input, and inject ``content-length``,
        /// overwriting any provided content-length header.
        #[prost(message, tag = "4")]
        FullyBufferAndAlwaysInjectContentLength(FullyBufferAndAlwaysInjectContentLength),
        /// Always buffer the entire input, do not modify ``content-length``.
        #[prost(message, tag = "5")]
        FullyBuffer(FullyBuffer),
    }
}
/// The configuration for one direction of the filter behavior.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamConfig {
    /// Whether to bypass / stream / fully buffer / etc.
    /// If unset in route, vhost and listener config, the default is ``stream_when_possible``.
    #[prost(message, optional, tag = "1")]
    pub behavior: ::core::option::Option<BufferBehavior>,
    /// The amount stored in the memory buffer before buffering to disk.
    /// If unset in route, vhost and listener config, defaults to a hardcoded value of 1MiB
    #[prost(message, optional, tag = "2")]
    pub memory_buffer_bytes_limit: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// The maximum storage (excluding memory) to be buffered in this filter.
    /// If unset in route, vhost and listener config, defaults to a hardcoded value of 32MiB
    #[prost(message, optional, tag = "3")]
    pub storage_buffer_bytes_limit: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt64Value,
    >,
    /// The maximum amount that can be queued for writing to storage, above which the
    /// source is requested to pause. If unset, defaults to the same value as
    /// ``memory_buffer_bytes_limit``.
    ///
    /// For example, assuming the recipient is not consuming data at all, if
    /// ``memory_buffer_bytes_limit`` was 32MiB, and ``storage_buffer_queue_high_watermark_bytes``
    /// was 64MiB, and the filesystem is backed up so writes are not occurring promptly,
    /// then:
    ///
    /// * Any request less than 32MiB will eventually pass through without ever attempting
    ///    to write to disk.
    /// * Any request with over 32MiB buffered will start trying to write to disk.
    ///    If it reaches (32+64)MiB buffered in memory (write to disk isn't keeping up), a high
    ///    watermark signal is sent to the source.
    /// * Any stream whose total size exceeds
    ///    ``memory_buffer_bytes_limit + storage_buffer_bytes_limit`` will provoke an error.
    ///    (Note, if the recipient *is* consuming data then it is possible for such an
    ///    oversized request to pass through the buffer filter, provided the recipient
    ///    isn't consuming data too slowly.)
    ///
    /// The low watermark signal is sent when the memory buffer is at size
    /// ``memory_buffer_bytes_limit + (storage_buffer_queue_high_watermark_bytes / 2)``.
    #[prost(message, optional, tag = "4")]
    pub storage_buffer_queue_high_watermark_bytes: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt64Value,
    >,
}
/// A :ref:`file system buffer <config_http_filters_file_system_buffer>` filter configuration.
///
/// Route-specific configs override only the fields they explicitly include; unset
/// fields inherit from the vhost or listener-level config, or, if never set,
/// and not required, use a default value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSystemBufferFilterConfig {
    /// A configuration for an AsyncFileManager.
    ///
    /// If unset in route, vhost and listener, and the behavior is not ``bypass``
    /// in both directions, an Internal Server Error response will be sent.
    #[prost(message, optional, tag = "1")]
    pub manager_config: ::core::option::Option<
        super::super::super::super::common::async_files::v3::AsyncFileManagerConfig,
    >,
    /// An optional path to which the unlinked files should be written - this may
    /// determine which physical storage device will be used.
    ///
    /// If unset in route, vhost and listener, will use the environment variable
    /// ``TMPDIR``, or, if that's also unset, will use ``/tmp``.
    #[prost(message, optional, tag = "2")]
    pub storage_buffer_path: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::StringValue,
    >,
    /// Optional configuration for how to buffer (or not) requests.
    /// If unset in route, vhost and listener, ``StreamConfig`` default values will be used
    /// (with behavior ``stream_when_possible``)
    #[prost(message, optional, tag = "3")]
    pub request: ::core::option::Option<StreamConfig>,
    /// Optional configuration for how to buffer (or not) responses.
    /// If unset in route, vhost and listener, ``StreamConfig`` default values will be used
    /// (with behavior ``stream_when_possible``)
    #[prost(message, optional, tag = "4")]
    pub response: ::core::option::Option<StreamConfig>,
}
