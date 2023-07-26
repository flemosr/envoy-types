/// The injected resource monitor allows injecting a synthetic resource pressure into Envoy
/// via a text file, which must contain a floating-point number in the range \[0..1\] representing
/// the resource pressure and be updated atomically by a symbolic link swap.
/// This is intended primarily for integration tests to force Envoy into an overloaded state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectedResourceConfig {
    #[prost(string, tag = "1")]
    pub filename: ::prost::alloc::string::String,
}
