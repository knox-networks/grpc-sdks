// @generated
/// For more details, refer to <https://www.w3.org/TR/vc-data-integrity/>
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIssuanceChallengeRequest {
    /// Type of the verifiable credential being requested for issuance.
    #[prost(enumeration="CredentialType", tag="1")]
    pub credential_type: i32,
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="2")]
    pub did: ::prost::alloc::string::String,
}
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueVerifiableCredentialResponse {
    /// Type of the verifiable credential being issued.
    #[prost(string, tag="1")]
    pub credential: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePresentationChallengeRequest {
    /// Type(s) of the verifiable credential(s) being presented.
    #[prost(enumeration="CredentialType", repeated, tag="1")]
    pub credential_types: ::prost::alloc::vec::Vec<i32>,
}
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
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PresentVerifiableCredentialResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForCompletionRequest {
    /// Value used for the challenge.
    #[prost(string, tag="1")]
    pub nonce: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForCompletionResponse {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// JSON representation of the credential.
    #[prost(string, tag="2")]
    pub data: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CredentialType {
    Unspecified = 0,
    PermanentResidentCard = 1,
    BankCard = 2,
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
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
