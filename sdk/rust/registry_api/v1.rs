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
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "resolutionOption": {
/// "accept": 0
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// W3C metadata structure for did resolution
    #[prost(message, optional, tag="2")]
    pub resolution_option: ::core::option::Option<ResolutionOption>,
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "resolutionOption": {
/// "accept": 0
/// }
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
    /// Resolution Option.
    #[prost(enumeration="ResolutionOptionAccept", tag="1")]
    pub accept: i32,
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
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DidDocumentMetadata {
    /// Timestamp representing the DID document creation time.
    #[prost(message, optional, tag="1")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    /// Timestamp representing the DID document last update time.
    #[prost(message, optional, tag="2")]
    pub updated: ::core::option::Option<::prost_types::Timestamp>,
}
/// \[Example\]
/// {
/// "didResolutionMetadata": {
/// "contentType": "MEDIA_TYPE",
/// "duration": 1000,
/// "didUrl": {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "methodName": "EXAMPLE_METHOD_NAME",
/// "methodSpecificId": "METHOD_SPECIFIC_IDENTIFIER"
/// },
/// "error": "OPTIONAL_ERROR_MESSAGE"
/// },
/// "didDocument": {},
/// "didDocumentMetadata": { 
/// "created": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }, 
/// "updated": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveResponse {
    /// Resolution Metadata object, with defined timestamps and content type.
    #[prost(message, optional, tag="1")]
    pub did_resolution_metadata: ::core::option::Option<ResolutionMetadata>,
    /// A set of data containing mechanisms to verify the DID and discover related services.
    #[prost(message, optional, tag="2")]
    pub did_document: ::core::option::Option<::prost_types::Struct>,
    /// This structure contains metadata about the DID document contained in the didDocument property
    #[prost(message, optional, tag="3")]
    pub did_document_metadata: ::core::option::Option<DidDocumentMetadata>,
}
/// \[Example\]
/// {
/// "didDocumentStream": "DID_DOCUMENT_STREAM",
/// "didResolutionMetadata": {
/// "contentType": "MEDIA_TYPE",
/// "duration": 1000,
/// "didUrl": {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "methodName": "EXAMPLE_METHOD_NAME",
/// "methodSpecificId": "METHOD_SPECIFIC_IDENTIFIER"
/// },
/// "error": "OPTIONAL_ERROR_MESSAGE"
/// },
/// "didDocumentMetadata": { 
/// "created": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }, 
/// "updated": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveRepresentationResponse {
    /// A set of data containing mechanisms to verify the DID and discover related services.
    #[prost(string, tag="1")]
    pub did_document_stream: ::prost::alloc::string::String,
    /// Resolution Metadata object, with defined timestamps and content type.
    #[prost(message, optional, tag="2")]
    pub did_resolution_metadata: ::core::option::Option<ResolutionMetadata>,
    /// This structure contains metadata about the DID document contained in the didDocument property
    #[prost(message, optional, tag="3")]
    pub did_document_metadata: ::core::option::Option<DidDocumentMetadata>,
}
/// \[Example\]
/// {
/// "contentType": "MEDIA_TYPE",
/// "duration": 1000,
/// "didUrl": {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "methodName": "EXAMPLE_METHOD_NAME",
/// "methodSpecificId": "METHOD_SPECIFIC_IDENTIFIER"
/// },
/// "error": "OPTIONAL_ERROR_MESSAGE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolutionMetadata {
    /// Media type of the W3C DID Document.
    #[prost(string, optional, tag="1")]
    pub content_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Duration of the Resolution.
    #[prost(int64, optional, tag="2")]
    pub duration: ::core::option::Option<i64>,
    /// Resolution DID URL.
    #[prost(message, optional, tag="3")]
    pub did_url: ::core::option::Option<ResolutionMetadataDidUrl>,
    /// Error Message.
    #[prost(string, optional, tag="4")]
    pub error: ::core::option::Option<::prost::alloc::string::String>,
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "methodName": "EXAMPLE_METHOD_NAME",
/// "methodSpecificId": "METHOD_SPECIFIC_IDENTIFIER"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolutionMetadataDidUrl {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// W3C Decentralized Scheme.
    #[prost(string, tag="2")]
    pub method_name: ::prost::alloc::string::String,
    /// Method specific identifier.
    #[prost(string, tag="3")]
    pub method_specific_id: ::prost::alloc::string::String,
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
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "document": "DOCUMENT"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// Document contains signature to proof the ownership of this DID.
    #[prost(string, tag="2")]
    pub document: ::prost::alloc::string::String,
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
    /// Country, e.g. United States
    #[prost(string, tag="1")]
    pub country: ::prost::alloc::string::String,
    /// City, e.g. Anytown
    #[prost(string, tag="2")]
    pub city: ::prost::alloc::string::String,
    /// State, e.g. AL
    #[prost(string, tag="3")]
    pub state: ::prost::alloc::string::String,
    /// Postal Code, e.g. 10101
    #[prost(string, tag="4")]
    pub postal_code: ::prost::alloc::string::String,
    /// Street Address, e.g. 123 Sesame Street
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
    /// DID of Credential Issuer.
    #[prost(string, tag="1")]
    pub issuer_did: ::prost::alloc::string::String,
    /// Name of Credential Issuer.
    #[prost(string, tag="2")]
    pub issuer_name: ::prost::alloc::string::String,
    /// Email of Credential Issuer.
    #[prost(string, tag="3")]
    pub email: ::prost::alloc::string::String,
    /// Website of Credential Issuer.
    #[prost(string, tag="4")]
    pub website: ::prost::alloc::string::String,
    /// Postal Address of Credential Issuer.
    #[prost(message, optional, tag="5")]
    pub postal_address: ::core::option::Option<PostalAddress>,
    /// Status of Credential Issuer, e.g. Active, Suspended.
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
    /// Credential Issuer.
    #[prost(message, optional, tag="1")]
    pub credential_issuer: ::core::option::Option<CredentialIssuer>,
}
/// \[Example\]
/// {
/// "ok": true
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCredentialIssuerResponse {
    #[prost(bool, tag="1")]
    pub ok: bool,
}
/// \[Example\]
/// { 
/// "issuer_did": "did:method-name:zDIDCredentialIssuerMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCredentialIssuerRequest {
    /// Credential Issuer DID.
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
    /// Returned Credential Issuer.
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
    /// Updated Credential Issuer.
    #[prost(message, optional, tag="1")]
    pub credential_issuer: ::core::option::Option<CredentialIssuer>,
}
/// \[Example\]
/// {
/// "ok": true
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCredentialIssuerResponse {
    #[prost(bool, tag="1")]
    pub ok: bool,
}
/// \[Example\]
/// { 
/// "credential_id": "<https://issuer.oidp.uscis.gov/credentials/83627465",>
/// "credential_issuer_id": "did:method-name:zDIDCredentialIssuerMultibase58Encoded",
/// "status_purpose": 0,
/// "credential_type": "BankAccount",
/// "status_list_index": 0,
/// "status_list_credential_id": "<https://reg.YOUR_SANDBOX_URL.knoxnetworks.io/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",>
/// "status": true
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusListEntry {
    /// Credential Unique ID, e.g. "<https://issuer.oidp.uscis.gov/credentials/83627465">
    #[prost(string, tag="1")]
    pub credential_id: ::prost::alloc::string::String,
    /// Credential Issuer DID.
    #[prost(string, tag="2")]
    pub credential_issuer_id: ::prost::alloc::string::String,
    /// Status Purpose, e.g. revocation.
    #[prost(enumeration="StatusPurpose", tag="3")]
    pub status_purpose: i32,
    /// Credential Type.
    #[prost(string, tag="4")]
    pub credential_type: ::prost::alloc::string::String,
    /// Status Index of the VC.
    #[prost(int64, tag="5")]
    pub status_list_index: i64,
    /// Credential Status List ID, e.g. "<https://reg.YOUR_SANDBOX_URL.knoxnetworks.io/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx">
    #[prost(string, tag="6")]
    pub status_list_credential_id: ::prost::alloc::string::String,
    /// Status.
    #[prost(bool, tag="7")]
    pub status: bool,
}
/// \[Example\]
/// { 
/// "credential_id": "<https://issuer.oidp.uscis.gov/credentials/83627465",>
/// "credential_issuer_id": "did:method-name:zDIDCredentialIssuerMultibase58Encoded",
/// "status_purpose": 0,
/// "credential_type": "BankAccount",
/// "status": true
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStatusListEntryRequest {
    /// Credential Unique ID, e.g. "<https://issuer.oidp.uscis.gov/credentials/83627465">
    #[prost(string, tag="1")]
    pub credential_id: ::prost::alloc::string::String,
    /// Credential Issuer DID.
    #[prost(string, tag="2")]
    pub credential_issuer_id: ::prost::alloc::string::String,
    /// Status Purpose, e.g. revocation.
    #[prost(enumeration="StatusPurpose", tag="3")]
    pub status_purpose: i32,
    /// Credential Type, e.g. BankAccount, Citizenship.
    #[prost(string, tag="4")]
    pub credential_type: ::prost::alloc::string::String,
    /// Status.
    #[prost(bool, tag="5")]
    pub status: bool,
}
/// \[Example\]
/// { 
/// "status_list_entry": { 
/// "credential_id": "<https://issuer.oidp.uscis.gov/credentials/83627465",>
/// "credential_issuer_id": "did:method-name:zDIDCredentialIssuerMultibase58Encoded",
/// "status_purpose": 0,
/// "credential_type": "BankAccount",
/// "status_list_index": 0,
/// "status_list_credential_id": "<https://reg.YOUR_SANDBOX_URL.knoxnetworks.io/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",>
/// "status": true
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStatusListEntryResponse {
    /// Created StatusListEntry.
    #[prost(message, optional, tag="1")]
    pub status_list_entry: ::core::option::Option<StatusListEntry>,
}
/// \[Example\]
/// { 
/// "status_list_credential_id": "<https://reg.YOUR_SANDBOX_URL.knoxnetworks.io/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx">
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusListCredentialRequest {
    /// Credential Status List ID, e.g. "<https://reg.YOUR_SANDBOX_URL.knoxnetworks.io/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx">
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
    /// Unique ID of StatusListCredential.
    #[prost(string, tag="1")]
    pub status_list_credential: ::prost::alloc::string::String,
}
/// \[Example\]
/// { 
/// "credential_id": "<https://issuer.oidp.uscis.gov/credentials/83627465",>
/// "credential_issuer_id": "did:method-name:zDIDCredentialIssuerMultibase58Encoded",
/// "status_purpose": 0,
/// "credential_type": "BankAccount",
/// "status": true
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStatusListEntryRequest {
    /// Credential Unique ID, e.g. "<https://issuer.oidp.uscis.gov/credentials/83627465">
    #[prost(string, tag="1")]
    pub credential_id: ::prost::alloc::string::String,
    /// Credential Issuer DID.
    #[prost(string, tag="2")]
    pub credential_issuer_id: ::prost::alloc::string::String,
    /// Status Purpose, e.g. revocation.
    #[prost(enumeration="StatusPurpose", tag="3")]
    pub status_purpose: i32,
    /// Credential Type, e.g. BankAccount, Citizenship.
    #[prost(string, tag="4")]
    pub credential_type: ::prost::alloc::string::String,
    /// Status.
    #[prost(bool, tag="5")]
    pub status: bool,
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStatusListEntryResponse {
}
/// Resolution Option Accept Method.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResolutionOptionAccept {
    /// Resolution Option Accept Method Unspecified.
    Unspecified = 0,
    /// Resolution Option Accept Method JSON-LD.
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
/// Issuer Status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IssuerStatus {
    /// Status of Issuer Unspecified.
    Unspecified = 0,
    /// Status of Issuer Active.
    Active = 1,
    /// Status of Issuer Suspended.
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
/// Purpose of Status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StatusPurpose {
    /// Status Unspecified.
    Unspecified = 0,
    /// Status Revocation.
    Revocation = 1,
    /// Status Suspension.
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
