#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brotli {
    /// If true, disables "canny" ring buffer allocation strategy.
    /// Ring buffer is allocated according to window size, despite the real size of the content.
    #[prost(bool, tag = "1")]
    pub disable_ring_buffer_reallocation: bool,
    /// Value for decompressor's next output buffer. If not set, defaults to 4096.
    #[prost(message, optional, tag = "2")]
    pub chunk_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
