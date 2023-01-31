// @generated
/// A log entry for a gateway connection event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogEntry {
    /// Public Key 
    #[prost(string, tag="1")]
    pub verifier: ::prost::alloc::string::String,
    /// Specific action being recorded.
    #[prost(string, tag="2")]
    pub note: ::prost::alloc::string::String,
    /// Timestamp
    #[prost(string, tag="3")]
    pub timestamp: ::prost::alloc::string::String,
}
/// GetConnections requests a list of connections known to the gateway.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionsRequest {
}
/// The response for GetConnections, containing a list of active connections, as well as lists of log entries for all created connections and for all removed connections.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionsResponse {
    /// Currently active Connections.
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
