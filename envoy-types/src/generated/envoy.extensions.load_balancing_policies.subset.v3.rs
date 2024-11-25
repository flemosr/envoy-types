// This file is @generated by prost-build.
/// Optionally divide the endpoints in this cluster into subsets defined by
/// endpoint metadata and selected by route and weighted cluster metadata.
/// \[\#next-free-field: 11\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subset {
    ///
    /// The behavior used when no endpoint subset matches the selected route's
    /// metadata. The value defaults to
    /// : ref:`NO_FALLBACK<envoy_v3_api_enum_value_extensions.load_balancing_policies.subset.v3.Subset.LbSubsetFallbackPolicy.NO_FALLBACK>`.
    #[prost(enumeration = "subset::LbSubsetFallbackPolicy", tag = "1")]
    pub fallback_policy: i32,
    ///
    /// Specifies the default subset of endpoints used during fallback if
    /// fallback_policy is
    /// : ref:`DEFAULT_SUBSET<envoy_v3_api_enum_value_extensions.load_balancing_policies.subset.v3.Subset.LbSubsetFallbackPolicy.DEFAULT_SUBSET>`.
    /// Each field in default_subset is
    /// compared to the matching LbEndpoint.Metadata under the `envoy.lb`
    /// namespace. It is valid for no hosts to match, in which case the behavior
    /// is the same as a fallback_policy of
    /// : ref:`NO_FALLBACK<envoy_v3_api_enum_value_extensions.load_balancing_policies.subset.v3.Subset.LbSubsetFallbackPolicy.NO_FALLBACK>`.
    #[prost(message, optional, tag = "2")]
    pub default_subset: ::core::option::Option<
        super::super::super::super::super::google::protobuf::Struct,
    >,
    /// For each entry, LbEndpoint.Metadata's
    /// `envoy.lb` namespace is traversed and a subset is created for each unique
    /// combination of key and value. For example:
    ///
    /// .. code-block:: json
    ///
    /// { "subset_selectors": \[
    /// { "keys": \[ "version" \] },
    /// { "keys": \[ "stage", "hardware_type" \] }
    /// \]}
    ///
    /// A subset is matched when the metadata from the selected route and
    /// weighted cluster contains the same keys and values as the subset's
    /// metadata. The same host may appear in multiple subsets.
    #[prost(message, repeated, tag = "3")]
    pub subset_selectors: ::prost::alloc::vec::Vec<subset::LbSubsetSelector>,
    /// By default, only when the request metadata has exactly the **same** keys as one of subset selectors and
    /// the values of the related keys are matched, the load balancer will have a valid subset for the request.
    /// For example, given the following subset selectors:
    ///
    /// .. code-block:: json
    ///
    /// { "subset_selectors": \[
    /// { "keys": \[ "version" \] },
    /// { "keys": \[ "stage", "version" \] }
    /// \]}
    ///
    /// A request with metadata `{"redundant-key": "redundant-value", "stage": "prod", "version": "v1"}` or
    /// `{"redundant-key": "redundant-value", "version": "v1"}` will not have a valid subset even if the values
    /// of keys `stage` and `version` are matched because of the redundant key/value pair in the request
    /// metadata.
    ///
    /// By setting this field to true, the most appropriate keys will be filtered out from the request metadata
    /// according to the subset selectors. And then the filtered keys and related values will be matched to find
    /// the valid host subset. By this way, redundant key/value pairs are allowed in the request metadata. The keys
    /// of a request metadata could be superset of the keys of the subset selectors and need not to be exactly the
    /// same as the keys of the subset selectors.
    ///
    /// More specifically, if the keys of a request metadata is a superset of one of the subset selectors, then only
    /// the values of the keys that in the selector keys will be matched. Take the above example, if the request
    /// metadata is `{"redundant-key": "redundant-value", "stage": "prod", "version": "v1"}`, the load balancer
    /// will only match the values of `stage` and `version` to find an appropriate subset because `stage`
    /// `version` are contained by the second subset selector and the redundant `redundant-key` will be
    /// ignored.
    ///
    /// .. note::
    /// If the keys of request metadata is superset of multiple different subset selectors keys, the subset
    /// selector with most keys to win. For example, given subset selectors
    /// `{"subset_selectors": \["keys": ["A", "B", "C"\], \["A", "B"]\]}` and request metadata `{"A": "-",    "B": "-", "C": "-", "D": "-"}`, keys `A`, `B`, `C` will be evaluated.
    /// If the keys of request metadata is superset of multiple different subset selectors keys and the number
    /// of selector keys are same, then the one placed in front to win. For example, given subset selectors
    /// `{"subset_selectors": \["keys": ["A", "B"\], \["C", "D"]\]}` and request metadata `{"A": "-", "B": "-",    "C": "-", "D": "-"}`, keys `A`, `B` will be evaluated.
    #[prost(bool, tag = "10")]
    pub allow_redundant_keys: bool,
    /// If true, routing to subsets will take into account the localities and locality weights of the
    /// endpoints when making the routing decision.
    ///
    /// There are some potential pitfalls associated with enabling this feature, as the resulting
    /// traffic split after applying both a subset match and locality weights might be undesirable.
    ///
    /// Consider for example a situation in which you have 50/50 split across two localities X/Y
    /// which have 100 hosts each without subsetting. If the subset LB results in X having only 1
    /// host selected but Y having 100, then a lot more load is being dumped on the single host in X
    /// than originally anticipated in the load balancing assignment delivered via EDS.
    #[prost(bool, tag = "4")]
    pub locality_weight_aware: bool,
    /// When used with locality_weight_aware, scales the weight of each locality by the ratio
    /// of hosts in the subset vs hosts in the original subset. This aims to even out the load
    /// going to an individual locality if said locality is disproportionately affected by the
    /// subset predicate.
    #[prost(bool, tag = "5")]
    pub scale_locality_weight: bool,
    /// If true, when a fallback policy is configured and its corresponding subset fails to find
    /// a host this will cause any host to be selected instead.
    ///
    /// This is useful when using the default subset as the fallback policy, given the default
    /// subset might become empty. With this option enabled, if that happens the LB will attempt
    /// to select a host from the entire cluster.
    #[prost(bool, tag = "6")]
    pub panic_mode_any: bool,
    /// If true, metadata specified for a metadata key will be matched against the corresponding
    /// endpoint metadata if the endpoint metadata matches the value exactly OR it is a list value
    /// and any of the elements in the list matches the criteria.
    #[prost(bool, tag = "7")]
    pub list_as_any: bool,
    ///
    /// Fallback mechanism that allows to try different route metadata until a host is found.
    /// If load balancing process, including all its mechanisms (like
    /// : ref:`fallback_policy<envoy_v3_api_field_extensions.load_balancing_policies.subset.v3.subset.fallback_policy>`)
    /// fails to select a host, this policy decides if and how the process is repeated using another metadata.
    ///
    /// The value defaults to
    /// : ref:`METADATA_NO_FALLBACK  <envoy_v3_api_enum_value_extensions.load_balancing_policies.subset.v3.subset.LbSubsetMetadataFallbackPolicy.METADATA_NO_FALLBACK>`.
    #[prost(enumeration = "subset::LbSubsetMetadataFallbackPolicy", tag = "8")]
    pub metadata_fallback_policy: i32,
    /// The child LB policy to create for endpoint-picking within the chosen subset.
    #[prost(message, optional, tag = "9")]
    pub subset_lb_policy: ::core::option::Option<
        super::super::super::super::config::cluster::v3::LoadBalancingPolicy,
    >,
}
/// Nested message and enum types in `Subset`.
pub mod subset {
    /// Specifications for subsets.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LbSubsetSelector {
        /// List of keys to match with the weighted cluster metadata.
        #[prost(string, repeated, tag = "1")]
        pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Selects a mode of operation in which each subset has only one host. This mode uses the same rules for
        /// choosing a host, but updating hosts is faster, especially for large numbers of hosts.
        ///
        /// If a match is found to a host, that host will be used regardless of priority levels.
        ///
        ///
        /// When this mode is enabled, configurations that contain more than one host with the same metadata value for the single key in `keys`
        /// will use only one of the hosts with the given key; no requests will be routed to the others. The cluster gauge
        /// : ref:`lb_subsets_single_host_per_subset_duplicate<config_cluster_manager_cluster_stats_subset_lb>` indicates how many duplicates are
        /// present in the current configuration.
        #[prost(bool, tag = "4")]
        pub single_host_per_subset: bool,
        /// The behavior used when no endpoint subset matches the selected route's
        /// metadata.
        #[prost(
            enumeration = "lb_subset_selector::LbSubsetSelectorFallbackPolicy",
            tag = "2"
        )]
        pub fallback_policy: i32,
        ///
        /// Subset of
        /// : ref:`keys<envoy_v3_api_field_extensions.load_balancing_policies.subset.v3.Subset.LbSubsetSelector.keys>` used by
        /// : ref:`KEYS_SUBSET<envoy_v3_api_enum_value_extensions.load_balancing_policies.subset.v3.Subset.LbSubsetSelector.LbSubsetSelectorFallbackPolicy.KEYS_SUBSET>`
        /// fallback policy.
        /// It has to be a non empty list if KEYS_SUBSET fallback policy is selected.
        /// For any other fallback policy the parameter is not used and should not be set.
        /// Only values also present in
        /// : ref:`keys<envoy_v3_api_field_extensions.load_balancing_policies.subset.v3.Subset.LbSubsetSelector.keys>` are allowed, but
        /// `fallback_keys_subset` cannot be equal to `keys`.
        #[prost(string, repeated, tag = "3")]
        pub fallback_keys_subset: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// Nested message and enum types in `LbSubsetSelector`.
    pub mod lb_subset_selector {
        /// Allows to override top level fallback policy per selector.
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
        pub enum LbSubsetSelectorFallbackPolicy {
            /// If NOT_DEFINED top level config fallback policy is used instead.
            NotDefined = 0,
            /// If NO_FALLBACK is selected, a result equivalent to no healthy hosts is reported.
            NoFallback = 1,
            /// If ANY_ENDPOINT is selected, any cluster endpoint may be returned
            /// (subject to policy, health checks, etc).
            AnyEndpoint = 2,
            /// If DEFAULT_SUBSET is selected, load balancing is performed over the
            /// endpoints matching the values from the default_subset field.
            DefaultSubset = 3,
            ///
            /// If KEYS_SUBSET is selected, subset selector matching is performed again with metadata
            /// keys reduced to
            /// : ref:`fallback_keys_subset<envoy_v3_api_field_extensions.load_balancing_policies.subset.v3.Subset.LbSubsetSelector.fallback_keys_subset>`.
            /// It allows for a fallback to a different, less specific selector if some of the keys of
            /// the selector are considered optional.
            KeysSubset = 4,
        }
        impl LbSubsetSelectorFallbackPolicy {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::NotDefined => "NOT_DEFINED",
                    Self::NoFallback => "NO_FALLBACK",
                    Self::AnyEndpoint => "ANY_ENDPOINT",
                    Self::DefaultSubset => "DEFAULT_SUBSET",
                    Self::KeysSubset => "KEYS_SUBSET",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NOT_DEFINED" => Some(Self::NotDefined),
                    "NO_FALLBACK" => Some(Self::NoFallback),
                    "ANY_ENDPOINT" => Some(Self::AnyEndpoint),
                    "DEFAULT_SUBSET" => Some(Self::DefaultSubset),
                    "KEYS_SUBSET" => Some(Self::KeysSubset),
                    _ => None,
                }
            }
        }
    }
    /// If NO_FALLBACK is selected, a result
    /// equivalent to no healthy hosts is reported. If ANY_ENDPOINT is selected,
    /// any cluster endpoint may be returned (subject to policy, health checks,
    /// etc). If DEFAULT_SUBSET is selected, load balancing is performed over the
    /// endpoints matching the values from the default_subset field.
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
    pub enum LbSubsetFallbackPolicy {
        NoFallback = 0,
        AnyEndpoint = 1,
        DefaultSubset = 2,
    }
    impl LbSubsetFallbackPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::NoFallback => "NO_FALLBACK",
                Self::AnyEndpoint => "ANY_ENDPOINT",
                Self::DefaultSubset => "DEFAULT_SUBSET",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NO_FALLBACK" => Some(Self::NoFallback),
                "ANY_ENDPOINT" => Some(Self::AnyEndpoint),
                "DEFAULT_SUBSET" => Some(Self::DefaultSubset),
                _ => None,
            }
        }
    }
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
    pub enum LbSubsetMetadataFallbackPolicy {
        /// No fallback. Route metadata will be used as-is.
        MetadataNoFallback = 0,
        /// A special metadata key `fallback_list` will be used to provide variants of metadata to try.
        /// Value of `fallback_list` key has to be a list. Every list element has to be a struct - it will
        /// be merged with route metadata, overriding keys that appear in both places.
        /// `fallback_list` entries will be used in order until a host is found.
        ///
        /// `fallback_list` key itself is removed from metadata before subset load balancing is performed.
        ///
        /// Example:
        ///
        /// for metadata:
        ///
        /// .. code-block:: yaml
        ///
        /// version: 1.0
        /// fallback_list:
        /// - version: 2.0
        /// hardware: c64
        /// - hardware: c32
        /// - version: 3.0
        ///
        /// at first, metadata:
        ///
        /// .. code-block:: json
        ///
        /// {"version": "2.0", "hardware": "c64"}
        ///
        /// will be used for load balancing. If no host is found, metadata:
        ///
        /// .. code-block:: json
        ///
        /// {"version": "1.0", "hardware": "c32"}
        ///
        /// is next to try. If it still results in no host, finally metadata:
        ///
        /// .. code-block:: json
        ///
        /// {"version": "3.0"}
        ///
        /// is used.
        FallbackList = 1,
    }
    impl LbSubsetMetadataFallbackPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::MetadataNoFallback => "METADATA_NO_FALLBACK",
                Self::FallbackList => "FALLBACK_LIST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "METADATA_NO_FALLBACK" => Some(Self::MetadataNoFallback),
                "FALLBACK_LIST" => Some(Self::FallbackList),
                _ => None,
            }
        }
    }
}
