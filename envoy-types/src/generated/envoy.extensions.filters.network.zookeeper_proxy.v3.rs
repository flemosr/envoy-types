/// [#next-free-field: 7]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZooKeeperProxy {
    /// The human readable prefix to use when emitting :ref:`statistics
    /// <config_network_filters_zookeeper_proxy_stats>`.
    #[prost(string, tag = "1")]
    pub stat_prefix: ::prost::alloc::string::String,
    /// \[#not-implemented-hide:\] The optional path to use for writing ZooKeeper access logs.
    /// If the access log field is empty, access logs will not be written.
    #[prost(string, tag = "2")]
    pub access_log: ::prost::alloc::string::String,
    /// Messages — requests, responses and events — that are bigger than this value will
    /// be ignored. If it is not set, the default value is 1Mb.
    ///
    /// The value here should match the jute.maxbuffer property in your cluster configuration:
    ///
    /// <https://zookeeper.apache.org/doc/r3.4.10/zookeeperAdmin.html#Unsafe+Options>
    ///
    /// if that is set. If it isn't, ZooKeeper's default is also 1Mb.
    #[prost(message, optional, tag = "3")]
    pub max_packet_bytes: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::UInt32Value,
    >,
    /// Whether to emit latency threshold metrics. If not set, defaults to false.
    /// If false, setting `default_latency_threshold` and `latency_threshold_overrides` will not have effect.
    #[prost(bool, tag = "4")]
    pub enable_latency_threshold_metrics: bool,
    /// The default latency threshold to decide the fast/slow responses and emit metrics (used for error budget calculation).
    ///
    /// <https://sre.google/workbook/implementing-slos/>
    ///
    /// If it is not set, the default value is 100 milliseconds.
    #[prost(message, optional, tag = "5")]
    pub default_latency_threshold: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
    /// List of latency threshold overrides for opcodes.
    /// If the threshold override of one opcode is not set, it will fallback to the default latency
    /// threshold.
    /// Specifying latency threshold overrides multiple times for one opcode is not allowed.
    #[prost(message, repeated, tag = "6")]
    pub latency_threshold_overrides: ::prost::alloc::vec::Vec<LatencyThresholdOverride>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatencyThresholdOverride {
    /// The ZooKeeper opcodes. Can be found as part of the ZooKeeper source code:
    ///
    /// <https://github.com/apache/zookeeper/blob/master/zookeeper-server/src/main/java/org/apache/zookeeper/ZooDefs.java>
    ///
    #[prost(enumeration = "latency_threshold_override::Opcode", tag = "1")]
    pub opcode: i32,
    /// The latency threshold override of certain opcode.
    #[prost(message, optional, tag = "2")]
    pub threshold: ::core::option::Option<
        super::super::super::super::super::super::google::protobuf::Duration,
    >,
}
/// Nested message and enum types in `LatencyThresholdOverride`.
pub mod latency_threshold_override {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Opcode {
        Connect = 0,
        Create = 1,
        Delete = 2,
        Exists = 3,
        GetData = 4,
        SetData = 5,
        GetAcl = 6,
        SetAcl = 7,
        GetChildren = 8,
        Sync = 9,
        Ping = 10,
        GetChildren2 = 11,
        Check = 12,
        Multi = 13,
        Create2 = 14,
        Reconfig = 15,
        CheckWatches = 16,
        RemoveWatches = 17,
        CreateContainer = 18,
        CreateTtl = 19,
        Close = 20,
        SetAuth = 21,
        SetWatches = 22,
        GetEphemerals = 23,
        GetAllChildrenNumber = 24,
        SetWatches2 = 25,
    }
    impl Opcode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Opcode::Connect => "Connect",
                Opcode::Create => "Create",
                Opcode::Delete => "Delete",
                Opcode::Exists => "Exists",
                Opcode::GetData => "GetData",
                Opcode::SetData => "SetData",
                Opcode::GetAcl => "GetAcl",
                Opcode::SetAcl => "SetAcl",
                Opcode::GetChildren => "GetChildren",
                Opcode::Sync => "Sync",
                Opcode::Ping => "Ping",
                Opcode::GetChildren2 => "GetChildren2",
                Opcode::Check => "Check",
                Opcode::Multi => "Multi",
                Opcode::Create2 => "Create2",
                Opcode::Reconfig => "Reconfig",
                Opcode::CheckWatches => "CheckWatches",
                Opcode::RemoveWatches => "RemoveWatches",
                Opcode::CreateContainer => "CreateContainer",
                Opcode::CreateTtl => "CreateTtl",
                Opcode::Close => "Close",
                Opcode::SetAuth => "SetAuth",
                Opcode::SetWatches => "SetWatches",
                Opcode::GetEphemerals => "GetEphemerals",
                Opcode::GetAllChildrenNumber => "GetAllChildrenNumber",
                Opcode::SetWatches2 => "SetWatches2",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Connect" => Some(Self::Connect),
                "Create" => Some(Self::Create),
                "Delete" => Some(Self::Delete),
                "Exists" => Some(Self::Exists),
                "GetData" => Some(Self::GetData),
                "SetData" => Some(Self::SetData),
                "GetAcl" => Some(Self::GetAcl),
                "SetAcl" => Some(Self::SetAcl),
                "GetChildren" => Some(Self::GetChildren),
                "Sync" => Some(Self::Sync),
                "Ping" => Some(Self::Ping),
                "GetChildren2" => Some(Self::GetChildren2),
                "Check" => Some(Self::Check),
                "Multi" => Some(Self::Multi),
                "Create2" => Some(Self::Create2),
                "Reconfig" => Some(Self::Reconfig),
                "CheckWatches" => Some(Self::CheckWatches),
                "RemoveWatches" => Some(Self::RemoveWatches),
                "CreateContainer" => Some(Self::CreateContainer),
                "CreateTtl" => Some(Self::CreateTtl),
                "Close" => Some(Self::Close),
                "SetAuth" => Some(Self::SetAuth),
                "SetWatches" => Some(Self::SetWatches),
                "GetEphemerals" => Some(Self::GetEphemerals),
                "GetAllChildrenNumber" => Some(Self::GetAllChildrenNumber),
                "SetWatches2" => Some(Self::SetWatches2),
                _ => None,
            }
        }
    }
}
