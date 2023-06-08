// @generated
// THIS PROTO FILE IS INTENDED FOR INTERNAL KNOX USE ONLY.

// IdentityRequest - Subject and query parameters for identity lookup

/// \[Example\]
/// {
/// "subject": "ByteLengthSubjectVaries=", 
/// "query": "?fields=\[name,email\]" 
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityRequest {
    /// Verifier of the subject for the identity request
    #[prost(bytes="vec", tag="1")]
    pub subject: ::prost::alloc::vec::Vec<u8>,
    /// Query string, e.g. ?fields=\[name,email\]
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
}
// IdentityResponse - Subject and media data with mime type for encoding

/// \[Example\]
/// {
/// "subject": "ByteLengthSubjectVaries=", 
/// "media": [
/// {
/// "mime_type": "MIME_TYPE", 
/// "data": "ByteLengthDataVaries="
/// }, 
/// {
/// "mime_type": "MIME_TYPE", 
/// "data": "ByteLengthDataVaries="
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityResponse {
    /// Verifier of the subject for the identity response
    #[prost(bytes="vec", tag="1")]
    pub subject: ::prost::alloc::vec::Vec<u8>,
    /// media response for identity response
    #[prost(message, repeated, tag="2")]
    pub media: ::prost::alloc::vec::Vec<super::common::Media>,
}
// ConnectionIdentity - Identity of a connection

/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionIdentity {
}
// @@protoc_insertion_point(module)
