// @generated
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

// A log entry for a Gateway connection event.

/// \[Example\]
/// {
/// "verifier": "VerfierLengthVariesByAlgo", 
/// "note": "NOTE", 
/// "timestamp": "2006-01-02T15:04:05Z", 
/// "origin": "ORIGIN", 
/// "connection_id": "CONNECTION_ID_UUID"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogEntry {
    /// Public Key
    #[prost(string, tag="1")]
    pub verifier: ::prost::alloc::string::String,
    /// Specific action being recorded.
    #[prost(string, tag="2")]
    pub note: ::prost::alloc::string::String,
    /// RFC3339 timestamp of the log entry.
    #[prost(string, tag="3")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub origin: ::prost::alloc::string::String,
    /// UUID representing a unique client connection
    #[prost(string, tag="5")]
    pub connection_id: ::prost::alloc::string::String,
}
// Requests a list of connections known to the Gateway.

/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionsRequest {
}
// Contains a list of active connections, as well as lists of log entries for all created connections and for all
// removed connections.

/// \[Example\]
/// {
/// "active": [
/// "ActiveConnection1", 
/// "ActiveConnection2"
/// ], 
/// "created": [
/// {
/// "verifier": "VerfierLengthVariesByAlgo", 
/// "note": "CreatedConnection1", 
/// "timestamp": "2006-01-02T15:04:05Z", 
/// "origin": "ORIGIN", 
/// "connection_id": "CONNECTION_ID_UUID"
/// }, 
/// {
/// "verifier": "VerfierLengthVariesByAlgo", 
/// "note": "CreatedConnection2", 
/// "timestamp": "2006-01-02T15:04:05Z", 
/// "origin": "ORIGIN", 
/// "connection_id": 
/// "CONNECTION_ID_UUID"
/// }
/// ], 
/// "removed": [
/// {
/// "verifier": "VerfierLengthVariesByAlgo", 
/// "note": "RemovedConnection1", 
/// "timestamp": "2006-01-02T15:04:05Z", 
/// "origin": "ORIGIN", 
/// "connection_id": "CONNECTION_ID_UUID"
/// }, 
/// {
/// "verifier": "VerfierLengthVariesByAlgo", 
/// "note": "RemovedConnection2", 
/// "timestamp": "2006-01-02T15:04:05Z", 
/// "origin": "ORIGIN", 
/// "connection_id": "CONNECTION_ID_UUID"
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionsResponse {
    /// Currently active connections.
    #[prost(string, repeated, tag="1")]
    pub active: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The number of connections that have been created.
    #[prost(message, repeated, tag="2")]
    pub created: ::prost::alloc::vec::Vec<LogEntry>,
    /// The number of connections that have been removed.
    #[prost(message, repeated, tag="3")]
    pub removed: ::prost::alloc::vec::Vec<LogEntry>,
}
include!("gateway.tonic.rs");
// @@protoc_insertion_point(module)
