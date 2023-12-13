// @generated
// THIS PROTO FILE IS INTENDED FOR INTERNAL KNOX USE ONLY.

/// \[Example\]
/// {
/// "authorities": [
/// {
/// "key": "AuthorityKey",
/// "value": {
/// "verifier": "VerifierLengthVariesByAlgo",
/// "url": "AUTHORITY_URL"
/// }
/// },
/// {
/// "key": "AuthorityKey2",
/// "value": {
/// "verifier": "VerifierLengthVariesByAlgo",
/// "url": "AUTHORITY2_URL"
/// }
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAssetAuthoritiesRequest {
    /// List of Authorities and Assets to Set.
    #[prost(map="string, message", tag="1")]
    pub authorities: ::std::collections::HashMap<::prost::alloc::string::String, super::common::AssetAuthority>,
}
/// \[Example\]
/// {}
///
/// Empty body
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAssetAuthoritiesResponse {
}
include!("notary.tonic.rs");
// @@protoc_insertion_point(module)
