#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Router {
    /// Set to true if the upstream connection should be bound to the downstream connection, false
    /// otherwise.
    ///
    /// By default, one random upstream connection will be selected from the upstream connection pool
    /// and used for every request. And after the request is finished, the upstream connection will be
    /// released back to the upstream connection pool.
    ///
    /// If this option is true, the upstream connection will be bound to the downstream connection and
    /// have same lifetime as the downstream connection. The same upstream connection will be used for
    /// all requests from the same downstream connection.
    ///
    /// And if this options is true, one of the following requirements must be met:
    ///
    /// 1. The request must be handled one by one. That is, the next request can not be sent to the
    ///    upstream until the previous request is finished.
    /// 1. Unique request id must be provided for each request and response. The request id must be
    ///    unique for each request and response pair in same connection pair. And the request id must
    ///    be the same for the corresponding request and response.
    ///
    /// This could be useful for some protocols that require the same upstream connection to be used
    /// for all requests from the same downstream connection. For example, the protocol using stateful
    /// connection.
    #[prost(bool, tag = "1")]
    pub bind_upstream_connection: bool,
}
