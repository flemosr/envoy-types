/// Configuration for a connection ID generator implementation which issues predictable CIDs with stable first 4 bytes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeterministicConnectionIdGeneratorConfig {}
