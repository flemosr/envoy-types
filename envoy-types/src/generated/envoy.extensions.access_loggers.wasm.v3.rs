/// Custom configuration for an :ref:`AccessLog <envoy_v3_api_msg_config.accesslog.v3.AccessLog>`
/// that calls into a WASM VM. Configures the built-in `envoy.access_loggers.wasm`
/// AccessLog.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WasmAccessLog {
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<super::super::super::wasm::v3::PluginConfig>,
}
