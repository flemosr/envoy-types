/// [#next-free-field: 6]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MongoProxy {
    /// The human readable prefix to use when emitting :ref:`statistics
    /// <config_network_filters_mongo_proxy_stats>`.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// The optional path to use for writing Mongo access logs. If not access log
    /// path is specified no access logs will be written. Note that access log is
    /// also gated :ref:`runtime <config_network_filters_mongo_proxy_runtime>`.
    #[prost(string, tag = "2")]
    pub access_log: ::prost::alloc::string::String,
    /// Inject a fixed delay before proxying a Mongo operation. Delays are
    /// applied to the following MongoDB operations: Query, Insert, GetMore,
    /// and KillCursors. Once an active delay is in progress, all incoming
    /// data up until the timer event fires will be a part of the delay.
    #[prost(message, optional, tag = "3")]
    pub delay: ::core::option::Option<
        super::super::super::common::fault::v3::FaultDelay,
    >,
    /// Flag to specify whether :ref:`dynamic metadata
    /// <config_network_filters_mongo_proxy_dynamic_metadata>` should be emitted. Defaults to false.
    #[prost(bool, tag = "4")]
    pub emit_dynamic_metadata: bool,
    /// List of commands to emit metrics for. Defaults to "delete", "insert", and "update".
    /// Note that metrics will not be emitted for "find" commands, since those are considered
    /// queries, and metrics for those are emitted under a dedicated "query" namespace.
    #[prost(string, repeated, tag = "5")]
    pub commands: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
