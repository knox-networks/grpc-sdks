// @generated
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

// Requests the issuance limit be set to the value specified in `Amount` for the Issuer identified by its public key
// (aka Verifier).

/// \[Example\]
/// {
/// "limit": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// },
/// "issuer_public_key": "zIssuerPublicKeyMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetIssuerLimitRequest {
    /// Requested issuance amount limits including the limit and the currency code.
    #[prost(message, optional, tag="1")]
    pub limit: ::core::option::Option<super::super::common::Amount>,
    /// Verifier of the issuer.
    #[prost(string, tag="2")]
    pub issuer_public_key: ::prost::alloc::string::String,
}
// Contains the newly set issuance limit. The `Amount` indicates both the numeric value of the limit and the currency
// code. The Authority signs the amount and the public key (aka Verifier) of the issuer, and the signature is included
// in the response.

/// \[Example\]
/// {
/// "limit": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// },
/// "signature": "ByteLengthSignatureVariesByAlgo="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetIssuerLimitResponse {
    /// Approved issuance amount limit including the limit and the currency code.
    #[prost(message, optional, tag="1")]
    pub limit: ::core::option::Option<super::super::common::Amount>,
    /// Hex encoded cryptographic signature of the issuer and limit by the Authority.
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
// Requests the issuance limits associated with the issuer for all currencies that the Issuer is allowed to issue. The
// Issuer is identified by its public key (aka Verifier).

/// \[Example\]
/// {
/// "issuer_public_key": "zIssuerPublicKeyMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIssuerLimitsRequest {
    /// Verifier of the Issuer.
    #[prost(string, tag="2")]
    pub issuer_public_key: ::prost::alloc::string::String,
}
// Responds with the limits applicable to the Issuer for various currencies. The Issuer is identified by its public key
// (aka Verifier).

/// \[Example\]
/// {
/// "limit": [
/// {
/// "currency_code":"USD",
/// "amount": 100,
/// "decimals":2
/// }
/// ],
/// "issuer_public_key": "zIssuerPublicKeyMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIssuerLimitsResponse {
    /// Approved issuance amount limit.
    #[prost(message, repeated, tag="1")]
    pub limit: ::prost::alloc::vec::Vec<super::super::common::Amount>,
    /// Verifier of the Issuer.
    #[prost(string, tag="2")]
    pub issuer_public_key: ::prost::alloc::string::String,
}
// A digital banknote that is to be authorized. Several such requests may be sent in a stream.

/// \[Example\]
/// {
/// "promissory_file": "LengthOfFileBytesVaries="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeRequest {
    /// Digital banknote to authorize.
    #[prost(bytes="vec", tag="1")]
    pub promissory_file: ::prost::alloc::vec::Vec<u8>,
}
// A digital banknote that has been authorized. Several such responses may be returned in a stream.

/// \[Example\]
/// {
/// "promissory_file": "LengthOfFileBytesVaries="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeResponse {
    /// Digital banknote with authorized signature.
    #[prost(bytes="vec", tag="1")]
    pub promissory_file: ::prost::alloc::vec::Vec<u8>,
}
// Requests redemption of a digital banknote in exchange for an increase in the issuance limit for an Issuer identified
// by its Verifier (aka public key).

/// \[Example\]
/// {
/// "promissory_file": "LengthOfFileBytesVaries=",
/// "verifier": "VerifierLengthVariesByAlgo"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemRequest {
    /// Digital banknote to redeem.
    #[prost(bytes="vec", tag="1")]
    pub promissory_file: ::prost::alloc::vec::Vec<u8>,
    /// Verifier of the Issuer in multibase format.
    #[prost(string, tag="2")]
    pub verifier: ::prost::alloc::string::String,
}
// Returns the new issuance limit for the Issuer after redeeming a digital banknote.

/// \[Example\]
/// {
/// "limit": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemResponse {
    /// Updated issuance limit.
    #[prost(message, optional, tag="1")]
    pub limit: ::core::option::Option<super::super::common::Amount>,
}
// Requests to get the Emissary for the Authority.

/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkInfoRequest {
}
// Responds with verifiers of the Emissary and Transaction Manager

/// \[Example\]
/// {
/// "emissary": "EmissaryVerifierLengthVariesByAlgo",
/// "transaction_manager": "TransactionManagerVerifierLengthVariesByAlgo"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkInfoResponse {
    /// Verifier of the Emissary in multibase format.
    #[prost(string, tag="1")]
    pub emissary: ::prost::alloc::string::String,
    /// Verifier of the Transaction Manager in multibase format.
    #[prost(string, tag="2")]
    pub transaction_manager: ::prost::alloc::string::String,
}
// Requests to get the Notary signature for a Notary public key

/// \[Example\]
/// {
/// "verifier": "NotaryVerifierLengthVariesByAlgo"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotaryRequest {
    /// The Public Key of the Notary.
    #[prost(string, tag="1")]
    pub verifier: ::prost::alloc::string::String,
}
// Responds with the signature of the requested Notary

/// \[Example\]
/// {
/// "signature": "ByteLengthSignatureVariesByAlgo="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotaryResponse {
    /// The signature of the Notary created by signing the Notary Public key with the Authority keypair
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGovernedAssetsRequest {
}
/// \[Example\]
/// {
/// "asset_data": [
/// {
/// "key": "USD",
/// "value": {
/// "denominations": [1, 5, 10, 25, 100, 200, 500, 1000, 2000, 5000, 10000, 50000],
/// "precision": 2
/// }
/// },
/// {
/// "key": "JPY",
/// "value": {
/// "denominations": [1, 5, 10, 50, 100, 500, 1000, 2000, 5000, 10000, 50000, 100000],
/// "precision": 0
/// }
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGovernedAssetsResponse {
    /// List of currently Governed Assets.
    #[prost(map="string, message", tag="1")]
    pub asset_data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::common::AssetDenomination>,
}
/// \[Example\]
/// {
/// "asset_data": [
/// {
/// "key": "USD",
/// "value": {
/// "denominations": [1, 5, 10, 25, 100, 200, 500, 1000, 2000, 5000, 10000, 50000],
/// "precision": 2
/// }
/// },
/// {
/// "key": "JPY",
/// "value": {
/// "denominations": [1, 5, 10, 50, 100, 500, 1000, 2000, 5000, 10000, 50000, 100000],
/// "precision": 0
/// }
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetGovernedAssetsRequest {
    /// List of requested Governed Assets.
    #[prost(map="string, message", tag="1")]
    pub asset_data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::common::AssetDenomination>,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetGovernedAssetsResponse {
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
