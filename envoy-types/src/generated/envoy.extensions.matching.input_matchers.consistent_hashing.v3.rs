// This file is @generated by prost-build.
/// The consistent hashing matchers computes a consistent hash from the input and matches if the resulting hash
/// is within the configured threshold.
/// More specifically, this matcher evaluates to true if hash(input, seed) % modulo >= threshold.
/// Note that the consistency of the match result relies on the internal hash function (xxhash) remaining
/// unchanged. While this is unlikely to happen intentionally, this could cause inconsistent match results
/// between deployments.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ConsistentHashing {
    /// The threshold the resulting hash must be over in order for this matcher to evaluate to true.
    /// This value must be below the configured modulo value.
    /// Setting this to 0 is equivalent to this matcher always matching.
    #[prost(uint32, tag = "1")]
    pub threshold: u32,
    /// The value to use for the modulus in the calculation. This effectively  bounds the hash output,
    /// specifying the range of possible values.
    /// This value must be above the configured threshold.
    #[prost(uint32, tag = "2")]
    pub modulo: u32,
    /// Optional seed passed through the hash function. This allows using additional information when computing
    /// the hash value: by changing the seed value, a different partition of matching and non-matching inputs will
    /// be created that remains consistent for that seed value.
    #[prost(uint64, tag = "3")]
    pub seed: u64,
}
