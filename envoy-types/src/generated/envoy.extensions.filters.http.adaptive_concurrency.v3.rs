/// Configuration parameters for the gradient controller.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GradientControllerConfig {
    /// The percentile to use when summarizing aggregated samples. Defaults to p50.
    #[prost(message, optional, tag = "1")]
    pub sample_aggregate_percentile: ::core::option::Option<
        super::super::super::super::super::r#type::v3::Percent,
    >,
    #[prost(message, optional, tag = "2")]
    pub concurrency_limit_params: ::core::option::Option<
        gradient_controller_config::ConcurrencyLimitCalculationParams,
    >,
    #[prost(message, optional, tag = "3")]
    pub min_rtt_calc_params: ::core::option::Option<
        gradient_controller_config::MinimumRttCalculationParams,
    >,
}
/// Nested message and enum types in `GradientControllerConfig`.
pub mod gradient_controller_config {
    /// Parameters controlling the periodic recalculation of the concurrency limit from sampled request
    /// latencies.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConcurrencyLimitCalculationParams {
        /// The allowed upper-bound on the calculated concurrency limit. Defaults to 1000.
        #[prost(message, optional, tag = "2")]
        pub max_concurrency_limit: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// The period of time samples are taken to recalculate the concurrency limit.
        #[prost(message, optional, tag = "3")]
        pub concurrency_update_interval: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
    }
    /// Parameters controlling the periodic minRTT recalculation.
    /// \[\#next-free-field: 6\]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MinimumRttCalculationParams {
        /// The time interval between recalculating the minimum request round-trip time. Has to be
        /// positive.
        #[prost(message, optional, tag = "1")]
        pub interval: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::Duration,
        >,
        /// The number of requests to aggregate/sample during the minRTT recalculation window before
        /// updating. Defaults to 50.
        #[prost(message, optional, tag = "2")]
        pub request_count: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// Randomized time delta that will be introduced to the start of the minRTT calculation window.
        /// This is represented as a percentage of the interval duration. Defaults to 15%.
        ///
        /// Example: If the interval is 10s and the jitter is 15%, the next window will begin
        /// somewhere in the range (10s - 11.5s).
        #[prost(message, optional, tag = "3")]
        pub jitter: ::core::option::Option<
            super::super::super::super::super::super::r#type::v3::Percent,
        >,
        /// The concurrency limit set while measuring the minRTT. Defaults to 3.
        #[prost(message, optional, tag = "4")]
        pub min_concurrency: ::core::option::Option<
            super::super::super::super::super::super::super::google::protobuf::UInt32Value,
        >,
        /// Amount added to the measured minRTT to add stability to the concurrency limit during natural
        /// variability in latency. This is expressed as a percentage of the measured value and can be
        /// adjusted to allow more or less tolerance to the sampled latency values.
        ///
        /// Defaults to 25%.
        #[prost(message, optional, tag = "5")]
        pub buffer: ::core::option::Option<
            super::super::super::super::super::super::r#type::v3::Percent,
        >,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdaptiveConcurrency {
    /// If set to false, the adaptive concurrency filter will operate as a pass-through filter. If the
    /// message is unspecified, the filter will be enabled.
    #[prost(message, optional, tag = "2")]
    pub enabled: ::core::option::Option<
        super::super::super::super::super::config::core::v3::RuntimeFeatureFlag,
    >,
    /// This field allows for a custom HTTP response status code to the downstream client when
    /// the concurrency limit has been exceeded.
    /// Defaults to 503 (Service Unavailable).
    ///
    /// .. note::
    /// If this is set to \< 400, 503 will be used instead.
    #[prost(message, optional, tag = "3")]
    pub concurrency_limit_exceeded_status: ::core::option::Option<
        super::super::super::super::super::r#type::v3::HttpStatus,
    >,
    #[prost(oneof = "adaptive_concurrency::ConcurrencyControllerConfig", tags = "1")]
    pub concurrency_controller_config: ::core::option::Option<
        adaptive_concurrency::ConcurrencyControllerConfig,
    >,
}
/// Nested message and enum types in `AdaptiveConcurrency`.
pub mod adaptive_concurrency {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConcurrencyControllerConfig {
        /// Gradient concurrency control will be used.
        #[prost(message, tag = "1")]
        GradientControllerConfig(super::GradientControllerConfig),
    }
}
