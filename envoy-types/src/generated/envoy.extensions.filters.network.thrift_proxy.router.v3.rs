#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Router {
    /// Close downstream connection in case of routing or upstream connection problem. Default: true
    #[prost(message, optional, tag = "1")]
    pub close_downstream_on_upstream_error: ::core::option::Option<
        super::super::super::super::super::super::super::google::protobuf::BoolValue,
    >,
}
