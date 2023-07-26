#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// The metadata namespace.
    #[prost(string, tag = "1")]
    pub metadata_namespace: ::prost::alloc::string::String,
    /// The value to update the namespace with. See
    /// :ref:`the filter documentation <config_http_filters_set_metadata>` for
    /// more information on how this value is merged with potentially existing
    /// ones.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Struct,
    >,
}
