// @generated
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

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
    /// Potential sort to apply to the list of wallets.
    #[prost(message, optional, tag="5")]
    pub sort_by: ::core::option::Option<super::super::common::SortBy>,
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
    /// \[Example\]
    /// {
    /// "wallet_name": "WALLET1",
    /// "wallet_id": "zWalletIDMultibase58Encoded",
    /// "public_key": "zPublicKeyMultibase58Encoded",
    /// "did": "did:method-name:zDIDMultibase58Encoded"
    /// }
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
/// "api_key": "API_KEY",
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
pub struct ListWalletBalancesRequest {
    /// UUID of the wallet.
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
    /// Sorting Method on Wallet.
    #[prost(message, optional, tag="3")]
    pub sort_by: ::core::option::Option<super::super::common::SortBy>,
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
    /// List of wallet balances.
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
    pub sort_by: ::core::option::Option<super::super::common::SortBy>,
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
    /// \[Example\]
    /// {
    /// "did": "did:method-name:zDIDMultibase58Encoded",
    /// "signature": "ByteLengthSignatureVariesByAlgo="
    /// }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Notary {
        /// DID of Notary
        #[prost(string, tag="1")]
        pub did: ::prost::alloc::string::String,
        /// Signature of Notary
        #[prost(bytes="vec", tag="2")]
        pub signature: ::prost::alloc::vec::Vec<u8>,
    }
    /// \[Example\]
    /// {
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
        /// List of Notaries.
        #[prost(message, repeated, tag="13")]
        pub notaries: ::prost::alloc::vec::Vec<Notary>,
    }
    /// \[Example\]
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
    /// }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Fund {
        /// UUID of the wallet.
        #[prost(string, tag="1")]
        pub wallet_id: ::prost::alloc::string::String,
        /// Digital Banknote object.
        #[prost(message, optional, tag="2")]
        pub promissory: ::core::option::Option<Promissory>,
        /// Whether or not the digital banknote is reserved.
        #[prost(bool, tag="3")]
        pub reserved: bool,
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
/// "page": 0,
/// "page_size": 10000,
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
    /// Potential filters to apply to the list of credentials.
    #[prost(message, optional, tag="5")]
    pub filters: ::core::option::Option<super::super::common::Filter>,
    /// Potential sort to apply to the list of credentials.
    #[prost(message, optional, tag="6")]
    pub sort_by: ::core::option::Option<super::super::common::SortBy>,
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
    /// Type of Verifiable Credential - e.g., "BankAccount", "PermanentResidentCard".
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
    /// Human-readable memo field
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
    /// Status of Simple Payment
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Simple Payment Unspecified.
        Unspecified = 0,
        /// Simple Payment Successful.
        Success = 1,
        /// Simple Payment Currently Needs Change.
        NeedChange = 2,
        /// Simple Payment Has Insufficient Funds.
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
    /// Human-readable memo field
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
    /// Prepare Funds Change Status.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Simple Payment Unspecified.
        Unspecified = 0,
        /// Simple Payment Successful.
        Success = 1,
        /// Simple Payment Has Insufficient Funds.
        InsufficientFunds = 2,
        /// Simple Payment Funds Present Noop.
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
/// "api_key": "API_KEY",
/// "wallet_id": "zWalletIDMultibase58Encoded",
/// "commitments": [
/// {
/// "sender": "zSenderPublicKeyMultibase58Encoded",
/// "recipient": "zRecipientPublicKeyMultibase58Encoded",
/// "amount": {
/// "currency_code": "USD",
/// "amount": 100,
/// "decimals": 2
/// }
/// }
/// ],
/// "conditions": [
/// {
/// "condition": {
/// "timeout": {
/// "timeout": 1687180000
/// }
/// }
/// }
/// ],
/// "memo": "Some contract"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareContractTransactionRequest {
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// UUID of the wallet.
    #[prost(string, tag="2")]
    pub wallet_id: ::prost::alloc::string::String,
    /// List of commitments
    #[prost(message, repeated, tag="3")]
    pub commitments: ::prost::alloc::vec::Vec<super::super::common::Commitment>,
    /// List of conditions
    #[prost(message, repeated, tag="4")]
    pub conditions: ::prost::alloc::vec::Vec<super::super::common::Condition>,
    /// Human-readable memo field
    #[prost(string, tag="5")]
    pub memo: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "status": 0,
