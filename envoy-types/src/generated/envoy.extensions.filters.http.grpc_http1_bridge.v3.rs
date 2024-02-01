/// gRPC HTTP/1.1 Bridge filter config.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// If true then requests with content type set to `application/x-protobuf` will be automatically converted to gRPC.
    /// This works by prepending the payload data with the gRPC header frame, as defined by the wiring format, and
    /// Content-Type will be updated accordingly before sending the request.
    /// For the requests that went through this upgrade the filter will also strip the frame before forwarding the
    /// response to the client.
    #[prost(bool, tag = "1")]
    pub upgrade_protobuf_to_grpc: bool,
    /// If true then query parameters in request's URL path will be removed.
    #[prost(bool, tag = "2")]
    pub ignore_query_parameters: bool,
}
