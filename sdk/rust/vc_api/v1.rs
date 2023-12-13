// @generated
// For more details, refer to https:// Www.w3.org/TR/vc-data-integrity/

/// \[Example\]
/// {
/// "type": "TYPE", 
/// "created": "2006-01-02T15:04:05Z", 
/// "verification_method": "VERIFICATION_METHOD", 
/// "proof_purpose": "PROOF_PURPOSE", 
/// "proof_value": "PROOF_VALUE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CredentialProof {
    /// The cryptographic suite used for the proof.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// Timestamp of when the cryptographic proof was created.
    #[prost(string, tag="2")]
    pub created: ::prost::alloc::string::String,
    /// Link to the public key required for verification process.
    #[prost(string, tag="3")]
    pub verification_method: ::prost::alloc::string::String,
    /// The reason the proof was created.
    #[prost(string, tag="4")]
    pub proof_purpose: ::prost::alloc::string::String,
    /// Multibase-encoded binary value that represents the hash of the payload upon decryption through verification_method.
    #[prost(string, tag="5")]
    pub proof_value: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "credential_type": 1, 
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIssuanceChallengeRequest {
    /// Type of the verifiable credential being requested for issuance.
    #[prost(enumeration="CredentialType", tag="1")]
    pub credential_type: i32,
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="2")]
    pub did: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "nonce": "NonceLengthVariesByAlgo", 
/// "endpoint": "QR_URL_ENDPOINT", 
/// "credential_type": 1
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIssuanceChallengeResponse {
    /// Value used for the challenge.
    #[prost(string, tag="1")]
    pub nonce: ::prost::alloc::string::String,
    /// Used by browser to construct URL for QR generation.
    #[prost(string, tag="2")]
    pub endpoint: ::prost::alloc::string::String,
    /// Type of the verifiable credential being requested for issuance.
    #[prost(enumeration="CredentialType", tag="3")]
    pub credential_type: i32,
}
/// \[Example\]
/// {
/// "credential_type": 1, 
/// "nonce": "NonceLengthVariesByAlgo", 
/// "signature": "SIGNATURExBYTES=", 
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueVerifiableCredentialRequest {
    /// Type of the verifiable credential being requested for issuance.
    #[prost(enumeration="CredentialType", tag="1")]
    pub credential_type: i32,
    /// Value used for the challenge.
    #[prost(string, tag="2")]
    pub nonce: ::prost::alloc::string::String,
    /// Signed value of the challenge by the wallet.
    #[prost(bytes="vec", tag="3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="4")]
    pub did: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "credential": 1
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueVerifiableCredentialResponse {
    /// Type of the verifiable credential being issued.
    #[prost(string, tag="1")]
    pub credential: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "credential_types": [
/// 0,
/// 1
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePresentationChallengeRequest {
    /// Type(s) of the verifiable credential(s) being presented.
    #[prost(enumeration="CredentialType", repeated, tag="1")]
    pub credential_types: ::prost::alloc::vec::Vec<i32>,
}
/// \[Example\]
/// {
/// "nonce": "NonceLengthVariesByAlgo", 
/// "endpoint": "QR_URL_ENDPOINT", 
/// "credential_types": [
/// 0, 
/// 1
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePresentationChallengeResponse {
    /// Value used for the challenge.
    #[prost(string, tag="1")]
    pub nonce: ::prost::alloc::string::String,
    /// Used by browser to construct URL for QR generation.
    #[prost(string, tag="2")]
    pub endpoint: ::prost::alloc::string::String,
    /// Type(s) of the verifiable credential(s) being presented.
    #[prost(enumeration="CredentialType", repeated, tag="3")]
    pub credential_types: ::prost::alloc::vec::Vec<i32>,
}
/// \[Example\]
/// {
/// "presentation": "PRESENTATION", 
/// "nonce": "NonceLengthVariesByAlgo", 
/// "signature": "SIGNATURExBYTES=", 
/// "did": "did:method-name:zDIDMultibase58Encoded", 
/// "credential_type": [
/// 0, 
/// 1
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PresentVerifiableCredentialRequest {
    /// The presentation to be verified.
    #[prost(string, tag="1")]
    pub presentation: ::prost::alloc::string::String,
    /// Value used for the challenge.
    #[prost(string, tag="2")]
    pub nonce: ::prost::alloc::string::String,
    /// Signed value of the challenge by the wallet.
    #[prost(bytes="vec", tag="3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="4")]
    pub did: ::prost::alloc::string::String,
    /// Type(s) of the verifiable credential(s) being presented.
    #[prost(enumeration="CredentialType", repeated, tag="5")]
    pub credential_type: ::prost::alloc::vec::Vec<i32>,
}
/// \[Example\]
/// {} 
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PresentVerifiableCredentialResponse {
}
/// \[Example\]
/// {
/// "nonce": "NonceLengthVariesByAlgo"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForCompletionRequest {
    /// Value used for the challenge.
    #[prost(string, tag="1")]
    pub nonce: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded", 
/// "data": "DATA"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForCompletionResponse {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// JSON representation of the credential.
    #[prost(string, tag="2")]
    pub data: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "credential_type": 1, 
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "update_type": 1
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVerifiableCredentialStatusRequest {
    /// Type of the verifiable credential being revoked.
    #[prost(enumeration="CredentialType", tag="1")]
    pub credential_type: i32,
    /// W3C Decentralized Identifier (DID) of the credential.
    #[prost(string, tag="2")]
    pub did: ::prost::alloc::string::String,
    /// Type of the revocation.
    #[prost(enumeration="UpdateVerifiableCredentialStatusType", tag="3")]
    pub update_type: i32,
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVerifiableCredentialStatusResponse {
}
/// Type of Credential.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CredentialType {
    /// Credential Type Unspecified.
    Unspecified = 0,
    /// Credential Type Permanent Resident Card.
    PermanentResidentCard = 1,
    /// Credential Type Bank Card.
    BankCard = 2,
    /// Credential Type Bank Account.
    BankAccount = 3,
}
impl CredentialType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CredentialType::Unspecified => "CREDENTIAL_TYPE_UNSPECIFIED",
            CredentialType::PermanentResidentCard => "CREDENTIAL_TYPE_PERMANENT_RESIDENT_CARD",
            CredentialType::BankCard => "CREDENTIAL_TYPE_BANK_CARD",
            CredentialType::BankAccount => "CREDENTIAL_TYPE_BANK_ACCOUNT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CREDENTIAL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CREDENTIAL_TYPE_PERMANENT_RESIDENT_CARD" => Some(Self::PermanentResidentCard),
            "CREDENTIAL_TYPE_BANK_CARD" => Some(Self::BankCard),
            "CREDENTIAL_TYPE_BANK_ACCOUNT" => Some(Self::BankAccount),
            _ => None,
        }
    }
}
/// Type of Update to Verifiable Credential Status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateVerifiableCredentialStatusType {
    /// Update Type to VC Status is Unspecified.
    Unspecified = 0,
    /// Update Type to VC Status is Revocation.
    Revocation = 1,
    /// Update Type to VC Status is Suspension.
    Suspension = 2,
}
impl UpdateVerifiableCredentialStatusType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpdateVerifiableCredentialStatusType::Unspecified => "UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_UNSPECIFIED",
            UpdateVerifiableCredentialStatusType::Revocation => "UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_REVOCATION",
            UpdateVerifiableCredentialStatusType::Suspension => "UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_SUSPENSION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_REVOCATION" => Some(Self::Revocation),
            "UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_SUSPENSION" => Some(Self::Suspension),
            _ => None,
        }
    }
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
