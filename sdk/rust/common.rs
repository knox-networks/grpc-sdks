// @generated
// Holds a bytes value public key in the `verifier` field that allows verification using the digital signature algorithm
// indicated by the `signature_system`.

/// \[Example\]
/// {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicVerifier {
    /// Signature system of the verifying key.
    #[prost(enumeration="SignatureSystem", tag="1")]
    pub signature_system: i32,
    /// Public key used in verifying digital signature authenticity.
    #[prost(bytes="vec", tag="2")]
    pub verifier: ::prost::alloc::vec::Vec<u8>,
}
// IEEE 754 decimal128 compatible amount type constructed from `amount` (the significand/mantissa) and `decimals` (the
// inverse exponent) for use with floating point arithmetic and decimal 128 compatible libraries. Ex. An `amount` of
// 12345 and `decimals` of 2 is equivalent to "123.45".

/// \[Example\]
/// {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Amount {
    /// ISO-4217 currency code to denote remitting Digital Banknotes in a specific currency.
    #[prost(string, tag="1")]
    pub currency_code: ::prost::alloc::string::String,
    /// Signed 64 bit representation of the significand.
    #[prost(int64, tag="2")]
    pub amount: i64,
    /// Unsigned 32 bit representation of the decimal precision, i.e. the inverse of the exponent. The allowed value is
    /// determined by the rust_decimal crate, which allows a scale (decimal) between 0-28 inclusive. e.g. amount = 10,
    /// decimals = 2 => value 0.01.
    #[prost(uint32, tag="3")]
    pub decimals: u32,
}
// Represents a desired denomination distribution.
// map<mantissa, quantity>
// sum must match corresponding amount.amount

/// \[Example\]
/// {
/// "value": []
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Distribution {
    /// Distribution Values and Amounts.
    #[prost(map="int64, int32", tag="1")]
    pub value: ::std::collections::HashMap<i64, i32>,
}
// Used to start an authentication flow

/// \[Example\]
/// {
/// "verifier": {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationRequest {
    /// Verifier of the requester.
    #[prost(message, optional, tag="1")]
    pub verifier: ::core::option::Option<DynamicVerifier>,
    /// UUID representing a distinct gateway client.
    #[prost(string, tag="2")]
    pub connection_id: ::prost::alloc::string::String,
}
// The returns a challenge nonce meant to be signed by the requester's private key.

/// \[Example\]
/// {
/// "challenge": "ByteLengthChallengeVariesByAlgo="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationResponse {
    /// Random unique string to be signed by the authorized key.
    #[prost(bytes="vec", tag="1")]
    pub challenge: ::prost::alloc::vec::Vec<u8>,
}
// Connection type associated with a single verifier.

/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneConnection {
}
// Connection type holding one or more `associated_verifiers` associated with a primary verifier and single gateway
// connection.

/// \[Example\]
/// {
/// "associated_verifiers": [
/// {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// },
/// {
/// "signature_system": 2,
/// "verifier": "VerifierLengthChangesByAlgo="
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManyConnection {
    /// associated verifiers
    #[prost(message, repeated, tag="2")]
    pub associated_verifiers: ::prost::alloc::vec::Vec<DynamicVerifier>,
}
// The message sent during an already established connection allowing the addition or removal of an
// `associated_verifier` tied to connection held by the `primary_verifier`.

/// \[Example\]
/// {
/// "primary_verifier": {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// },
/// "associated_verifier": {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociatedConnection {
    /// verifier holding a gateway connection
    #[prost(message, optional, tag="1")]
    pub primary_verifier: ::core::option::Option<DynamicVerifier>,
    /// associated verifier
    #[prost(message, optional, tag="2")]
    pub associated_verifier: ::core::option::Option<DynamicVerifier>,
}
// Presented when establishing a gateway connection. `request_type` type indicates whether there will be one or more
// verifiers (public keys) associated with a particular connection. The primary verifier (connection owner) is
// represented by the `signature.verifier` field.

/// \[Example\]
/// {
/// "one": {},
/// "signature": {
/// "verifier": {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// },
/// "signature": "ByteLengthSignatureVariesByAlgo="
/// },
/// "connection_id": "CONNECTION_ID"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionRequest {
    /// The crypto signature and signers public key.
    #[prost(message, optional, tag="3")]
    pub signature: ::core::option::Option<AuthenticationSignature>,
    /// UUID representing a distinct gateway client.
    #[prost(string, tag="4")]
    pub connection_id: ::prost::alloc::string::String,
    /// The connection request may include  a connection type that is of type OneConnection or ManyConnection.
    #[prost(oneof="connection_request::RequestType", tags="1, 2")]
    pub request_type: ::core::option::Option<connection_request::RequestType>,
}
/// Nested message and enum types in `ConnectionRequest`.
pub mod connection_request {
    /// The connection request may include  a connection type that is of type OneConnection or ManyConnection.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestType {
        /// The connection request is of type OneConnection.
        #[prost(message, tag="1")]
        One(super::OneConnection),
        /// The connection request is of type ManyConnection.
        #[prost(message, tag="2")]
        Many(super::ManyConnection),
    }
}
// Presented when establishing a gateway connection as a field of `ConnectionRequest`. The `verifier` should match the
// verifier used in `AuthenticationRequest`. The `bytes` field is the signed nonce from `AuthenticationResponse`.

/// \[Example\]
/// {
/// "verifier": {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// },
/// "signature": "ByteLengthSignatureVariesByAlgo="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationSignature {
    /// Verifier of the requester.
    #[prost(message, optional, tag="1")]
    pub verifier: ::core::option::Option<DynamicVerifier>,
    /// Cryptographic signature of the challenge nonce.
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
// Authorized digital banknote returned in the Notary service flow.

/// \[Example\]
/// {
/// "promissory_file": "FileByteLengthVaries="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedSignatureResponse {
    /// Return digital banknote with authorized signature.
    #[prost(bytes="vec", tag="1")]
    pub promissory_file: ::prost::alloc::vec::Vec<u8>,
}
// Media - Mime type and binary data encoding For common MIME types see:
// <https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types>

/// \[Example\]
/// {
/// "mime_type": "MIME_TYPE",
/// "data": "ByteLengthDataVaries="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Media {
    /// Mime type URI.
    #[prost(string, tag="1")]
    pub mime_type: ::prost::alloc::string::String,
    /// Media data.
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
// Resource based access control (RBAC) role used to define currency limits for a public key that the issuer allows to
// create issuance requests.

/// \[Example\]
/// {
/// "verifier": "VerifierLengthVariesByAlgo",
/// "role_enum": {
/// "issue_permissions": {
/// "inner": [
/// {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// },
/// {
/// "currency_code":"USD",
/// "amount":500,
/// "decimals":2
/// }
/// ]
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Role {
    /// MultibaseVerifier.
    #[prost(string, tag="1")]
    pub verifier: ::prost::alloc::string::String,
    /// The role granted by an Issuer to an entity to make issuance requests.
    #[prost(oneof="role::RoleEnum", tags="3, 4")]
    pub role_enum: ::core::option::Option<role::RoleEnum>,
}
/// Nested message and enum types in `Role`.
pub mod role {
    /// \[Example\]
    /// {
    /// "inner": [
    /// {
    /// "currency_code":"USD",
    /// "amount":100,
    /// "decimals":2
    /// },
    /// {
    /// "currency_code":"USD",
    /// "amount":500,
    /// "decimals":2
    /// }
    /// ]
    /// }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IssuePermissions {
        /// The amounts up to which issuance is authorized in respective currencies.
        #[prost(message, repeated, tag="2")]
        pub inner: ::prost::alloc::vec::Vec<super::Amount>,
    }
    /// The role granted by an Issuer to an entity to make issuance requests.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RoleEnum {
        /// Authorized Role allows authorized keys to request a challenge nonce. Authorized keys must have additional role
        /// permissions to use signed challenge nonce.
        #[prost(bool, tag="3")]
        Authorized(bool),
        /// Issuer Role Permissions.
        #[prost(message, tag="4")]
        IssuePermissions(IssuePermissions),
    }
}
// Request the identity of a given service endpoint, e.g. Issuer, Authority, Treasury.

