// This file is @generated by prost-build.
/// The fixed heap resource monitor reports the Envoy process memory pressure, computed as a
/// fraction of currently reserved heap memory divided by a statically configured maximum
/// specified in the FixedHeapConfig.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct FixedHeapConfig {
    #[prost(uint64, tag = "1")]
    pub max_heap_size_bytes: u64,
}
