/// Custom configuration for the RBAC audit logger that writes log entries
/// directly to the operating system's standard output.
/// The logger outputs in JSON format and is currently not configurable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StdoutAuditLog {}
