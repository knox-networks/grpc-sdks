// @generated
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

// A request to issue new digital banknotes.

/// \[Example\]
/// {
/// "authentication_signature": {
/// "signature": "ByteLengthSignatureVariesByAlgo=",
/// "verifier": {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// }
/// },
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// },
/// "authority": {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueRequest {
    /// Authentication challenge signature.
    #[prost(message, optional, tag="1")]
    pub authentication_signature: ::core::option::Option<super::super::common::AuthenticationSignature>,
    /// Total amount to be issued.
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<super::super::common::Amount>,
    /// Verifier of the Authority.
    #[prost(message, optional, tag="3")]
    pub authority: ::core::option::Option<super::super::common::DynamicVerifier>,
    /// Verifier of the Recipient.
    #[prost(message, optional, tag="5")]
    pub recipient: ::core::option::Option<super::super::common::DynamicVerifier>,
    /// sum must match corresponding amount.amount
    #[prost(message, optional, tag="4")]
    pub distribution: ::core::option::Option<super::super::common::Distribution>,
    /// Authentication session ID that should match the one given by the AuthenticationResponse
    #[prost(string, tag="6")]
    pub session: ::prost::alloc::string::String,
    #[prost(enumeration="super::super::common::SignatureSystem", tag="7")]
    pub signature_system: i32,
}
// Responds to request with a count of generated promissory files

/// \[Example\]
/// {
/// "num_promissory_files": 4
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueResponse {
    /// Count of Files Returned.
    #[prost(uint32, tag="1")]
    pub num_promissory_files: u32,
}
// Authentication challenge nonce.

/// \[Example\]
/// {
/// "verifier": {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateRequest {
    /// Verifier of the requester.
    #[prost(message, optional, tag="1")]
    pub verifier: ::core::option::Option<super::super::common::DynamicVerifier>,
}
// Responds to request with authentication challenge nonce.

/// \[Example\]
/// {
/// "challenge": "ByteLengthChallengeVariesByAlgo="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateResponse {
    /// Random unique string to be signed by the authorized key.
    #[prost(bytes="vec", tag="1")]
    pub challenge: ::prost::alloc::vec::Vec<u8>,
    /// UUID representing this authentication session
    #[prost(string, tag="2")]
    pub session: ::prost::alloc::string::String,
}
// Gets roles set in the Issuer for configuring access.

/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRolesRequest {
}
// Returns roles set in the Issuer for configuring access.

/// \[Example\]
/// {
/// "roles": [
/// {
/// "key": "KEY1",
/// "value": "VALUE1"
/// },
/// {
/// "key": "KEY2",
/// "value": "VALUE2"
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRolesResponse {
    /// Key is the verifier and the Value is <authorized/unauthorized>: <currency limits>.
    #[prost(map="string, string", tag="1")]
    pub roles: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
// Sets roles in the Issuer for configuring access.

/// \[Example\]
/// {
/// "role": {
/// "verifier": "VerifierLengthVariesByAlgo",
/// "role_enum": {
/// "issue_permissions": {
/// "inner": [
/// {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// },
/// {
/// "currency_code":"USD",
/// "amount":500,
/// "decimals":2
/// }
/// ]
/// }
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoleRequest {
    /// Configured roles set in the Issuer.
    #[prost(message, optional, tag="1")]
    pub role: ::core::option::Option<super::super::common::Role>,
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoleResponse {
}
/// \[Example\]
/// {
/// "denominations": []
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAssetDenominationsRequest {
    /// Asset Denominations to Set.
    #[prost(map="string, message", tag="1")]
    pub denominations: ::std::collections::HashMap<::prost::alloc::string::String, super::super::common::AssetDenomination>,
}
/// \[Example\]
/// {}
///
/// Empty body
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAssetDenominationsResponse {
}
/// \[Example\]
/// {
/// "authorities": []
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAssetAuthoritiesRequest {
    #[prost(map="string, message", tag="1")]
    pub authorities: ::std::collections::HashMap<::prost::alloc::string::String, super::super::common::AssetAuthority>,
}
/// \[Example\]
/// {}
///
/// Empty body
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAssetAuthoritiesResponse {
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
