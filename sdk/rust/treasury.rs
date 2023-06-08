// @generated
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

// Requests the count of files of various denominations for a given currency.

/// \[Example\]
/// {
/// "currency_code": "USD"
/// } 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSupplyRequest {
    /// The currency code of the requested supply.
    #[prost(string, tag="1")]
    pub currency_code: ::prost::alloc::string::String,
}
// Lists the count of the files of specific denominations held in the Treasury service's vault, and the total value held
// in files of that denomination.

/// \[Example\]
/// {
/// "denomination": {
/// "currency_code":"USD", 
/// "amount":2000, 
/// "decimals":2
/// }, 
/// "count": 5, 
/// "total": {
/// "currency_code":"USD", 
/// "amount":10000, 
/// "decimals":2
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenominationCount {
    /// The amount of the denomination.
    #[prost(message, optional, tag="1")]
    pub denomination: ::core::option::Option<super::common::Amount>,
    /// The number of digital banknotes of this amount held in supply.
    #[prost(uint32, tag="2")]
    pub count: u32,
    /// The total value represented by digital banknotes of this denomination.
    #[prost(message, optional, tag="3")]
    pub total: ::core::option::Option<super::common::Amount>,
}
// Provides a list of file denominations in the Treasury service's Vault.

/// \[Example\]
/// {
/// "supply": [
/// {
/// "denomination": {
/// "currency_code":"USD", 
/// "amount":2000, 
/// "decimals":2
/// }, 
/// "count": 5, 
/// "total": {
/// "currency_code":"USD", 
/// "amount":10000, "decimals":2
/// }
/// }, 
/// {
/// "denomination": {
/// "currency_code":"USD", 
/// "amount":1000, 
/// "decimals":2
/// }, 
/// "count": 5, 
/// "total": {
/// "currency_code":"USD", 
/// "amount":5000, 
/// "decimals":2
/// }
/// }
/// ], 
/// "total": {
/// "currency_code":"USD", 
/// "amount": 15000, 
/// "decimals":2
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSupplyResponse {
    /// A list of denomination counts of the digital banknotes held in supply.
    #[prost(message, repeated, tag="1")]
    pub supply: ::prost::alloc::vec::Vec<DenominationCount>,
    /// grand total sum of all promissories
    #[prost(message, optional, tag="2")]
    pub total: ::core::option::Option<super::common::Amount>,
}
include!("treasury.tonic.rs");
// @@protoc_insertion_point(module)
