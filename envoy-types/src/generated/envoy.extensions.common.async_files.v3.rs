/// Configuration to instantiate or select a singleton ``AsyncFileManager``.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncFileManagerConfig {
    /// An optional identifier for the manager. An empty string is a valid identifier
    /// for a common, default ``AsyncFileManager``.
    ///
    /// Reusing the same id with different configurations in the same envoy instance
    /// is an error.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(oneof = "async_file_manager_config::ManagerType", tags = "2")]
    pub manager_type: ::core::option::Option<async_file_manager_config::ManagerType>,
}
/// Nested message and enum types in `AsyncFileManagerConfig`.
pub mod async_file_manager_config {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThreadPool {
        /// The number of threads to use. If unset or zero, will default to the number
        /// of concurrent threads the hardware supports. This default is subject to
        /// change if performance analysis suggests it.
        #[prost(uint32, tag = "1")]
        pub thread_count: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ManagerType {
        /// Configuration for a thread-pool based async file manager.
        #[prost(message, tag = "2")]
        ThreadPool(ThreadPool),
    }
}
