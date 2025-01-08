// This file is @generated by prost-build.
/// Configuration of a dynamic module. A dynamic module is a shared object file that can be loaded via dlopen
/// by various Envoy extension points. Currently, only HTTP filter (envoy.filters.http.dynamic_modules) is supported.
///
/// How a module is loaded is determined by the extension point that uses it. For example, the HTTP filter
/// loads the module with dlopen when Envoy receives a configuration that references the module at load time.
/// If loading the module fails, the configuration will be rejected.
///
/// Whether or not the shared object is the same is determined by the file path as well as the file's inode depending
/// on the platform. Notably, if the file path and the content of the file are the same, the shared object will be reused.
///
/// A module must be compatible with the ABI specified in :repo:`abi.h <source/extensions/dynamic_modules/abi.h>`.
/// Currently, compatibility is only guaranteed by an exact version match between the Envoy
/// codebase and the dynamic module SDKs. In the future, after the ABI is stabilized, we will revisit
/// this restriction and hopefully provide a wider compatibility guarantee. Until then, Envoy
/// checks the hash of the ABI header files to ensure that the dynamic modules are built against the
/// same version of the ABI.
///
/// Currently, the implementation is work in progress and not usable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicModuleConfig {
    /// The name of the dynamic module. The client is expected to have some configuration indicating where to search for the module.
    /// In Envoy, the search path can only be configured via the environment variable `ENVOY_DYNAMIC_MODULES_SEARCH_PATH`.
    /// The actual search path is `${ENVOY_DYNAMIC_MODULES_SEARCH_PATH}/lib${name}.so`. TODO: make the search path configurable via
    /// command line options.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Set true to prevent the module from being unloaded with dlclose.
    /// This is useful for modules that have global state that should not be unloaded.
    /// A module is closed when no more references to it exist in the process. For example,
    /// no HTTP filters are using the module (e.g. after configuration update).
    #[prost(bool, tag = "3")]
    pub do_not_close: bool,
}