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
    #[prost(enumeration="agent_connection::Status", tag="2")]
    pub status: i32,
}
/// Nested message and enum types in `AgentConnection`.
pub mod agent_connection {
    /// Connection Status
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Status: Connected.
        Unknown = 0,
        /// Status: Connected.
        Connected = 1,
        /// Status: Disconnected.
        Disconnected = 2,
        /// Status: Execute transfer funds
        Execute = 3,
        /// Status: Terminate agent process
        Shutdown = 4,
        /// Status: Perform setup process
        Setup = 5,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unknown => "STATUS_UNKNOWN",
                Status::Connected => "STATUS_CONNECTED",
                Status::Disconnected => "STATUS_DISCONNECTED",
                Status::Execute => "STATUS_EXECUTE",
                Status::Shutdown => "STATUS_SHUTDOWN",
                Status::Setup => "STATUS_SETUP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNKNOWN" => Some(Self::Unknown),
                "STATUS_CONNECTED" => Some(Self::Connected),
                "STATUS_DISCONNECTED" => Some(Self::Disconnected),
                "STATUS_EXECUTE" => Some(Self::Execute),
                "STATUS_SHUTDOWN" => Some(Self::Shutdown),
                "STATUS_SETUP" => Some(Self::Setup),
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
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShutdownRequest {
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShutdownResponse {
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetupRequest {
}
/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetupResponse {
}
include!("orchestrator.tonic.rs");
// @@protoc_insertion_point(module)
