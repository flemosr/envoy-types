/// Health check event file sink.
/// The health check event will be converted to JSON.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckEventFileSink {
    /// Specifies the path to the health check event log.
    #[prost(string, tag = "1")]
    pub event_log_path: ::prost::alloc::string::String,
}
