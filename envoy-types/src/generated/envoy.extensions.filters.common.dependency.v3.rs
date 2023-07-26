/// Dependency specification and string identifier.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dependency {
    /// The kind of dependency.
    #[prost(enumeration = "dependency::DependencyType", tag = "1")]
    pub r#type: i32,
    /// The string identifier for the dependency.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Dependency`.
pub mod dependency {
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
    pub enum DependencyType {
        Header = 0,
        FilterStateKey = 1,
        DynamicMetadata = 2,
    }
    impl DependencyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DependencyType::Header => "HEADER",
                DependencyType::FilterStateKey => "FILTER_STATE_KEY",
                DependencyType::DynamicMetadata => "DYNAMIC_METADATA",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HEADER" => Some(Self::Header),
                "FILTER_STATE_KEY" => Some(Self::FilterStateKey),
                "DYNAMIC_METADATA" => Some(Self::DynamicMetadata),
                _ => None,
            }
        }
    }
}
/// Dependency specification for a filter. For a filter chain to be valid, any
/// dependency that is required must be provided by an earlier filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterDependencies {
    /// A list of dependencies required on the decode path.
    #[prost(message, repeated, tag = "1")]
    pub decode_required: ::prost::alloc::vec::Vec<Dependency>,
    /// A list of dependencies provided on the encode path.
    #[prost(message, repeated, tag = "2")]
    pub decode_provided: ::prost::alloc::vec::Vec<Dependency>,
    /// A list of dependencies required on the decode path.
    #[prost(message, repeated, tag = "3")]
    pub encode_required: ::prost::alloc::vec::Vec<Dependency>,
    /// A list of dependencies provided on the encode path.
    #[prost(message, repeated, tag = "4")]
    pub encode_provided: ::prost::alloc::vec::Vec<Dependency>,
}
/// Matching requirements for a filter. For a match tree to be used with a filter, the match
/// requirements must be satisfied.
///
/// This protobuf is provided by the filter implementation as a way to communicate the matching
/// requirements to the filter factories, allowing for config rejection if the requirements are
/// not satisfied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchingRequirements {
    #[prost(message, optional, tag = "1")]
    pub data_input_allow_list: ::core::option::Option<
        matching_requirements::DataInputAllowList,
    >,
}
/// Nested message and enum types in `MatchingRequirements`.
pub mod matching_requirements {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataInputAllowList {
        /// An explicit list of data inputs that are allowed to be used with this filter.
        #[prost(string, repeated, tag = "1")]
        pub type_url: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
