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
/// "contract_id": "CONTRACT_ID"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractProposalCompletedPayload {
    /// Contract ID.
    #[prost(string, tag="1")]
    pub contract_id: ::prost::alloc::string::String,
    /// Owner ID
    #[prost(string, tag="2")]
    pub owner_id: ::prost::alloc::string::String,
    /// Wallet ID
    #[prost(string, tag="3")]
    pub wallet_id: ::prost::alloc::string::String,
    /// Contract Proposals
    #[prost(message, repeated, tag="4")]
    pub commitments: ::prost::alloc::vec::Vec<super::super::common::Commitment>,
}
/// \[Example\]
/// {
/// "contract_id": "CONTRACT_ID"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractPaymentCompletedPayload {
    /// Contract ID.
    #[prost(string, tag="1")]
    pub contract_id: ::prost::alloc::string::String,
    /// Owner ID
    #[prost(string, tag="2")]
    pub owner_id: ::prost::alloc::string::String,
    /// Wallet ID
    #[prost(string, tag="3")]
    pub wallet_id: ::prost::alloc::string::String,
    /// Contract Proposals
    #[prost(message, repeated, tag="4")]
    pub commitments: ::prost::alloc::vec::Vec<super::super::common::Commitment>,
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
    #[prost(oneof="event::Payload", tags="6, 7, 8, 10, 11")]
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
        /// Event Payload for contract proposal completed.
        #[prost(message, tag="10")]
        ContractProposalCompletedPayload(super::ContractProposalCompletedPayload),
        /// Event Payload for contract payment completed.
        #[prost(message, tag="11")]
        ContractPaymentCompletedPayload(super::ContractPaymentCompletedPayload),
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduledReaction {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Webhook name.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Reaction type.
    #[prost(enumeration="ScheduleReactionType", tag="3")]
    pub reaction_type: i32,
    /// Cron tab. Requires 5 asterisks/values. (e.g. * * * * *). Only supports minute granularity.
    #[prost(string, tag="4")]
    pub cron_tab: ::prost::alloc::string::String,
    #[prost(oneof="scheduled_reaction::ReactionPayload", tags="5")]
    pub reaction_payload: ::core::option::Option<scheduled_reaction::ReactionPayload>,
}
/// Nested message and enum types in `ScheduledReaction`.
pub mod scheduled_reaction {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ReactionPayload {
        /// Reaction Payload for sending payment.
        #[prost(message, tag="5")]
        CreateContractPayload(super::CreateContractReactionPayload),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContractReactionPayload {
    /// Wallet ID.
    #[prost(string, tag="1")]
    pub sender_wallet_id: ::prost::alloc::string::String,
    /// List of commitments
    #[prost(message, repeated, tag="2")]
    pub commitments: ::prost::alloc::vec::Vec<super::super::common::Commitment>,
    /// List of conditions
    #[prost(message, repeated, tag="3")]
    pub conditions: ::prost::alloc::vec::Vec<super::super::common::Condition>,
    /// Timeout in seconds for contracts created by the scheduled reaction
    #[prost(uint32, tag="4")]
    pub timeout_secs: u32,
    /// Memo for contracts created by the scheduled reaction
    #[prost(string, tag="5")]
    pub memo: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateScheduledReactionRequest {
    /// API Key as a JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Webhook name.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Reaction type.
    #[prost(enumeration="ScheduleReactionType", tag="3")]
    pub reaction_type: i32,
    /// *
    /// Recurring schedule is in the form of a crontab, requiring five values. This example will execute once per minute.
    /// ┌───────────── minute (0 - 59)
    /// │ ┌───────────── hour (0 - 23)
    /// │ │ ┌───────────── day of the month (1 - 31)
    /// │ │ │ ┌───────────── month (1 - 12)
    /// │ │ │ │ ┌───────────── day of the week (0 - 6) (Sunday to Saturday
    /// │ │ │ │ │
    /// │ │ │ │ │
    /// * * * *
    #[prost(string, tag="4")]
    pub cron_tab: ::prost::alloc::string::String,
    #[prost(oneof="create_scheduled_reaction_request::ReactionPayload", tags="5")]
    pub reaction_payload: ::core::option::Option<create_scheduled_reaction_request::ReactionPayload>,
}
/// Nested message and enum types in `CreateScheduledReactionRequest`.
pub mod create_scheduled_reaction_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ReactionPayload {
        /// Reaction Payload for sending payment.
        #[prost(message, tag="5")]
        CreateContractPayload(super::CreateContractReactionPayload),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateScheduledReactionResponse {
    #[prost(message, optional, tag="1")]
    pub scheduled_reaction: ::core::option::Option<ScheduledReaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListScheduledReactionsRequest {
    /// API Key as a JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Page number for pagination.
    #[prost(int32, tag="2")]
    pub page: i32,
    /// Size of the page for pagination.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Potential filters to apply to the list of scheduled reactions.
    #[prost(message, optional, tag="4")]
    pub filters: ::core::option::Option<super::super::common::Filter>,
    /// Potential sort to apply to the list of scheduled reactions.
    #[prost(message, optional, tag="5")]
    pub sort_by: ::core::option::Option<super::super::common::SortBy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListScheduledReactionsResponse {
    #[prost(message, repeated, tag="1")]
    pub scheduled_reactions: ::prost::alloc::vec::Vec<ScheduledReaction>,
    #[prost(uint32, tag="2")]
    pub total_owned_scheduled_reactions: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteScheduledReactionRequest {
    /// API Key as a JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Scheduled Reaction ID.
    #[prost(string, tag="2")]
    pub scheduled_reaction_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteScheduledReactionResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Prevalidation {
    /// UUID ID
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Name
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Trigger for the prevalidation.
    #[prost(oneof="prevalidation::Trigger", tags="3")]
    pub trigger: ::core::option::Option<prevalidation::Trigger>,
}
/// Nested message and enum types in `Prevalidation`.
pub mod prevalidation {
    /// Trigger for the prevalidation.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trigger {
        #[prost(message, tag="3")]
        ConsideringContractPrevalidation(super::ConsideringContractPrevalidation),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsideringContractPrevalidation {
    /// Conditions to be evaluated
    #[prost(message, repeated, tag="1")]
    pub condition: ::prost::alloc::vec::Vec<considering_contract_prevalidation::ValidationCondition>,
    /// Outcome of the prevalidation if the condition is true
    #[prost(enumeration="considering_contract_prevalidation::ValidationOutcome", tag="2")]
    pub outcome: i32,
    #[prost(enumeration="super::super::common::LogicalOperator", tag="3")]
    pub logical_base: i32,
}
/// Nested message and enum types in `ConsideringContractPrevalidation`.
pub mod considering_contract_prevalidation {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValidationCondition {
        #[prost(enumeration="ValidationTarget", tag="1")]
        pub target: i32,
        #[prost(enumeration="super::ValidationOperator", tag="2")]
        pub operator: i32,
        #[prost(oneof="validation_condition::Value", tags="3")]
        pub value: ::core::option::Option<validation_condition::Value>,
    }
    /// Nested message and enum types in `ValidationCondition`.
    pub mod validation_condition {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            #[prost(message, tag="3")]
            AmountValue(super::super::super::super::common::Amount),
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ValidationTarget {
        Unspecified = 0,
        /// balance of the account receiving the assets
        RecipientBalance = 1,
    }
    impl ValidationTarget {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValidationTarget::Unspecified => "VALIDATION_TARGET_UNSPECIFIED",
                ValidationTarget::RecipientBalance => "VALIDATION_TARGET_RECIPIENT_BALANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "VALIDATION_TARGET_UNSPECIFIED" => Some(Self::Unspecified),
                "VALIDATION_TARGET_RECIPIENT_BALANCE" => Some(Self::RecipientBalance),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ValidationOutcome {
        ConsideringContractOutcomeUnspecified = 0,
        ConsideringContractOutcomeAccepted = 1,
        ConsideringContractOutcomeRejected = 2,
    }
    impl ValidationOutcome {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValidationOutcome::ConsideringContractOutcomeUnspecified => "CONSIDERING_CONTRACT_OUTCOME_UNSPECIFIED",
                ValidationOutcome::ConsideringContractOutcomeAccepted => "CONSIDERING_CONTRACT_OUTCOME_ACCEPTED",
                ValidationOutcome::ConsideringContractOutcomeRejected => "CONSIDERING_CONTRACT_OUTCOME_REJECTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONSIDERING_CONTRACT_OUTCOME_UNSPECIFIED" => Some(Self::ConsideringContractOutcomeUnspecified),
                "CONSIDERING_CONTRACT_OUTCOME_ACCEPTED" => Some(Self::ConsideringContractOutcomeAccepted),
                "CONSIDERING_CONTRACT_OUTCOME_REJECTED" => Some(Self::ConsideringContractOutcomeRejected),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePrevalidationRequest {
    /// API Key as a JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Name of the prevalidation.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Trigger for the prevalidation.
    #[prost(oneof="create_prevalidation_request::Trigger", tags="3")]
    pub trigger: ::core::option::Option<create_prevalidation_request::Trigger>,
}
/// Nested message and enum types in `CreatePrevalidationRequest`.
pub mod create_prevalidation_request {
    /// Trigger for the prevalidation.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trigger {
        #[prost(message, tag="3")]
        ConsideringContractPrevalidation(super::ConsideringContractPrevalidation),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePrevalidationResponse {
    /// Prevalidation.
    #[prost(message, optional, tag="1")]
    pub prevalidation: ::core::option::Option<Prevalidation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrevalidationsRequest {
    /// API Key as a JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Page number for pagination.
    #[prost(int32, tag="2")]
    pub page: i32,
    /// Size of the page for pagination.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Potential filters to apply to the list of prevalidations.
    #[prost(message, optional, tag="4")]
    pub filters: ::core::option::Option<super::super::common::Filter>,
    /// Potential sort to apply to the list of prevalidations.
    #[prost(message, optional, tag="5")]
    pub sort_by: ::core::option::Option<super::super::common::SortBy>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrevalidationsResponse {
    /// List of Prevalidations returned.
    #[prost(message, repeated, tag="1")]
    pub prevalidations: ::prost::alloc::vec::Vec<Prevalidation>,
    /// Total number of owned prevalidations returned.
    #[prost(uint32, tag="2")]
    pub total_owned_prevalidations: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePrevalidationRequest {
    /// API Key as a JWT
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// Prevalidation ID.
    #[prost(string, tag="2")]
    pub prevalidation_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePrevalidationResponse {
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
    /// Event Type Contract Proposal Completed.
    ContractProposalCompleted = 4,
    /// Event Type Contract Payment Completed.
    ContractPaymentCompleted = 5,
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
            EventType::ContractProposalCompleted => "EVENT_TYPE_CONTRACT_PROPOSAL_COMPLETED",
            EventType::ContractPaymentCompleted => "EVENT_TYPE_CONTRACT_PAYMENT_COMPLETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EVENT_TYPE_WALLET_CREATED" => Some(Self::WalletCreated),
            "EVENT_TYPE_PROMISSORY_RECEIVED" => Some(Self::PromissoryReceived),
            "EVENT_TYPE_PING" => Some(Self::Ping),
            "EVENT_TYPE_CONTRACT_PROPOSAL_COMPLETED" => Some(Self::ContractProposalCompleted),
            "EVENT_TYPE_CONTRACT_PAYMENT_COMPLETED" => Some(Self::ContractPaymentCompleted),
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScheduleReactionType {
    /// Scheduled Reaction Type Unspecified.
    Unspecified = 0,
    /// Scheduled Reaction Type Wallet Created.
    CreateContract = 1,
}
impl ScheduleReactionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScheduleReactionType::Unspecified => "SCHEDULE_REACTION_TYPE_UNSPECIFIED",
            ScheduleReactionType::CreateContract => "SCHEDULE_REACTION_TYPE_CREATE_CONTRACT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCHEDULE_REACTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SCHEDULE_REACTION_TYPE_CREATE_CONTRACT" => Some(Self::CreateContract),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ValidationOperator {
    Unspecified = 0,
    Equal = 1,
    GreaterThan = 2,
    GreaterThanOrEqual = 3,
    LessThan = 4,
    LessThanOrEqual = 5,
}
impl ValidationOperator {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ValidationOperator::Unspecified => "VALIDATION_OPERATOR_UNSPECIFIED",
            ValidationOperator::Equal => "VALIDATION_OPERATOR_EQUAL",
            ValidationOperator::GreaterThan => "VALIDATION_OPERATOR_GREATER_THAN",
            ValidationOperator::GreaterThanOrEqual => "VALIDATION_OPERATOR_GREATER_THAN_OR_EQUAL",
            ValidationOperator::LessThan => "VALIDATION_OPERATOR_LESS_THAN",
            ValidationOperator::LessThanOrEqual => "VALIDATION_OPERATOR_LESS_THAN_OR_EQUAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VALIDATION_OPERATOR_UNSPECIFIED" => Some(Self::Unspecified),
            "VALIDATION_OPERATOR_EQUAL" => Some(Self::Equal),
            "VALIDATION_OPERATOR_GREATER_THAN" => Some(Self::GreaterThan),
            "VALIDATION_OPERATOR_GREATER_THAN_OR_EQUAL" => Some(Self::GreaterThanOrEqual),
            "VALIDATION_OPERATOR_LESS_THAN" => Some(Self::LessThan),
            "VALIDATION_OPERATOR_LESS_THAN_OR_EQUAL" => Some(Self::LessThanOrEqual),
            _ => None,
        }
    }
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
