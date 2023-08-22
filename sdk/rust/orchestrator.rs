// @generated
// THIS PROTO FILE IS INTENDED FOR INTERNAL KNOX USE ONLY.

/// \[Example\]
/// {
/// "verifier": {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// },
/// "status": 0
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentConnection {
    /// Verifier of the Agent.
    #[prost(message, optional, tag="1")]
    pub verifier: ::core::option::Option<super::common::DynamicVerifier>,
    /// Status of the Agent.
    #[prost(enumeration="agent_connection::ConnectionStatus", tag="2")]
    pub status: i32,
}
/// Nested message and enum types in `AgentConnection`.
pub mod agent_connection {
    /// Connection Status
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConnectionStatus {
        /// Status: Connected.
        Connected = 0,
        /// Status: Disconnected.
        Disconnected = 1,
        /// Status: Execute.
        Execute = 2,
    }
    impl ConnectionStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConnectionStatus::Connected => "CONNECTED",
                ConnectionStatus::Disconnected => "DISCONNECTED",
                ConnectionStatus::Execute => "EXECUTE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONNECTED" => Some(Self::Connected),
                "DISCONNECTED" => Some(Self::Disconnected),
                "EXECUTE" => Some(Self::Execute),
                _ => None,
            }
        }
    }
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteRequest {
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteResponse {
}
include!("orchestrator.tonic.rs");
// @@protoc_insertion_point(module)