/// "uetr": "UETR"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareContractTransactionResponse {
    /// Status of the transaction
    #[prost(enumeration="prepare_contract_transaction_response::Status", tag="1")]
    pub status: i32,
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="2")]
    pub uetr: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PrepareContractTransactionResponse`.
pub mod prepare_contract_transaction_response {
    /// Prepare Contract Transaction Status.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Status Unspecified.
        Unspecified = 0,
        /// Status Successful.
        Success = 1,
        /// Status Unsuccessful.
        Failure = 2,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "UNSPECIFIED",
                Status::Success => "SUCCESS",
                Status::Failure => "FAILURE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCESS" => Some(Self::Success),
                "FAILURE" => Some(Self::Failure),
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
/// },
/// "page": 0,
/// "page_size": 10000,
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
pub struct ListTransactionsRequest {
    /// Wallet_id as UUID or public_key as multibase.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
    /// Filters applied to list of transactions.
    #[prost(message, optional, tag="3")]
    pub filters: ::core::option::Option<super::super::common::Filter>,
    /// Page number for pagination.
    #[prost(int32, tag="4")]
    pub page: i32,
    /// Size of the page for pagination.
    #[prost(int32, tag="5")]
    pub page_size: i32,
    /// Sort By
    #[prost(message, optional, tag="6")]
    pub sort_by: ::core::option::Option<super::super::common::SortBy>,
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
/// ],
/// "total_matching_transactions": 2
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionsResponse {
    /// Transaction objects.
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<super::super::common::Transaction>,
    /// Total number of returned transactions.
    #[prost(uint32, tag="2")]
    pub total_matching_transactions: u32,
}
/// \[Example\]
/// {
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
/// "page": 0,
/// "page_size": 10000,
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
pub struct ListContractTransactionsRequest {
    /// Wallet_id as UUID or public_key as multibase.
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
    /// Filters applied to list of transactions.
    #[prost(message, optional, tag="3")]
    pub filters: ::core::option::Option<super::super::common::Filter>,
    /// Page number for pagination.
    #[prost(int32, tag="4")]
    pub page: i32,
    /// Size of the page for pagination.
    #[prost(int32, tag="5")]
    pub page_size: i32,
    /// Sort By
    #[prost(message, optional, tag="6")]
    pub sort_by: ::core::option::Option<super::super::common::SortBy>,
}
// \[Example\]
// {
// "contract_id": "9628a2d9-f932-41f0-b2da-b9de3103d945",
// "contract_type": "Swap",
// "created": "2022-12-19T16:39:57-08:00",
// "counterparties": [
// "zCounterpartyPublicKeyMultibase58Encoded",
// "zCounterpartyPublicKeyMultibase58Encoded2",
// ],
// "status": 1,
// }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractSummary {
    /// UUID of the contract
    #[prost(string, tag="1")]
    pub contract_id: ::prost::alloc::string::String,
    /// Type of the contract
    #[prost(string, tag="2")]
    pub contract_type: ::prost::alloc::string::String,
    /// RFC3339 timestamp of the contract creation date.
    #[prost(string, tag="3")]
    pub created: ::prost::alloc::string::String,
    /// List of participant public keys excluding self
    #[prost(string, repeated, tag="4")]
    pub counterparties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Current status of contract
    #[prost(string, tag="5")]
    pub status: ::prost::alloc::string::String,
    /// Contract memo
    #[prost(string, tag="6")]
    pub memo: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "contracts": [
/// {
/// "contract_id": "9628a2d9-f932-41f0-b2da-b9de3103d945",
/// "contract_type": "Swap",
/// "created": "2022-12-19T16:39:57-08:00",
/// "counterparties": [
/// "zCounterparty1PublicKeyMultibase58Encoded",
/// "zCounterparty2PublicKeyMultibase58Encoded"
/// ],
/// "status": 1
/// },
/// {
/// "contract_id": "9628a2d9-f932-41f0-b2da-b9de3103d945",
/// "contract_type": "Swap",
/// "created": "2022-12-20T16:39:57-08:00",
/// "counterparties": [
/// "zCounterparty3PublicKeyMultibase58Encoded",
/// "zCounterparty4PublicKeyMultibase58Encoded"
/// ],
/// "status": 1
/// }
/// ],
/// "totalContracts": 2
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContractTransactionsResponse {
    /// List of contracts with status metadata
    #[prost(message, repeated, tag="1")]
    pub contracts: ::prost::alloc::vec::Vec<ContractSummary>,
    /// Total number of contracts in paginated set
    #[prost(uint32, tag="2")]
    pub total_contracts: u32,
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
/// "PROMISSORY_ID_1",
/// "PROMISSORY_ID_2"
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
/// "PROMISSORY_ID_1",
/// "PROMISSORY_ID_2"
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
/// "PROMISSORY_ID_3",
/// "PROMISSORY_ID_4"
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransactionSnapshotsResponse {
    /// Transaction snapshots.
    #[prost(message, repeated, tag="1")]
    pub snapshots: ::prost::alloc::vec::Vec<super::super::common::TransactionSnapshot>,
}
/// \[Example\]
/// {
/// "balance": {
/// "amount": {
/// "currency_code":"USD",
/// "amount":5700,
/// "decimals":2
/// },
/// "file_count": 57
/// },
/// "kind": 1
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceDetail {
    /// Sum of a particular currency in the wallet
    #[prost(message, optional, tag="1")]
    pub balance: ::core::option::Option<WalletBalance>,
    /// Classification of the balance
    #[prost(enumeration="balance_detail::Kind", tag="2")]
    pub kind: i32,
}
/// Nested message and enum types in `BalanceDetail`.
pub mod balance_detail {
    /// Balance Type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        /// Indicates a balance that is available to be transferred
        Available = 0,
        /// Indicates a balance that has been earmarked to be transferred
        Reserved = 1,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Available => "KIND_AVAILABLE",
                Kind::Reserved => "KIND_RESERVED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "KIND_AVAILABLE" => Some(Self::Available),
                "KIND_RESERVED" => Some(Self::Reserved),
                _ => None,
            }
        }
    }
}
/// \[Example\]
/// {
/// "wallet_id": "zWalletIDMultibase58Encoded",
/// "api_key": "API_KEY"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWalletBalanceDetailsRequest {
    /// UUID of the wallet.
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "balances": [
/// {
/// "balance": {
/// "amount": {
/// "currency_code":"USD",
/// "amount":5700,
/// "decimals":2
/// },
/// "file_count": 57
/// },
/// "kind": 1
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWalletBalanceDetailsResponse {
    /// List of Balances in Wallet.
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<BalanceDetail>,
}
/// \[Example\]
/// {
/// "wallet_id": "zWalletIDMultibase58Encoded",
/// "api_key": "API_KEY",
/// "uetr": "UETR"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContractTransactionDetailRequest {
    /// UUID of the wallet.
    #[prost(string, tag="1")]
    pub wallet_id: ::prost::alloc::string::String,
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="3")]
    pub uetr: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "contract_type": "Asset Swap",
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
/// "currency_code":"MXN",
/// "amount":100,
/// "decimals":2
/// }
/// }
/// ],
/// "originator": "zSender1PublicKey",
/// "created": "2023-12-19T16:39:56-08:00",
/// "hash_presenter": "zSender1PublicKey",
/// "hash": "d04b98f48e8f8bcc15c6ae5ac050801cd6dcfd428fb5f9e65c4e16e7807340fa",
/// "expiration": "2023-12-19T16:39:57-08:00",
/// "memo": "Currency swap",
/// "log": [
/// {
/// "sender": "zSender1PublicKey",
/// "receiver": "zRecipient1PublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// },
/// "timestamp": "2006-01-02T15:04:05Z",
/// "kind": "ContractPayment",
/// "state": "Awaiting",
/// "event": "Received promissory",
/// "promissory_id": "00000000-0000-0000-0000-000000000000"
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContractTransactionDetailResponse {
    /// Type of this contract
    #[prost(string, tag="1")]
    pub contract_type: ::prost::alloc::string::String,
    /// Lists all fund transfers specified in the contract
    #[prost(message, repeated, tag="2")]
    pub commitments: ::prost::alloc::vec::Vec<super::super::common::Commitment>,
    /// \[MultibaseVerifier\] Participant that created the contract
    #[prost(string, tag="3")]
    pub originator: ::prost::alloc::string::String,
    /// [RFC3339 Timestamp] Time the contract was created
    #[prost(string, tag="4")]
    pub created: ::prost::alloc::string::String,
    /// \[MultibaseVerifier\] that is responsible for presenting the secret
    #[prost(string, tag="5")]
    pub hash_presenter: ::prost::alloc::string::String,
    /// [Sha256 Hash] Hash of the secret
    #[prost(bytes="vec", tag="6")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// [RFC3339 Timestamp] Expiration time of the contract where all funds are returned to all participants
    #[prost(string, tag="7")]
    pub expiration: ::prost::alloc::string::String,
    /// Human-readable memo field
    #[prost(string, tag="8")]
    pub memo: ::prost::alloc::string::String,
    /// Log of all interactions of this contract
    #[prost(message, repeated, tag="9")]
    pub log: ::prost::alloc::vec::Vec<get_contract_transaction_detail_response::LogEntry>,
}
/// Nested message and enum types in `GetContractTransactionDetailResponse`.
pub mod get_contract_transaction_detail_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LogEntry {
        /// \[MultibaseVerifier\]
        #[prost(string, tag="1")]
        pub sender: ::prost::alloc::string::String,
        /// \[MultibaseVerifier\]
        #[prost(string, tag="2")]
        pub receiver: ::prost::alloc::string::String,
        /// \[Amount\] Promissory amount if it applies
        #[prost(message, optional, tag="3")]
        pub amount: ::core::option::Option<super::super::super::common::Amount>,
        /// [RFC3339 Timestamp] Event time
        #[prost(string, tag="4")]
        pub timestamp: ::prost::alloc::string::String,
        /// State machine this occurred in
        #[prost(string, tag="5")]
        pub kind: ::prost::alloc::string::String,
        /// State in the state machine
        #[prost(string, tag="6")]
        pub state: ::prost::alloc::string::String,
        /// Description of the event
        #[prost(string, tag="7")]
        pub event: ::prost::alloc::string::String,
        /// \[UUID\] The unique identifier of a promissory if it applies
        #[prost(string, tag="8")]
        pub promissory_id: ::prost::alloc::string::String,
    }
}
/// \[Example\]
/// {
/// "api_key": "API_KEY",
/// "wallet_id": "UUID",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// },
/// "respondent": "zRespondentPublicKey",
/// "respondent_amount": {
/// "currency_code":"MXN",
/// "amount":100,
/// "decimals":2
/// },
/// "memo": "Currency Swap",
/// "timeout": 30
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareAtomicAssetSwapRequest {
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// UUID of the wallet.
    #[prost(string, tag="2")]
    pub wallet_id: ::prost::alloc::string::String,
    /// \[Amount\] Amount to transfer from this wallet to the respondent
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::common::Amount>,
    /// \[MultibaseVerifier\] Participant that is responsible for Hash generation
    #[prost(string, tag="4")]
    pub respondent: ::prost::alloc::string::String,
    /// \[Amount\] Amount to transfer from the respondent to the originator
    #[prost(message, optional, tag="5")]
    pub respondent_amount: ::core::option::Option<super::super::common::Amount>,
    /// Human-readable memo field
    #[prost(string, tag="6")]
    pub memo: ::prost::alloc::string::String,
    /// \[Seconds\] The duration in which this request should be completed in
    #[prost(uint32, tag="7")]
    pub timeout: u32,
}
/// \[Example\]
/// {
/// "status": 0,
/// "uetr": "UETR"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareAtomicAssetSwapResponse {
    /// Status of the transaction
    #[prost(enumeration="prepare_atomic_asset_swap_response::Status", tag="1")]
    pub status: i32,
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="2")]
    pub uetr: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PrepareAtomicAssetSwapResponse`.
