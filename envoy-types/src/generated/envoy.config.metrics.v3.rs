// This file is @generated by prost-build.
/// Configuration for pluggable stats sinks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatsSink {
    /// The name of the stats sink to instantiate. The name must match a supported
    /// stats sink.
    /// See the :ref:`extensions listed in typed_config below <extension_category_envoy.stats_sinks>` for the default list of available stats sink.
    /// Sinks optionally support tagged/multiple dimensional metrics.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    ///
    /// Stats sink specific configuration which depends on the sink being instantiated. See
    /// : ref:`StatsdSink <envoy_v3_api_msg_config.metrics.v3.StatsdSink>` for an example.
    /// \[\#extension-category: envoy.stats_sinks\]
    #[prost(oneof = "stats_sink::ConfigType", tags = "3")]
    pub config_type: ::core::option::Option<stats_sink::ConfigType>,
}
/// Nested message and enum types in `StatsSink`.
pub mod stats_sink {
    ///
    /// Stats sink specific configuration which depends on the sink being instantiated. See
    /// : ref:`StatsdSink <envoy_v3_api_msg_config.metrics.v3.StatsdSink>` for an example.
    /// \[\#extension-category: envoy.stats_sinks\]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag = "3")]
        TypedConfig(super::super::super::super::super::google::protobuf::Any),
    }
}
/// Statistics configuration such as tagging.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatsConfig {
    ///
    /// Each stat name is independently processed through these tag specifiers. When a tag is
    /// matched, the first capture group is not immediately removed from the name, so later
    /// : ref:`TagSpecifiers <envoy_v3_api_msg_config.metrics.v3.TagSpecifier>` can also match that
    /// same portion of the match. After all tag matching is complete, a tag-extracted version of
    /// the name is produced and is used in stats sinks that represent tags, such as Prometheus.
    #[prost(message, repeated, tag = "1")]
    pub stats_tags: ::prost::alloc::vec::Vec<TagSpecifier>,
    /// Use all default tag regexes specified in Envoy. These can be combined with
    /// custom tags specified in :ref:`stats_tags  <envoy_v3_api_field_config.metrics.v3.StatsConfig.stats_tags>`. They will be processed before
    /// the custom tags.
    ///
    /// .. note::
    ///
    /// If any default tags are specified twice, the config will be considered
    /// invalid.
    ///
    /// See :repo:`well_known_names.h <source/common/config/well_known_names.h>` for a list of the
    /// default tags in Envoy.
    ///
    /// If not provided, the value is assumed to be true.
    #[prost(message, optional, tag = "2")]
    pub use_all_default_tags: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    /// Inclusion/exclusion matcher for stat name creation. If not provided, all stats are instantiated
    /// as normal. Preventing the instantiation of certain families of stats can improve memory
    /// performance for Envoys running especially large configs.
    ///
    /// .. warning::
    /// Excluding stats may affect Envoy's behavior in undocumented ways. See
    /// `issue #8771 <<https://github.com/envoyproxy/envoy/issues/8771>`\_> for more information.
    /// If any unexpected behavior changes are observed, please open a new issue immediately.
    #[prost(message, optional, tag = "3")]
    pub stats_matcher: ::core::option::Option<StatsMatcher>,
    /// Defines rules for setting the histogram buckets. Rules are evaluated in order, and the first
    /// match is applied. If no match is found (or if no rules are set), the following default buckets
    /// are used:
    ///
    /// .. code-block:: json
    ///
    /// ```text
    /// [
    ///    0.5,
    ///    1,
    ///    5,
    ///    10,
    ///    25,
    ///    50,
    ///    100,
    ///    250,
    ///    500,
    ///    1000,
    ///    2500,
    ///    5000,
    ///    10000,
    ///    30000,
    ///    60000,
    ///    300000,
    ///    600000,
    ///    1800000,
    ///    3600000
    /// ]
    /// ```
    #[prost(message, repeated, tag = "4")]
    pub histogram_bucket_settings: ::prost::alloc::vec::Vec<HistogramBucketSettings>,
}
/// Configuration for disabling stat instantiation.
///
/// The instantiation of stats is unrestricted by default. If the goal is to configure Envoy to
/// instantiate all stats, there is no need to construct a StatsMatcher.
///
/// However, StatsMatcher can be used to limit the creation of families of stats in order to
/// conserve memory. Stats can either be disabled entirely, or they can be
/// limited by either an exclusion or an inclusion list of :ref:`StringMatcher  <envoy_v3_api_msg_type.matcher.v3.StringMatcher>` protos:
///
/// * If `reject_all` is set to `true`, no stats will be instantiated. If `reject_all` is set to
///   `false`, all stats will be instantiated.
///
/// * If an exclusion list is supplied, any stat name matching *any* of the StringMatchers in the
///   list will not instantiate.
///
/// * If an inclusion list is supplied, no stats will instantiate, except those matching *any* of
///   the StringMatchers in the list.
///
/// A StringMatcher can be used to match against an exact string, a suffix / prefix, or a regex.
/// **NB:** For performance reasons, it is highly recommended to use a prefix- or suffix-based
/// matcher rather than a regex-based matcher.
///
/// Example 1. Excluding all stats.
///
/// .. code-block:: json
///
/// {
/// "statsMatcher": {
/// "rejectAll": "true"
/// }
/// }
///
/// Example 2. Excluding all cluster-specific stats, but not cluster-manager stats:
///
/// .. code-block:: json
///
/// {
/// "statsMatcher": {
/// "exclusionList": {
/// "patterns": \[
/// {
/// "prefix": "cluster."
/// }
/// \]
/// }
/// }
/// }
///
/// Example 3. Including only manager-related stats:
///
/// .. code-block:: json
///
/// {
/// "statsMatcher": {
/// "inclusionList": {
/// "patterns": \[
/// {
/// "prefix": "cluster_manager."
/// },
/// {
/// "prefix": "listener_manager."
/// }
/// \]
/// }
/// }
/// }
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatsMatcher {
    #[prost(oneof = "stats_matcher::StatsMatcher", tags = "1, 2, 3")]
    pub stats_matcher: ::core::option::Option<stats_matcher::StatsMatcher>,
}
/// Nested message and enum types in `StatsMatcher`.
pub mod stats_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StatsMatcher {
        /// If `reject_all` is true, then all stats are disabled. If `reject_all` is false, then all
        /// stats are enabled.
        #[prost(bool, tag = "1")]
        RejectAll(bool),
        /// Exclusive match. All stats are enabled except for those matching one of the supplied
        /// StringMatcher protos.
        #[prost(message, tag = "2")]
        ExclusionList(
            super::super::super::super::r#type::matcher::v3::ListStringMatcher,
        ),
        /// Inclusive match. No stats are enabled except for those matching one of the supplied
        /// StringMatcher protos.
        #[prost(message, tag = "3")]
        InclusionList(
            super::super::super::super::r#type::matcher::v3::ListStringMatcher,
        ),
    }
}
/// Designates a tag name and value pair. The value may be either a fixed value
/// or a regex providing the value via capture groups. The specified tag will be
/// unconditionally set if a fixed value, otherwise it will only be set if one
/// or more capture groups in the regex match.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagSpecifier {
    ///
    /// Attaches an identifier to the tag values to identify the tag being in the
    /// sink. Envoy has a set of default names and regexes to extract dynamic
    /// portions of existing stats, which can be found in :repo:`well_known_names.h  <source/common/config/well_known_names.h>` in the Envoy repository. If a :ref:`tag_name  <envoy_v3_api_field_config.metrics.v3.TagSpecifier.tag_name>` is provided in the config and
    /// neither :ref:`regex <envoy_v3_api_field_config.metrics.v3.TagSpecifier.regex>` or
    /// : ref:`fixed_value <envoy_v3_api_field_config.metrics.v3.TagSpecifier.fixed_value>` were specified,
    /// Envoy will attempt to find that name in its set of defaults and use the accompanying regex.
    /// .. note::
    ///
    /// A stat name may be spelled in such a way that it matches two different
    /// tag extractors for the same tag name. In that case, all but one of the
    /// tag values will be dropped. It is not specified which tag value will be
    /// retained. The extraction will only occur for one of the extractors, and
    /// only the matched extraction will be removed from the tag name.
    #[prost(string, tag = "1")]
    pub tag_name: ::prost::alloc::string::String,
    #[prost(oneof = "tag_specifier::TagValue", tags = "2, 3")]
    pub tag_value: ::core::option::Option<tag_specifier::TagValue>,
}
/// Nested message and enum types in `TagSpecifier`.
pub mod tag_specifier {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TagValue {
        /// Designates a tag to strip from the tag extracted name and provide as a named
        /// tag value for all statistics. This will only occur if any part of the name
        /// matches the regex provided with one or more capture groups.
        ///
        /// The first capture group identifies the portion of the name to remove. The
        /// second capture group (which will normally be nested inside the first) will
        /// designate the value of the tag for the statistic. If no second capture
        /// group is provided, the first will also be used to set the value of the tag.
        /// All other capture groups will be ignored.
        ///
        /// Example 1. a stat name `cluster.foo_cluster.upstream_rq_timeout` and
        /// one tag specifier:
        ///
        /// .. code-block:: json
        ///
        /// {
        /// "tag_name": "envoy.cluster_name",
        /// "regex": "^cluster\\.((.+?)\\.)"
        /// }
        ///
        /// Note that the regex will remove `foo_cluster.` making the tag extracted
        /// name `cluster.upstream_rq_timeout` and the tag value for
        /// `envoy.cluster_name` will be `foo_cluster` (note: there will be no
        /// `.` character because of the second capture group).
        ///
        /// Example 2. a stat name
        /// `http.connection_manager_1.user_agent.ios.downstream_cx_total` and two
        /// tag specifiers:
        ///
        /// .. code-block:: json
        ///
        /// \[
        /// {
        /// "tag_name": "envoy.http_user_agent",
        /// "regex": "^http(?=\\.).*?\\.user_agent\\.((.+?)\\.)\\w+?$"
        /// },
        /// {
        /// "tag_name": "envoy.http_conn_manager_prefix",
        /// "regex": "^http\\.((.*?)\\.)"
        /// }
        /// \]
        ///
        /// The two regexes of the specifiers will be processed from the elaborated
        /// stat name.
        ///
        /// The first regex will save `ios.` as the tag value for `envoy.http_user_agent`. It will
        /// leave it in the name for potential matching with additional tag specifiers. After all tag
        /// specifiers are processed the tags will be removed from the name.
        ///
        /// The second regex will populate tag `envoy.http_conn_manager_prefix` with value
        /// `connection_manager_1.`, based on the original stat name.
        ///
        /// As a final step, the matched tags are removed, leaving
        /// `http.user_agent.downstream_cx_total` as the tag extracted name.
        #[prost(string, tag = "2")]
        Regex(::prost::alloc::string::String),
        /// Specifies a fixed tag value for the `tag_name`.
        #[prost(string, tag = "3")]
        FixedValue(::prost::alloc::string::String),
    }
}
/// Specifies a matcher for stats and the buckets that matching stats should use.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramBucketSettings {
    /// The stats that this rule applies to. The match is applied to the original stat name
    /// before tag-extraction, for example `cluster.exampleclustername.upstream_cx_length_ms`.
    #[prost(message, optional, tag = "1")]
    pub r#match: ::core::option::Option<
        super::super::super::r#type::matcher::v3::StringMatcher,
    >,
    /// Each value is the upper bound of a bucket. Each bucket must be greater than 0 and unique.
    /// The order of the buckets does not matter.
    #[prost(double, repeated, packed = "false", tag = "2")]
    pub buckets: ::prost::alloc::vec::Vec<f64>,
}
/// Stats configuration proto schema for built-in `envoy.stat_sinks.statsd` sink. This sink does not support
/// tagged metrics.
/// \[\#extension: envoy.stat_sinks.statsd\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatsdSink {
    /// Optional custom prefix for StatsdSink. If
    /// specified, this will override the default prefix.
    /// For example:
    ///
    /// .. code-block:: json
    ///
    /// {
    /// "prefix" : "envoy-prod"
    /// }
    ///
    /// will change emitted stats to
    ///
    /// .. code-block:: cpp
    ///
    /// envoy-prod.test_counter:1|c
    /// envoy-prod.test_timer:5|ms
    ///
    /// Note that the default prefix, "envoy", will be used if a prefix is not
    /// specified.
    ///
    /// Stats with default prefix:
    ///
    /// .. code-block:: cpp
    ///
    /// envoy.test_counter:1|c
    /// envoy.test_timer:5|ms
    #[prost(string, tag = "3")]
    pub prefix: ::prost::alloc::string::String,
    #[prost(oneof = "statsd_sink::StatsdSpecifier", tags = "1, 2")]
    pub statsd_specifier: ::core::option::Option<statsd_sink::StatsdSpecifier>,
}
/// Nested message and enum types in `StatsdSink`.
pub mod statsd_sink {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StatsdSpecifier {
        /// The UDP address of a running `statsd <<https://github.com/etsy/statsd>`\_>
        /// compliant listener. If specified, statistics will be flushed to this
        /// address.
        #[prost(message, tag = "1")]
        Address(super::super::super::core::v3::Address),
        /// The name of a cluster that is running a TCP `statsd  <<https://github.com/etsy/statsd>`\_> compliant listener. If specified,
        /// Envoy will connect to this cluster to flush statistics.
        #[prost(string, tag = "2")]
        TcpClusterName(::prost::alloc::string::String),
    }
}
/// Stats configuration proto schema for built-in `envoy.stat_sinks.dog_statsd` sink.
/// The sink emits stats with `DogStatsD <<https://docs.datadoghq.com/guides/dogstatsd/>`\_>
/// compatible tags. Tags are configurable via :ref:`StatsConfig  <envoy_v3_api_msg_config.metrics.v3.StatsConfig>`.
/// \[\#extension: envoy.stat_sinks.dog_statsd\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DogStatsdSink {
    /// Optional custom metric name prefix. See :ref:`StatsdSink's prefix field  <envoy_v3_api_field_config.metrics.v3.StatsdSink.prefix>` for more details.
    #[prost(string, tag = "3")]
    pub prefix: ::prost::alloc::string::String,
    /// Optional max datagram size to use when sending UDP messages. By default Envoy
    /// will emit one metric per datagram. By specifying a max-size larger than a single
    /// metric, Envoy will emit multiple, new-line separated metrics. The max datagram
    /// size should not exceed your network's MTU.
    ///
    /// Note that this value may not be respected if smaller than a single metric.
    #[prost(message, optional, tag = "4")]
    pub max_bytes_per_datagram: ::core::option::Option<
        super::super::super::super::google::protobuf::UInt64Value,
    >,
    #[prost(oneof = "dog_statsd_sink::DogStatsdSpecifier", tags = "1")]
    pub dog_statsd_specifier: ::core::option::Option<
        dog_statsd_sink::DogStatsdSpecifier,
    >,
}
/// Nested message and enum types in `DogStatsdSink`.
pub mod dog_statsd_sink {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DogStatsdSpecifier {
        /// The UDP address of a running DogStatsD compliant listener. If specified,
        /// statistics will be flushed to this address.
        #[prost(message, tag = "1")]
        Address(super::super::super::core::v3::Address),
    }
}
/// Stats configuration proto schema for built-in `envoy.stat_sinks.hystrix` sink.
/// The sink emits stats in `text/event-stream  <<https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events>`\_>
/// formatted stream for use by `Hystrix dashboard  <<https://github.com/Netflix-Skunkworks/hystrix-dashboard/wiki>`\_.>
///
/// Note that only a single HystrixSink should be configured.
///
/// Streaming is started through an admin endpoint :http:get:`/hystrix_event_stream`.
/// \[\#extension: envoy.stat_sinks.hystrix\]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HystrixSink {
    /// The number of buckets the rolling statistical window is divided into.
    ///
    /// Each time the sink is flushed, all relevant Envoy statistics are sampled and
    /// added to the rolling window (removing the oldest samples in the window
    /// in the process). The sink then outputs the aggregate statistics across the
    /// current rolling window to the event stream(s).
    ///
    /// `rolling_window(ms)` = `stats_flush_interval(ms)` * `num_of_buckets`
    ///
    /// More detailed explanation can be found in `Hystrix wiki  <<https://github.com/Netflix/Hystrix/wiki/Metrics-and-Monitoring#hystrixrollingnumber>`\_.>
    #[prost(int64, tag = "1")]
    pub num_buckets: i64,
}
/// Metrics Service is configured as a built-in `envoy.stat_sinks.metrics_service` :ref:`StatsSink  <envoy_v3_api_msg_config.metrics.v3.StatsSink>`. This opaque configuration will be used to create
/// Metrics Service.
///
/// Example:
///
/// .. code-block:: yaml
///
/// ```text
/// stats_sinks:
///    - name: envoy.stat_sinks.metrics_service
///      typed_config:
///        "@type": type.googleapis.com/envoy.config.metrics.v3.MetricsServiceConfig
/// ```
///
/// \[\#extension: envoy.stat_sinks.metrics_service\]
/// \[\#next-free-field: 6\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricsServiceConfig {
    /// The upstream gRPC cluster that hosts the metrics service.
    #[prost(message, optional, tag = "1")]
    pub grpc_service: ::core::option::Option<super::super::core::v3::GrpcService>,
    /// API version for metric service transport protocol. This describes the metric service gRPC
    /// endpoint and version of messages used on the wire.
    #[prost(enumeration = "super::super::core::v3::ApiVersion", tag = "3")]
    pub transport_api_version: i32,
    /// If true, counters are reported as the delta between flushing intervals. Otherwise, the current
    /// counter value is reported. Defaults to false.
    /// Eventually (<https://github.com/envoyproxy/envoy/issues/10968>) if this value is not set, the
    /// sink will take updates from the :ref:`MetricsResponse <envoy_v3_api_msg_service.metrics.v3.StreamMetricsResponse>`.
    #[prost(message, optional, tag = "2")]
    pub report_counters_as_deltas: ::core::option::Option<
        super::super::super::super::google::protobuf::BoolValue,
    >,
    /// If true, metrics will have their tags emitted as labels on the metrics objects sent to the MetricsService,
    /// and the tag extracted name will be used instead of the full name, which may contain values used by the tag
    /// extractor or additional tags added during stats creation.
    #[prost(bool, tag = "4")]
    pub emit_tags_as_labels: bool,
    /// Specify which metrics types to emit for histograms. Defaults to SUMMARY_AND_HISTOGRAM.
    #[prost(enumeration = "HistogramEmitMode", tag = "5")]
    pub histogram_emit_mode: i32,
}
/// HistogramEmitMode is used to configure which metric types should be emitted for histograms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HistogramEmitMode {
    /// Emit Histogram and Summary metric types.
    SummaryAndHistogram = 0,
    /// Emit only Summary metric types.
    Summary = 1,
    /// Emit only Histogram metric types.
    Histogram = 2,
}
impl HistogramEmitMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::SummaryAndHistogram => "SUMMARY_AND_HISTOGRAM",
            Self::Summary => "SUMMARY",
            Self::Histogram => "HISTOGRAM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUMMARY_AND_HISTOGRAM" => Some(Self::SummaryAndHistogram),
            "SUMMARY" => Some(Self::Summary),
            "HISTOGRAM" => Some(Self::Histogram),
            _ => None,
        }
    }
}
