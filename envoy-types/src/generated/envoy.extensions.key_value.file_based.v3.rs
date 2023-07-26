/// \[\#extension: envoy.key_value.file_based\]
/// This is configuration to flush a key value store out to disk.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileBasedKeyValueStoreConfig {
    /// The filename to read the keys and values from, and write the keys and
    /// values to.
    #[prost(string, tag = "1")]
    pub filename: ::prost::alloc::string::String,
    /// The interval at which the key value store should be flushed to the file.
    #[prost(message, optional, tag = "2")]
    pub flush_interval: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The maximum number of entries to cache, or 0 to allow for unlimited entries.
    /// Defaults to 1000 if not present.
    #[prost(message, optional, tag = "3")]
    pub max_entries: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
}
