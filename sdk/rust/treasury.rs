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
/// \[Example\]
/// {
/// "page": 0,
/// "page_size": 10,
/// "filters": {
/// "filters": [
/// {
/// "field": "FIELD1",
/// "operator": 0,
/// "value": {
/// "str_value": "STRINGVAL"
/// }
/// },
/// {
/// "field": "FIELD2",
/// "operator": 0,
/// "value": {
/// "int_value": 10
/// }
/// },
/// {
/// "field": "FIELD3",
/// "operator": 0,
/// "value": {
/// "bool_value": false
/// }
/// }
/// ],
/// "operator": 0
/// },
/// "sort_by": {
/// "items": [
/// {
/// "field": "FIELD1",
/// "direction": 0
/// },
/// {
/// "field": "FIELD2",
/// "direction": 0
/// }
/// ]
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionsRequest {
    #[prost(int32, tag="1")]
    pub page: i32,
    #[prost(int32, tag="2")]
    pub page_size: i32,
    #[prost(message, optional, tag="3")]
    pub sort_by: ::core::option::Option<super::common::SortBy>,
    #[prost(message, optional, tag="4")]
    pub filters: ::core::option::Option<super::common::Filter>,
}
// \[Example\]
// {
// "contract_id": "9628a2d9-f932-41f0-b2da-b9de3103d945",
// "contract_type": "Relay",
// "created": "2022-12-19T16:39:57-08:00",
// "counterparties": [
// "zCounterpartyPublicKeyMultibase58Encoded",
// ],
// "memo": "Payment",
// }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractSummary {
    /// UUID of the contract
    #[prost(string, tag="1")]
    pub contract_id: ::prost::alloc::string::String,
    /// Type of the contract
    #[prost(string, tag="2")]
    pub contract_type: ::prost::alloc::string::String,
    /// RFC3339 timestamp of the contract creation date.
    #[prost(string, tag="3")]
    pub created: ::prost::alloc::string::String,
    /// List of participant public keys excluding self
    #[prost(string, repeated, tag="4")]
    pub counterparties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Contract memo
    #[prost(string, tag="6")]
    pub memo: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "contracts": [
/// {
/// "contract_id": "9628a2d9-f932-41f0-b2da-b9de3103d945",
/// "contract_type": "Relay",
/// "created": "2022-12-19T16:39:57-08:00",
/// "counterparties": [
/// "zCounterpartyPublicKeyMultibase58Encoded",
/// "zCounterpartyPublicKey2Multibase58Encoded"
/// ],
/// "memo": "Payment"
/// }
/// ],
/// "totalContracts": 1
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionsResponse {
    /// List of contracts with status metadata
    #[prost(message, repeated, tag="1")]
    pub contracts: ::prost::alloc::vec::Vec<ContractSummary>,
    /// Total number of contracts in paginated set
    #[prost(uint32, tag="2")]
    pub total_contracts: u32,
}
include!("treasury.tonic.rs");
// @@protoc_insertion_point(module)
