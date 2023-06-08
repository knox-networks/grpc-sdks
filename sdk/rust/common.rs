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
/// "value": [
/// {
/// "key": "KEY1",
/// "value": "VALUE1"
/// },
/// {
/// "key": "KEY2",
/// "value": "VALUE2"
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Distribution {
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
    /// Verifier of the requestor.
    #[prost(message, optional, tag="1")]
    pub verifier: ::core::option::Option<DynamicVerifier>,
}
// The returns a challenge nonce meant to be signed by the requestor's private key.

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
    /// Verifier of the requestor.
    #[prost(message, optional, tag="1")]
    pub verifier: ::core::option::Option<DynamicVerifier>,
    /// Cryptographic signature of the challenge nonce.
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
// Digital banknote to be authorized in the Notary service flow.

/// \[Example\]
/// {
/// "promissory_file": "FileByteLengthVaries="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedSignatureRequest {
    /// Digital banknote to authorize.
    #[prost(bytes="vec", tag="1")]
    pub promissory_file: ::prost::alloc::vec::Vec<u8>,
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
/// "verifier": "VerfierLengthVariesByAlgo",
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
    /// Issuer Role Permissions.
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
    #[prost(string, tag="1")]
    pub instance_id: ::prost::alloc::string::String,
}
// Initiates a Simple Payment transaction. Sender is implied to be the owner of the digital banknotes to be sent.

/// \[Example\]
/// {
/// "amount": {
/// "currency_code":"USD", 
/// "amount":100, 
/// "decimals":2
/// },
/// "recipient": "RECIPIENT",
/// "promissories": [
/// "PROMISSORY1",
/// "PROMISSORY2",
/// "PROMISSORY3"
/// ],
/// "user_reference": "USER_PROVIDED_REFERENCE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartSimplePaymentRequest {
    /// The amount of the payment inclusive of currency.
    #[prost(message, optional, tag="1")]
    pub amount: ::core::option::Option<Amount>,
    /// The recipient public key in multibase format.
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    /// Digital banknotes that make up the Simple Payment.
    #[prost(string, repeated, tag="3")]
    pub promissories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// User provided reference.
    #[prost(string, tag="4")]
    pub user_reference: ::prost::alloc::string::String,
}
// Returns the Universal E2E Transaction Reference (UUID v4 format) of a successfully initiated Simple Payment
// transaction.

/// \[Example\]
/// {
/// "uetr": "UETR"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartSimplePaymentResponse {
    /// Universal E2E Transaction Reference (UUID v4 format).
    #[prost(string, tag="1")]
    pub uetr: ::prost::alloc::string::String,
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
/// "promissory": "PROMISSORY1",
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
// Represents the status of a transaction with pertinent metadata attached.

/// \[Example\]
/// {
/// "status": 5, 
/// "promissories": [
/// "PROMISSORYxIDx1", 
/// "PROMISSORYxIDx2"
/// ], 
/// "amount_transferred": {
/// "currency_code":"USD", 
/// "amount":100, 
/// "decimals":2
/// }, 
/// "message": "MESSAGE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionSnapshot {
    /// Status of the transaction.
    #[prost(enumeration="TransactionStatus", tag="1")]
    pub status: i32,
    /// Digital banknote UUIDs.
    #[prost(string, repeated, tag="3")]
    pub promissories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Amount transferred so far.
    #[prost(message, optional, tag="4")]
    pub amount_transferred: ::core::option::Option<Amount>,
    /// Diagnostic message.
    #[prost(string, tag="5")]
    pub message: ::prost::alloc::string::String,
}
// Encapsulates a complete transaction object. Contains all required fields to decode a transaction state. This message
// only has local significance. On the wire messages use various packet types.

