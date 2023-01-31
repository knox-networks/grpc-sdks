// @generated
/// Faucet request with the total amount and recipient.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaucetRequest {
    /// Total amount to be issued.
    #[prost(message, optional, tag="1")]
    pub amount: ::core::option::Option<super::super::common::Amount>,
    /// Verifier public key of the intended recipient of the issued promissory
    /// files (multibase encoding).
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
}
/// Faucet response including the number of promissory files issued.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FaucetResponse {
    #[prost(uint32, tag="1")]
    pub num_promissory_files: u32,
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