pub mod prepare_atomic_asset_swap_response {
    /// Prepare Contract Transaction Status.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Status Unspecified.
        Unspecified = 0,
        /// Status Successful.
        Success = 1,
        /// Status Unsuccessful.
        Failure = 2,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "UNSPECIFIED",
                Status::Success => "SUCCESS",
                Status::Failure => "FAILURE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCESS" => Some(Self::Success),
                "FAILURE" => Some(Self::Failure),
                _ => None,
            }
        }
    }
}
/// \[Example\]
/// {
/// "api_key": "API_KEY",
/// "wallet_id": "UUID",
/// "treasury": "zTreasuryPublicKey",
/// "respondent": "zRespondentPublicKey",
/// "amount": {
/// "currency_code":"USD",
/// "amount":100,
/// "decimals":2
/// },
/// "respondent_amount": {
/// "currency_code":"MXN",
/// "amount":100,
/// "decimals":2
/// },
/// "memo": "Cross border to MX",
/// "timeout": 30
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareCrossBorderPaymentRequest {
    /// API key given to the wallet owner at the time of owner creation.
    #[prost(string, tag="1")]
    pub api_key: ::prost::alloc::string::String,
    /// UUID of the wallet.
    #[prost(string, tag="2")]
    pub wallet_id: ::prost::alloc::string::String,
    /// \[MultibaseVerifier\] Participant that will act as exchange.
    #[prost(string, tag="3")]
    pub treasury: ::prost::alloc::string::String,
    /// \[Amount\] Amount to transfer from this wallet to the treasury.
    #[prost(message, optional, tag="4")]
    pub amount: ::core::option::Option<super::super::common::Amount>,
    /// \[MultibaseVerifier\] Participant that receives funds and is responsible for Hash generation.
    #[prost(string, tag="5")]
    pub respondent: ::prost::alloc::string::String,
    /// \[Amount\] Amount to transfer from the treasury to the respondent.
    #[prost(message, optional, tag="6")]
    pub respondent_amount: ::core::option::Option<super::super::common::Amount>,
    /// Human-readable memo field.
    #[prost(string, tag="7")]
    pub memo: ::prost::alloc::string::String,
    /// \[Seconds\] The duration in which this request should be completed in.
    #[prost(uint32, tag="8")]
    pub timeout: u32,
}
/// \[Example\]
/// {
/// "status": 0,
/// "uetr": "UETR"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareCrossBorderPaymentResponse {
    /// Status of the transaction
    #[prost(enumeration="prepare_cross_border_payment_response::Status", tag="1")]
    pub status: i32,
    /// A Unique End-to-end Transaction Reference, string of 36 unique characters.
    #[prost(string, tag="2")]
    pub uetr: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PrepareCrossBorderPaymentResponse`.
pub mod prepare_cross_border_payment_response {
    /// Prepare Cross Border Payment Status.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Status Unspecified.
        Unspecified = 0,
        /// Status Successful.
        Success = 1,
        /// Status Unsuccessful.
        Failure = 2,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "UNSPECIFIED",
                Status::Success => "SUCCESS",
                Status::Failure => "FAILURE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCESS" => Some(Self::Success),
                "FAILURE" => Some(Self::Failure),
                _ => None,
            }
        }
    }
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
