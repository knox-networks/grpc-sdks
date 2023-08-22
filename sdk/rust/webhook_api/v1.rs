// @generated
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

/// \[Example\]
/// {
/// "id": "ID1",
/// "owner_id": "eyJwtHeader.eyJwtPayload.Secret1",
/// "name": "NAME1",
/// "webhook_url": "WEBHOOK_URL1",
/// "supported_events": [
/// 0,
/// 1
/// ],
/// "delivery_method": 0
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Webhook {
    /// Webhook ID.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Owner ID.
    #[prost(string, tag="2")]
    pub owner_id: ::prost::alloc::string::String,
    /// Webhook Name.
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Webhook URL.
    #[prost(string, tag="4")]
    pub webhook_url: ::prost::alloc::string::String,
    /// List of supported Events.
    #[prost(enumeration="EventType", repeated, tag="5")]
    pub supported_events: ::prost::alloc::vec::Vec<i32>,
    /// Specification of Delivery method.
    #[prost(enumeration="DeliveryMethodType", tag="6")]
    pub delivery_method: i32,
}
/// \[Example\]
/// {
/// "name": "NAME",
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "created": "2006-01-02T15:04:05Z"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWalletPayload {
    /// Wallet name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Wallet DID.
    #[prost(string, tag="2")]
    pub did: ::prost::alloc::string::String,
    /// Timestamp when wallet was created.
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
    /// Wallet ID.
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
    /// Message payload
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
/// },
/// "created_date": "2006-01-02T15:04:05Z"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Type of event.
    #[prost(enumeration="EventType", tag="1")]
    pub event_type: i32,
    /// Event signature.
    #[prost(string, tag="2")]
    pub signature: ::prost::alloc::string::String,
    /// Event ID.
    #[prost(string, tag="3")]
    pub event_id: ::prost::alloc::string::String,
    /// Delivery date of the event.
    #[prost(string, tag="4")]
    pub delivery_date: ::prost::alloc::string::String,
    /// Owner ID
    #[prost(string, tag="5")]
    pub owner_id: ::prost::alloc::string::String,
    /// Timestamp when wallet was created.
    #[prost(string, tag="9")]
    pub created_date: ::prost::alloc::string::String,
    /// Event payload.
    #[prost(oneof="event::Payload", tags="6, 7, 8")]
    pub payload: ::core::option::Option<event::Payload>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    /// Event payload.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Event Payload for creation of wallet.
        #[prost(message, tag="6")]
        WalletCreatedPayload(super::CreateWalletPayload),
        /// Event Payload for receiving promissory.
        #[prost(message, tag="7")]
        PromissoryReceivedPayload(super::PromissoryReceivedPayload),
        /// Event Payload for pinging.
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
    /// API Key as a JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Webhook name.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// List of supported_events.
    #[prost(enumeration="EventType", repeated, tag="3")]
    pub supported_events: ::prost::alloc::vec::Vec<i32>,
    /// Specified webhook URL.
    #[prost(string, tag="4")]
    pub webhook_url: ::prost::alloc::string::String,
    /// Specification of delivery method.
    #[prost(enumeration="DeliveryMethodType", tag="5")]
    pub delivery_method: i32,
}
/// \[Example\]
/// {
/// "webhook_public_key": "zWebhookMultibase58Encoded",
/// "webhook": {
/// "id": "ID1",
/// "owner_id": "eyJwtHeader.eyJwtPayload.Secret1",
/// "name": "NAME1",
/// "webhook_url": "WEBHOOK_URL1",
/// "supported_events": [
/// 0,
/// 1
/// ],
/// "delivery_method": 0
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWebhookResponse {
    /// Webhook Public Key
    #[prost(string, tag="1")]
    pub webhook_public_key: ::prost::alloc::string::String,
    /// Webhook
    #[prost(message, optional, tag="2")]
    pub webhook: ::core::option::Option<Webhook>,
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
    /// API Key as a JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Page number for pagination.
    #[prost(int32, tag="2")]
    pub page: i32,
    /// Size of the page for pagination.
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
/// ],
/// "delivery_method": 0
/// },
/// {
/// "id": "ID2",
/// "owner_id": "eyJwtHeader.eyJwtPayload.Secret2",
/// "name": "NAME2",
/// "webhook_url": "WEBHOOK_URL2",
/// "supported_events": [
/// 0,
/// 1
/// ],
/// "delivery_method": 0
/// }
/// ],
/// "total_owned_webhooks": 2
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWebhooksResponse {
    /// List of webhooks.
    #[prost(message, repeated, tag="1")]
    pub webhooks: ::prost::alloc::vec::Vec<Webhook>,
    /// Total number of webhooks returned.
    #[prost(uint32, tag="2")]
    pub total_owned_webhooks: u32,
}
/// \[Example\]
/// {
/// "api_key": "API_KEY",
/// "webhook_id": "WEBHOOK_ID"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWebhookRequest {
    /// API Key, JWT.
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Webhook ID.
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
    /// Event.
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
/// "page_size": 10000,
/// "filters": {
/// "filters": [
/// {
/// "field": "FIELD1",
/// "operator": 0,
/// "value": {
/// "str_value": "STRINGVAL"
/// }
/// },
/// {
/// "field": "FIELD2",
/// "operator": 0,
/// "value": {
/// "int_value": 10
/// }
/// },
/// {
/// "field": "FIELD3",
/// "operator": 0,
/// "value": {
/// "bool_value": false
/// }
/// }
/// ],
/// "operator": 0
/// },
/// "sort_by": {
/// "items": [
/// {
/// "field": "FIELD1",
/// "direction": 0
/// },
/// {
/// "field": "FIELD2",
/// "direction": 0
/// }
/// ]
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsRequest {
    /// API Key, JWT.
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Page number for pagination.
    #[prost(int32, tag="2")]
    pub page: i32,
    /// Size of the page for pagination.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Potential filters to apply to the list of events.
    #[prost(message, optional, tag="4")]
    pub filters: ::core::option::Option<super::super::common::Filter>,
    /// Potential sort to apply to the list of events.
    #[prost(message, optional, tag="5")]
    pub sort_by: ::core::option::Option<super::super::common::SortBy>,
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
    /// List of Events returned.
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// Total number of owned events returned.
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
    /// API Key, JWT.
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Event ID.
    #[prost(string, tag="2")]
    pub event_id: ::prost::alloc::string::String,
    /// Webhook ID.
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
    /// API Key, JWT.
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Ping Message.
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
    /// Ping Message.
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "api_key": "API_KEY",
/// "page": 0,
/// "page_size": 10000,
/// "filters": {
/// "filters": [
/// {
/// "field": "FIELD1",
/// "operator": 0,
/// "value": {
/// "str_value": "STRINGVAL"
/// }
/// },
/// {
/// "field": "FIELD2",
/// "operator": 0,
/// "value": {
/// "int_value": 10
/// }
/// },
/// {
/// "field": "FIELD3",
/// "operator": 0,
/// "value": {
/// "bool_value": false
/// }
/// }
/// ],
/// "operator": 0
/// },
/// "sort_by": {
/// "items": [
/// {
/// "field": "FIELD1",
/// "direction": 0
/// },
/// {
/// "field": "FIELD2",
/// "direction": 0
/// }
/// ]
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeliveryHistoryRequest {
    /// API Key, JWT.
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Page number for pagination.
    #[prost(int32, tag="2")]
    pub page: i32,
    /// Size of the page for pagination.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Potential filters to apply to the list of delivery history.
    #[prost(message, optional, tag="4")]
    pub filters: ::core::option::Option<super::super::common::Filter>,
    /// Potential sort to apply to the list of delivery history.
    #[prost(message, optional, tag="5")]
    pub sort_by: ::core::option::Option<super::super::common::SortBy>,
}
/// \[Example\]
/// {
/// "delivery_history": [{
/// "id": "ID1",
/// "webhook_id": "WEBHOOK_ID1",
/// "event_id": "EVENT_ID1",
/// "owner_id": "OWNER_ID1",
/// "delivery_stage": 0
/// }],
/// "total_owned_delivery_history": 1
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeliveryHistoryResponse {
    /// List of Delivery History returned.
    #[prost(message, repeated, tag="1")]
    pub delivery_history: ::prost::alloc::vec::Vec<list_delivery_history_response::DeliveryHistory>,
    /// Total number of owned delivery history returned.
    #[prost(uint32, tag="2")]
    pub total_owned_delivery_history: u32,
}
/// Nested message and enum types in `ListDeliveryHistoryResponse`.
pub mod list_delivery_history_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeliveryHistory {
        /// Delivery History ID.
        #[prost(string, tag="1")]
        pub id: ::prost::alloc::string::String,
        /// Webhook ID.
        #[prost(string, tag="2")]
        pub webhook_id: ::prost::alloc::string::String,
        /// Event ID.
        #[prost(string, tag="3")]
        pub event_id: ::prost::alloc::string::String,
        /// Owner ID
        #[prost(string, tag="4")]
        pub owner_id: ::prost::alloc::string::String,
        /// Delivery Stage.
        #[prost(enumeration="super::DeliveryStage", tag="5")]
        pub delivery_stage: i32,
        /// Created Date. Rfc3339 format.
        #[prost(string, tag="6")]
        pub created: ::prost::alloc::string::String,
    }
}
/// Delivery Stage Options.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeliveryStage {
    /// Delivery Stage Unspecified.
    Unspecified = 0,
    /// Delivery Stage First (1) Retry.
    FirstRetry = 1,
    /// Delivery Stage Second (2) Retry.
    SecondRetry = 2,
    /// Delivery Stage Third (3) Retry.
    ThirdRetry = 3,
    /// Delivery Stage Fourth (4) Retry.
    FourthRetry = 4,
    /// Delivery Stage Fifth (5) Retry.
    FifthRetry = 5,
    /// Delivery Stage Failed.
    Failed = 6,
    /// Delivery Stage Success.
    Success = 7,
}
impl DeliveryStage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeliveryStage::Unspecified => "DELIVERY_STAGE_UNSPECIFIED",
            DeliveryStage::FirstRetry => "DELIVERY_STAGE_FIRST_RETRY",
            DeliveryStage::SecondRetry => "DELIVERY_STAGE_SECOND_RETRY",
            DeliveryStage::ThirdRetry => "DELIVERY_STAGE_THIRD_RETRY",
            DeliveryStage::FourthRetry => "DELIVERY_STAGE_FOURTH_RETRY",
            DeliveryStage::FifthRetry => "DELIVERY_STAGE_FIFTH_RETRY",
            DeliveryStage::Failed => "DELIVERY_STAGE_FAILED",
            DeliveryStage::Success => "DELIVERY_STAGE_SUCCESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DELIVERY_STAGE_UNSPECIFIED" => Some(Self::Unspecified),
            "DELIVERY_STAGE_FIRST_RETRY" => Some(Self::FirstRetry),
            "DELIVERY_STAGE_SECOND_RETRY" => Some(Self::SecondRetry),
            "DELIVERY_STAGE_THIRD_RETRY" => Some(Self::ThirdRetry),
            "DELIVERY_STAGE_FOURTH_RETRY" => Some(Self::FourthRetry),
            "DELIVERY_STAGE_FIFTH_RETRY" => Some(Self::FifthRetry),
            "DELIVERY_STAGE_FAILED" => Some(Self::Failed),
            "DELIVERY_STAGE_SUCCESS" => Some(Self::Success),
            _ => None,
        }
    }
}
/// Webhook Event Type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventType {
    /// Event Type Unspecified.
    Unspecified = 0,
    /// Event Type Wallet Created.
    WalletCreated = 1,
    /// Event Type Promissory File Received.
    PromissoryReceived = 2,
    /// Event Type Ping.
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
/// Delivery Method Options.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeliveryMethodType {
    /// Delivery Method Unspecified.
    Unspecified = 0,
    /// Delivery Method using HTTP.
    Http = 1,
    /// Delivery Method using GRPC.
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