/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVerifierRequest {
}
// Response that contains the verifier (public key) of a service endpoint. Returns both Dynamic Verifier bytes, and
// multibase encoded formats.

/// \[Example\]
/// {
/// "bytes": {
/// "signature_system": 1,
/// "verifier": "VerifierLengthChangesByAlgo="
/// },
/// "multibase": "MULTIBASE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVerifierResponse {
    /// Verifier (public key) in Dynamic verifier format.
    #[prost(message, optional, tag="1")]
    pub bytes: ::core::option::Option<DynamicVerifier>,
    /// Verifier (public key) in multibase encoding.
    #[prost(string, tag="2")]
    pub multibase: ::prost::alloc::string::String,
}
// Request the `ServiceType` of a given service endpoint. e.g. Issuer, Authority, Treasury.

/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceTypeRequest {
}
// Response that contains the `ServiceType` of a service endpoint.

/// \[Example\]
/// {
/// "service_type": 1
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceTypeResponse {
    /// One of the service types defined in ServiceType.
    #[prost(enumeration="ServiceType", tag="1")]
    pub service_type: i32,
}
// GetInstanceRequest - Request the Instance of the service, a value generated each time a service
// is started, can be correlated to a gateway connection_id

/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
}
// GetInstanceResponse - Respond with a UUID of the particular instance

/// \[Example\]
/// {
/// "instance_id": "INSTANCE_ID"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceResponse {
    /// Instance ID.
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
}
// Initiates a Funds Change transaction. Sender is implied to be the owner of the promissories to be sent. Recipient is
// the well known Treasury verifier provided at startup.

/// \[Example\]
/// {
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// },
/// "promissory": "PROMISSORY_ID_1",
/// "user_reference": "USER_PROVIDED_REFERENCE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartFundsChangeRequest {
    /// The specific amount that the returned change must be able to make a payment of. Inclusive of currency.
    #[prost(message, optional, tag="1")]
    pub amount: ::core::option::Option<Amount>,
    /// The Digital Banknote to be replaced by other denominations. The  change returned will be a number of digital
    /// banknotes totalling the same value as this digital banknote.
    #[prost(string, tag="2")]
    pub promissory: ::prost::alloc::string::String,
    /// User provided reference.
    #[prost(string, tag="3")]
    pub user_reference: ::prost::alloc::string::String,
}
// Returns the Universal E2E Transaction Reference (UUID v4 format) of a successfully initiated Funds Change
// transaction.

/// \[Example\]
/// {
/// "uetr": "UETR"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartFundsChangeResponse {
    /// Universal E2E Transaction Reference (UUID v4 format).
    #[prost(string, tag="1")]
    pub uetr: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "field": "FIELD",
/// "operator": 0,
/// "value": {
/// "str_value": "STRINGVAL"
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterItem {
    /// Field.
    #[prost(string, tag="1")]
    pub field: ::prost::alloc::string::String,
    /// Operator.
    #[prost(enumeration="filter_item::FilterOperator", tag="2")]
    pub operator: i32,
    /// Value of Operator.
    #[prost(oneof="filter_item::Value", tags="3, 4, 5, 6, 7")]
    pub value: ::core::option::Option<filter_item::Value>,
}
/// Nested message and enum types in `FilterItem`.
pub mod filter_item {
    /// Filter Operator Options.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FilterOperator {
        /// Filter Operator Unspecified.
        Unspecified = 0,
        /// Filter Operator Equals "=".
        Eq = 1,
        /// Filter Operator Does Not Equal "!="
        Ne = 2,
        /// Filter Operator Greater Than ">"
        Gt = 3,
        /// Filter Operator Greater Than or Equal To ">="
        Gte = 4,
        /// Filter Operator Less Than "<"
        Lt = 5,
        /// Filter Operator Less Than or Equal to "<="
        Lte = 6,
        /// Filter Operator IN
        In = 7,
        /// Filter Operator NOT IN
        NotIn = 8,
    }
    impl FilterOperator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FilterOperator::Unspecified => "FILTER_OPERATOR_UNSPECIFIED",
                FilterOperator::Eq => "FILTER_OPERATOR_EQ",
                FilterOperator::Ne => "FILTER_OPERATOR_NE",
                FilterOperator::Gt => "FILTER_OPERATOR_GT",
                FilterOperator::Gte => "FILTER_OPERATOR_GTE",
                FilterOperator::Lt => "FILTER_OPERATOR_LT",
                FilterOperator::Lte => "FILTER_OPERATOR_LTE",
                FilterOperator::In => "FILTER_OPERATOR_IN",
                FilterOperator::NotIn => "FILTER_OPERATOR_NOT_IN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FILTER_OPERATOR_UNSPECIFIED" => Some(Self::Unspecified),
                "FILTER_OPERATOR_EQ" => Some(Self::Eq),
                "FILTER_OPERATOR_NE" => Some(Self::Ne),
                "FILTER_OPERATOR_GT" => Some(Self::Gt),
                "FILTER_OPERATOR_GTE" => Some(Self::Gte),
                "FILTER_OPERATOR_LT" => Some(Self::Lt),
                "FILTER_OPERATOR_LTE" => Some(Self::Lte),
                "FILTER_OPERATOR_IN" => Some(Self::In),
                "FILTER_OPERATOR_NOT_IN" => Some(Self::NotIn),
                _ => None,
            }
        }
    }
    /// Value of Operator.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// String Value of Operator, e.g. "abc".
        #[prost(string, tag="3")]
        StrValue(::prost::alloc::string::String),
        /// Integer Value of Operator, e.g. 123.
        #[prost(int64, tag="4")]
        IntValue(i64),
        /// Boolean Value of Operator, e.g. true.
        #[prost(bool, tag="5")]
        BoolValue(bool),
        /// RFC3339 DateTime Value of Operator, e.g. "2019-10-12T07:20:50.52Z".
        #[prost(string, tag="6")]
        Rfc3339DatetimeValue(::prost::alloc::string::String),
        /// UUID Value of Operator, e.g. "123e4567-e89b-12d3-a456-426655440000".
        #[prost(string, tag="7")]
        UuidValue(::prost::alloc::string::String),
    }
}
/// \[Example\]
/// {
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
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// List of Filters.
    #[prost(message, repeated, tag="1")]
    pub filters: ::prost::alloc::vec::Vec<FilterItem>,
    /// Operator for Filter.
    #[prost(enumeration="LogicalOperator", tag="2")]
    pub operator: i32,
}
/// \[Example\]
/// {
/// "field": "FIELD",
/// "direction": 0
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortItem {
    /// Name of the field to sort on.
    #[prost(string, tag="1")]
    pub field: ::prost::alloc::string::String,
    /// Direction of the sort.
    #[prost(enumeration="SortDirection", tag="2")]
    pub direction: i32,
}
/// \[Example\]
/// {
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortBy {
    /// List of sort items.
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<SortItem>,
}
/// \[Example\]
/// {
/// "verifier": "VerifierLengthVariesByAlgo",
/// "url": "AUTHORITY_URL"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetAuthority {
    /// Verifier of the Asset Authority.
    #[prost(string, tag="1")]
    pub verifier: ::prost::alloc::string::String,
    /// URL of the Asset Authority.
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "denominations": [1, 5, 10, 25, 100, 200, 500, 1000],
/// "precision": 2
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetDenomination {
    /// Denominations allowed, e.g. [1, 5, 10, 25, 100, etc.].
    #[prost(int64, repeated, tag="1")]
    pub denominations: ::prost::alloc::vec::Vec<i64>,
    /// Precision for Asset (decimals places), e.g. USD $1.00 = 2.
    #[prost(uint32, tag="2")]
    pub precision: u32,
}
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

