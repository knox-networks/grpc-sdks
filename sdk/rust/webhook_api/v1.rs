// @generated
/// \[Example\]
/// {
/// "name": "NAME", 
/// "did": "did:method-name:zDIDMultibase58Encoded", 
/// "created": "2006-01-02T15:04:05Z"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWalletPayload {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub did: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub created: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "wallet_id": "zWalletIDMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromissoryReceivedPayload {
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "message": "MESSAGE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingPayload {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "event_type": 0,
/// "signature": "SignatureLenVariesByAlgorithm",
/// "event_id": "EVENT_ID",
/// "delivery_date": "2007-01-02T15:04:05Z",
/// "owner_id": "eyJwtHeader.eyJwtPayload.Secret", 
/// "payload": {
/// "wallet_created_payload": {
/// "name": "NAME", 
/// "did": "did:method-name:zDIDMultibase58Encoded", 
/// "created": "2006-01-02T15:04:05Z"
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(enumeration="EventType", tag="1")]
    pub event_type: i32,
    #[prost(string, tag="2")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub event_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub delivery_date: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub owner_id: ::prost::alloc::string::String,
    #[prost(oneof="event::Payload", tags="6, 7, 8")]
    pub payload: ::core::option::Option<event::Payload>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag="6")]
        WalletCreatedPayload(super::CreateWalletPayload),
        #[prost(message, tag="7")]
        PromissoryReceivedPayload(super::PromissoryReceivedPayload),
        #[prost(message, tag="8")]
        PingPayload(super::PingPayload),
    }
}
/// \[Example\]
/// {
/// "api_key": "API_KEY",
/// "name": "NAME",
/// "supported_events": [
/// 0, 1, 2
/// ],
/// "webhook_url": "WEBHOOK_URL",
/// "delivery_method": 0
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWebhookRequest {
    /// JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="EventType", repeated, tag="3")]
    pub supported_events: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag="4")]
    pub webhook_url: ::prost::alloc::string::String,
    #[prost(enumeration="DeliveryMethodType", tag="5")]
    pub delivery_method: i32,
}
/// \[Example\]
/// {
/// "webhook_public_key": "zWebhookMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWebhookResponse {
    #[prost(string, tag="1")]
    pub webhook_public_key: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "api_key": "API_KEY",
