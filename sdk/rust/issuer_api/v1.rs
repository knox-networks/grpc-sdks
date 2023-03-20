// @generated
/// A request to issue new digital banknotes.
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
    /// sum must match corresponding amount.amount
    #[prost(message, optional, tag="4")]
    pub distribution: ::core::option::Option<super::super::common::Distribution>,
}
/// A single authorized digital banknote as part of a stream response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IssueResponse {
    /// Stream of a digital banknote.
    #[prost(bytes="vec", tag="1")]
    pub promissory_file: ::prost::alloc::vec::Vec<u8>,
}
/// A single digital banknote to be authorized as part of a stream request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeRequest {
    /// Stream of a digital banknote to authorize.
    #[prost(bytes="vec", tag="1")]
    pub promissory_file: ::prost::alloc::vec::Vec<u8>,
}
/// A single digital banknote with an updated authorized signature as part of a stream response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeResponse {
    /// Stream of a digital banknote with authorized signature.
    #[prost(bytes="vec", tag="1")]
    pub promissory_file: ::prost::alloc::vec::Vec<u8>,
}
/// Authentication challenge nonce.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateRequest {
    /// Verifier of the requester.
    #[prost(message, optional, tag="1")]
    pub verifier: ::core::option::Option<super::super::common::DynamicVerifier>,
}
/// Responds to request with authentication challenge nonce.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateResponse {
    /// Random unique string to be signed by the authorized key.
    #[prost(bytes="vec", tag="1")]
    pub challenge: ::prost::alloc::vec::Vec<u8>,
}
/// Gets roles set in the Issuer for configuring access.
///
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRolesRequest {
}
/// Returns roles set in the Issuer for configuring access.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRolesResponse {
    /// Key is the verifier and the Value is <authorized/unauthorized>: <currency limits>.
    #[prost(map="string, string", tag="1")]
    pub roles: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Sets roles in the Issuer for configuring access.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoleRequest {
    /// Configured roles set in the Issuer.
    #[prost(message, optional, tag="1")]
    pub role: ::core::option::Option<super::super::common::Role>,
}
/// Responds to the request to set roles in the Issuer for configuring access.
///
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRoleResponse {
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
