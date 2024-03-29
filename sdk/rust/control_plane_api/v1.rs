// @generated
// service_suffix here is the suffix of the release name. The relationship between service_suffix, helm release name 
// and kubernetes service name are:
// <namespace>-service_suffix = helm release name = kubernetes service name

/// \[Example\] 
/// {
/// "service_type": 0, 
/// "service_suffix": "SERVICE_SUFFIX", 
/// "service_configs": { 
/// "values": {
/// "fields": [
/// {
/// "key": "KEY1",
/// "value": {}
/// },
/// {
/// "key": "KEY2",
/// "value": {}
/// }
/// ]
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRequest {
    /// Service Type
    #[prost(enumeration="ServiceType", tag="1")]
    pub service_type: i32,
    /// Service Suffix
    #[prost(string, tag="2")]
    pub service_suffix: ::prost::alloc::string::String,
    /// Service Configurations
    #[prost(message, optional, tag="3")]
    pub service_configs: ::core::option::Option<ServiceConfigurations>,
}
/// \[Example\]
/// {
/// "status": 0, 
/// "service_name": "SERVICE_NAME", 
/// "public_key": "zPublicKey" 
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceResponse {
    /// Status of Deploymnet
    #[prost(enumeration="DeployStatus", tag="1")]
    pub status: i32,
    /// Service Name
    #[prost(string, tag="2")]
    pub service_name: ::prost::alloc::string::String,
    /// Public Key
    #[prost(string, tag="3")]
    pub public_key: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "values": {
/// "fields": [
/// {
/// "key": "KEY1",
/// "value": {}
/// },
/// {
/// "key": "KEY2",
/// "value": {}
/// }
/// ]
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceDetails {
    /// Values
    #[prost(message, optional, tag="1")]
    pub values: ::core::option::Option<::prost_types::Struct>,
}
// Returns the status of a Service

/// \[Example\]
/// {
/// "service_type": 0, 
/// "service_suffix": "SERVICE_SUFFIX"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Service Type.
    #[prost(enumeration="ServiceType", tag="1")]
    pub service_type: i32,
    /// Service Suffix.
    #[prost(string, tag="2")]
    pub service_suffix: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "instance_status": 0, 
/// "reason": "REASON"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceState {
    /// Instance Status.
    #[prost(enumeration="InstanceStatus", tag="1")]
    pub instance_status: i32,
    /// Reason.
    #[prost(string, tag="2")]
    pub reason: ::prost::alloc::string::String,
}
/// \[Example\] 
/// {
/// "service_name": "SERVICE_NAME", 
/// "instance_states": [
/// {
/// "instance_status": 0, 
/// "reason": "REASON0"
/// }, 
/// {
/// "instance_status": 1, 
/// "reason": "REASON1"
/// }
/// ], 
/// "public_key": "zPublicKey", 
/// "service_details": {
/// "values": {
/// "fields": [
/// {
/// "key": "KEY1",
/// "value": {}
/// },
/// {
/// "key": "KEY2",
/// "value": {}
/// }
/// ]
/// }
/// } 
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceResponse {
    /// Service Name.
    #[prost(string, tag="1")]
    pub service_name: ::prost::alloc::string::String,
    /// Instance States of Service.
    #[prost(message, repeated, tag="2")]
    pub instance_states: ::prost::alloc::vec::Vec<InstanceState>,
    /// Public Key.
    #[prost(string, tag="3")]
    pub public_key: ::prost::alloc::string::String,
    /// Service Details.
    #[prost(message, optional, tag="4")]
    pub service_details: ::core::option::Option<ServiceDetails>,
}
// Returns the status of a Service

/// \[Example\]
/// {
/// "service_type": 0
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServicesRequest {
    /// Service Type.
    #[prost(enumeration="ServiceType", tag="1")]
    pub service_type: i32,
}
/// \[Example\]
/// {
/// "services": [
/// {
/// "service_name": "SERVICE_NAME1", 
/// "instance_states": [
/// {
/// "instance_status": 0, 
/// "reason": "REASON0"
/// }, 
/// {
/// "instance_status": 1, 
/// "reason": "REASON1"
/// }
/// ], 
/// "public_key": "zPublicKey1", 
/// "service_details": {
/// "values": {
/// "fields": [
/// {
/// "key": "KEY1",
/// "value": {}
/// },
/// {
/// "key": "KEY2",
/// "value": {}
/// }
/// ]
/// }
/// } 
/// },
/// {
/// "service_name": "SERVICE_NAME2", 
/// "instance_states": [
/// {
/// "instance_status": 0, 
/// "reason": "REASON0"
/// }, 
/// {
/// "instance_status": 1, 
/// "reason": "REASON1"
/// }
/// ], 
/// "public_key": "zPublicKey2", 
/// "service_details": {
/// "values": {
/// "fields": [
/// {
/// "key": "KEY1",
/// "value": {}
/// },
/// {
/// "key": "KEY2",
/// "value": {}
/// }
/// ]
/// }
/// } 
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServicesResponse {
    /// List of Service Responses.
    #[prost(message, repeated, tag="1")]
    pub services: ::prost::alloc::vec::Vec<GetServiceResponse>,
}
/// \[Example\]
/// {
/// "service_type": 0,
/// "service_suffix": "SERVICE_SUFFIX" 
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Service Type to be Deleted.
    #[prost(enumeration="ServiceType", tag="1")]
    pub service_type: i32,
    /// Suffix of Service to be Deleted.
    #[prost(string, tag="2")]
    pub service_suffix: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "service_name": "SERVICE_NAME", 
/// "status": 0 
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceResponse {
    /// Name of Service Attempted to be Deleted.
    #[prost(string, tag="1")]
    pub service_name: ::prost::alloc::string::String,
    /// Status of Service Attempted to be Deleted.
    #[prost(enumeration="DeployStatus", tag="2")]
    pub status: i32,
}
// TODO: Need to decide which values can be configurable
// What else? (Possibly: memory, cpu, resource limits, etc.)

/// \[Example\]
/// {
/// "values": {
/// "fields": [
/// {
/// "key": "KEY1",
/// "value": {}
/// },
/// {
/// "key": "KEY2",
/// "value": {}
/// }
/// ]
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceConfigurations {
    /// Check Service Configuations.
    #[prost(message, optional, tag="1")]
    pub values: ::core::option::Option<::prost_types::Struct>,
}
// Corresponds to a helm upgrade command

/// \[Example\]
/// {
/// "service_type": 0,
/// "service_suffix": "SERVICE_SUFFIX",
/// "service_configs": {
/// "values": {
/// "fields": [
/// {
/// "key": "KEY1",
/// "value": {}
/// },
/// {
/// "key": "KEY2",
/// "value": {}
/// }
/// ]
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Name of to be Updated.
    #[prost(enumeration="ServiceType", tag="1")]
    pub service_type: i32,
    /// Suffix of Service to be Updated.
    #[prost(string, tag="2")]
    pub service_suffix: ::prost::alloc::string::String,
    /// Configuration to Update Service With.
    #[prost(message, optional, tag="3")]
    pub service_configs: ::core::option::Option<ServiceConfigurations>,
}
/// \[Example\]
/// {
/// "service_name": "SERVICE_NAME", 
/// "status": 0 
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceResponse {
    /// Name of Service Attempted to be Updated.
    #[prost(string, tag="1")]
    pub service_name: ::prost::alloc::string::String,
    /// Status of Service Attempted to be Updated.
    #[prost(enumeration="DeployStatus", tag="2")]
    pub status: i32,
}
// THIS PROTO FILE IS INTENDED FOR INTERNAL KNOX USE ONLY.

