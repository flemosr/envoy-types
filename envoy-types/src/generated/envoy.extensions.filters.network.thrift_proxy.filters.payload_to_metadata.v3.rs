#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadToMetadata {
    /// The list of rules to apply to requests.
    #[prost(message, repeated, tag = "1")]
    pub request_rules: ::prost::alloc::vec::Vec<payload_to_metadata::Rule>,
}
/// Nested message and enum types in `PayloadToMetadata`.
pub mod payload_to_metadata {
    /// [#next-free-field: 6]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyValuePair {
        /// The namespace — if this is empty, the filter's namespace will be used.
        #[prost(string, tag = "1")]
        pub metadata_namespace: ::prost::alloc::string::String,
        /// The key to use within the namespace.
        #[prost(string, tag = "2")]
        pub key: ::prost::alloc::string::String,
        /// The value's type — defaults to string.
        #[prost(enumeration = "ValueType", tag = "5")]
        pub r#type: i32,
        #[prost(oneof = "key_value_pair::ValueType", tags = "3, 4")]
        pub value_type: ::core::option::Option<key_value_pair::ValueType>,
    }
    /// Nested message and enum types in `KeyValuePair`.
    pub mod key_value_pair {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ValueType {
            /// The value to pair with the given key.
            ///
            /// When used for on_present case, if value is non-empty it'll be used instead
            /// of the field value. If both are empty, the field value is used as-is.
            ///
            /// When used for on_missing case, a non-empty value must be provided.
            #[prost(string, tag = "3")]
            Value(::prost::alloc::string::String),
            /// If present, the header's value will be matched and substituted with this.
            /// If there is no match or substitution, the field value is used as-is.
            ///
            /// This is only used for on_present.
            #[prost(message, tag = "4")]
            RegexValueRewrite(
                super::super::super::super::super::super::super::super::super::r#type::matcher::v3::RegexMatchAndSubstitute,
            ),
        }
    }
    /// A Rule defines what metadata to apply when a field is present or missing.
    /// [#next-free-field: 6]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rule {
        /// Specifies that a match will be performed on the value of a field.
        #[prost(message, optional, tag = "3")]
        pub field_selector: ::core::option::Option<FieldSelector>,
        /// If the field is present, apply this metadata KeyValuePair.
        #[prost(message, optional, tag = "4")]
        pub on_present: ::core::option::Option<KeyValuePair>,
        /// If the field is missing, apply this metadata KeyValuePair.
        ///
        /// The value in the KeyValuePair must be set, since it'll be used in lieu
        /// of the missing field value.
        #[prost(message, optional, tag = "5")]
        pub on_missing: ::core::option::Option<KeyValuePair>,
        #[prost(oneof = "rule::MatchSpecifier", tags = "1, 2")]
        pub match_specifier: ::core::option::Option<rule::MatchSpecifier>,
    }
    /// Nested message and enum types in `Rule`.
    pub mod rule {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum MatchSpecifier {
            /// If specified, the route must exactly match the request method name. As a special case,
            /// an empty string matches any request method name.
            #[prost(string, tag = "1")]
            MethodName(::prost::alloc::string::String),
            /// If specified, the route must have the service name as the request method name prefix.
            /// As a special case, an empty string matches any service name. Only relevant when service
            /// multiplexing.
            #[prost(string, tag = "2")]
            ServiceName(::prost::alloc::string::String),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldSelector {
        /// field name to log
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// field id to match
        #[prost(int32, tag = "2")]
        pub id: i32,
        /// next node of the field selector
        #[prost(message, optional, boxed, tag = "3")]
        pub child: ::core::option::Option<::prost::alloc::boxed::Box<FieldSelector>>,
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
    pub enum ValueType {
        String = 0,
        Number = 1,
    }
    impl ValueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueType::String => "STRING",
                ValueType::Number => "NUMBER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STRING" => Some(Self::String),
                "NUMBER" => Some(Self::Number),
                _ => None,
            }
        }
    }
}
