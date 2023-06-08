// @generated
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
/// "from_wallet_id": "zFromWalletIDMultibase58Encoded",
/// "recipient_public_key": "zRecipientPublicKeyMultibase58Encoded", 
/// "promissory_id": "PROMISSORY_ID", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferResponse {
}
/// \[Example\]
/// {
/// "owner_name": "OWNER_NAME"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOwnerRequest {
    /// Name of the wallet owner.
    #[prost(string, tag="1")]
    pub owner_name: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "owner_id": "eyJwtHeader.eyJwtPayload.Secret", 
/// "api_key": "API_KEY", 
/// "owner_name": "OWNER_NAME"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "wallet_name": "WALLET_NAME", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWalletRequest {
    /// Human friendly name of the wallet.
    #[prost(string, tag="1")]
    pub wallet_name: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "wallet_name": "WALLET_NAME", 
/// "public_key": "zPublicKeyMultibase58Encoded", 
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "page": 0, 
/// "page_size": 10, 
/// "api_key": "API_KEY",
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
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
    /// Potential filters to apply to the list of wallets.
    #[prost(message, optional, tag="4")]
    pub filters: ::core::option::Option<super::super::common::Filter>,
}
/// \[Example\]
/// {
/// "wallets": [
/// {
/// "wallet_name": "WALLET1", 
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "public_key": "zPublicKeyMultibase58Encoded", 
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }, 
/// {
/// "wallet_name": "WALLET2", 
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "public_key": "zPublicKeyMultibase58Encoded", 
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }
/// ], 
/// "total_owned_wallets": 5
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
    #[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWalletBalancesRequest {
    /// UUID of the wallet.
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "amount": {
/// "currency_code":"USD", 
/// "amount":100, 
/// "decimals":2
/// }, 
/// "file_count": 57
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalletBalance {
    /// Sum of a particular currency available in the wallet.
    #[prost(message, optional, tag="1")]
    pub amount: ::core::option::Option<super::super::common::Amount>,
    /// Total count of a particular banknote
    #[prost(int64, tag="2")]
    pub file_count: i64,
}
/// \[Example\]
/// {
/// "balances": [
/// {
/// "amount": {
/// "currency_code":"USD", 
/// "amount":5700, 
/// "decimals":2
/// }, 
/// "file_count": 57
/// }, 
/// {
/// "amount": {
/// "currency_code":"USD", 
/// "amount":500, 
/// "decimals":2
/// }, 
/// "file_count": 3
/// } 
/// ] 
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWalletBalancesResponse {
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<WalletBalance>,
}
/// \[Example\]
/// {
/// "page": 0, 
/// "page_size": 10000, 
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "api_key": "API_KEY",
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
pub struct ListAssetFilesRequest {
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
    /// Filter by object notation relative to the `promissory` object. Supports filtering by amount, decimals & currency.
    #[prost(message, optional, tag="5")]
    pub filters: ::core::option::Option<super::super::common::Filter>,
    /// Sort by object notation relative to the `promissory` object. Supports sorting by amount, decimals & currency.
    #[prost(message, optional, tag="6")]
    pub sort_by: ::core::option::Option<SortBy>,
}
/// \[Example\]
/// {
/// "total_owned_funds": 2, 
/// "funds": [
/// {
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "promissory": {
/// "version": "0.1.0", 
/// "id": "PROMISSORY_ID", 
/// "currency": "USD", 
/// "amount": 100, 
/// "decimals": 2, 
/// "authority": "zAuthorityPublicKeyMultibase58Encoded", 
/// "issuer": "zIssuerPublicKeyMultibase58Encoded", 
/// "issuance_date": "2006-01-02T15:04:05Z", 
/// "block_depth": 3, 
/// "authorization_signature": "ByteLengthAuthSignatureVariesByAlgo=", 
/// "transfer_signature": "ByteLengthTransferSignatureVariesByAlgo=",
/// "notaries": [
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "signature": "ByteLengthSignatureVariesByAlgo="
/// },
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "signature": "ByteLengthSignatureVariesByAlgo="
/// }
/// ]
/// }
/// },
/// {
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "promissory": {
/// "version": "0.1.0", 
/// "id": "PROMISSORY_ID", 
/// "currency": "USD", 
/// "amount": 500, 
/// "decimals": 2, 
/// "authority": "zAuthorityPublicKeyMultibase58Encoded", 
/// "issuer": "zIssuerPublicKeyMultibase58Encoded", 
/// "issuance_date": "2006-01-02T15:04:05Z", 
/// "block_depth": 3, 
/// "authorization_signature": "ByteLengthAuthSignatureVariesByAlgo=", 
/// "transfer_signature": "ByteLengthTransferSignatureVariesByAlgo=",
/// "notaries": [
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "signature": "ByteLengthSignatureVariesByAlgo="
/// },
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded",
/// "signature": "ByteLengthSignatureVariesByAlgo="
/// }
/// ]
/// }
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetFilesResponse {
    /// List of fund objects.
    #[prost(message, repeated, tag="1")]
    pub funds: ::prost::alloc::vec::Vec<list_asset_files_response::Fund>,
    /// Number of funds belonging to the owner.
    #[prost(uint32, tag="2")]
    pub total_owned_funds: u32,
}
/// Nested message and enum types in `ListAssetFilesResponse`.
pub mod list_asset_files_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Notary {
        #[prost(string, tag="1")]
        pub did: ::prost::alloc::string::String,
        #[prost(bytes="vec", tag="2")]
        pub signature: ::prost::alloc::vec::Vec<u8>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
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
        #[prost(message, repeated, tag="13")]
        pub notaries: ::prost::alloc::vec::Vec<Notary>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "from_wallet_id": "zFromWalletIDMultibase58Encoded",
/// "issuer_public_key": "zIssuerPublicKeyMultibase58Encoded", 
/// "promissory_id": "PROMISSORY_ID", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "new_limit": {
/// "currency_code":"USD", 
/// "amount":100, 
/// "decimals":2
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedeemResponse {
    /// Amount representation of the updated issuance limit.
    #[prost(message, optional, tag="1")]
    pub new_limit: ::core::option::Option<super::super::common::Amount>,
}
/// \[Example\]
/// {
/// "page": 0, 
/// "page_size": 10000, 
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "credentials": [
/// "CREDENTIAL1", 
/// "CREDENTIAL2"
/// ], 
/// "total_owned_credentials": 2
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCredentialsResponse {
    /// Can be one or many of credential types. Ex. "BankAccount", "PermanentResidentCard", "VerifiableCredential"
    #[prost(string, repeated, tag="1")]
    pub credentials: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Number of credentials belonging to the wallet.
    #[prost(uint32, tag="2")]
    pub total_owned_credentials: u32,
}
/// \[Example\]
/// {
/// "credential_type":"CREDENTIAL_TYPE",
/// "wallet_id": "zWalletIDMultibase58Encoded",
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "credential": "CREDENTIAL"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCredentialResponse {
    /// W3C Verifiable Credential representing the subject.
    #[prost(string, tag="1")]
    pub credential: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateWalletRequest {
    /// UUID of the wallet.
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "did": "did:method-name:zDIDMultibase58Encoded", 
/// "public_key": "zPublicKeyMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "wallet_name": "WALLET_NAME", 
/// "api_key": "API_KEY", 
/// "token": "TOKEN"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "wallet_name": "WALLET_NAME", 
/// "public_key": "zPublicKeyMultibase58Encoded", 
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// } 
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded", 
/// "credential_type": "CREDENTIAL_TYPE", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "presentation": "PRESENTATION"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestPresentationResponse {
    /// The JSON Verifiable Presentation containing the PII of the subject.
    #[prost(string, tag="1")]
    pub presentation: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "amount": {
/// "currency_code":"USD", 
/// "amount":100, 
/// "decimals":2
/// }, 
/// "recipient": "zRecipientPublicKeyMultibase58Encoded", 
/// "user_reference": "TRANSACTION_REFERENCE", 
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "status": 0, 
/// "uetr": "UETR"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "STATUS_SUCCESS" => Some(Self::Success),
                "STATUS_NEED_CHANGE" => Some(Self::NeedChange),
                "STATUS_INSUFFICIENT_FUNDS" => Some(Self::InsufficientFunds),
                _ => None,
            }
        }
    }
}
/// \[Example\]
/// {
/// "amount": {
/// "currency_code":"USD", 
/// "amount":100, 
/// "decimals":2
/// }, 
/// "user_reference": "TRANSACTION_REFERENCE", 
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "status": 0, 
/// "uetr": "UETR"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "STATUS_SUCCESS" => Some(Self::Success),
                "STATUS_INSUFFICIENT_FUNDS" => Some(Self::InsufficientFunds),
                "STATUS_FUNDS_PRESENT_NOOP" => Some(Self::FundsPresentNoop),
                _ => None,
            }
        }
    }
}
/// \[Example\]
/// {
/// "amount": {
/// "currency_code":"USD", 
/// "amount":100, 
/// "decimals":2
/// }, 
/// "recipient": "zRecipientPublicKeyMultibase58Encoded", 
/// "user_reference": "TRANSACTION_REFERENCE", 
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareTwoStepPaymentRequest {
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
/// \[Example\]
/// {
/// "status": 0, 
/// "uetr": "UETR"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareTwoStepPaymentResponse {
    /// Status of the transaction.
    #[prost(enumeration="prepare_two_step_payment_response::Status", tag="1")]
    pub status: i32,
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="2")]
    pub uetr: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PrepareTwoStepPaymentResponse`.
pub mod prepare_two_step_payment_response {
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
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "STATUS_SUCCESS" => Some(Self::Success),
                "STATUS_NEED_CHANGE" => Some(Self::NeedChange),
                "STATUS_INSUFFICIENT_FUNDS" => Some(Self::InsufficientFunds),
                _ => None,
            }
        }
    }
}
/// \[Example\]
/// {
/// "uetr": "UETR", 
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "transaction": {
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
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionResponse {
    /// Transaction objects.
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<super::super::common::Transaction>,
}
/// \[Example\]
/// {
/// "id": "zWalletIDMultibase58Encoded", 
/// "api_key": "API_KEY",
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
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionsRequest {
    /// Wallet_id as UUID or public_key as multibase.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub filters: ::core::option::Option<super::super::common::Filter>,
    #[prost(int32, tag="4")]
    pub page: i32,
    #[prost(int32, tag="5")]
    pub page_size: i32,
}
/// \[Example\]
/// {
/// "transactions": [
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
/// }, 
/// {
/// "type": 2, 
/// "role": 0, 
/// "uetr": "UETR", 
/// "amount": {
/// "currency_code":"USD", 
/// "amount":500, 
/// "decimals":2
/// }, 
/// "created": "2006-01-02T15:04:05Z", 
/// "updated": "2007-01-02T15:04:05Z", 
/// "memo": "MEMO", 
/// "owner": "zOwnerPublicKeyMultibase58Encoded", 
/// "counterparty": "zCounterpartyPublicKeyMultibase58Encoded", 
/// "status": 1
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionsResponse {
    /// Transaction objects.
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<super::super::common::Transaction>,
    #[prost(uint32, tag="2")]
    pub total_matching_transactions: u32,
}
/// \[Example\]
/// {
/// "uetr": "UETR", 
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "snapshot": {
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
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransactionSnapshotResponse {
    /// Transaction snapshots.
    #[prost(message, optional, tag="1")]
    pub snapshot: ::core::option::Option<super::super::common::TransactionSnapshot>,
}
/// \[Example\]
/// {
/// "uetr": "UETR", 
/// "wallet_id": "zWalletIDMultibase58Encoded", 
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// \[Example\]
/// {
/// "snapshots": [
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
/// }, 
/// {
/// "status": 3, 
/// "promissories": [
/// "PROMISSORYxIDx3", 
/// "PROMISSORYxIDx4"
/// ], 
/// "amount_transferred": {
/// "currency_code":"USD", 
/// "amount":500, 
/// "decimals":2
/// }, 
/// "message": "MESSAGE2"
/// }
/// ]
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionSnapshotsResponse {
    /// Transaction snapshots.
    #[prost(message, repeated, tag="1")]
    pub snapshots: ::prost::alloc::vec::Vec<super::super::common::TransactionSnapshot>,
}
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortDirection {
    Unspecified = 0,
    Ascending = 1,
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
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
