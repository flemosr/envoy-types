/// Metadata matcher for metadata from http matching input data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// The Metadata is matched if the value retrieved by metadata matching input is matched to this value.
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<
        super::super::super::super::super::r#type::matcher::v3::ValueMatcher,
    >,
    /// If true, the match result will be inverted.
    #[prost(bool, tag = "4")]
    pub invert: bool,
}
