#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gzip {
    /// Value from 9 to 15 that represents the base two logarithmic of the decompressor's window size.
    /// The decompression window size needs to be equal or larger than the compression window size.
    /// The default window size is 15.
    /// This is so that the decompressor can decompress a response compressed by a compressor with any compression window size.
    /// For more details about this parameter, please refer to `zlib manual <<https://www.zlib.net/manual.html>`_> > inflateInit2.
    #[prost(message, optional, tag = "1")]
    pub window_bits: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Value for zlib's decompressor output buffer. If not set, defaults to 4096.
    /// See <https://www.zlib.net/manual.html> for more details.
    #[prost(message, optional, tag = "2")]
    pub chunk_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// An upper bound to the number of times the output buffer is allowed to be bigger than the size of
    /// the accumulated input. This value is used to prevent decompression bombs. If not set, defaults to 100.
    /// [#comment:TODO(rojkov): Re-design the Decompressor interface to handle compression bombs gracefully instead of this quick solution.
    /// See <https://github.com/envoyproxy/envoy/commit/d4c39e635603e2f23e1e08ddecf5a5fb5a706338> for details.]
    #[prost(message, optional, tag = "3")]
    pub max_inflate_ratio: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
