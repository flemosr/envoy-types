// This file is @generated by prost-build.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DnsLookupFamily {
    Unspecified = 0,
    Auto = 1,
    V4Only = 2,
    V6Only = 3,
    V4Preferred = 4,
    All = 5,
}
impl DnsLookupFamily {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Auto => "AUTO",
            Self::V4Only => "V4_ONLY",
            Self::V6Only => "V6_ONLY",
            Self::V4Preferred => "V4_PREFERRED",
            Self::All => "ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "AUTO" => Some(Self::Auto),
            "V4_ONLY" => Some(Self::V4Only),
            "V6_ONLY" => Some(Self::V6Only),
            "V4_PREFERRED" => Some(Self::V4Preferred),
            "ALL" => Some(Self::All),
            _ => None,
        }
    }
}
