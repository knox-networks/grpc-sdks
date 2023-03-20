// @generated
/// *
/// IdentityRequest - Subject and query parameters for identity lookup
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityRequest {
    /// Verifier of the subject for the identity request
    #[prost(bytes="vec", tag="1")]
    pub subject: ::prost::alloc::vec::Vec<u8>,
    /// Query string, e.g. ?fields=\[name,email\]
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
}
/// *
/// IdentityResponse - Subject and media data with mime type for encoding
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityResponse {
    /// Verifier of the subject for the identity response
    #[prost(bytes="vec", tag="1")]
    pub subject: ::prost::alloc::vec::Vec<u8>,
    /// media response for identity response
    #[prost(message, repeated, tag="2")]
    pub media: ::prost::alloc::vec::Vec<super::common::Media>,
}
/// *
/// ConnectionIdentity - Identity of a connection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionIdentity {
}
// @@protoc_insertion_point(module)
