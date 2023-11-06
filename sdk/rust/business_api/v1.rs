// @generated
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRequest {
}
/// \[Example\]
/// {
/// "list": [
/// "item1",
/// "item2",
/// "item3"
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResponse {
    /// List.
    #[prost(string, repeated, tag="1")]
    pub list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearRequest {
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearResponse {
}
/// \[Example\]
/// {
/// "payload": {
/// "payload_type": 1,
/// "data": "xDATAxBYTES="
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPayloadRequest {
    /// Payload
    #[prost(message, optional, tag="1")]
    pub payload: ::core::option::Option<super::super::common::Payload>,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPayloadResponse {
}
/// \[Example\]
/// {
/// "owner": "zOwnerPublicKey",
/// "memo": "SAMPLE MEMO",
/// "commitments": [
/// {
/// "sender": "zSenderPublicKey",
/// "recipient": "zRecipientPublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// },
/// {
/// "sender": "zSenderPublicKey2",
/// "recipient": "zRecipientPublicKey2",
/// "amount": {
/// "currency_code":"USD",
/// "amount":500,
/// "decimals":2
/// }
/// }
/// ],
/// "conditions": [
/// {
/// "condition": {
/// "hash": {
/// "presenter": "PresenterPublicKey",
/// "hash": "13550350a8681c84"
/// }
/// }
/// },
/// {
/// "condition": {
/// "hash": {
/// "presenter": "PresenterPublicKey",
/// "hash": "13550350a8681c84"
/// }
/// }
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendNewTransactionRequest {
    /// Owner Public Key.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// Transaction Memo.
    #[prost(string, tag="2")]
    pub memo: ::prost::alloc::string::String,
    /// Commitments.
    #[prost(message, repeated, tag="3")]
    pub commitments: ::prost::alloc::vec::Vec<super::super::common::Commitment>,
    /// Conditions.
    #[prost(message, repeated, tag="4")]
    pub conditions: ::prost::alloc::vec::Vec<super::super::common::Condition>,
}
/// \[Example\]
/// {
/// "contract_id": "a18f7224-a7c1-41f8-b14f-75e920b8ea81"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendNewTransactionResponse {
    /// Contract ID.
    #[prost(string, tag="1")]
    pub contract_id: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "owner": "zOwnerPublicKey",
/// "contract_id": "a18f7224-a7c1-41f8-b14f-75e920b8ea81",
/// "secret": "secret",
/// "algorithm": 0
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendProposalAckedRequest {
    /// Owner Public Key.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// Contract ID.
    #[prost(string, tag="2")]
    pub contract_id: ::prost::alloc::string::String,
    /// Base64 Encoding of Secret bytes.
    #[prost(string, tag="3")]
    pub secret: ::prost::alloc::string::String,
    /// Hash Algorithm.
    #[prost(enumeration="super::super::common::HashAlgorithm", tag="4")]
    pub algorithm: i32,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendProposalAckedResponse {
}
/// \[Example\]
/// {
/// "owner": "zOwnerPublicKey",
/// "contract_id": "a18f7224-a7c1-41f8-b14f-75e920b8ea81"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendProposalNackedRequest {
    /// Owner Public Key.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// Contract ID.
    #[prost(string, tag="2")]
    pub contract_id: ::prost::alloc::string::String,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendProposalNackedResponse {
}
/// \[Example\]
/// {
/// "owner": "zOwnerPublicKey",
/// "contract_id": "a18f7224-a7c1-41f8-b14f-75e920b8ea81",
/// "secret": "secret"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendSecretRevealedRequest {
    /// Owner Public Key.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// Contract ID.
    #[prost(string, tag="2")]
    pub contract_id: ::prost::alloc::string::String,
    /// Base64 Encoding of Secret bytes.
    #[prost(string, tag="3")]
    pub secret: ::prost::alloc::string::String,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendSecretRevealedResponse {
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
