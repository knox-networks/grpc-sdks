// @generated
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "document": "DOCUMENT"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// A set of data containing mechanisms to verify the DID and discover related services. 
    #[prost(string, tag="2")]
    pub document: ::prost::alloc::string::String,
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResponse {
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveRepresentationRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// W3C metadata structure for did resolution
    #[prost(message, optional, tag="2")]
    pub resolution_option: ::core::option::Option<ResolutionOption>,
}
/// \[Example\]
/// {
/// "accept": 0
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolutionOption {
    #[prost(enumeration="ResolutionOptionAccept", tag="1")]
    pub accept: i32,
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "document": "DOCUMENT",
/// "metadata": { 
/// "created": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }, 
/// "updated": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }, 
/// "contentType": "MEDIA_TYPE"
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveResponse {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// A set of data containing mechanisms to verify the DID and discover related services. 
    #[prost(string, tag="2")]
    pub document: ::prost::alloc::string::String,
    /// Resolution Metadata object, with defined timestamps and content type.
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<ResolutionMetadata>,
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "document": "DOCUMENT",
/// "metadata": { 
/// "created": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }, 
/// "updated": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }, 
/// "contentType": "MEDIA_TYPE"
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveRepresentationResponse {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// A set of data containing mechanisms to verify the DID and discover related services.
    #[prost(string, tag="2")]
    pub document: ::prost::alloc::string::String,
    /// Resolution Metadata object, with defined timestamps and content type.
    #[prost(message, optional, tag="3")]
    pub metadata: ::core::option::Option<ResolutionMetadata>,
}
/// \[Example\]
/// {
/// "created": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }, 
/// "updated": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }, 
/// "contentType": "MEDIA_TYPE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolutionMetadata {
    /// Timestamp representing the DID document creation time.
    #[prost(message, optional, tag="1")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    /// Timestamp representing the DID document last update time.
    #[prost(message, optional, tag="2")]
    pub updated: ::core::option::Option<::prost_types::Timestamp>,
    /// Media type of the W3C DID Document. 
    #[prost(string, tag="3")]
    pub content_type: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "document": "DOCUMENT"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// A set of data containing mechanisms to verify the DID and discover related services. 
    #[prost(string, tag="2")]
    pub document: ::prost::alloc::string::String,
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeResponse {
}
/// \[Example\]
/// {
/// "country": "United States",
/// "city": "Anytown",
/// "state": "AL",
/// "postal_code": "10101",
/// "street": "123 Sesame Street"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostalAddress {
    #[prost(string, tag="1")]
    pub country: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub city: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub postal_code: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub street: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "issuer_did": "did:method-name:zDIDIssuerMultibase58Encoded",
/// "issuer_name": "ISSUER_NAME",
/// "email": "example@gmail.com",
/// "website": "google.com",
/// "postal_address": {
/// "country": "United States",
/// "city": "Anytown",
/// "state": "AL",
/// "postal_code": "10101",
/// "street": "123 Sesame Street"
/// },
/// "status": 0
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CredentialIssuer {
    #[prost(string, tag="1")]
    pub issuer_did: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub issuer_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub website: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub postal_address: ::core::option::Option<PostalAddress>,
    #[prost(enumeration="IssuerStatus", tag="6")]
    pub status: i32,
}
/// \[Example\]
/// { 
/// "credential_issuer": {
/// "issuer_did": "did:method-name:zDIDIssuerMultibase58Encoded",
/// "issuer_name": "ISSUER_NAME",
/// "email": "example@gmail.com",
/// "website": "google.com",
/// "postal_address": {
/// "country": "United States",
/// "city": "Anytown",
/// "state": "AL",
/// "postal_code": "10101",
/// "street": "123 Sesame Street"
/// },
/// "status": 0
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCredentialIssuerRequest {
    #[prost(message, optional, tag="1")]
    pub credential_issuer: ::core::option::Option<CredentialIssuer>,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCredentialIssuerResponse {
}
/// \[Example\]
/// { 
/// "issuer_did": "did:method-name:zDIDCredentialIssuerMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCredentialIssuerRequest {
    #[prost(string, tag="1")]
    pub issuer_did: ::prost::alloc::string::String,
}
/// \[Example\]
/// { 
/// "credential_issuer": {
/// "issuer_did": "did:method-name:zDIDIssuerMultibase58Encoded",
/// "issuer_name": "ISSUER_NAME",
/// "email": "example@gmail.com",
/// "website": "google.com",
/// "postal_address": {
/// "country": "United States",
/// "city": "Anytown",
/// "state": "AL",
/// "postal_code": "10101",
/// "street": "123 Sesame Street"
/// },
/// "status": 0
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCredentialIssuerResponse {
    #[prost(message, optional, tag="1")]
    pub credential_issuer: ::core::option::Option<CredentialIssuer>,
}
/// \[Example\]
/// { 
/// "credential_issuer": {
/// "issuer_did": "did:method-name:zDIDIssuerMultibase58Encoded",
/// "issuer_name": "ISSUER_NAME",
/// "email": "example@gmail.com",
/// "website": "google.com",
/// "postal_address": {
/// "country": "United States",
/// "city": "Anytown",
/// "state": "AL",
/// "postal_code": "10101",
/// "street": "123 Sesame Street"
/// },
/// "status": 0
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCredentialIssuerRequest {
    #[prost(message, optional, tag="1")]
    pub credential_issuer: ::core::option::Option<CredentialIssuer>,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCredentialIssuerResponse {
}
/// \[Example\]
/// { 
/// "credential_id": "CREDENTIAL_ID",
/// "credential_issuer_id": "did:method-name:zDIDCredentialIssuerMultibase58Encoded",
/// "status_purpose": 0,
/// "credential_type": "CREDENTIAL_TYPE",
/// "status_list_index": 0,
/// "status_list_credential_id": "STATUS_LIST_CREDENTIAL_ID",
/// "status": true
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusListEntry {
    #[prost(string, tag="1")]
    pub credential_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub credential_issuer_id: ::prost::alloc::string::String,
    #[prost(enumeration="StatusPurpose", tag="3")]
    pub status_purpose: i32,
    #[prost(string, tag="4")]
    pub credential_type: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub status_list_index: i64,
    #[prost(string, tag="6")]
    pub status_list_credential_id: ::prost::alloc::string::String,
    #[prost(bool, tag="7")]
    pub status: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStatusListEntryRequest {
    #[prost(string, tag="1")]
    pub credential_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub credential_issuer_id: ::prost::alloc::string::String,
    #[prost(enumeration="StatusPurpose", tag="3")]
    pub status_purpose: i32,
    #[prost(string, tag="4")]
    pub credential_type: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub status: bool,
}
/// \[Example\]
/// { 
/// "status_list_entry": { 
/// "credential_id": "CREDENTIAL_ID",
/// "credential_issuer_id": "did:method-name:zDIDCredentialIssuerMultibase58Encoded",
/// "status_purpose": 0,
/// "credential_type": "CREDENTIAL_TYPE",
/// "status_list_index": 0,
/// "status_list_credential_id": "STATUS_LIST_CREDENTIAL_ID",
/// "status": true
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStatusListEntryResponse {
    #[prost(message, optional, tag="1")]
    pub status_list_entry: ::core::option::Option<StatusListEntry>,
}
/// \[Example\]
/// { 
/// "status_list_credential_id": "STATUS_LIST_CREDENTIAL_ID"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusListCredentialRequest {
    /// this is the unique id of statusListCredential
    #[prost(string, tag="1")]
    pub status_list_credential_id: ::prost::alloc::string::String,
}
/// \[Example\]
/// { 
/// "status_list_credential": "STATUS_LIST_CREDENTIAL"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusListCredentialResponse {
    #[prost(string, tag="1")]
    pub status_list_credential: ::prost::alloc::string::String,
}
/// \[Example\]
/// { 
/// "credential_id": "CREDENTIAL_ID",
/// "credential_issuer_id": "did:method-name:zDIDCredentialIssuerMultibase58Encoded",
/// "status_purpose": 0,
/// "credential_type": "CREDENTIAL_TYPE",
/// "status": true
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStatusListEntryRequest {
    #[prost(string, tag="1")]
    pub credential_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub credential_issuer_id: ::prost::alloc::string::String,
    #[prost(enumeration="StatusPurpose", tag="3")]
    pub status_purpose: i32,
    #[prost(string, tag="4")]
    pub credential_type: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub status: bool,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStatusListEntryResponse {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResolutionOptionAccept {
    Unspecified = 0,
    JsonLd = 1,
}
impl ResolutionOptionAccept {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResolutionOptionAccept::Unspecified => "RESOLUTION_OPTION_ACCEPT_UNSPECIFIED",
            ResolutionOptionAccept::JsonLd => "RESOLUTION_OPTION_ACCEPT_JSON_LD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESOLUTION_OPTION_ACCEPT_UNSPECIFIED" => Some(Self::Unspecified),
            "RESOLUTION_OPTION_ACCEPT_JSON_LD" => Some(Self::JsonLd),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IssuerStatus {
    Unspecified = 0,
    Active = 1,
    Suspended = 2,
}
impl IssuerStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IssuerStatus::Unspecified => "ISSUER_STATUS_UNSPECIFIED",
            IssuerStatus::Active => "ISSUER_STATUS_ACTIVE",
            IssuerStatus::Suspended => "ISSUER_STATUS_SUSPENDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ISSUER_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "ISSUER_STATUS_ACTIVE" => Some(Self::Active),
            "ISSUER_STATUS_SUSPENDED" => Some(Self::Suspended),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StatusPurpose {
    Unspecified = 0,
    Revocation = 1,
    Suspension = 2,
}
impl StatusPurpose {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StatusPurpose::Unspecified => "STATUS_PURPOSE_UNSPECIFIED",
            StatusPurpose::Revocation => "STATUS_PURPOSE_REVOCATION",
            StatusPurpose::Suspension => "STATUS_PURPOSE_SUSPENSION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATUS_PURPOSE_UNSPECIFIED" => Some(Self::Unspecified),
            "STATUS_PURPOSE_REVOCATION" => Some(Self::Revocation),
            "STATUS_PURPOSE_SUSPENSION" => Some(Self::Suspension),
            _ => None,
        }
    }
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)