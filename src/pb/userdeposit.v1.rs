// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeposits {
    #[prost(message, repeated, tag="1")]
    pub deposits: ::prost::alloc::vec::Vec<UserDeposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDeposit {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub referral: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub usd_value: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
