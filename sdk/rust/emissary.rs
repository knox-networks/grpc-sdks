// @generated
/// *
/// EmissaryConnectionIdentityRequest - Identity of a connection
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmissaryConnectionIdentityRequest {
    #[prost(message, optional, tag="1")]
    pub verifier: ::core::option::Option<super::common::DynamicVerifier>,
    #[prost(enumeration="super::common::ServiceType", tag="2")]
    pub service: i32,
}
include!("emissary.tonic.rs");
// @@protoc_insertion_point(module)
