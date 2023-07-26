/// A matcher, which may traverse a matching tree in order to result in a match action.
/// During matching, the tree will be traversed until a match is found, or if no match
/// is found the action specified by the most specific on_no_match will be evaluated.
/// As an on_no_match might result in another matching tree being evaluated, this process
/// might repeat several times until the final OnMatch (or no match) is decided.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Matcher {
    /// Optional OnMatch to use if the matcher failed.
    /// If specified, the OnMatch is used, and the matcher is considered
    /// to have matched.
    /// If not specified, the matcher is considered not to have matched.
    #[prost(message, optional, boxed, tag = "3")]
    pub on_no_match: ::core::option::Option<
        ::prost::alloc::boxed::Box<matcher::OnMatch>,
    >,
    #[prost(oneof = "matcher::MatcherType", tags = "1, 2")]
    pub matcher_type: ::core::option::Option<matcher::MatcherType>,
}
/// Nested message and enum types in `Matcher`.
pub mod matcher {
    /// What to do if a match is successful.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OnMatch {
        #[prost(oneof = "on_match::OnMatch", tags = "1, 2")]
        pub on_match: ::core::option::Option<on_match::OnMatch>,
    }
    /// Nested message and enum types in `OnMatch`.
    pub mod on_match {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OnMatch {
            /// Nested matcher to evaluate.
            /// If the nested matcher does not match and does not specify
            /// on_no_match, then this matcher is considered not to have
            /// matched, even if a predicate at this level or above returned
            /// true.
            #[prost(message, tag = "1")]
            Matcher(::prost::alloc::boxed::Box<super::super::Matcher>),
            /// Protocol-specific action to take.
            #[prost(message, tag = "2")]
            Action(super::super::super::super::super::core::v3::TypedExtensionConfig),
        }
    }
    /// A linear list of field matchers.
    /// The field matchers are evaluated in order, and the first match
    /// wins.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatcherList {
        /// A list of matchers. First match wins.
        #[prost(message, repeated, tag = "1")]
        pub matchers: ::prost::alloc::vec::Vec<matcher_list::FieldMatcher>,
    }
    /// Nested message and enum types in `MatcherList`.
    pub mod matcher_list {
        /// Predicate to determine if a match is successful.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Predicate {
            #[prost(oneof = "predicate::MatchType", tags = "1, 2, 3, 4")]
            pub match_type: ::core::option::Option<predicate::MatchType>,
        }
        /// Nested message and enum types in `Predicate`.
        pub mod predicate {
            /// Predicate for a single input field.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct SinglePredicate {
                /// Protocol-specific specification of input field to match on.
                /// [#extension-category: envoy.matching.common_inputs]
                #[prost(message, optional, tag = "1")]
                pub input: ::core::option::Option<
                    super::super::super::super::super::super::core::v3::TypedExtensionConfig,
                >,
                #[prost(oneof = "single_predicate::Matcher", tags = "2, 3")]
                pub matcher: ::core::option::Option<single_predicate::Matcher>,
            }
            /// Nested message and enum types in `SinglePredicate`.
            pub mod single_predicate {
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Matcher {
                    /// Built-in string matcher.
                    #[prost(message, tag = "2")]
                    ValueMatch(
                        super::super::super::super::super::super::super::super::r#type::matcher::v3::StringMatcher,
                    ),
                    /// Extension for custom matching logic.
                    /// [#extension-category: envoy.matching.input_matchers]
                    #[prost(message, tag = "3")]
                    CustomMatch(
                        super::super::super::super::super::super::super::core::v3::TypedExtensionConfig,
                    ),
                }
            }
            /// A list of two or more matchers. Used to allow using a list within a oneof.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct PredicateList {
                #[prost(message, repeated, tag = "1")]
                pub predicate: ::prost::alloc::vec::Vec<super::Predicate>,
            }
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum MatchType {
                /// A single predicate to evaluate.
                #[prost(message, tag = "1")]
                SinglePredicate(SinglePredicate),
                /// A list of predicates to be OR-ed together.
                #[prost(message, tag = "2")]
                OrMatcher(PredicateList),
                /// A list of predicates to be AND-ed together.
                #[prost(message, tag = "3")]
                AndMatcher(PredicateList),
                /// The invert of a predicate
                #[prost(message, tag = "4")]
                NotMatcher(::prost::alloc::boxed::Box<super::Predicate>),
            }
        }
        /// An individual matcher.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FieldMatcher {
            /// Determines if the match succeeds.
            #[prost(message, optional, tag = "1")]
            pub predicate: ::core::option::Option<Predicate>,
            /// What to do if the match succeeds.
            #[prost(message, optional, tag = "2")]
            pub on_match: ::core::option::Option<super::OnMatch>,
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatcherTree {
        /// Protocol-specific specification of input field to match on.
        #[prost(message, optional, tag = "1")]
        pub input: ::core::option::Option<
            super::super::super::super::core::v3::TypedExtensionConfig,
        >,
        /// Exact or prefix match maps in which to look up the input value.
        /// If the lookup succeeds, the match is considered successful, and
        /// the corresponding OnMatch is used.
        #[prost(oneof = "matcher_tree::TreeType", tags = "2, 3, 4")]
        pub tree_type: ::core::option::Option<matcher_tree::TreeType>,
    }
    /// Nested message and enum types in `MatcherTree`.
    pub mod matcher_tree {
        /// A map of configured matchers. Used to allow using a map within a oneof.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MatchMap {
            #[prost(map = "string, message", tag = "1")]
            pub map: ::std::collections::HashMap<
                ::prost::alloc::string::String,
                super::OnMatch,
            >,
        }
        /// Exact or prefix match maps in which to look up the input value.
        /// If the lookup succeeds, the match is considered successful, and
        /// the corresponding OnMatch is used.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum TreeType {
            #[prost(message, tag = "2")]
            ExactMatchMap(MatchMap),
            /// Longest matching prefix wins.
            #[prost(message, tag = "3")]
            PrefixMatchMap(MatchMap),
            /// Extension for custom matching logic.
            #[prost(message, tag = "4")]
            CustomMatch(
                super::super::super::super::super::core::v3::TypedExtensionConfig,
            ),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatcherType {
        /// A linear list of matchers to evaluate.
        #[prost(message, tag = "1")]
        MatcherList(MatcherList),
        /// A match tree to evaluate.
        #[prost(message, tag = "2")]
        MatcherTree(MatcherTree),
    }
}
/// Match configuration. This is a recursive structure which allows complex nested match
/// configurations to be built using various logical operators.
/// [#next-free-field: 11]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchPredicate {
    #[prost(oneof = "match_predicate::Rule", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub rule: ::core::option::Option<match_predicate::Rule>,
}
/// Nested message and enum types in `MatchPredicate`.
pub mod match_predicate {
    /// A set of match configurations used for logical operations.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchSet {
        /// The list of rules that make up the set.
        #[prost(message, repeated, tag = "1")]
        pub rules: ::prost::alloc::vec::Vec<super::MatchPredicate>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rule {
        /// A set that describes a logical OR. If any member of the set matches, the match configuration
        /// matches.
        #[prost(message, tag = "1")]
        OrMatch(MatchSet),
        /// A set that describes a logical AND. If all members of the set match, the match configuration
        /// matches.
        #[prost(message, tag = "2")]
        AndMatch(MatchSet),
        /// A negation match. The match configuration will match if the negated match condition matches.
        #[prost(message, tag = "3")]
        NotMatch(::prost::alloc::boxed::Box<super::MatchPredicate>),
        /// The match configuration will always match.
        #[prost(bool, tag = "4")]
        AnyMatch(bool),
        /// HTTP request headers match configuration.
        #[prost(message, tag = "5")]
        HttpRequestHeadersMatch(super::HttpHeadersMatch),
        /// HTTP request trailers match configuration.
        #[prost(message, tag = "6")]
        HttpRequestTrailersMatch(super::HttpHeadersMatch),
        /// HTTP response headers match configuration.
        #[prost(message, tag = "7")]
        HttpResponseHeadersMatch(super::HttpHeadersMatch),
        /// HTTP response trailers match configuration.
        #[prost(message, tag = "8")]
        HttpResponseTrailersMatch(super::HttpHeadersMatch),
        /// HTTP request generic body match configuration.
        #[prost(message, tag = "9")]
        HttpRequestGenericBodyMatch(super::HttpGenericBodyMatch),
        /// HTTP response generic body match configuration.
        #[prost(message, tag = "10")]
        HttpResponseGenericBodyMatch(super::HttpGenericBodyMatch),
    }
}
/// HTTP headers match configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpHeadersMatch {
    /// HTTP headers to match.
    #[prost(message, repeated, tag = "1")]
    pub headers: ::prost::alloc::vec::Vec<super::super::super::route::v3::HeaderMatcher>,
}
/// HTTP generic body match configuration.
/// List of text strings and hex strings to be located in HTTP body.
/// All specified strings must be found in the HTTP body for positive match.
/// The search may be limited to specified number of bytes from the body start.
///
/// .. attention::
///
///    Searching for patterns in HTTP body is potentially cpu intensive. For each specified pattern, http body is scanned byte by byte to find a match.
///    If multiple patterns are specified, the process is repeated for each pattern. If location of a pattern is known, ``bytes_limit`` should be specified
///    to scan only part of the http body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpGenericBodyMatch {
    /// Limits search to specified number of bytes - default zero (no limit - match entire captured buffer).
    #[prost(uint32, tag = "1")]
    pub bytes_limit: u32,
    /// List of patterns to match.
    #[prost(message, repeated, tag = "2")]
    pub patterns: ::prost::alloc::vec::Vec<http_generic_body_match::GenericTextMatch>,
}
/// Nested message and enum types in `HttpGenericBodyMatch`.
pub mod http_generic_body_match {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GenericTextMatch {
        #[prost(oneof = "generic_text_match::Rule", tags = "1, 2")]
        pub rule: ::core::option::Option<generic_text_match::Rule>,
    }
    /// Nested message and enum types in `GenericTextMatch`.
    pub mod generic_text_match {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Rule {
            /// Text string to be located in HTTP body.
            #[prost(string, tag = "1")]
            StringMatch(::prost::alloc::string::String),
            /// Sequence of bytes to be located in HTTP body.
            #[prost(bytes, tag = "2")]
            BinaryMatch(::prost::alloc::vec::Vec<u8>),
        }
    }
}
