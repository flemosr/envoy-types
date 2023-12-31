/// Reads an environment variable to provide an input for matching.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// Name of the environment variable to read from.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
