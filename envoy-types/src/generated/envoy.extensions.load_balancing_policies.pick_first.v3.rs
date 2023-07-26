/// This configuration allows the built-in PICK_FIRST LB policy to be configured
/// via the LB policy extension point.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickFirst {
    /// If set to true, instructs the LB policy to shuffle the list of addresses
    /// received from the name resolver before attempting to connect to them.
    #[prost(bool, tag = "1")]
    pub shuffle_address_list: bool,
}
