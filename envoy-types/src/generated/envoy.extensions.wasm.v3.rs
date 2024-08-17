/// Configuration for restricting Proxy-Wasm capabilities available to modules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityRestrictionConfig {
    /// The Proxy-Wasm capabilities which will be allowed. Capabilities are mapped by
    /// name. The `SanitizationConfig` which each capability maps to is currently unimplemented and ignored,
    /// and so should be left empty.
    ///
    /// The capability names are given in the
    /// `Proxy-Wasm ABI <<https://github.com/proxy-wasm/spec/tree/master/abi-versions/vNEXT>`*.>
    /// Additionally, the following WASI capabilities from
    /// `this list <<https://github.com/WebAssembly/WASI/blob/master/phases/snapshot/docs.md#modules>`*>
    /// are implemented and can be allowed:
    /// `fd_write`, `fd_read`, `fd_seek`, `fd_close`, `fd_fdstat_get`, `environ_get`, `environ_sizes_get`,
    /// `args_get`, `args_sizes_get`, `proc_exit`, `clock_time_get`, `random_get`.
    #[prost(map = "string, message", tag = "1")]
    pub allowed_capabilities: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        SanitizationConfig,
    >,
}
/// Configuration for sanitization of inputs to an allowed capability.
///
/// NOTE: This is currently unimplemented.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SanitizationConfig {}
/// Configuration for a Wasm VM.
/// \[\#next-free-field: 8\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmConfig {
    /// An ID which will be used along with a hash of the wasm code (or the name of the registered Null
    /// VM plugin) to determine which VM will be used for the plugin. All plugins which use the same
    /// `vm_id` and code will use the same VM. May be left blank. Sharing a VM between plugins can
    /// reduce memory utilization and make sharing of data easier which may have security implications.
    /// \[\#comment: TODO: add ref for details.\]
    #[prost(string, tag = "1")]
    pub vm_id: ::prost::alloc::string::String,
    /// The Wasm runtime type, defaults to the first available Wasm engine used at Envoy build-time.
    /// The priority to search for the available engine is: v8 -> wasmtime -> wamr.
    /// Available Wasm runtime types are registered as extensions. The following runtimes are included
    /// in Envoy code base:
    ///
    /// .. \_extension_envoy.wasm.runtime.null:
    ///
    /// **envoy.wasm.runtime.null**: Null sandbox, the Wasm module must be compiled and linked into the
    /// Envoy binary. The registered name is given in the `code` field as `inline_string`.
    ///
    /// .. \_extension_envoy.wasm.runtime.v8:
    ///
    /// **envoy.wasm.runtime.v8**: `V8 <<https://v8.dev/>`\_-based> WebAssembly runtime.
    ///
    /// .. \_extension_envoy.wasm.runtime.wamr:
    ///
    /// **envoy.wasm.runtime.wamr**: `WAMR <<https://github.com/bytecodealliance/wasm-micro-runtime/>`\_-based> WebAssembly runtime.
    /// This runtime is not enabled in the official build.
    ///
    /// .. \_extension_envoy.wasm.runtime.wasmtime:
    ///
    /// **envoy.wasm.runtime.wasmtime**: `Wasmtime <<https://wasmtime.dev/>`\_-based> WebAssembly runtime.
    /// This runtime is not enabled in the official build.
    ///
    /// \[\#extension-category: envoy.wasm.runtime\]
    #[prost(string, tag = "2")]
    pub runtime: ::prost::alloc::string::String,
    /// The Wasm code that Envoy will execute.
    #[prost(message, optional, tag = "3")]
    pub code: ::core::option::Option<
        super::super::super::config::core::v3::AsyncDataSource,
    >,
    /// The Wasm configuration used in initialization of a new VM
    /// (proxy_on_start). `google.protobuf.Struct` is serialized as JSON before
    /// passing it to the plugin. `google.protobuf.BytesValue` and
    /// `google.protobuf.StringValue` are passed directly without the wrapper.
    #[prost(message, optional, tag = "4")]
    pub configuration: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// Allow the wasm file to include pre-compiled code on VMs which support it.
    /// Warning: this should only be enable for trusted sources as the precompiled code is not
    /// verified.
    #[prost(bool, tag = "5")]
    pub allow_precompiled: bool,
    /// If true and the code needs to be remotely fetched and it is not in the cache then NACK the configuration
    /// update and do a background fetch to fill the cache, otherwise fetch the code asynchronously and enter
    /// warming state.
    #[prost(bool, tag = "6")]
    pub nack_on_code_cache_miss: bool,
    /// Specifies environment variables to be injected to this VM which will be available through
    /// WASI's `environ_get` and `environ_get_sizes` system calls. Note that these functions
    /// are generally called implicitly by your language's standard library. Therefore, you do not
    /// need to call them directly. You can access environment variables in the same way you would
    /// on native platforms.
    /// Warning: Envoy rejects the configuration if there's conflict of key space.
    #[prost(message, optional, tag = "7")]
    pub environment_variables: ::core::option::Option<EnvironmentVariables>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentVariables {
    /// The keys of *Envoy's* environment variables exposed to this VM. In other words, if a key exists in Envoy's environment
    /// variables, then that key-value pair will be injected. Note that if a key does not exist, it will be ignored.
    #[prost(string, repeated, tag = "1")]
    pub host_env_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Explicitly given key-value pairs to be injected to this VM in the form of "KEY=VALUE".
    #[prost(map = "string, string", tag = "2")]
    pub key_values: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Base Configuration for Wasm Plugins e.g. filters and services.
/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginConfig {
    /// A unique name for a filters/services in a VM for use in identifying the filter/service if
    /// multiple filters/services are handled by the same `vm_id` and `root_id` and for
    /// logging/debugging.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A unique ID for a set of filters/services in a VM which will share a RootContext and Contexts
    /// if applicable (e.g. an Wasm HttpFilter and an Wasm AccessLog). If left blank, all
    /// filters/services with a blank root_id with the same `vm_id` will share Context(s).
    #[prost(string, tag = "2")]
    pub root_id: ::prost::alloc::string::String,
    /// Filter/service configuration used to configure or reconfigure a plugin
    /// (`proxy_on_configure`).
    /// `google.protobuf.Struct` is serialized as JSON before
    /// passing it to the plugin. `google.protobuf.BytesValue` and
    /// `google.protobuf.StringValue` are passed directly without the wrapper.
    #[prost(message, optional, tag = "4")]
    pub configuration: ::core::option::Option<
        super::super::super::super::google::protobuf::Any,
    >,
    /// If there is a fatal error on the VM (e.g. exception, abort(), on_start or on_configure return false),
    /// then all plugins associated with the VM will either fail closed (by default), e.g. by returning an HTTP 503 error,
    /// or fail open (if 'fail_open' is set to true) by bypassing the filter. Note: when on_start or on_configure return false
    /// during xDS updates the xDS configuration will be rejected and when on_start or on_configuration return false on initial
    /// startup the proxy will not start.
    #[prost(bool, tag = "5")]
    pub fail_open: bool,
    /// Configuration for restricting Proxy-Wasm capabilities available to modules.
    #[prost(message, optional, tag = "6")]
    pub capability_restriction_config: ::core::option::Option<
        CapabilityRestrictionConfig,
    >,
    /// Configuration for finding or starting VM.
    #[prost(oneof = "plugin_config::Vm", tags = "3")]
    pub vm: ::core::option::Option<plugin_config::Vm>,
}
/// Nested message and enum types in `PluginConfig`.
pub mod plugin_config {
    /// Configuration for finding or starting VM.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Vm {
        /// TODO: add referential VM configurations.
        #[prost(message, tag = "3")]
        VmConfig(super::VmConfig),
    }
}
/// WasmService is configured as a built-in `envoy.wasm_service` :ref:`WasmService <config_wasm_service>` This opaque configuration will be used to create a Wasm Service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WasmService {
    /// General plugin configuration.
    #[prost(message, optional, tag = "1")]
    pub config: ::core::option::Option<PluginConfig>,
    /// If true, create a single VM rather than creating one VM per worker. Such a singleton can
    /// not be used with filters.
    #[prost(bool, tag = "2")]
    pub singleton: bool,
}
