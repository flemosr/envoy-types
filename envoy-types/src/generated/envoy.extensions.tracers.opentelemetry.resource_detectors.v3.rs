/// Configuration for the Dynatrace Resource Detector extension.
/// The resource detector reads from the Dynatrace enrichment files
/// and adds host/process related attributes to the OpenTelemetry resource.
///
/// See:
///
/// `Enrich ingested data with Dynatrace-specific dimensions <<https://docs.dynatrace.com/docs/shortlink/enrichment-files>`\_>
///
/// \[\#extension: envoy.tracers.opentelemetry.resource_detectors.dynatrace\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynatraceResourceDetectorConfig {}
/// Configuration for the Environment Resource detector extension.
/// The resource detector reads from the `OTEL_RESOURCE_ATTRIBUTES`
/// environment variable, as per the OpenTelemetry specification.
///
/// See:
///
/// `OpenTelemetry specification <<https://github.com/open-telemetry/opentelemetry-specification/blob/v1.24.0/specification/resource/sdk.md#detecting-resource-information-from-the-environment>`\_>
///
/// \[\#extension: envoy.tracers.opentelemetry.resource_detectors.environment\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentResourceDetectorConfig {}
/// Configuration for the Static Resource detector extension.
/// The resource detector uses static config for resource attribute,
/// as per the OpenTelemetry specification.
///
/// \[\#extension: envoy.tracers.opentelemetry.resource_detectors.static_config\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticConfigResourceDetectorConfig {
    /// Custom Resource attributes to be included.
    #[prost(map = "string, string", tag = "1")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