/// Indicates the interbank service type associated with a particular endpoint or routable identifier - these
/// enumerations are returned in Service info calls. Any service type marked 'Reserved' should not be directly used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceType {
    /// Service type unknown guards against accidental use of a zero value.
    UnknownService = 0,
    /// The gateway Service.
    Gateway = 1,
    /// Notary Service (Reserved).
    Notary = 2,
    /// The Authority Service.
    Authority = 3,
    /// The Treasury Service.
    Treasury = 4,
    /// The Emissary Service (Reserved).
    Emissary = 5,
    /// The Shared Service (Reserved).
    Shared = 6,
    /// The Identity Service (Reserved).
    Identity = 7,
    /// The Vault Service (Reserved).
    Vault = 8,
    /// The Custodial Wallet Service.
    CustodialWallet = 9,
    /// The Issuer Service.
    Issuer = 10,
    /// The Overload Service (Reserved).
    Overload = 11,
    /// The Faucet Service (Reserved).
    Faucet = 12,
    /// The TransactionManager Service (Reserved).
    TransactionManager = 13,
}
impl ServiceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServiceType::UnknownService => "UnknownService",
            ServiceType::Gateway => "Gateway",
            ServiceType::Notary => "Notary",
            ServiceType::Authority => "Authority",
            ServiceType::Treasury => "Treasury",
            ServiceType::Emissary => "Emissary",
            ServiceType::Shared => "Shared",
            ServiceType::Identity => "Identity",
            ServiceType::Vault => "Vault",
            ServiceType::CustodialWallet => "CustodialWallet",
            ServiceType::Issuer => "Issuer",
            ServiceType::Overload => "Overload",
            ServiceType::Faucet => "Faucet",
            ServiceType::TransactionManager => "TransactionManager",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UnknownService" => Some(Self::UnknownService),
            "Gateway" => Some(Self::Gateway),
            "Notary" => Some(Self::Notary),
            "Authority" => Some(Self::Authority),
            "Treasury" => Some(Self::Treasury),
            "Emissary" => Some(Self::Emissary),
            "Shared" => Some(Self::Shared),
            "Identity" => Some(Self::Identity),
            "Vault" => Some(Self::Vault),
            "CustodialWallet" => Some(Self::CustodialWallet),
            "Issuer" => Some(Self::Issuer),
            "Overload" => Some(Self::Overload),
            "Faucet" => Some(Self::Faucet),
            "TransactionManager" => Some(Self::TransactionManager),
            _ => None,
        }
    }
}
/// Used to denote the cryptographic signing & verifying algorithm associated with a particular digital signature or
/// verifier (public key).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignatureSystem {
    /// The unspecified value is used to guard against accidental use of a zero value.
    Unspecified = 0,
    /// The ED25519 signature system
    Ed25519 = 1,
    /// The SECP2516 signature system
    Secp256k1 = 2,
}
impl SignatureSystem {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SignatureSystem::Unspecified => "SIGNATURE_SYSTEM_UNSPECIFIED",
            SignatureSystem::Ed25519 => "SIGNATURE_SYSTEM_ED25519",
            SignatureSystem::Secp256k1 => "SIGNATURE_SYSTEM_SECP256K1",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGNATURE_SYSTEM_UNSPECIFIED" => Some(Self::Unspecified),
            "SIGNATURE_SYSTEM_ED25519" => Some(Self::Ed25519),
            "SIGNATURE_SYSTEM_SECP256K1" => Some(Self::Secp256k1),
            _ => None,
        }
    }
}
/// Logical Operator Options.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogicalOperator {
    /// Logical Operator Unspecified.
    Unspecified = 0,
    /// Logical Operator AND.
    And = 1,
    /// Logical Operator OR.
    Or = 2,
}
impl LogicalOperator {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LogicalOperator::Unspecified => "LOGICAL_OPERATOR_UNSPECIFIED",
            LogicalOperator::And => "LOGICAL_OPERATOR_AND",
            LogicalOperator::Or => "LOGICAL_OPERATOR_OR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOGICAL_OPERATOR_UNSPECIFIED" => Some(Self::Unspecified),
            "LOGICAL_OPERATOR_AND" => Some(Self::And),
            "LOGICAL_OPERATOR_OR" => Some(Self::Or),
            _ => None,
        }
    }
}
/// Sorting Direction, i.e. ascending or descending.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortDirection {
    /// Sort Direction Unspecified.
    Unspecified = 0,
    /// Sort Direction Ascending, i.e. Low to High
    Ascending = 1,
    /// Sort Direction Ascending, i.e. High to Low
    Descending = 2,
}
impl SortDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SortDirection::Unspecified => "SORT_DIRECTION_UNSPECIFIED",
            SortDirection::Ascending => "SORT_DIRECTION_ASCENDING",
            SortDirection::Descending => "SORT_DIRECTION_DESCENDING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SORT_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "SORT_DIRECTION_ASCENDING" => Some(Self::Ascending),
            "SORT_DIRECTION_DESCENDING" => Some(Self::Descending),
            _ => None,
        }
    }
}
// IsProvisionedRequest - Request the service to determine if it is provisioned to perform its duties

/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsProvisionedRequest {
}
// IsProvisionedResponse - Respond with the provision status of the service

/// \[Example\]
/// {
/// "status": 1,
/// "reason": "REASON"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsProvisionedResponse {
    /// Provisioning Status.
    #[prost(enumeration="is_provisioned_response::Status", tag="1")]
    pub status: i32,
    /// Provisioning Status Reason.
    #[prost(string, tag="2")]
    pub reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `IsProvisionedResponse`.
pub mod is_provisioned_response {
    /// Status of Provisioned Service.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Unknown Provisioning Status.
        Unknown = 0,
        /// Provisioned.
        Provisioned = 1,
        /// Not Provisioned.
        NotProvisioned = 2,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unknown => "STATUS_UNKNOWN",
                Status::Provisioned => "STATUS_PROVISIONED",
                Status::NotProvisioned => "STATUS_NOT_PROVISIONED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNKNOWN" => Some(Self::Unknown),
                "STATUS_PROVISIONED" => Some(Self::Provisioned),
                "STATUS_NOT_PROVISIONED" => Some(Self::NotProvisioned),
                _ => None,
            }
        }
    }
}
// Streaming data packet used across Gateway connections.

/// \[Example\]
/// {
/// "packet_type": 100,
/// "data": "LengthVariesBasedOnData=",
/// "channel": "YOUR_CHANNEL",
/// "sender": {
/// "signature_system": 1,
/// "verifier": "BytesLengthOfSenderVerifierVariesByAlgo="
/// },
/// "recipient": {
/// "signature_system": 1,
/// "verifier": "AlgoVariesRecipientVerifierBytesLen="
/// },
/// "nonce": "NonceBytesLenVaryByAlgo=",
/// "transaction_header": {
/// "starter": {
/// "signature_system": 1,
/// "verifier": "BytesLenStarterVerifierVariesByAlgo="
/// },
/// "responder": {
/// "signature_system": 1,
/// "verifier": "AlgoVariesResponderVerifierBytesLen="
/// },
/// "uetr": "UETR",
/// "timestamp": "2006-01-02T15:04:05Z",
/// "signature": "ByteLengthSignatureVariesByAlgo=",
/// "user_reference": "USER_REFERENCE",
/// "related_uetrs": [
/// "RELATED_UETR1",
/// "RELATED_UETR2"
/// ]
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Packet {
    /// Packet type for determining deserialization scheme and processing
    #[prost(enumeration="PacketType", tag="1")]
    pub packet_type: i32,
    /// Raw byte data corresponding to message type
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Unique channel identifier
    #[prost(string, tag="3")]
    pub channel: ::prost::alloc::string::String,
    /// Verifier of sender
    #[prost(message, optional, tag="4")]
    pub sender: ::core::option::Option<DynamicVerifier>,
    /// Verifier of recipient
    #[prost(message, optional, tag="5")]
    pub recipient: ::core::option::Option<DynamicVerifier>,
    /// Nonce used to guard against replay attacks
    #[prost(bytes="vec", tag="6")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    /// optional transaction header
    #[prost(message, optional, tag="7")]
    pub transaction_header: ::core::option::Option<TransactionHeader>,
}
// Encasulates all cross-network metadata that associates
// a `Packet` with a given transaction.

