/// Rate limit :ref:`configuration overview <config_rate_limit_service>`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateLimitServiceConfig {
    /// Specifies the gRPC service that hosts the rate limit service. The client
    /// will connect to this cluster when it needs to make rate limit service
    /// requests.
    #[prost(message, optional, tag = "2")]
    pub grpc_service: ::core::option::Option<super::super::core::v3::GrpcService>,
    /// API version for rate limit transport protocol. This describes the rate limit gRPC endpoint and
    /// version of messages used on the wire.
    #[prost(enumeration = "super::super::core::v3::ApiVersion", tag = "4")]
    pub transport_api_version: i32,
}
