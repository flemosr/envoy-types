// This file is @generated by prost-build.
/// Identifies a percentage, in the range \[0.0, 100.0\].
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Percent {
    #[prost(double, tag = "1")]
    pub value: f64,
}
/// A fractional percentage is used in cases in which for performance reasons performing floating
/// point to integer conversions during randomness calculations is undesirable. The message includes
/// both a numerator and denominator that together determine the final fractional value.
///
/// * **Example**: 1/100 = 1%.
/// * **Example**: 3/10000 = 0.03%.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct FractionalPercent {
    /// Specifies the numerator. Defaults to 0.
    #[prost(uint32, tag = "1")]
    pub numerator: u32,
    /// Specifies the denominator. If the denominator specified is less than the numerator, the final
    /// fractional percentage is capped at 1 (100%).
    #[prost(enumeration = "fractional_percent::DenominatorType", tag = "2")]
    pub denominator: i32,
}
/// Nested message and enum types in `FractionalPercent`.
pub mod fractional_percent {
    /// Fraction percentages support several fixed denominator values.
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
    pub enum DenominatorType {
        /// 100.
        ///
        /// **Example**: 1/100 = 1%.
        Hundred = 0,
        /// 10,000.
        ///
        /// **Example**: 1/10000 = 0.01%.
        TenThousand = 1,
        /// 1,000,000.
        ///
        /// **Example**: 1/1000000 = 0.0001%.
        Million = 2,
    }
    impl DenominatorType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Hundred => "HUNDRED",
                Self::TenThousand => "TEN_THOUSAND",
                Self::Million => "MILLION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "HUNDRED" => Some(Self::Hundred),
                "TEN_THOUSAND" => Some(Self::TenThousand),
                "MILLION" => Some(Self::Million),
                _ => None,
            }
        }
    }
}
/// Envoy uses SemVer (<https://semver.org/>). Major/minor versions indicate
/// expected behaviors and APIs, the patch version field is used only
/// for security fixes and can be generally ignored.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SemanticVersion {
    #[prost(uint32, tag = "1")]
    pub major_number: u32,
    #[prost(uint32, tag = "2")]
    pub minor_number: u32,
    #[prost(uint32, tag = "3")]
    pub patch: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CodecClientType {
    Http1 = 0,
    Http2 = 1,
    /// \[\#not-implemented-hide:\] QUIC implementation is not production ready yet. Use this enum with
    /// caution to prevent accidental execution of QUIC code. I.e. `!= HTTP2` is no longer sufficient
    /// to distinguish HTTP1 and HTTP2 traffic.
    Http3 = 2,
}
impl CodecClientType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Http1 => "HTTP1",
            Self::Http2 => "HTTP2",
            Self::Http3 => "HTTP3",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HTTP1" => Some(Self::Http1),
            "HTTP2" => Some(Self::Http2),
            "HTTP3" => Some(Self::Http3),
            _ => None,
        }
    }
}
/// Specifies the int64 start and end of the range using half-open interval semantics \[start,
/// end).
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Int64Range {
    /// start of the range (inclusive)
    #[prost(int64, tag = "1")]
    pub start: i64,
    /// end of the range (exclusive)
    #[prost(int64, tag = "2")]
    pub end: i64,
}
/// Specifies the int32 start and end of the range using half-open interval semantics \[start,
/// end).
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Int32Range {
    /// start of the range (inclusive)
    #[prost(int32, tag = "1")]
    pub start: i32,
    /// end of the range (exclusive)
    #[prost(int32, tag = "2")]
    pub end: i32,
}
/// Specifies the double start and end of the range using half-open interval semantics \[start,
/// end).
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct DoubleRange {
    /// start of the range (inclusive)
    #[prost(double, tag = "1")]
    pub start: f64,
    /// end of the range (exclusive)
    #[prost(double, tag = "2")]
    pub end: f64,
}
/// Identifies the unit of of time for rate limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RateLimitUnit {
    /// The time unit is not known.
    Unknown = 0,
    /// The time unit representing a second.
    Second = 1,
    /// The time unit representing a minute.
    Minute = 2,
    /// The time unit representing an hour.
    Hour = 3,
    /// The time unit representing a day.
    Day = 4,
    /// The time unit representing a month.
    Month = 5,
    /// The time unit representing a year.
    Year = 6,
}
impl RateLimitUnit {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unknown => "UNKNOWN",
            Self::Second => "SECOND",
            Self::Minute => "MINUTE",
            Self::Hour => "HOUR",
            Self::Day => "DAY",
            Self::Month => "MONTH",
            Self::Year => "YEAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "SECOND" => Some(Self::Second),
            "MINUTE" => Some(Self::Minute),
            "HOUR" => Some(Self::Hour),
            "DAY" => Some(Self::Day),
            "MONTH" => Some(Self::Month),
            "YEAR" => Some(Self::Year),
            _ => None,
        }
    }
}
/// Configures a token bucket, typically used for rate limiting.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TokenBucket {
    /// The maximum tokens that the bucket can hold. This is also the number of tokens that the bucket
    /// initially contains.
    #[prost(uint32, tag = "1")]
    pub max_tokens: u32,
    /// The number of tokens added to the bucket during each fill interval. If not specified, defaults
    /// to a single token.
    #[prost(message, optional, tag = "2")]
    pub tokens_per_fill: ::core::option::Option<
        super::super::super::google::protobuf::UInt32Value,
    >,
    /// The fill interval that tokens are added to the bucket. During each fill interval
    /// `tokens_per_fill` are added to the bucket. The bucket will never contain more than
    /// `max_tokens` tokens.
    #[prost(message, optional, tag = "3")]
    pub fill_interval: ::core::option::Option<
        super::super::super::google::protobuf::Duration,
    >,
}
/// HTTP status.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HttpStatus {
    /// Supplies HTTP response code.
    #[prost(enumeration = "StatusCode", tag = "1")]
    pub code: i32,
}
/// HTTP response codes supported in Envoy.
/// For more details: <https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml>
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StatusCode {
    /// Empty - This code not part of the HTTP status code specification, but it is needed for proto
    /// `enum` type.
    Empty = 0,
    /// Continue - `100` status code.
    Continue = 100,
    /// OK - `200` status code.
    Ok = 200,
    /// Created - `201` status code.
    Created = 201,
    /// Accepted - `202` status code.
    Accepted = 202,
    /// NonAuthoritativeInformation - `203` status code.
    NonAuthoritativeInformation = 203,
    /// NoContent - `204` status code.
    NoContent = 204,
    /// ResetContent - `205` status code.
    ResetContent = 205,
    /// PartialContent - `206` status code.
    PartialContent = 206,
    /// MultiStatus - `207` status code.
    MultiStatus = 207,
    /// AlreadyReported - `208` status code.
    AlreadyReported = 208,
    /// IMUsed - `226` status code.
    ImUsed = 226,
    /// MultipleChoices - `300` status code.
    MultipleChoices = 300,
    /// MovedPermanently - `301` status code.
    MovedPermanently = 301,
    /// Found - `302` status code.
    Found = 302,
    /// SeeOther - `303` status code.
    SeeOther = 303,
    /// NotModified - `304` status code.
    NotModified = 304,
    /// UseProxy - `305` status code.
    UseProxy = 305,
    /// TemporaryRedirect - `307` status code.
    TemporaryRedirect = 307,
    /// PermanentRedirect - `308` status code.
    PermanentRedirect = 308,
    /// BadRequest - `400` status code.
    BadRequest = 400,
    /// Unauthorized - `401` status code.
    Unauthorized = 401,
    /// PaymentRequired - `402` status code.
    PaymentRequired = 402,
    /// Forbidden - `403` status code.
    Forbidden = 403,
    /// NotFound - `404` status code.
    NotFound = 404,
    /// MethodNotAllowed - `405` status code.
    MethodNotAllowed = 405,
    /// NotAcceptable - `406` status code.
    NotAcceptable = 406,
    /// ProxyAuthenticationRequired - `407` status code.
    ProxyAuthenticationRequired = 407,
    /// RequestTimeout - `408` status code.
    RequestTimeout = 408,
    /// Conflict - `409` status code.
    Conflict = 409,
    /// Gone - `410` status code.
    Gone = 410,
    /// LengthRequired - `411` status code.
    LengthRequired = 411,
    /// PreconditionFailed - `412` status code.
    PreconditionFailed = 412,
    /// PayloadTooLarge - `413` status code.
    PayloadTooLarge = 413,
    /// URITooLong - `414` status code.
    UriTooLong = 414,
    /// UnsupportedMediaType - `415` status code.
    UnsupportedMediaType = 415,
    /// RangeNotSatisfiable - `416` status code.
    RangeNotSatisfiable = 416,
    /// ExpectationFailed - `417` status code.
    ExpectationFailed = 417,
    /// MisdirectedRequest - `421` status code.
    MisdirectedRequest = 421,
    /// UnprocessableEntity - `422` status code.
    UnprocessableEntity = 422,
    /// Locked - `423` status code.
    Locked = 423,
    /// FailedDependency - `424` status code.
    FailedDependency = 424,
    /// UpgradeRequired - `426` status code.
    UpgradeRequired = 426,
    /// PreconditionRequired - `428` status code.
    PreconditionRequired = 428,
    /// TooManyRequests - `429` status code.
    TooManyRequests = 429,
    /// RequestHeaderFieldsTooLarge - `431` status code.
    RequestHeaderFieldsTooLarge = 431,
    /// InternalServerError - `500` status code.
    InternalServerError = 500,
    /// NotImplemented - `501` status code.
    NotImplemented = 501,
    /// BadGateway - `502` status code.
    BadGateway = 502,
    /// ServiceUnavailable - `503` status code.
    ServiceUnavailable = 503,
    /// GatewayTimeout - `504` status code.
    GatewayTimeout = 504,
    /// HTTPVersionNotSupported - `505` status code.
    HttpVersionNotSupported = 505,
    /// VariantAlsoNegotiates - `506` status code.
    VariantAlsoNegotiates = 506,
    /// InsufficientStorage - `507` status code.
    InsufficientStorage = 507,
    /// LoopDetected - `508` status code.
    LoopDetected = 508,
    /// NotExtended - `510` status code.
    NotExtended = 510,
    /// NetworkAuthenticationRequired - `511` status code.
    NetworkAuthenticationRequired = 511,
}
impl StatusCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Empty => "Empty",
            Self::Continue => "Continue",
            Self::Ok => "OK",
            Self::Created => "Created",
            Self::Accepted => "Accepted",
            Self::NonAuthoritativeInformation => "NonAuthoritativeInformation",
            Self::NoContent => "NoContent",
            Self::ResetContent => "ResetContent",
            Self::PartialContent => "PartialContent",
            Self::MultiStatus => "MultiStatus",
            Self::AlreadyReported => "AlreadyReported",
            Self::ImUsed => "IMUsed",
            Self::MultipleChoices => "MultipleChoices",
            Self::MovedPermanently => "MovedPermanently",
            Self::Found => "Found",
            Self::SeeOther => "SeeOther",
            Self::NotModified => "NotModified",
            Self::UseProxy => "UseProxy",
            Self::TemporaryRedirect => "TemporaryRedirect",
            Self::PermanentRedirect => "PermanentRedirect",
            Self::BadRequest => "BadRequest",
            Self::Unauthorized => "Unauthorized",
            Self::PaymentRequired => "PaymentRequired",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "NotFound",
            Self::MethodNotAllowed => "MethodNotAllowed",
            Self::NotAcceptable => "NotAcceptable",
            Self::ProxyAuthenticationRequired => "ProxyAuthenticationRequired",
            Self::RequestTimeout => "RequestTimeout",
            Self::Conflict => "Conflict",
            Self::Gone => "Gone",
            Self::LengthRequired => "LengthRequired",
            Self::PreconditionFailed => "PreconditionFailed",
            Self::PayloadTooLarge => "PayloadTooLarge",
            Self::UriTooLong => "URITooLong",
            Self::UnsupportedMediaType => "UnsupportedMediaType",
            Self::RangeNotSatisfiable => "RangeNotSatisfiable",
            Self::ExpectationFailed => "ExpectationFailed",
            Self::MisdirectedRequest => "MisdirectedRequest",
            Self::UnprocessableEntity => "UnprocessableEntity",
            Self::Locked => "Locked",
            Self::FailedDependency => "FailedDependency",
            Self::UpgradeRequired => "UpgradeRequired",
            Self::PreconditionRequired => "PreconditionRequired",
            Self::TooManyRequests => "TooManyRequests",
            Self::RequestHeaderFieldsTooLarge => "RequestHeaderFieldsTooLarge",
            Self::InternalServerError => "InternalServerError",
            Self::NotImplemented => "NotImplemented",
            Self::BadGateway => "BadGateway",
            Self::ServiceUnavailable => "ServiceUnavailable",
            Self::GatewayTimeout => "GatewayTimeout",
            Self::HttpVersionNotSupported => "HTTPVersionNotSupported",
            Self::VariantAlsoNegotiates => "VariantAlsoNegotiates",
            Self::InsufficientStorage => "InsufficientStorage",
            Self::LoopDetected => "LoopDetected",
            Self::NotExtended => "NotExtended",
            Self::NetworkAuthenticationRequired => "NetworkAuthenticationRequired",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Empty" => Some(Self::Empty),
            "Continue" => Some(Self::Continue),
            "OK" => Some(Self::Ok),
            "Created" => Some(Self::Created),
            "Accepted" => Some(Self::Accepted),
            "NonAuthoritativeInformation" => Some(Self::NonAuthoritativeInformation),
            "NoContent" => Some(Self::NoContent),
            "ResetContent" => Some(Self::ResetContent),
            "PartialContent" => Some(Self::PartialContent),
            "MultiStatus" => Some(Self::MultiStatus),
            "AlreadyReported" => Some(Self::AlreadyReported),
            "IMUsed" => Some(Self::ImUsed),
            "MultipleChoices" => Some(Self::MultipleChoices),
            "MovedPermanently" => Some(Self::MovedPermanently),
            "Found" => Some(Self::Found),
            "SeeOther" => Some(Self::SeeOther),
            "NotModified" => Some(Self::NotModified),
            "UseProxy" => Some(Self::UseProxy),
            "TemporaryRedirect" => Some(Self::TemporaryRedirect),
            "PermanentRedirect" => Some(Self::PermanentRedirect),
            "BadRequest" => Some(Self::BadRequest),
            "Unauthorized" => Some(Self::Unauthorized),
            "PaymentRequired" => Some(Self::PaymentRequired),
            "Forbidden" => Some(Self::Forbidden),
            "NotFound" => Some(Self::NotFound),
            "MethodNotAllowed" => Some(Self::MethodNotAllowed),
            "NotAcceptable" => Some(Self::NotAcceptable),
            "ProxyAuthenticationRequired" => Some(Self::ProxyAuthenticationRequired),
            "RequestTimeout" => Some(Self::RequestTimeout),
            "Conflict" => Some(Self::Conflict),
            "Gone" => Some(Self::Gone),
            "LengthRequired" => Some(Self::LengthRequired),
            "PreconditionFailed" => Some(Self::PreconditionFailed),
            "PayloadTooLarge" => Some(Self::PayloadTooLarge),
            "URITooLong" => Some(Self::UriTooLong),
            "UnsupportedMediaType" => Some(Self::UnsupportedMediaType),
            "RangeNotSatisfiable" => Some(Self::RangeNotSatisfiable),
            "ExpectationFailed" => Some(Self::ExpectationFailed),
            "MisdirectedRequest" => Some(Self::MisdirectedRequest),
            "UnprocessableEntity" => Some(Self::UnprocessableEntity),
            "Locked" => Some(Self::Locked),
            "FailedDependency" => Some(Self::FailedDependency),
            "UpgradeRequired" => Some(Self::UpgradeRequired),
            "PreconditionRequired" => Some(Self::PreconditionRequired),
            "TooManyRequests" => Some(Self::TooManyRequests),
            "RequestHeaderFieldsTooLarge" => Some(Self::RequestHeaderFieldsTooLarge),
            "InternalServerError" => Some(Self::InternalServerError),
            "NotImplemented" => Some(Self::NotImplemented),
            "BadGateway" => Some(Self::BadGateway),
            "ServiceUnavailable" => Some(Self::ServiceUnavailable),
            "GatewayTimeout" => Some(Self::GatewayTimeout),
            "HTTPVersionNotSupported" => Some(Self::HttpVersionNotSupported),
            "VariantAlsoNegotiates" => Some(Self::VariantAlsoNegotiates),
            "InsufficientStorage" => Some(Self::InsufficientStorage),
            "LoopDetected" => Some(Self::LoopDetected),
            "NotExtended" => Some(Self::NotExtended),
            "NetworkAuthenticationRequired" => Some(Self::NetworkAuthenticationRequired),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RateLimitStrategy {
    #[prost(oneof = "rate_limit_strategy::Strategy", tags = "1, 2, 3")]
    pub strategy: ::core::option::Option<rate_limit_strategy::Strategy>,
}
/// Nested message and enum types in `RateLimitStrategy`.
pub mod rate_limit_strategy {
    /// Best-effort limit of the number of requests per time unit.
    ///
    /// Allows to specify the desired requests per second (RPS, QPS), requests per minute (QPM, RPM),
    /// etc., without specifying a rate limiting algorithm implementation.
    ///
    /// `RequestsPerTimeUnit` strategy does not demand any specific rate limiting algorithm to be
    /// used (in contrast to the :ref:`TokenBucket <envoy_v3_api_msg_type.v3.TokenBucket>`,
    /// for example). It implies that the implementation details of rate limiting algorithm are
    /// irrelevant as long as the configured number of "requests per time unit" is achieved.
    ///
    /// Note that the `TokenBucket` is still a valid implementation of the `RequestsPerTimeUnit`
    /// strategy, and may be chosen to enforce the rate limit. However, there's no guarantee it will be
    /// the `TokenBucket` in particular, and not the Leaky Bucket, the Sliding Window, or any other
    /// rate limiting algorithm that fulfills the requirements.
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RequestsPerTimeUnit {
        /// The desired number of requests per :ref:`time_unit  <envoy_v3_api_field_type.v3.RateLimitStrategy.RequestsPerTimeUnit.time_unit>` to allow.
        /// If set to `0`, deny all (equivalent to `BlanketRule.DENY_ALL`).
        ///
        /// .. note::
        /// Note that the algorithm implementation determines the course of action for the requests
        /// over the limit. As long as the `requests_per_time_unit` converges on the desired value,
        /// it's allowed to treat this field as a soft-limit: allow bursts, redistribute the allowance
        /// over time, etc.
        #[prost(uint64, tag = "1")]
        pub requests_per_time_unit: u64,
        /// The unit of time. Ignored when :ref:`requests_per_time_unit  <envoy_v3_api_field_type.v3.RateLimitStrategy.RequestsPerTimeUnit.requests_per_time_unit>`
        /// is `0` (deny all).
        #[prost(enumeration = "super::RateLimitUnit", tag = "2")]
        pub time_unit: i32,
    }
    /// Choose between allow all and deny all.
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
    pub enum BlanketRule {
        AllowAll = 0,
        DenyAll = 1,
    }
    impl BlanketRule {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::AllowAll => "ALLOW_ALL",
                Self::DenyAll => "DENY_ALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ALLOW_ALL" => Some(Self::AllowAll),
                "DENY_ALL" => Some(Self::DenyAll),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum Strategy {
        /// Allow or Deny the requests.
        /// If unset, allow all.
        #[prost(enumeration = "BlanketRule", tag = "1")]
        BlanketRule(i32),
        /// Best-effort limit of the number of requests per time unit, f.e. requests per second.
        /// Does not prescribe any specific rate limiting algorithm, see :ref:`RequestsPerTimeUnit  <envoy_v3_api_msg_type.v3.RateLimitStrategy.RequestsPerTimeUnit>` for details.
        #[prost(message, tag = "2")]
        RequestsPerTimeUnit(RequestsPerTimeUnit),
        /// Limit the requests by consuming tokens from the Token Bucket.
        /// Allow the same number of requests as the number of tokens available in
        /// the token bucket.
        #[prost(message, tag = "3")]
        TokenBucket(super::TokenBucket),
    }
}
/// Specifies the hash policy
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashPolicy {
    #[prost(oneof = "hash_policy::PolicySpecifier", tags = "1, 2")]
    pub policy_specifier: ::core::option::Option<hash_policy::PolicySpecifier>,
}
/// Nested message and enum types in `HashPolicy`.
pub mod hash_policy {
    /// The source IP will be used to compute the hash used by hash-based load balancing
    /// algorithms.
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct SourceIp {}
    /// An Object in the :ref:`filterState <arch_overview_data_sharing_between_filters>` will be used
    /// to compute the hash used by hash-based load balancing algorithms.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FilterState {
        /// The name of the Object in the filterState, which is an Envoy::Hashable object. If there is no
        /// data associated with the key, or the stored object is not Envoy::Hashable, no hash will be
        /// produced.
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PolicySpecifier {
        #[prost(message, tag = "1")]
        SourceIp(SourceIp),
        #[prost(message, tag = "2")]
        FilterState(FilterState),
    }
}