/// \[Example\]
/// {
/// "type": 1, 
/// "role": 0, 
/// "uetr": "UETR", 
/// "amount": {
/// "currency_code":"USD", 
/// "amount":100, 
/// "decimals":2
/// }, 
/// "created": "2006-01-02T15:04:05Z", 
/// "updated": "2007-01-02T15:04:05Z", 
/// "memo": "MEMO", 
/// "owner": "zOwnerPublicKeyMultibase58Encoded", 
/// "counterparty": "zCounterpartyPublicKeyMultibase58Encoded", 
/// "status": 0
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    /// The Type of the transaction.
    #[prost(enumeration="transaction::Type", tag="1")]
    pub r#type: i32,
    /// The role of the entity represented in this record.
    #[prost(enumeration="transaction::Role", tag="2")]
    pub role: i32,
    /// The Universal E2E Identifier (UUID v4) of this transaction.
    #[prost(string, tag="3")]
    pub uetr: ::prost::alloc::string::String,
    /// The Amount inclusive of currency code.
    #[prost(message, optional, tag="4")]
    pub amount: ::core::option::Option<Amount>,
    /// RFC 3339 Timestamp at which this entry was created.
    #[prost(string, tag="5")]
    pub created: ::prost::alloc::string::String,
    /// RFC 3339 Timestamp at which this entry was last updated.
    #[prost(string, tag="6")]
    pub updated: ::prost::alloc::string::String,
    /// User Reference field.
    #[prost(string, tag="7")]
    pub memo: ::prost::alloc::string::String,
    /// The entity owning this data record, a public key in multibase format. The initiator and responder each get their
    /// own separate entry, also distinguished by role.
    #[prost(string, tag="8")]
    pub owner: ::prost::alloc::string::String,
    /// The counter party to the transaction from the perspective of the owner of this record. Public key in multibase
    /// format.
    #[prost(string, tag="9")]
    pub counterparty: ::prost::alloc::string::String,
    /// The status of the transaction.
    #[prost(enumeration="TransactionStatus", tag="10")]
    pub status: i32,
}
/// Nested message and enum types in `Transaction`.
pub mod transaction {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Role {
        /// Initiator of the transaction.
        Starter = 0,
        /// Responder in a transaction.
        Responder = 1,
    }
    impl Role {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Role::Starter => "ROLE_STARTER",
                Role::Responder => "ROLE_RESPONDER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ROLE_STARTER" => Some(Self::Starter),
                "ROLE_RESPONDER" => Some(Self::Responder),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// The Unknown type is used to guard against accidental use of a zero value.
        Unknown = 0,
        /// Simple Payment Transaction.
        SimplePayment = 1,
        /// Funds Change Transaction.
        FundsChange = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unknown => "TYPE_UNKNOWN",
                Type::SimplePayment => "TYPE_SIMPLE_PAYMENT",
                Type::FundsChange => "TYPE_FUNDS_CHANGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNKNOWN" => Some(Self::Unknown),
                "TYPE_SIMPLE_PAYMENT" => Some(Self::SimplePayment),
                "TYPE_FUNDS_CHANGE" => Some(Self::FundsChange),
                _ => None,
            }
        }
    }
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
    #[prost(string, tag="1")]
    pub field: ::prost::alloc::string::String,
    #[prost(enumeration="filter_item::FilterOperator", tag="2")]
    pub operator: i32,
    #[prost(oneof="filter_item::Value", tags="3, 4, 5")]
    pub value: ::core::option::Option<filter_item::Value>,
}
/// Nested message and enum types in `FilterItem`.
pub mod filter_item {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FilterOperator {
        Unspecified = 0,
        Eq = 1,
        Ne = 2,
        Gt = 3,
        Gte = 4,
        Lt = 5,
        Lte = 6,
        In = 7,
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
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag="3")]
        StrValue(::prost::alloc::string::String),
        #[prost(int64, tag="4")]
        IntValue(i64),
        #[prost(bool, tag="5")]
        BoolValue(bool),
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
    #[prost(message, repeated, tag="1")]
    pub filters: ::prost::alloc::vec::Vec<FilterItem>,
    #[prost(enumeration="LogicalOperator", tag="2")]
    pub operator: i32,
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
/// Enumerated states of both starter and responder transaction clients at a specific point in time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionStatus {
    /// The transaction is initializing.
    StateInitializing = 0,
    /// Digital banknotes are being authorized.
    StateAuthorizing = 1,
    /// Digital banknotes are being sent.
    StateSending = 2,
    /// Digital banknotes are being received.
    StateReceiving = 5,
    /// The transaction has successfully completed.
    StateComplete = 3,
    /// Transaction has been terminated due to some failure.
    StateFailed = 4,
}
impl TransactionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionStatus::StateInitializing => "STATE_INITIALIZING",
            TransactionStatus::StateAuthorizing => "STATE_AUTHORIZING",
            TransactionStatus::StateSending => "STATE_SENDING",
            TransactionStatus::StateReceiving => "STATE_RECEIVING",
            TransactionStatus::StateComplete => "STATE_COMPLETE",
            TransactionStatus::StateFailed => "STATE_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATE_INITIALIZING" => Some(Self::StateInitializing),
            "STATE_AUTHORIZING" => Some(Self::StateAuthorizing),
            "STATE_SENDING" => Some(Self::StateSending),
            "STATE_RECEIVING" => Some(Self::StateReceiving),
            "STATE_COMPLETE" => Some(Self::StateComplete),
            "STATE_FAILED" => Some(Self::StateFailed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogicalOperator {
    Unspecified = 0,
    And = 1,
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
    #[prost(enumeration="is_provisioned_response::Status", tag="1")]
    pub status: i32,
    #[prost(string, tag="2")]
    pub reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `IsProvisionedResponse`.
pub mod is_provisioned_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Unknown = 0,
        Provisioned = 1,
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
    /// Signature on all fields in Transaction header (except signature field itself)
    /// and Routing header + (sha256 on serialized(payload).
    /// TODO investigate viability
    /// should be sent by the emitter of the transaction
    #[prost(bytes="vec", tag="5")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// User reference string for transactions acting as a memo field
    #[prost(string, tag="6")]
    pub user_reference: ::prost::alloc::string::String,
    /// Related Uetrs 0..N
    #[prost(string, repeated, tag="7")]
    pub related_uetrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// Payload for `LockPromissory` and `PromissoryLocked` packets.

/// \[Example\]
/// {
/// "promissory": "LengthOfFileBytesVaries=",
/// "lock_id": "00000000-0000-0000-0000-000000000000"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockedPromissory {
    /// Transferred promissory bytes. These are expected to be the flatbuffer-serialized bytes of a Promissory, not protobuf
    #[prost(bytes="vec", tag="1")]
    pub promissory: ::prost::alloc::vec::Vec<u8>,
    /// UUIDv4 identifier for the lock.
    #[prost(string, tag="2")]
    pub lock_id: ::prost::alloc::string::String,
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
    #[prost(enumeration="simple_payment_nack::RejectionReason", tag="1")]
    pub rejection_reason: i32,
    #[prost(string, tag="2")]
    pub other: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SimplePaymentNack`.
pub mod simple_payment_nack {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RejectionReason {
        /// Default enum field
        ReasonUnknown = 0,
        /// "Other" e.g. sanctions blocked
        ReasonOther = 1,
        /// request amount field was empty or value set to zero
        ReasonAmountEmpty = 2,
        /// Amounts being remitted do not match a previous ChangeRequest (if part of a change request)
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
/// "RECEIVED_PROMISSORY1",
/// "RECEIVED_PROMISSORY2",
/// "RECEIVED_PROMISSORY3"
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimplePaymentResponse {
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
    #[prost(message, optional, tag="1")]
    pub gross_amount: ::core::option::Option<Amount>,
    #[prost(message, optional, tag="2")]
    pub exact_amount: ::core::option::Option<Amount>,
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
    #[prost(enumeration="funds_change_nack::RejectionReason", tag="1")]
    pub rejection_reason: i32,
    #[prost(string, tag="2")]
    pub other: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FundsChangeNack`.
pub mod funds_change_nack {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RejectionReason {
        /// Default enum field
        ReasonUnknown = 0,
        /// "Other" e.g. sanctions blocked
        ReasonOther = 1,
        /// request amount field was empty or value set to zero
        ReasonAmountEmpty = 2,
        /// GrossAmount - Fees must be >= ExactAmount, else its not possible to get change
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
// Signal meant to terminate a transaction that is in progress

/// \[Example\]
/// {
/// "rejection_reason": 0,
/// "other": "OTHER"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelTransaction {
    #[prost(enumeration="cancel_transaction::RejectionReason", tag="1")]
    pub rejection_reason: i32,
    #[prost(string, tag="2")]
    pub other: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CancelTransaction`.
pub mod cancel_transaction {
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
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<Amount>,
}
// * Represents a series of `Commitment`s that
// once signed by all signatories will mark the contract as fulfilled.
// * For a `Contract` to be valid: all signatories must have their respective
// public key mapped to a valid signature in the `signatories` map.
// * The signature is generated by a signatory signing the _final_ `Contract` message
// with an empty `signatories` field.
// * Signatories include but are not limited to all participants
// in all included `Commitment`s

/// \[Example\]
/// {
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
/// "recipient": "zRecipient1PublicKey",
/// "amount": {
/// "currency_code":"USD", 
/// "amount":100, 
/// "decimals":2
/// }
/// }
/// ],
/// "signatories": [
/// {
/// "key": "KEY1",
/// "value": "VALUExBYTES="
/// },
/// {
/// "key": "KEY2",
/// "value": "VALUExBYTES="
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contract {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub commitments: ::prost::alloc::vec::Vec<Commitment>,
    /// map<pk_signer, signature>
    #[prost(map="string, bytes", tag="3")]
    pub signatories: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
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
    /// Request an authorized signature for a promissory and return to the sender
    /// corresponds to `AuthorizedSignatureRequest` message
    AuthorizePromissory = 101,
    /// Authorize and Forward Promissory
    /// Request an authorized signature for a promissory and forward to the recipient
    /// corresponds to `AuthorizedSignatureForward` message
    AuthorizeAndForwardPromissory = 102,
    /// Archive Promissory Request
    /// After a promissory file is removed from circulation archive the promissory file
    ArchivePromissory = 103,
    /// Promissory receipt acknowledgement
    /// Respond with promissory receipt to a promissory request
    /// TODO: Decide if this msg or a general acknowledgement message should be used.
    PromissoryReceipt = 104,
    /// Lock Promissory
    /// Request to notary to lock promissory in double spend database for transaction
    LockPromissory = 105,
    /// Promissory Locked
    /// Notice from notary that a promissory is locked
    PromissoryLocked = 106,
    /// Authorize Locked Promissories
    /// Request to authorize all locked promissories for a given transaction.
    AuthorizeLockedPromissories = 107,
    /// Authorization Status
    /// Response from Notary to sender of an `AuthorizeLockedPromissories` message. Indicates
    /// the notary has completed authorization.
    AuthorizationStatus = 108,
    /// Retrieve Assets
    /// Request to have a notary send Promissory packets back to the requester for each
    /// promissory related to a given transaction and owned by the requestor.
    RetrieveAssets = 109,
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

    /// Notary Authorize Promissory
    NotaryNotarizePromissory = 300,
    /// Notarize and Forward Promissory
    NotaryNotarizeAndForwardPromissory = 301,
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
    /// Contract Commitment
    /// Update a contract with the addition of a commitment
    ContractAck = 501,
    /// Contract Signature
    /// Updates a contract with the addition of a signature
    ContractSignature = 502,
    /// Holds a finalized contract in the packet data field
    ContractComplete = 503,
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
    // TwoStepPayment Packets 1200..1299
    // ---------------------------

    /// Payer instruction to notify the payee of a two-step payment comprising a
    /// number of CPFs.
    /// TwoStepPaymentRequest
    TspRequest = 1200,
    /// Payee acknowledgement of the two-step payment, also confirms the existence
    /// of the Payee.
    /// TwoStepPaymentAck
    TspAck = 1201,
    /// Payee acknowledgement of the two-step payment, also confirms the existence
    /// of the Payee.
    /// TwoStepPaymentNack
    TspNack = 1202,
    /// Payer instruction to the Transaction Manager to start a new two-step
    /// payment using a signed contract.
    /// TwoStepPaymentContract
    TspContract = 1203,
    /// Transaction Manager acknowledgement of the two-step payment using the
    /// provided signed contract.
    /// TwoStepPaymentContract
    TspContractAck = 1204,
    /// Transaction Manager acknowledgement to the payee that the assets have been
    /// authorized and are ready to be retrieved.
    /// TwoStepPaymentAssetsAuthorized
    TspAssetsAuthorized = 1205,
    /// Transaction Manager acknowledgement that the two-step payment is complete.
    /// TwoStepPaymentComplete
    TspComplete = 1206,
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
            PacketType::LockPromissory => "LockPromissory",
            PacketType::PromissoryLocked => "PromissoryLocked",
            PacketType::AuthorizeLockedPromissories => "AuthorizeLockedPromissories",
            PacketType::AuthorizationStatus => "AuthorizationStatus",
            PacketType::RetrieveAssets => "RetrieveAssets",
            PacketType::IdentityRequest => "IdentityRequest",
            PacketType::IdentityResponse => "IdentityResponse",
            PacketType::IdentityProof => "IdentityProof",
            PacketType::NotaryNotarizePromissory => "NotaryNotarizePromissory",
            PacketType::NotaryNotarizeAndForwardPromissory => "NotaryNotarizeAndForwardPromissory",
            PacketType::EmissaryConnectionIdentity => "EmissaryConnectionIdentity",
            PacketType::ContractProposal => "ContractProposal",
            PacketType::ContractAck => "ContractAck",
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
            PacketType::TspRequest => "TSPRequest",
            PacketType::TspAck => "TSPAck",
            PacketType::TspNack => "TSPNack",
            PacketType::TspContract => "TSPContract",
            PacketType::TspContractAck => "TSPContractAck",
            PacketType::TspAssetsAuthorized => "TSPAssetsAuthorized",
            PacketType::TspComplete => "TSPComplete",
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
            "LockPromissory" => Some(Self::LockPromissory),
            "PromissoryLocked" => Some(Self::PromissoryLocked),
            "AuthorizeLockedPromissories" => Some(Self::AuthorizeLockedPromissories),
            "AuthorizationStatus" => Some(Self::AuthorizationStatus),
            "RetrieveAssets" => Some(Self::RetrieveAssets),
            "IdentityRequest" => Some(Self::IdentityRequest),
            "IdentityResponse" => Some(Self::IdentityResponse),
            "IdentityProof" => Some(Self::IdentityProof),
            "NotaryNotarizePromissory" => Some(Self::NotaryNotarizePromissory),
            "NotaryNotarizeAndForwardPromissory" => Some(Self::NotaryNotarizeAndForwardPromissory),
            "EmissaryConnectionIdentity" => Some(Self::EmissaryConnectionIdentity),
            "ContractProposal" => Some(Self::ContractProposal),
            "ContractAck" => Some(Self::ContractAck),
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
            "TSPRequest" => Some(Self::TspRequest),
            "TSPAck" => Some(Self::TspAck),
            "TSPNack" => Some(Self::TspNack),
            "TSPContract" => Some(Self::TspContract),
            "TSPContractAck" => Some(Self::TspContractAck),
            "TSPAssetsAuthorized" => Some(Self::TspAssetsAuthorized),
            "TSPComplete" => Some(Self::TspComplete),
            _ => None,
        }
    }
}
include!("common.tonic.rs");
// @@protoc_insertion_point(module)
