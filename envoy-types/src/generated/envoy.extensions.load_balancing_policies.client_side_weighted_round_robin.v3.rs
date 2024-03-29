/// Configuration for the client_side_weighted_round_robin LB policy.
///
/// This policy differs from the built-in ROUND_ROBIN policy in terms of
/// how the endpoint weights are determined. In the ROUND_ROBIN policy,
/// the endpoint weights are sent by the control plane via EDS. However,
/// in this policy, the endpoint weights are instead determined via qps (queries
/// per second), eps (errors per second), and utilization metrics sent by the
/// endpoint using the Open Request Cost Aggregation (ORCA) protocol. Utilization
/// is determined by using the ORCA application_utilization field, if set, or
/// else falling back to the cpu_utilization field. All queries count toward qps,
/// regardless of result. Only failed queries count toward eps. A config
/// parameter error_utilization_penalty controls the penalty to adjust endpoint
/// weights using eps and qps. The weight of a given endpoint is computed as:
/// qps / (utilization + eps/qps * error_utilization_penalty)
///
/// See the :ref:`load balancing architecture overview<arch_overview_load_balancing_types>` for more information.
///
/// \[\#next-free-field: 7\]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientSideWeightedRoundRobin {
    /// Whether to enable out-of-band utilization reporting collection from
    /// the endpoints. By default, per-request utilization reporting is used.
    #[prost(message, optional, tag = "1")]
    pub enable_oob_load_report: ::core::option::Option<
        super::super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Load reporting interval to request from the server. Note that the
    /// server may not provide reports as frequently as the client requests.
    /// Used only when enable_oob_load_report is true. Default is 10 seconds.
    #[prost(message, optional, tag = "2")]
    pub oob_reporting_period: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// A given endpoint must report load metrics continuously for at least
    /// this long before the endpoint weight will be used. This avoids
    /// churn when the set of endpoint addresses changes. Takes effect
    /// both immediately after we establish a connection to an endpoint and
    /// after weight_expiration_period has caused us to stop using the most
    /// recent load metrics. Default is 10 seconds.
    #[prost(message, optional, tag = "3")]
    pub blackout_period: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// If a given endpoint has not reported load metrics in this long,
    /// then we stop using the reported weight. This ensures that we do
    /// not continue to use very stale weights. Once we stop using a stale
    /// value, if we later start seeing fresh reports again, the
    /// blackout_period applies. Defaults to 3 minutes.
    #[prost(message, optional, tag = "4")]
    pub weight_expiration_period: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// How often endpoint weights are recalculated. Values less than 100ms are
    /// capped at 100ms. Default is 1 second.
    #[prost(message, optional, tag = "5")]
    pub weight_update_period: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Duration,
    >,
    /// The multiplier used to adjust endpoint weights with the error rate
    /// calculated as eps/qps. Configuration is rejected if this value is negative.
    /// Default is 1.0.
    #[prost(message, optional, tag = "6")]
    pub error_utilization_penalty: ::core::option::Option<
        super::super::super::super::super::google::protobuf::FloatValue,
    >,
}
