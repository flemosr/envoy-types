#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlwaysOnSamplerConfig {}
/// Configuration for the Dynatrace Sampler extension.
/// \[\#extension: envoy.tracers.opentelemetry.samplers.dynatrace\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynatraceSamplerConfig {
    /// The Dynatrace tenant.
    ///
    /// The value can be obtained from the Envoy deployment page in Dynatrace.
    #[prost(string, tag = "1")]
    pub tenant: ::prost::alloc::string::String,
    /// The id of the Dynatrace cluster id.
    ///
    /// The value can be obtained from the Envoy deployment page in Dynatrace.
    #[prost(int32, tag = "2")]
    pub cluster_id: i32,
    /// The HTTP service to fetch the sampler configuration from the Dynatrace API (root spans per minute). For example:
    ///
    /// .. code-block:: yaml
    ///
    /// ```text
    /// http_service:
    ///   http_uri:
    ///     cluster: dynatrace
    ///     uri: <tenant>.dev.dynatracelabs.com/api/v2/samplingConfiguration
    ///     timeout: 10s
    ///   request_headers_to_add:
    ///   - header:
    ///       key : "authorization"
    ///       value: "Api-Token dt..."
    /// ```
    #[prost(message, optional, tag = "3")]
    pub http_service: ::core::option::Option<
        super::super::super::super::super::config::core::v3::HttpService,
    >,
    /// Default number of root spans per minute, used when the value can't be obtained from the Dynatrace API.
    ///
    /// A default value of `1000` is used when:
    ///
    /// * `root_spans_per_minute` is unset
    /// * `root_spans_per_minute` is set to 0
    #[prost(uint32, tag = "4")]
    pub root_spans_per_minute: u32,
}
