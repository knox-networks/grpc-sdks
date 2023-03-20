// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentConnection {
    #[prost(message, optional, tag="1")]
    pub verifier: ::core::option::Option<super::common::DynamicVerifier>,
    #[prost(enumeration="agent_connection::ConnectionStatus", tag="2")]
    pub status: i32,
}
/// Nested message and enum types in `AgentConnection`.
pub mod agent_connection {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConnectionStatus {
        Connected = 0,
        Disconnected = 1,
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
            }
        }
    }
}
include!("orchestrator.tonic.rs");
// @@protoc_insertion_point(module)
