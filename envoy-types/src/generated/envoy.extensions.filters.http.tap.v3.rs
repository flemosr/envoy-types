/// Top level configuration for the tap filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tap {
    /// Common configuration for the HTTP tap filter.
    #[prost(message, optional, tag = "1")]
    pub common_config: ::core::option::Option<
        super::super::super::super::common::tap::v3::CommonExtensionConfig,
    >,
}
