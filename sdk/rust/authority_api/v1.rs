// @generated
/// Requests the issuance limit be set to the value specified in `Amount` for the Issuer identified by its public key (aka Verifier).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetIssuerLimitRequest {
    /// Requested issuance amount limits including the limit and the currency code.
    #[prost(message, optional, tag="1")]
    pub limit: ::core::option::Option<super::super::common::Amount>,
    /// Verifier of the issuer.
    #[prost(string, tag="2")]
    pub issuer_public_key: ::prost::alloc::string::String,
}
/// Contains the newly set issuance limit. The `Amount` indicates both the numeric value of the limit and the currency code. The Authority signs the amount and the public key (aka Verifier) of the issuer, and the signature is included in the response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetIssuerLimitResponse {
    /// Approved issuance amount limit including the limit and the currency code.
    #[prost(message, optional, tag="1")]
    pub limit: ::core::option::Option<super::super::common::Amount>,
    /// Hex encoded cryptographic signature of the issuer and limit by the Authority.
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Requests the issuance limits associated with the issuer for all currencies that the Issuer is allowed to issue. The Issuer is identified by its public key (aka Verifier).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIssuerLimitsRequest {
    /// Verifier of the Issuer.
    #[prost(string, tag="2")]
    pub issuer_public_key: ::prost::alloc::string::String,
}
/// Responds with the limits applicable to the Issuer for various currencies. The Issuer is identified by its public key (aka Verifier).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIssuerLimitsResponse {
    /// Approved issuance amount limit.
    #[prost(message, repeated, tag="1")]
    pub limit: ::prost::alloc::vec::Vec<super::super::common::Amount>,
    /// Verifier of the Issuer.
    #[prost(string, tag="2")]
    pub issuer_public_key: ::prost::alloc::string::String,
}
/// A digital banknote that is to be authorized. Several such requests may be sent in a stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeRequest {
    /// Digital banknote to authorize.
    #[prost(bytes="vec", tag="1")]
    pub promissory_file: ::prost::alloc::vec::Vec<u8>,
}
/// A digital banknote that has been authorized. Several such responses may be returned in a stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeResponse {
    /// Digital banknote with authorized signature.
    #[prost(bytes="vec", tag="1")]
    pub promissory_file: ::prost::alloc::vec::Vec<u8>,
}
/// Requests redemption of a digital banknote in exchange for an increase in the issuance limit for an Issuer identified by its Verifier (aka public key).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemRequest {
    /// Digital banknote to redeem.
    #[prost(bytes="vec", tag="1")]
    pub promissory_file: ::prost::alloc::vec::Vec<u8>,
    /// Verifier of the Issuer in multi-base format.
    #[prost(string, tag="2")]
    pub verifier: ::prost::alloc::string::String,
}
/// Returns the new issuance limit for the Issuer after redeeming a digital banknote.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemResponse {
    /// Updated issuance limit.
    #[prost(message, optional, tag="1")]
    pub limit: ::core::option::Option<super::super::common::Amount>,
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
