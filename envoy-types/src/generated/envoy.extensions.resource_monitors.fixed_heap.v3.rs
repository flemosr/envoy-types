/// The fixed heap resource monitor reports the Envoy process memory pressure, computed as a
/// fraction of currently reserved heap memory divided by a statically configured maximum
/// specified in the FixedHeapConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedHeapConfig {
    #[prost(uint64, tag = "1")]
    pub max_heap_size_bytes: u64,
}