/// \[Example\]
/// {
/// "starter": {
/// "signature_system": 1,
/// "verifier": "BytesLenStarterVerifierVariesByAlgo="
/// },
/// "responder": {
/// "signature_system": 1,
/// "verifier": "AlgoVariesResponderVerifierBytesLen="
/// },
/// "uetr": "UETR",
/// "timestamp": "2006-01-02T15:04:05Z",
/// "signature": "LengthSignatureVariesByAlgo=",
/// "user_reference": "USER_REFERENCE",
/// "related_uetrs": [
/// "RELATED_UETR1",
/// "RELATED_UETR2"
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionHeader {
    /// Verifier of the entity that started the transaction process.
    #[prost(message, optional, tag="1")]
    pub starter: ::core::option::Option<DynamicVerifier>,
    /// Verifier of the entity which is responding to the transaction processes
    /// that was initiated by the starter.
    #[prost(message, optional, tag="2")]
    pub responder: ::core::option::Option<DynamicVerifier>,
    /// Universal e2e transaction reference. Term used for UUIDv4 in financial
    /// messaging such as ISO20022.
    #[prost(string, tag="3")]
    pub uetr: ::prost::alloc::string::String,
    /// Seconds (minus leapsconds) since epoch 1970 - Unix timestamp - always
    /// interpreted as being in the UTC timezone. Internal operations will convert
    /// this to UTC time.
    #[prost(uint64, tag="4")]
    pub timestamp: u64,
    /// Signature on all fields in Transaction header (except signature field
    /// itself) and Routing header + (sha256 on serialized(payload).
    /// TODO investigate viability
    /// should be sent by the emitter of the transaction
    #[prost(bytes="vec", tag="5")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// User reference string for transactions acting as a memo field.
    #[prost(string, tag="6")]
    pub user_reference: ::prost::alloc::string::String,
    /// Related uetrs 0..N.
    #[prost(string, repeated, tag="7")]
    pub related_uetrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// \[Example\]
/// {
/// "promissory_batch": [
/// "LengthOfFileBytesVaries=",
/// "LengthOfFileBytesVaries=",
/// "LengthOfFileBytesVaries="
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchedPromissory {
    /// List of transferred promissory bytes. These are expected to be the
    /// flatbuffer-serialized bytes of a Promissory, not protobuf.
    #[prost(bytes="vec", repeated, tag="1")]
    pub promissory_batch: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
// Sent by a Simple Payment starter allowing a responder to
// accept or reject a transfer of funds.

/// \[Example\]
/// {
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimplePaymentRequest {
    /// Amount in simple payment.
    #[prost(message, optional, tag="1")]
    pub amount: ::core::option::Option<Amount>,
}
// Sent by a Simple Payment responder, signals readiness to accept funds.

/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimplePaymentAck {
}
// Sent by a Simple Payment responder, indicates a rejection to accept funds.

/// \[Example\]
/// {
/// "rejection_reason": 1,
/// "other": "OTHER"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimplePaymentNack {
    /// Rejection Reason.
    #[prost(enumeration="simple_payment_nack::RejectionReason", tag="1")]
    pub rejection_reason: i32,
    /// If Rejection Reason "Other", Message
    #[prost(string, tag="2")]
    pub other: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SimplePaymentNack`.
pub mod simple_payment_nack {
    /// Simple Payment Rejection Reason.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RejectionReason {
        /// Default enum field
        ReasonUnknown = 0,
        /// "Other" e.g. sanctions blocked
        ReasonOther = 1,
        /// request amount field was empty or value set to zero
        ReasonAmountEmpty = 2,
        /// Amounts being remitted do not match a previous ChangeRequest (if part of
        /// a change request)
        ReasonChangeRequestAmountMismatch = 3,
    }
    impl RejectionReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RejectionReason::ReasonUnknown => "REASON_UNKNOWN",
                RejectionReason::ReasonOther => "REASON_OTHER",
                RejectionReason::ReasonAmountEmpty => "REASON_AMOUNT_EMPTY",
                RejectionReason::ReasonChangeRequestAmountMismatch => "REASON_CHANGE_REQUEST_AMOUNT_MISMATCH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_UNKNOWN" => Some(Self::ReasonUnknown),
                "REASON_OTHER" => Some(Self::ReasonOther),
                "REASON_AMOUNT_EMPTY" => Some(Self::ReasonAmountEmpty),
                "REASON_CHANGE_REQUEST_AMOUNT_MISMATCH" => Some(Self::ReasonChangeRequestAmountMismatch),
                _ => None,
            }
        }
    }
}
// * Sent by the responder.
// * Paired with a `PacketType::SPStatus`.
// * Always sent after the end of a simple payment flow
// * Acts as confirmation of full or partial funds transfer

/// \[Example\]
/// {
/// "received_amount": {
/// "currency_code":"USD",
/// "amount":500,
/// "decimals":2
/// },
/// "received_promissories": [
/// "RECEIVED_PROMISSORY_1",
/// "RECEIVED_PROMISSORY_2",
/// "RECEIVED_PROMISSORY_3"
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimplePaymentResponse {
    /// Amount received.
    #[prost(message, optional, tag="1")]
    pub received_amount: ::core::option::Option<Amount>,
    /// The first three characters are often the ISO-4217 currency
    /// code (though not always).
    ///
    /// The second section, delimited by `-`, is the amount
    /// represented as a unsigned integer expanded decimal format.
    ///
    /// The third section, delimited by `-`, is the decimal precision
    /// of the amount. In the example above, the amount of the note
    /// is one U.S. dollar and zero cents, i.e. $1.00.
    ///
    /// The fourth section, delimited by `-`, is the prefix
    /// This is the fixed value `C3-A6`.
    /// ex: USD-100-2-C3-A6-DE8F1D7B-3322-4BE9-AD61-9CB801E748C1
    ///
    /// "<currency>-<amount>-<decimals>-<protocol>-<id>"
    #[prost(string, repeated, tag="2")]
    pub received_promissories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// Sent by a Funds Change starter allowing a responder to
// accept or reject a transfer of funds.

/// \[Example\]
/// {
/// "gross_amount": {
/// "currency_code":"USD",
/// "amount":500,
/// "decimals":2
/// },
/// "exact_amount": {
/// "currency_code":"USD",
/// "amount":499,
/// "decimals":2
/// },
/// "fees": {
/// "currency_code":"USD",
/// "amount":1,
/// "decimals":2
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundsChangeRequest {
    /// Gross Amount Requested
    #[prost(message, optional, tag="1")]
    pub gross_amount: ::core::option::Option<Amount>,
    /// Exact Amount Requested
    #[prost(message, optional, tag="2")]
    pub exact_amount: ::core::option::Option<Amount>,
    /// Fees
    #[prost(message, optional, tag="3")]
    pub fees: ::core::option::Option<Amount>,
}
// Sent by a Funds Change responder, signals readiness to accept funds.

/// \[Example\]
/// {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundsChangeAck {
}
// Sent by a Funds Change responder, indicates a rejection to accept funds.

/// \[Example\]
/// {
/// "rejection_reason": 0,
/// "other": "OTHER"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundsChangeNack {
    /// Rejection Reason.
    #[prost(enumeration="funds_change_nack::RejectionReason", tag="1")]
    pub rejection_reason: i32,
    /// If Rejection Reason "Other", Message
    #[prost(string, tag="2")]
    pub other: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FundsChangeNack`.
pub mod funds_change_nack {
    /// Funds Change Rejection Reason.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RejectionReason {
        /// Default enum field
        ReasonUnknown = 0,
        /// "Other" e.g. sanctions blocked
        ReasonOther = 1,
        /// request amount field was empty or value set to zero
        ReasonAmountEmpty = 2,
        /// GrossAmount - Fees must be >= ExactAmount, else its not possible to get
        /// change
        ReasonAmountsMismatch = 3,
    }
    impl RejectionReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RejectionReason::ReasonUnknown => "REASON_UNKNOWN",
                RejectionReason::ReasonOther => "REASON_OTHER",
                RejectionReason::ReasonAmountEmpty => "REASON_AMOUNT_EMPTY",
                RejectionReason::ReasonAmountsMismatch => "REASON_AMOUNTS_MISMATCH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_UNKNOWN" => Some(Self::ReasonUnknown),
                "REASON_OTHER" => Some(Self::ReasonOther),
                "REASON_AMOUNT_EMPTY" => Some(Self::ReasonAmountEmpty),
                "REASON_AMOUNTS_MISMATCH" => Some(Self::ReasonAmountsMismatch),
                _ => None,
            }
        }
    }
}
// Signal meant to terminate a transaction that is in progress.

