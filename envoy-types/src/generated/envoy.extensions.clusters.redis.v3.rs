/// [#next-free-field: 7]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedisClusterConfig {
    /// Interval between successive topology refresh requests. If not set, this defaults to 5s.
    #[prost(message, optional, tag = "1")]
    pub cluster_refresh_rate: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// Timeout for topology refresh request. If not set, this defaults to 3s.
    #[prost(message, optional, tag = "2")]
    pub cluster_refresh_timeout: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The minimum interval that must pass after triggering a topology refresh request before a new
    /// request can possibly be triggered again. Any errors received during one of these
    /// time intervals are ignored. If not set, this defaults to 5s.
    #[prost(message, optional, tag = "3")]
    pub redirect_refresh_interval: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The number of redirection errors that must be received before
    /// triggering a topology refresh request. If not set, this defaults to 5.
    /// If this is set to 0, topology refresh after redirect is disabled.
    #[prost(message, optional, tag = "4")]
    pub redirect_refresh_threshold: ::core::option::Option<
        super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// The number of failures that must be received before triggering a topology refresh request.
    /// If not set, this defaults to 0, which disables the topology refresh due to failure.
    #[prost(uint32, tag = "5")]
    pub failure_refresh_threshold: u32,
    /// The number of hosts became degraded or unhealthy before triggering a topology refresh request.
    /// If not set, this defaults to 0, which disables the topology refresh due to degraded or
    /// unhealthy host.
    #[prost(uint32, tag = "6")]
    pub host_degraded_refresh_threshold: u32,
}
