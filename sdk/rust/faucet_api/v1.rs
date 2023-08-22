// @generated
// THIS PROTO FILE IS INTENDED FOR INTERNAL KNOX USE ONLY.

// Faucet request with the total amount and recipient.

/// \[Example\]
/// {
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// },
/// "recipient": "zRecipientPublicKeyMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaucetRequest {
    /// Total amount to be issued.
    #[prost(message, optional, tag="1")]
    pub amount: ::core::option::Option<super::super::common::Amount>,
    /// Verifier public key of the intended recipient of the issued promissory
    /// files (multibase encoding).
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    /// sum must match corresponding amount.amount
    #[prost(message, optional, tag="3")]
    pub distribution: ::core::option::Option<super::super::common::Distribution>,
}
// Faucet response including the number of promissory files issued.

/// \[Example\]
/// {
/// "num_promissory_files": 10
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaucetResponse {
    /// Number of Promissory Files Returned.
    #[prost(uint32, tag="1")]
    pub num_promissory_files: u32,
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
