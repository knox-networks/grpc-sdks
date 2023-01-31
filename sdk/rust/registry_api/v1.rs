// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// A set of data containing mechanisms to verify the DID and discover related services. 
    #[prost(string, tag="2")]
    pub document: ::prost::alloc::string::String,
}
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// A set of data containing mechanisms to verify the DID and discover related services. 
    #[prost(string, tag="2")]
    pub document: ::prost::alloc::string::String,
}
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
}
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeResponse {
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
