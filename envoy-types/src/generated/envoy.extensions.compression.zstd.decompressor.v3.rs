#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Zstd {
    /// Dictionaries for decompression. Zstd offers dictionary compression, which greatly improves
    /// efficiency on small files and messages. It is necessary to ensure that the dictionary used for
    /// decompression is the same as the compression dictionary. Multiple dictionaries can be set, and the
    /// dictionary will be automatically selected for decompression according to the dictionary ID in the
    /// source content.
    /// Please refer to `zstd manual <<https://github.com/facebook/zstd/blob/dev/programs/zstd.1.md#dictionary-builder>`_>
    /// to train specific dictionaries for decompression.
    #[prost(message, repeated, tag = "1")]
    pub dictionaries: ::prost::alloc::vec::Vec<
        super::super::super::super::super::config::core::v3::DataSource,
    >,
    /// Value for decompressor's next output buffer. If not set, defaults to 4096.
    #[prost(message, optional, tag = "2")]
    pub chunk_size: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
