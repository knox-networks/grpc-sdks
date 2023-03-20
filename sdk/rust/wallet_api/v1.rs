// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferRequest {
    /// UUID of the wallet sending the transfer.
    #[prost(string, tag="1")]
    pub from_wallet_id: ::prost::alloc::string::String,
    /// Public key of the recipient wallet.
    #[prost(string, tag="2")]
    pub recipient_public_key: ::prost::alloc::string::String,
    /// ID of the digital bank note.
    #[prost(string, tag="3")]
    pub promissory_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="4")]
    pub api_key: ::prost::alloc::string::String,
}
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOwnerRequest {
    /// Name of the wallet owner.
    #[prost(string, tag="1")]
    pub owner_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOwnerResponse {
    /// UUID of the created owner of the wallet(s).
    #[prost(string, tag="1")]
    pub owner_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner as part of creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
    /// Name of the wallet owner.
    #[prost(string, tag="3")]
    pub owner_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWalletRequest {
    /// Human friendly name of the wallet.
    #[prost(string, tag="1")]
    pub wallet_name: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWalletResponse {
    /// UUID of the wallet.
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
    /// Human friendly name of the wallet.
    #[prost(string, tag="2")]
    pub wallet_name: ::prost::alloc::string::String,
    /// Public key of the wallet.
    #[prost(string, tag="3")]
    pub public_key: ::prost::alloc::string::String,
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="4")]
    pub did: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWalletsRequest {
    /// Page number for pagination.
    #[prost(int32, tag="1")]
    pub page: i32,
    /// Size of the page for pagination.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="3")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWalletsResponse {
    /// List of Wallet objects.
    #[prost(message, repeated, tag="1")]
    pub wallets: ::prost::alloc::vec::Vec<list_wallets_response::Wallet>,
    /// Number of wallets belonging to the owner.
    #[prost(uint32, tag="2")]
    pub total_owned_wallets: u32,
}
/// Nested message and enum types in `ListWalletsResponse`.
pub mod list_wallets_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Wallet {
        /// Human friendly name of the wallet.
        #[prost(string, tag="1")]
        pub wallet_name: ::prost::alloc::string::String,
        /// UUID of the wallet.
        #[prost(string, tag="2")]
        pub wallet_id: ::prost::alloc::string::String,
        /// Public key of the wallet.
        #[prost(string, tag="3")]
        pub public_key: ::prost::alloc::string::String,
        /// W3C Decentralized Identifier (DID) of the wallet.
        #[prost(string, tag="4")]
        pub did: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWalletBalancesRequest {
    /// UUID of the wallet.
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletBalance {
    /// Sum of a particular currency available in the wallet.
    #[prost(message, optional, tag="1")]
    pub amount: ::core::option::Option<super::super::common::Amount>,
    /// Total count of a particular banknote
    #[prost(int64, tag="2")]
    pub file_count: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWalletBalancesResponse {
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<WalletBalance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFundsRequest {
    /// Page number for pagination.
    #[prost(int32, tag="1")]
    pub page: i32,
    /// Size of the page for pagination.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// UUID of the wallet.
    #[prost(string, tag="3")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="4")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFundsResponse {
    /// List of fund objects.
    #[prost(message, repeated, tag="1")]
    pub funds: ::prost::alloc::vec::Vec<list_funds_response::Fund>,
    /// Number of funds belonging to the owner.
    #[prost(uint32, tag="2")]
    pub total_owned_funds: u32,
}
/// Nested message and enum types in `ListFundsResponse`.
pub mod list_funds_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Promissory {
        /// Version of the digital banknote file.
        #[prost(string, tag="1")]
        pub version: ::prost::alloc::string::String,
        /// UUID of the digital banknote file.
        #[prost(string, tag="3")]
        pub id: ::prost::alloc::string::String,
        /// ISO-4217 currency code.
        #[prost(string, tag="4")]
        pub currency: ::prost::alloc::string::String,
        /// Amount of the digital banknote.
        #[prost(int64, tag="5")]
        pub amount: i64,
        /// Decimal precision.
        #[prost(uint32, tag="6")]
        pub decimals: u32,
        /// public key of the monetary Authority.
        #[prost(string, tag="7")]
        pub authority: ::prost::alloc::string::String,
        /// public key of the Issuer.
        #[prost(string, tag="8")]
        pub issuer: ::prost::alloc::string::String,
        /// RFC3339 timestamp of the issuance date.
        #[prost(string, tag="9")]
        pub issuance_date: ::prost::alloc::string::String,
        /// Number of signature blocks, representing the owner history.
        #[prost(uint32, tag="10")]
        pub block_depth: u32,
        /// Signature of the latest block hash from the Authority.
        #[prost(bytes="vec", tag="11")]
        pub authorization_signature: ::prost::alloc::vec::Vec<u8>,
        /// Signature of the previous owner.
        #[prost(bytes="vec", tag="12")]
        pub transfer_signature: ::prost::alloc::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Fund {
        /// UUID of the wallet.
        #[prost(string, tag="1")]
        pub wallet_id: ::prost::alloc::string::String,
        /// Digital Banknote object.
        #[prost(message, optional, tag="2")]
        pub promissory: ::core::option::Option<Promissory>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemRequest {
    /// UUID of the wallet initiating the redemption.
    #[prost(string, tag="1")]
    pub from_wallet_id: ::prost::alloc::string::String,
    /// Public key of the issuer.
    #[prost(string, tag="2")]
    pub issuer_public_key: ::prost::alloc::string::String,
    /// ID of the Digital Banknote.
    #[prost(string, tag="3")]
    pub promissory_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="4")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemResponse {
    /// Amount representation of the updated issuance limit.
    #[prost(message, optional, tag="1")]
    pub new_limit: ::core::option::Option<super::super::common::Amount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCredentialsRequest {
    /// Page number for pagination.
    #[prost(int32, tag="1")]
    pub page: i32,
    /// Size of the page for pagination.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// UUID of the wallet.
    #[prost(string, tag="3")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="4")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCredentialsResponse {
    /// Can be one or many of credential types. Ex. "BankAccount", "PermanentResidentCard", "VerifiableCredential"
    #[prost(string, repeated, tag="1")]
    pub credentials: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Number of credentials belonging to the wallet.
    #[prost(uint32, tag="2")]
    pub total_owned_credentials: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCredentialRequest {
    /// Type of Verifiable Credential- Ex "BankAccount", "PermanentResidentCard".
    #[prost(string, tag="1")]
    pub credential_type: ::prost::alloc::string::String,
    /// UUID of the wallet.
    #[prost(string, tag="2")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="3")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCredentialResponse {
    /// W3C Verifiable Credential representing the subject.
    #[prost(string, tag="1")]
    pub credential: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateWalletRequest {
    /// UUID of the wallet.
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateWalletResponse {
    /// UUID of the wallet.
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="2")]
    pub did: ::prost::alloc::string::String,
    /// Public key of the wallet.
    #[prost(string, tag="3")]
    pub public_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAndAssociateWalletRequest {
    /// Human friendly name of the wallet.
    #[prost(string, tag="1")]
    pub wallet_name: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
    /// JWT token of the authenticated account to associate.
    #[prost(string, tag="3")]
    pub token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAndAssociateWalletResponse {
    /// UUID of the wallet.
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
    /// Human friendly name of the wallet.
    #[prost(string, tag="2")]
    pub wallet_name: ::prost::alloc::string::String,
    /// Public key of the wallet.
    #[prost(string, tag="3")]
    pub public_key: ::prost::alloc::string::String,
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="4")]
    pub did: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPresentationRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
    /// Type of Verifiable Credential - Ex "BankAccount", "PermanentResidentCard".
    #[prost(string, tag="2")]
    pub credential_type: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="3")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPresentationResponse {
    /// The JSON Verifiable Presentation containing the PII of the subject.
    #[prost(string, tag="1")]
    pub presentation: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareSimplePaymentRequest {
    /// Amount object representing the transfer amount.
    #[prost(message, optional, tag="1")]
    pub amount: ::core::option::Option<super::super::common::Amount>,
    /// Public key of the recipient.
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="3")]
    pub user_reference: ::prost::alloc::string::String,
    /// UUID of the wallet.
    #[prost(string, tag="4")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="5")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareSimplePaymentResponse {
    /// Status of the transaction.
    #[prost(enumeration="prepare_simple_payment_response::Status", tag="1")]
    pub status: i32,
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="2")]
    pub uetr: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PrepareSimplePaymentResponse`.
pub mod prepare_simple_payment_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Unspecified = 0,
        Success = 1,
        NeedChange = 2,
        InsufficientFunds = 3,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::Success => "STATUS_SUCCESS",
                Status::NeedChange => "STATUS_NEED_CHANGE",
                Status::InsufficientFunds => "STATUS_INSUFFICIENT_FUNDS",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareFundsChangeRequest {
    /// Requested change amount.
    #[prost(message, optional, tag="1")]
    pub amount: ::core::option::Option<super::super::common::Amount>,
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="4")]
    pub user_reference: ::prost::alloc::string::String,
    /// UUID of the wallet.
    #[prost(string, tag="2")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="3")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareFundsChangeResponse {
    /// Status of the transaction.
    #[prost(enumeration="prepare_funds_change_response::Status", tag="1")]
    pub status: i32,
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="2")]
    pub uetr: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PrepareFundsChangeResponse`.
pub mod prepare_funds_change_response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Unspecified = 0,
        Success = 1,
        InsufficientFunds = 2,
        FundsPresentNoop = 3,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::Success => "STATUS_SUCCESS",
                Status::InsufficientFunds => "STATUS_INSUFFICIENT_FUNDS",
                Status::FundsPresentNoop => "STATUS_FUNDS_PRESENT_NOOP",
            }
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionRequest {
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="1")]
    pub uetr: ::prost::alloc::string::String,
    /// UUID of the wallet.
    #[prost(string, tag="2")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="3")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResponse {
    /// Transaction objects.
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<super::super::common::Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionsRequest {
    /// Wallet_id as UUID or public_key as multibase.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionsResponse {
    /// Transaction objects.
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<super::super::common::Transaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionSnapshotRequest {
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="1")]
    pub uetr: ::prost::alloc::string::String,
    /// UUID of the wallet.
    #[prost(string, tag="2")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="3")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionSnapshotResponse {
    /// Transaction snapshots.
    #[prost(message, optional, tag="1")]
    pub snapshot: ::core::option::Option<super::super::common::TransactionSnapshot>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionSnapshotsRequest {
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="1")]
    pub uetr: ::prost::alloc::string::String,
    /// UUID of the wallet.
    #[prost(string, tag="2")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="3")]
    pub api_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionSnapshotsResponse {
    /// Transaction snapshots.
    #[prost(message, repeated, tag="1")]
    pub snapshots: ::prost::alloc::vec::Vec<super::super::common::TransactionSnapshot>,
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