/// \[Example\]
/// {
/// "rejection_reason": 0,
/// "other": "OTHER"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelTransaction {
    /// Rejection Reason.
    #[prost(enumeration="cancel_transaction::RejectionReason", tag="1")]
    pub rejection_reason: i32,
    /// If Rejection Reason "Other", Message
    #[prost(string, tag="2")]
    pub other: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CancelTransaction`.
pub mod cancel_transaction {
    /// Reason for Cancelled Transaction
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RejectionReason {
        /// Default enum field
        ReasonUnknown = 0,
        /// "Other" e.g. sanctions blocked
        ReasonOther = 1,
        /// One or more related Transactions had a mismatch between them
        ReasonMismatchedTransactions = 2,
    }
    impl RejectionReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RejectionReason::ReasonUnknown => "REASON_UNKNOWN",
                RejectionReason::ReasonOther => "REASON_OTHER",
                RejectionReason::ReasonMismatchedTransactions => "REASON_MISMATCHED_TRANSACTIONS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_UNKNOWN" => Some(Self::ReasonUnknown),
                "REASON_OTHER" => Some(Self::ReasonOther),
                "REASON_MISMATCHED_TRANSACTIONS" => Some(Self::ReasonMismatchedTransactions),
                _ => None,
            }
        }
    }
}
// Represents a repeatable condition that `Contract` signatories agree must
// come to pass as a condition of `Contract` fulfillment.

/// \[Example\]
/// {
/// "sender": "zSenderPublicKey",
/// "recipient": "zRecipientPublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commitment {
    /// Sender Public Key
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// Recipient Public Key
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    /// Amount of Commitment
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<Amount>,
}
// Represents a condition on the `Contract` to expire and rollback the contract
// transaction if it is not fulfilled by the given timeout timestamp.

/// \[Example\]
/// {
/// "timeout": 1687180000
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeoutCondition {
    /// Unix timestamp of timeout datetime.
    #[prost(uint64, tag="1")]
    pub timeout: u64,
}
// Represents a condition on the `Contract` to require a secret to be presented
// that matches the given hash before fund transfers are fully committed.

/// \[Example\]
/// {
/// "presenter": "PresenterPublicKey",
/// "hash": "13550350a8681c84"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HashCondition {
    /// Public key of the participant that will present the secret.
    #[prost(string, tag="1")]
    pub presenter: ::prost::alloc::string::String,
    /// Hash of the secret, populated by the presenter during contract
    /// negotiation.
    #[prost(bytes="vec", tag="2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// OID of algorithm used to generate hash.
    #[prost(bytes="vec", tag="3")]
    pub algorithm_oid: ::prost::alloc::vec::Vec<u8>,
}
// Represents one of the possible conditions applied to the `Contract`.

/// \[Example\]
/// {
/// "condition": {
/// "hash": {
/// "presenter": "PresenterPublicKey",
/// "hash": "13550350a8681c84"
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    /// Condition Options.
    #[prost(oneof="condition::Condition", tags="1, 2")]
    pub condition: ::core::option::Option<condition::Condition>,
}
/// Nested message and enum types in `Condition`.
pub mod condition {
    /// Condition Options.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Condition {
        /// Timeout Condition Option.
        #[prost(message, tag="1")]
        Timeout(super::TimeoutCondition),
        /// Hash Condition Option.
        #[prost(message, tag="2")]
        Hash(super::HashCondition),
    }
}
// * Represents a series of `Commitment`s that once signed by all signatories
// will mark the contract as fulfilled.
// * For a `Contract` to be valid: all signatories must have their respective
// public key mapped to a valid signature in the `signatories` map.
// * The signature is generated by a signatory signing the _final_ `Contract`
// message with an empty `signatories` field.
// * Signatories include but are not limited to all participants in all
// included `Commitment`s.

/// \[Example\]
/// {
/// "urn": "urn:knox:swap:001",
/// "id": "CONTRACT_ID",
/// "commitments": [
/// {
/// "sender": "zSender1PublicKey",
/// "recipient": "zRecipient1PublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// },
/// {
/// "sender": "zSender2PublicKey",
/// "recipient": "zRecipient2PublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// }
/// ],
/// "conditions": [
/// {
/// "condition": {
/// "timeout": 1687180000
/// }
/// }
/// ],
/// "signatories": {
/// "Key1": "BYTES=",
/// "Key2": "BYTES="
/// },
/// "memo": "AtomicAssetSwap"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    /// URN describing contract type.
    #[prost(string, tag="1")]
    pub urn: ::prost::alloc::string::String,
    /// Contract ID.
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// List of contract commitments.
    #[prost(message, repeated, tag="3")]
    pub commitments: ::prost::alloc::vec::Vec<Commitment>,
    /// List of contract conditions.
    #[prost(message, repeated, tag="4")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    /// map<pk_signer, signature>
    #[prost(map="string, bytes", tag="5")]
    pub signatories: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    /// Human-meaningful message about the purpose of this contract.
    #[prost(string, tag="6")]
    pub memo: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "distribution": {
/// "value": []
/// },
/// "decimals": 2,
/// "currency_code": "USD",
/// "issuer": {
/// "signature_system": 1,
/// "verifier": "BytesLengthOfSenderVerifierVariesByAlgo="
/// },
/// "recipient": {
/// "signature_system": 1,
/// "verifier": "AlgoVariesRecipientVerifierBytesLen="
/// },
/// "signature_system": 1
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintRequest {
    /// Distribution of file denominations.
    #[prost(message, optional, tag="1")]
    pub distribution: ::core::option::Option<Distribution>,
    /// Decimals.
    #[prost(uint32, tag="2")]
    pub decimals: u32,
    /// Currency code.
    #[prost(string, tag="3")]
    pub currency_code: ::prost::alloc::string::String,
    /// Dynamic Verifier of the Issuer.
    #[prost(message, optional, tag="4")]
    pub issuer: ::core::option::Option<DynamicVerifier>,
    /// Dynamic Verifier of the Recipient.
    #[prost(message, optional, tag="5")]
    pub recipient: ::core::option::Option<DynamicVerifier>,
    /// Signature System.
    #[prost(enumeration="SignatureSystem", tag="6")]
    pub signature_system: i32,
}
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

/// Describes the `bytes` field contents of the parent `Packet` object.
/// Used in routing and decoding.
///
/// ---------------------------
/// Gateway Control Flow Packets 0..99
/// ---------------------------
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PacketType {
    /// Unspecified packet type
    /// default behavior is to drop the packet
    Unknown = 0,
    /// Unsupported packet
    /// default behavior is to return the packet
    Unsupported = 1,
    /// Authenticate connection
    /// Used to verify the verifier public key ownership
    /// corresponds to `AuthenticationRequest` message
    Authenticate = 2,
    /// Return to sender when unable to route packet
    /// default behavior is to inverse the sender and recipient fields
    ReturnToSender = 3,
    /// Terminate data stream
    /// TODO: decide if this is necessary after "Acknowledgement" type is implemented
    TerminateStream = 4,
    /// Life signal packet
    /// Used to signal the gateway and/or client that the connection is still alive
    KeepAlive = 5,
    /// Acknowledgement of packet
    /// Used by a recipient to signal to the sender of the a packet that it has been received.
    Acknowledgement = 6,
    /// Authentication Request
    /// Used to request the verifier public key ownership
    AuthenticateRequest = 7,
    /// Authentication Response
    /// Used to respond to the verifier public key ownership authentication request
    AuthenticateResponse = 8,
    /// --- ConnectionPool flow ---
    /// Used to indicate an associated identity addition
    AddAssociatedConnection = 9,
    /// Used to indicate an associated identity drop
    DropAssociatedConnection = 10,
    // ---------------------------
    // Promissory Packets 100..199
    // ---------------------------

    /// Send Promissory to recipient request
    /// This is a direct end-to-end message, and will not Authorize the promissory
    Promissory = 100,
    /// Authorize Promissory Request
    /// Request an authorized signature for a promissory and return to the sender.
    AuthorizePromissory = 101,
    /// Authorize and Forward Promissory
    /// Request an authorized signature for a promissory and forward to the recipient.
    AuthorizeAndForwardPromissory = 102,
    /// Archive Promissory Request
    /// After a promissory file is removed from circulation archive the promissory file.
    ArchivePromissory = 103,
    /// Promissory receipt acknowledgement
    /// Respond with promissory receipt to a promissory request
    /// TODO: Decide if this msg or a general acknowledgement message should be used.
    PromissoryReceipt = 104,
    /// Authorization Status
    /// Response from Notary to sender of an `AuthorizeLockedPromissories` message. Indicates
    /// the notary has completed authorization.
    AuthorizationStatus = 108,
    /// Retrieve Assets
    /// Request to have a notary send Promissory packets back to the requester for each
    /// promissory related to a given transaction and owned by the requester.
    RetrieveAssets = 109,
    /// PromissoryBatch
    /// Send a batch of Promissory to recipient.
    PromissoryBatch = 110,
    /// Lock Promissory Batch
    /// Request to TM to submit transferred promissories toward a contract-based transaction.
    LockPromissoryBatch = 111,
    /// Lock Promissory Batch Ack
    /// Notice from TM that the batch of promissories was submitted.
    LockPromissoryBatchAck = 112,
    /// Retrieve Assets Batch
    /// Request to have a notary send PromissoryBatch packets back to the requester for each
    /// promissory related to a given transaction and owned by the requester.
    RetrieveAssetsBatch = 113,
    /// Authorize and Forward Promissory Batch
    /// Request an authorized signature for a promissory batch and forward to the recipients.
    AuthorizeAndForwardPromissoryBatch = 116,
    /// Validation Block Batch
    /// Request an authorized signature for a ValidationBlockBatch and reply with an AuthorizedBlockBatch.
    ValidationBlockBatch = 117,
    /// Authorized Block Batch
    /// Response to a ValidationBlockBatch which contains a batch of AuthorizedBlock.
    AuthorizedBlockBatch = 118,
    /// Request to the authority to mint new promissories
    MintPromissory = 119,
    /// Represents a batch of newly-minted and notarized promissories sent to their final recipient
    FreshPromissoryBatch = 120,
    // ---------------------------
    // Identity Packets 200..299
    // ---------------------------

    /// Identity Request
    /// Request identity of a verifier in the network
    /// Corresponds to `IdentityRequest` message
    IdentityRequest = 200,
    /// Identity Response
    /// Response to identity request
    /// Corresponds to `IdentityResponse` message
    IdentityResponse = 201,
    /// Identity proof
    /// Identity proof to be added to subject DID document
    /// Corresponds to `IdentityProof`
    IdentityProof = 202,
    // ---------------------------
    // Notary Packets 300..399
    // ---------------------------

    /// AuthorizationFailure
    /// Response from the Notary in the even of a failure to Notarize.
    AuthorizationFailed = 300,
    // ---------------------------
    // Emissary Packets 400..499
    // ---------------------------

    /// EmissaryConnectionIdentity
    /// Identity of a service connection
    /// Corresponds to `EmissaryConnectionIdentityRequest` message
    EmissaryConnectionIdentity = 400,
    // ---------------------------
    // Contract Packets 500..599
    // ---------------------------

    /// Contract Commitment
    /// Update a contract with the addition of a commitment
    ContractProposal = 500,
    /// Contract Ack
    /// Acknowledgement to participate in contract
    ContractAck = 501,
    /// Contract Nack
    /// Acknowledgement to reject a contract
    ContractNack = 502,
    /// Contract Signature
    /// Updates a contract with the addition of a signature
    ContractSignature = 503,
    /// Holds a finalized contract in the packet data field
    ContractComplete = 504,
    // ---------------------------
    // Treasury Packets 700..799
    // ---------------------------

    /// Remittance Request
    /// Corresponds to `RemittanceRequest` message
    /// Used by Treasury service to remit payments
    RemittanceRequest = 700,
    /// Remittance Response
    /// Corresponds to `RemittanceResponse` message
    /// Response from Treasury service as a payment response
    RemittanceResponse = 701,
    // ---------------------------
    // Transaction Packets 1000..1099
    // ---------------------------

    /// Payer instruction to notify the payee of a payment comprising a number of
    /// CPFs. Checks the existence of Payee and in future allows blocking a
    /// transaction that would otherwise be sanctioned, before any CPF ownership is
    /// transferred.
    /// SimplePaymentRequest
    SpRequest = 1000,
    /// Payee acknowledgement of the SimplePayment, also confirms the existence of
    /// the Payee.
    /// SimplePaymentAck
    SpAck = 1001,
    /// Payee acknowledgement of the SimplePayment, also confirms the existence of
    /// the Payee.
    /// SimplePaymentNack
    SpNack = 1002,
    /// Indicates status of the transfer from the recipient (payee)
    /// SimplePaymentResponse
    SpStatus = 1003,
    /// StartSimplePaymentRequest
    SpStart = 1004,
    // ---------------------------
    // FundsChange Packets 1100..1199
    // ---------------------------

    /// Sender requests Treasury to provide change that can be used to make a
    /// specific amount payment
    FcRequest = 1100,
    /// Treasury acceptance of the request, indicating willingness to provide the
    /// change.
    FcAck = 1101,
    /// Treasury is unable to provide change, Error code in payload will provide
    /// details
    FcNack = 1102,
    // ---------------------------
    // ContractPayment Packets 1300..1399
    // ---------------------------

    /// Payer instruction to notify the payee of a contract payment comprising a
    /// number of CPFs.
    /// ContractPaymentRequest
    CpRequest = 1300,
    /// Payee acknowledgement of the contract payment, also confirms the existence
    /// of the Payee.
    /// ContractPaymentAck
    CpAck = 1301,
    /// Payee acknowledgement of the contract payment, also confirms the existence
    /// of the Payee.
    /// ContractPaymentNack
    CpNack = 1302,
    /// Payer instruction to the Transaction Manager to start a new contract
    /// payment using a signed contract.
    /// ContractPaymentContract
    CpContract = 1303,
    /// Transaction Manager acknowledgement of the contract payment using the
    /// provided signed contract.
    /// ContractPaymentContract
    CpContractAck = 1304,
    /// Transaction Manager acknowledgement to the payee that the assets have been
    /// authorized and are ready to be retrieved.
    /// ContractPaymentAssetsAuthorized
    CpAssetsAuthorized = 1305,
    /// Transaction Manager acknowledgement that the contract payment is complete.
    /// ContractPaymentComplete
    CpComplete = 1306,
    /// Message sent from the Transaction Manager to the Hash Provider in a HTLC contract. This
    /// indicates a request for the recipient to provide the secret.
    /// RequestSecret
    RequestSecret = 1308,
    /// Message sent from a participant in response to receiving a `RequestSecret` packet
    /// PresentSecret
    PresentSecret = 1309,
    /// Message sent by the Transaction Manager to the contract participants in the event
    /// the transaction was unable to complete.
    /// CPAssetsReturned
    CpAssetsReturned = 1310,
}
impl PacketType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PacketType::Unknown => "Unknown",
            PacketType::Unsupported => "Unsupported",
            PacketType::Authenticate => "Authenticate",
            PacketType::ReturnToSender => "ReturnToSender",
            PacketType::TerminateStream => "TerminateStream",
            PacketType::KeepAlive => "KeepAlive",
            PacketType::Acknowledgement => "Acknowledgement",
            PacketType::AuthenticateRequest => "AuthenticateRequest",
            PacketType::AuthenticateResponse => "AuthenticateResponse",
            PacketType::AddAssociatedConnection => "AddAssociatedConnection",
            PacketType::DropAssociatedConnection => "DropAssociatedConnection",
            PacketType::Promissory => "Promissory",
            PacketType::AuthorizePromissory => "AuthorizePromissory",
            PacketType::AuthorizeAndForwardPromissory => "AuthorizeAndForwardPromissory",
            PacketType::ArchivePromissory => "ArchivePromissory",
            PacketType::PromissoryReceipt => "PromissoryReceipt",
            PacketType::AuthorizationStatus => "AuthorizationStatus",
            PacketType::RetrieveAssets => "RetrieveAssets",
            PacketType::PromissoryBatch => "PromissoryBatch",
            PacketType::LockPromissoryBatch => "LockPromissoryBatch",
            PacketType::LockPromissoryBatchAck => "LockPromissoryBatchAck",
            PacketType::RetrieveAssetsBatch => "RetrieveAssetsBatch",
            PacketType::AuthorizeAndForwardPromissoryBatch => "AuthorizeAndForwardPromissoryBatch",
            PacketType::ValidationBlockBatch => "ValidationBlockBatch",
            PacketType::AuthorizedBlockBatch => "AuthorizedBlockBatch",
            PacketType::MintPromissory => "MintPromissory",
            PacketType::FreshPromissoryBatch => "FreshPromissoryBatch",
            PacketType::IdentityRequest => "IdentityRequest",
            PacketType::IdentityResponse => "IdentityResponse",
            PacketType::IdentityProof => "IdentityProof",
            PacketType::AuthorizationFailed => "AuthorizationFailed",
            PacketType::EmissaryConnectionIdentity => "EmissaryConnectionIdentity",
            PacketType::ContractProposal => "ContractProposal",
            PacketType::ContractAck => "ContractAck",
            PacketType::ContractNack => "ContractNack",
            PacketType::ContractSignature => "ContractSignature",
            PacketType::ContractComplete => "ContractComplete",
            PacketType::RemittanceRequest => "RemittanceRequest",
            PacketType::RemittanceResponse => "RemittanceResponse",
            PacketType::SpRequest => "SPRequest",
            PacketType::SpAck => "SPAck",
            PacketType::SpNack => "SPNack",
            PacketType::SpStatus => "SPStatus",
            PacketType::SpStart => "SPStart",
            PacketType::FcRequest => "FCRequest",
            PacketType::FcAck => "FCAck",
            PacketType::FcNack => "FCNack",
            PacketType::CpRequest => "CPRequest",
            PacketType::CpAck => "CPAck",
            PacketType::CpNack => "CPNack",
            PacketType::CpContract => "CPContract",
            PacketType::CpContractAck => "CPContractAck",
            PacketType::CpAssetsAuthorized => "CPAssetsAuthorized",
            PacketType::CpComplete => "CPComplete",
            PacketType::RequestSecret => "RequestSecret",
            PacketType::PresentSecret => "PresentSecret",
            PacketType::CpAssetsReturned => "CPAssetsReturned",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unknown" => Some(Self::Unknown),
            "Unsupported" => Some(Self::Unsupported),
            "Authenticate" => Some(Self::Authenticate),
            "ReturnToSender" => Some(Self::ReturnToSender),
            "TerminateStream" => Some(Self::TerminateStream),
            "KeepAlive" => Some(Self::KeepAlive),
            "Acknowledgement" => Some(Self::Acknowledgement),
            "AuthenticateRequest" => Some(Self::AuthenticateRequest),
            "AuthenticateResponse" => Some(Self::AuthenticateResponse),
            "AddAssociatedConnection" => Some(Self::AddAssociatedConnection),
            "DropAssociatedConnection" => Some(Self::DropAssociatedConnection),
            "Promissory" => Some(Self::Promissory),
            "AuthorizePromissory" => Some(Self::AuthorizePromissory),
            "AuthorizeAndForwardPromissory" => Some(Self::AuthorizeAndForwardPromissory),
            "ArchivePromissory" => Some(Self::ArchivePromissory),
            "PromissoryReceipt" => Some(Self::PromissoryReceipt),
            "AuthorizationStatus" => Some(Self::AuthorizationStatus),
            "RetrieveAssets" => Some(Self::RetrieveAssets),
            "PromissoryBatch" => Some(Self::PromissoryBatch),
            "LockPromissoryBatch" => Some(Self::LockPromissoryBatch),
            "LockPromissoryBatchAck" => Some(Self::LockPromissoryBatchAck),
            "RetrieveAssetsBatch" => Some(Self::RetrieveAssetsBatch),
            "AuthorizeAndForwardPromissoryBatch" => Some(Self::AuthorizeAndForwardPromissoryBatch),
            "ValidationBlockBatch" => Some(Self::ValidationBlockBatch),
            "AuthorizedBlockBatch" => Some(Self::AuthorizedBlockBatch),
            "MintPromissory" => Some(Self::MintPromissory),
            "FreshPromissoryBatch" => Some(Self::FreshPromissoryBatch),
            "IdentityRequest" => Some(Self::IdentityRequest),
            "IdentityResponse" => Some(Self::IdentityResponse),
            "IdentityProof" => Some(Self::IdentityProof),
            "AuthorizationFailed" => Some(Self::AuthorizationFailed),
            "EmissaryConnectionIdentity" => Some(Self::EmissaryConnectionIdentity),
            "ContractProposal" => Some(Self::ContractProposal),
            "ContractAck" => Some(Self::ContractAck),
            "ContractNack" => Some(Self::ContractNack),
            "ContractSignature" => Some(Self::ContractSignature),
            "ContractComplete" => Some(Self::ContractComplete),
            "RemittanceRequest" => Some(Self::RemittanceRequest),
            "RemittanceResponse" => Some(Self::RemittanceResponse),
            "SPRequest" => Some(Self::SpRequest),
            "SPAck" => Some(Self::SpAck),
            "SPNack" => Some(Self::SpNack),
            "SPStatus" => Some(Self::SpStatus),
            "SPStart" => Some(Self::SpStart),
            "FCRequest" => Some(Self::FcRequest),
            "FCAck" => Some(Self::FcAck),
            "FCNack" => Some(Self::FcNack),
            "CPRequest" => Some(Self::CpRequest),
            "CPAck" => Some(Self::CpAck),
            "CPNack" => Some(Self::CpNack),
            "CPContract" => Some(Self::CpContract),
            "CPContractAck" => Some(Self::CpContractAck),
            "CPAssetsAuthorized" => Some(Self::CpAssetsAuthorized),
            "CPComplete" => Some(Self::CpComplete),
            "RequestSecret" => Some(Self::RequestSecret),
            "PresentSecret" => Some(Self::PresentSecret),
            "CPAssetsReturned" => Some(Self::CpAssetsReturned),
            _ => None,
        }
    }
}
/// Represents the algorithm used to derive the hash value in the `HashCondition`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HashAlgorithm {
    Sha256 = 0,
    Sha512 = 1,
}
impl HashAlgorithm {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HashAlgorithm::Sha256 => "SHA256",
            HashAlgorithm::Sha512 => "SHA512",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SHA256" => Some(Self::Sha256),
            "SHA512" => Some(Self::Sha512),
            _ => None,
        }
    }
}
/// Contract type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractType {
    /// Status Unspecified.
    Unspecified = 0,
    /// Simple a-to-b payment.
    Payment = 1,
    /// Transfer between 2 or more parties.
    Swap = 2,
    /// Multi-party unidirectional payment.
    Relay = 3,
    /// Custom contract type.
    Custom = 4,
}
impl ContractType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContractType::Unspecified => "CONTRACT_TYPE_UNSPECIFIED",
            ContractType::Payment => "CONTRACT_TYPE_PAYMENT",
            ContractType::Swap => "CONTRACT_TYPE_SWAP",
            ContractType::Relay => "CONTRACT_TYPE_RELAY",
            ContractType::Custom => "CONTRACT_TYPE_CUSTOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTRACT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CONTRACT_TYPE_PAYMENT" => Some(Self::Payment),
            "CONTRACT_TYPE_SWAP" => Some(Self::Swap),
            "CONTRACT_TYPE_RELAY" => Some(Self::Relay),
            "CONTRACT_TYPE_CUSTOM" => Some(Self::Custom),
            _ => None,
        }
    }
}
/// \[Example\]
/// {
/// "payload_type": 1,
/// "data": "xDATAxBYTES="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payload {
    /// Payload type.
    #[prost(enumeration="PayloadType", tag="1")]
    pub payload_type: i32,
    /// Payload data.
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
///
/// {
/// "contract": {
/// "urn": "urn:knox:swap:001",
/// "id": "CONTRACT_ID",
/// "commitments": [
/// {
/// "sender": "zSender1PublicKey",
/// "recipient": "zRecipient1PublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// },
/// {
/// "sender": "zSender2PublicKey",
/// "recipient": "zRecipient2PublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// }
/// ],
/// "conditions": [
/// {
/// "condition": {
/// "timeout": 1687180000
/// }
/// }
/// ],
/// "signatories": {
/// "Key1": "BYTES=",
/// "Key2": "BYTES="
/// },
/// "memo": "AtomicAssetSwap"
/// },
/// "owner": "zOwnerPublicKey"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTransactionData {
    /// Contract.
    #[prost(message, optional, tag="1")]
    pub contract: ::core::option::Option<Contract>,
    /// Owner Public Key.
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
///
/// {
/// "contract": {
/// "urn": "urn:knox:swap:001",
/// "id": "CONTRACT_ID",
/// "commitments": [
/// {
/// "sender": "zSender1PublicKey",
/// "recipient": "zRecipient1PublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// },
/// {
/// "sender": "zSender2PublicKey",
/// "recipient": "zRecipient2PublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// }
/// ],
/// "conditions": [
/// {
/// "condition": {
/// "timeout": 1687180000
/// }
/// }
/// ],
/// "signatories": {
/// "Key1": "BYTES=",
/// "Key2": "BYTES="
/// },
/// "memo": "AtomicAssetSwap"
/// },
/// "owner": "zOwnerPublicKey",
/// "originator": "zOriginatorPublicKey"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AckProposalData {
    /// Contract.
    #[prost(message, optional, tag="1")]
    pub contract: ::core::option::Option<Contract>,
    /// Owner Public Key.
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// Originator Public Key.
    #[prost(string, tag="3")]
    pub originator: ::prost::alloc::string::String,
}
///
/// {
/// "owner": "zOwnerPublicKey",
/// "originator": "zOriginatorPublicKey",
/// "contract": {
/// "urn": "urn:knox:swap:001",
/// "id": "CONTRACT_ID",
/// "commitments": [
/// {
/// "sender": "zSender1PublicKey",
/// "recipient": "zRecipient1PublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// },
/// {
/// "sender": "zSender2PublicKey",
/// "recipient": "zRecipient2PublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// }
/// }
/// ],
/// "conditions": [
/// {
/// "condition": {
/// "timeout": 1687180000
/// }
/// }
/// ],
/// "signatories": {
/// "Key1": "BYTES=",
/// "Key2": "BYTES="
/// },
/// "memo": "AtomicAssetSwap"
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalAckNackData {
    /// Owner Public Key.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// Originator Public Key.
    #[prost(string, tag="2")]
    pub originator: ::prost::alloc::string::String,
    /// Contract.
    #[prost(message, optional, tag="3")]
    pub contract: ::core::option::Option<Contract>,
}
/// \[Example\]
/// {
/// "contract_id": "a18f7224-a7c1-41f8-b14f-75e920b8ea81",
/// "owner": "zOwnerPublicKey"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevealSecretData {
    /// Contract ID.
    #[prost(string, tag="1")]
    pub contract_id: ::prost::alloc::string::String,
    /// Owner Public Key.
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "owner": "zOwnerPublicKey",
/// "contract_id": "a18f7224-a7c1-41f8-b14f-75e920b8ea81",
/// "secret": "SecretBytes"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretRevealedData {
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
/// {
/// "contract_id": "a18f7224-a7c1-41f8-b14f-75e920b8ea81",
/// "owner": "zOwnerPublicKey",
/// "secret": "SecretBytes"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveSecretData {
    /// Contract ID.
    #[prost(string, tag="1")]
    pub contract_id: ::prost::alloc::string::String,
    /// Owner Public Key.
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// Base64 Encoding of Secret bytes.
    #[prost(string, tag="3")]
    pub secret: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "contract_id": "a18f7224-a7c1-41f8-b14f-75e920b8ea81",
/// "owner": "zOwnerPublicKey",
/// "result": "Completed"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveTerminationData {
    /// Contract ID.
    #[prost(string, tag="1")]
    pub contract_id: ::prost::alloc::string::String,
    /// Owner Public Key.
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// Result.
    #[prost(enumeration="TerminationResult", tag="3")]
    pub result: i32,
}
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

/// Payload Type, subject to change.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayloadType {
    /// New Transaction payload, subject to change.
    NewTransaction = 0,
    /// Ack Proposal payload, subject to change.
    AckProposal = 1,
    /// Acked Proposal payload, subject to change.
    ProposalAcked = 2,
    /// Nacked Proposal payload, subject to change.
    ProposalNacked = 3,
    /// Reveal Secret payload, subject to change.
    RevealSecret = 4,
    /// Revealed Secret payload, subject to change.
    SecretRevealed = 5,
    /// Received Secret payload, subject to change.
    ReceiveSecret = 6,
    /// Received Termination payload, subject to change.
    ReceiveTermination = 7,
}
impl PayloadType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PayloadType::NewTransaction => "NewTransaction",
            PayloadType::AckProposal => "AckProposal",
            PayloadType::ProposalAcked => "ProposalAcked",
            PayloadType::ProposalNacked => "ProposalNacked",
            PayloadType::RevealSecret => "RevealSecret",
            PayloadType::SecretRevealed => "SecretRevealed",
            PayloadType::ReceiveSecret => "ReceiveSecret",
            PayloadType::ReceiveTermination => "ReceiveTermination",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NewTransaction" => Some(Self::NewTransaction),
            "AckProposal" => Some(Self::AckProposal),
            "ProposalAcked" => Some(Self::ProposalAcked),
            "ProposalNacked" => Some(Self::ProposalNacked),
            "RevealSecret" => Some(Self::RevealSecret),
            "SecretRevealed" => Some(Self::SecretRevealed),
            "ReceiveSecret" => Some(Self::ReceiveSecret),
            "ReceiveTermination" => Some(Self::ReceiveTermination),
            _ => None,
        }
    }
}
/// Termination Result, subject to change.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TerminationResult {
    /// Contract Failed, subject to change.
    ContractFailed = 0,
    /// Transaction Failed, subject to change.
    TxnFailed = 1,
    /// Transaction Completed, subject to change.
    TxnCompleted = 2,
}
impl TerminationResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TerminationResult::ContractFailed => "ContractFailed",
            TerminationResult::TxnFailed => "TxnFailed",
            TerminationResult::TxnCompleted => "TxnCompleted",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ContractFailed" => Some(Self::ContractFailed),
            "TxnFailed" => Some(Self::TxnFailed),
            "TxnCompleted" => Some(Self::TxnCompleted),
            _ => None,
        }
    }
}
include!("common.tonic.rs");
// @@protoc_insertion_point(module)