/// "page": 0,
/// "page_size": 10000
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebhooksRequest {
    /// JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub page: i32,
    #[prost(int32, tag="3")]
    pub page_size: i32,
}
/// \[Example\]
/// {
/// "webhooks": [
/// {
/// "id": "ID1", 
/// "owner_id": "eyJwtHeader.eyJwtPayload.Secret1", 
/// "name": "NAME1", 
/// "webhook_url": "WEBHOOK_URL1", 
/// "supported_events": [
/// 0, 
/// 1
/// ]
/// }, 
/// {
/// "id": "ID2", 
/// "owner_id": "eyJwtHeader.eyJwtPayload.Secret2", 
/// "name": "NAME2", 
/// "webhook_url": "WEBHOOK_URL2", 
/// "supported_events": [
/// 0, 
/// 1
/// ]
/// }
/// ], 
/// "total_owned_webhooks": 2
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebhooksResponse {
    #[prost(message, repeated, tag="1")]
    pub webhooks: ::prost::alloc::vec::Vec<list_webhooks_response::Webhook>,
    #[prost(uint32, tag="2")]
    pub total_owned_webhooks: u32,
}
/// Nested message and enum types in `ListWebhooksResponse`.
pub mod list_webhooks_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Webhook {
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub owner_id: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="4")]
        pub webhook_url: ::prost::alloc::string::String,
        #[prost(enumeration="super::EventType", repeated, tag="5")]
        pub supported_events: ::prost::alloc::vec::Vec<i32>,
        #[prost(enumeration="super::DeliveryMethodType", tag="6")]
        pub delivery_method: i32,
    }
}
/// \[Example\]
/// {
/// "api_key": "API_KEY",
/// "webhook_id": "WEBHOOK_ID"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWebhookRequest {
    /// JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub webhook_id: ::prost::alloc::string::String,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWebhookResponse {
}
/// \[Example\]
/// {
/// "event": {
/// "event_type": 0,
/// "signature": "SignatureLenVariesByAlgorithm",
/// "event_id": "EVENT_ID",
/// "delivery_date": "2007-01-02T15:04:05Z",
/// "owner_id": "eyJwtHeader.eyJwtPayload.Secret", 
/// "payload": {
/// "wallet_created_payload": {
/// "name": "NAME", 
/// "did": "did:method-name:zDIDMultibase58Encoded", 
/// "created": "2006-01-02T15:04:05Z"
/// }
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRequest {
    #[prost(message, optional, tag="1")]
    pub event: ::core::option::Option<Event>,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventResponse {
}
/// \[Example\]
/// {
/// "api_key": "API_KEY",
/// "page": 0,
/// "page_size": 10000
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsRequest {
    /// JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub page: i32,
    #[prost(int32, tag="3")]
    pub page_size: i32,
}
/// \[Example\]
/// {
/// "events": [
/// {
/// "event_type": 0,
/// "signature": "SignatureLenVariesByAlgorithm",
/// "event_id": "EVENT_ID1",
/// "delivery_date": "2007-01-02T15:04:05Z",
/// "owner_id": "eyJwtHeader.eyJwtPayload.Secret1", 
/// "payload": {
/// "wallet_created_payload": {
/// "name": "NAME", 
/// "did": "did:method-name:zDIDMultibase58Encoded", 
/// "created": "2006-01-02T15:04:05Z"
/// }
/// }
/// },
/// {
/// "event_type": 0,
/// "signature": "SignatureLenVariesByAlgorithm",
/// "event_id": "EVENT_ID2",
/// "delivery_date": "2007-01-02T15:04:05Z",
/// "owner_id": "eyJwtHeader.eyJwtPayload.Secret2", 
/// "payload": {
/// "wallet_created_payload": {
/// "name": "NAME", 
/// "did": "did:method-name:zDIDMultibase58Encoded", 
/// "created": "2006-01-02T15:04:05Z"
/// }
/// }
/// }
/// ],
/// "total_owned_events": 2
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsResponse {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    #[prost(uint32, tag="2")]
    pub total_owned_events: u32,
}
/// \[Example\]
/// {
/// "api_key": "API_KEY",
/// "event_id": "EVENT_ID",
/// "webhook_id": "WEBHOOK_ID"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryEventRequest {
    /// JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub event_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub webhook_id: ::prost::alloc::string::String,
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryEventResponse {
}
/// \[Example\]
/// {
/// "api_key": "API_KEY",
/// "message": "MESSAGE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
    /// JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "message": "MESSAGE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventType {
    Unspecified = 0,
    WalletCreated = 1,
    PromissoryReceived = 2,
    Ping = 3,
}
impl EventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventType::Unspecified => "EVENT_TYPE_UNSPECIFIED",
            EventType::WalletCreated => "EVENT_TYPE_WALLET_CREATED",
            EventType::PromissoryReceived => "EVENT_TYPE_PROMISSORY_RECEIVED",
            EventType::Ping => "EVENT_TYPE_PING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EVENT_TYPE_WALLET_CREATED" => Some(Self::WalletCreated),
            "EVENT_TYPE_PROMISSORY_RECEIVED" => Some(Self::PromissoryReceived),
            "EVENT_TYPE_PING" => Some(Self::Ping),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeliveryMethodType {
    Unspecified = 0,
    Http = 1,
    Grpc = 2,
}
impl DeliveryMethodType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeliveryMethodType::Unspecified => "DELIVERY_METHOD_TYPE_UNSPECIFIED",
            DeliveryMethodType::Http => "DELIVERY_METHOD_TYPE_HTTP",
            DeliveryMethodType::Grpc => "DELIVERY_METHOD_TYPE_GRPC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DELIVERY_METHOD_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "DELIVERY_METHOD_TYPE_HTTP" => Some(Self::Http),
            "DELIVERY_METHOD_TYPE_GRPC" => Some(Self::Grpc),
            _ => None,
        }
    }
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