/// Type of Service.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceType {
    /// Service type is Unspecified.
    Unspecified = 0,
    /// Service type is Authority.
    Authority = 1,
    /// Service type is Issuer.
    Issuer = 2,
    /// Service type is Faucet.
    Faucet = 3,
    /// Service type is Overload.
    Overload = 4,
    /// Service type is Notary.
    Notary = 5,
    /// Service type is Treasury.
    Treasury = 6,
    /// Service type is Sanctions.
    Sanctions = 7,
    /// Service type is Vault.
    Vault = 8,
    /// Service type is Emissary.
    Emissary = 9,
    /// Service type is Gateway.
    Gateway = 10,
    /// Service type is Custodial Wallet.
    CustodialWallet = 11,
    /// Service type is User Management.
    UserMgmt = 12,
    /// Service type is Credential Adapter.
    CredentialAdapter = 13,
    /// Service type is Registry Management.
    RegistryMgmt = 14,
    /// Service type is Sandbox UI.
    SandboxUi = 15,
    /// Service type is GRPC UI.
    Grpcui = 16,
}
impl ServiceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServiceType::Unspecified => "SERVICE_TYPE_UNSPECIFIED",
            ServiceType::Authority => "SERVICE_TYPE_AUTHORITY",
            ServiceType::Issuer => "SERVICE_TYPE_ISSUER",
            ServiceType::Faucet => "SERVICE_TYPE_FAUCET",
            ServiceType::Overload => "SERVICE_TYPE_OVERLOAD",
            ServiceType::Notary => "SERVICE_TYPE_NOTARY",
            ServiceType::Treasury => "SERVICE_TYPE_TREASURY",
            ServiceType::Sanctions => "SERVICE_TYPE_SANCTIONS",
            ServiceType::Vault => "SERVICE_TYPE_VAULT",
            ServiceType::Emissary => "SERVICE_TYPE_EMISSARY",
            ServiceType::Gateway => "SERVICE_TYPE_GATEWAY",
            ServiceType::CustodialWallet => "SERVICE_TYPE_CUSTODIAL_WALLET",
            ServiceType::UserMgmt => "SERVICE_TYPE_USER_MGMT",
            ServiceType::CredentialAdapter => "SERVICE_TYPE_CREDENTIAL_ADAPTER",
            ServiceType::RegistryMgmt => "SERVICE_TYPE_REGISTRY_MGMT",
            ServiceType::SandboxUi => "SERVICE_TYPE_SANDBOX_UI",
            ServiceType::Grpcui => "SERVICE_TYPE_GRPCUI",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SERVICE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SERVICE_TYPE_AUTHORITY" => Some(Self::Authority),
            "SERVICE_TYPE_ISSUER" => Some(Self::Issuer),
            "SERVICE_TYPE_FAUCET" => Some(Self::Faucet),
            "SERVICE_TYPE_OVERLOAD" => Some(Self::Overload),
            "SERVICE_TYPE_NOTARY" => Some(Self::Notary),
            "SERVICE_TYPE_TREASURY" => Some(Self::Treasury),
            "SERVICE_TYPE_SANCTIONS" => Some(Self::Sanctions),
            "SERVICE_TYPE_VAULT" => Some(Self::Vault),
            "SERVICE_TYPE_EMISSARY" => Some(Self::Emissary),
            "SERVICE_TYPE_GATEWAY" => Some(Self::Gateway),
            "SERVICE_TYPE_CUSTODIAL_WALLET" => Some(Self::CustodialWallet),
            "SERVICE_TYPE_USER_MGMT" => Some(Self::UserMgmt),
            "SERVICE_TYPE_CREDENTIAL_ADAPTER" => Some(Self::CredentialAdapter),
            "SERVICE_TYPE_REGISTRY_MGMT" => Some(Self::RegistryMgmt),
            "SERVICE_TYPE_SANDBOX_UI" => Some(Self::SandboxUi),
            "SERVICE_TYPE_GRPCUI" => Some(Self::Grpcui),
            _ => None,
        }
    }
}
/// Status of Deployment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeployStatus {
    /// StatusUnknown indicates that a release is in an uncertain state.
    Unspecified = 0,
    /// StatusDeployed indicates that the release has been pushed to Kubernetes.
    Deployed = 1,
    /// StatusUninstalled indicates that a release has been uninstalled from Kubernetes.
    Uninstalled = 2,
    /// StatusSuperseded indicates that this release object is outdated and a newer one exists.
    Superseded = 3,
    /// StatusFailed indicates that the release was not successfully deployed.
    Failed = 4,
    /// StatusUninstalling indicates that a uninstall operation is underway.
    Uninstalling = 5,
    /// StatusPendingInstall indicates that an install operation is underway.
    Pending = 6,
}
impl DeployStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeployStatus::Unspecified => "DEPLOY_STATUS_UNSPECIFIED",
            DeployStatus::Deployed => "DEPLOY_STATUS_DEPLOYED",
            DeployStatus::Uninstalled => "DEPLOY_STATUS_UNINSTALLED",
            DeployStatus::Superseded => "DEPLOY_STATUS_SUPERSEDED",
            DeployStatus::Failed => "DEPLOY_STATUS_FAILED",
            DeployStatus::Uninstalling => "DEPLOY_STATUS_UNINSTALLING",
            DeployStatus::Pending => "DEPLOY_STATUS_PENDING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEPLOY_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "DEPLOY_STATUS_DEPLOYED" => Some(Self::Deployed),
            "DEPLOY_STATUS_UNINSTALLED" => Some(Self::Uninstalled),
            "DEPLOY_STATUS_SUPERSEDED" => Some(Self::Superseded),
            "DEPLOY_STATUS_FAILED" => Some(Self::Failed),
            "DEPLOY_STATUS_UNINSTALLING" => Some(Self::Uninstalling),
            "DEPLOY_STATUS_PENDING" => Some(Self::Pending),
            _ => None,
        }
    }
}
/// Instance Status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstanceStatus {
    /// Instance Status Unspecified.
    Unspecified = 0,
    /// Instance Status Running.
    Running = 1,
    /// Instance Status Waiting.
    Waiting = 2,
    /// Instance Status Terminated.
    Terminated = 3,
}
impl InstanceStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstanceStatus::Unspecified => "INSTANCE_STATUS_UNSPECIFIED",
            InstanceStatus::Running => "INSTANCE_STATUS_RUNNING",
            InstanceStatus::Waiting => "INSTANCE_STATUS_WAITING",
            InstanceStatus::Terminated => "INSTANCE_STATUS_TERMINATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTANCE_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "INSTANCE_STATUS_RUNNING" => Some(Self::Running),
            "INSTANCE_STATUS_WAITING" => Some(Self::Waiting),
            "INSTANCE_STATUS_TERMINATED" => Some(Self::Terminated),
            _ => None,
        }
    }
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
