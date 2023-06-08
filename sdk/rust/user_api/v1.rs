// @generated
// THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

/// \[Example\]
/// {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": [
/// {
/// "key": "VALUE"
/// }
/// ]
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// Unique identifier of the user for internal use.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
    /// First name of the user.
    #[prost(string, tag="2")]
    pub first_name: ::prost::alloc::string::String,
    /// Last name of the user.
    #[prost(string, tag="3")]
    pub last_name: ::prost::alloc::string::String,
    /// Email address of the user.
    #[prost(string, tag="6")]
    pub email: ::prost::alloc::string::String,
    /// Role of the user, either but not limited to Admin or User.
    #[prost(enumeration="Role", tag="7")]
    pub role: i32,
    /// Avatar URL of the user.
    #[prost(string, tag="8")]
    pub avatar: ::prost::alloc::string::String,
    /// Phone number of the user.
    #[prost(string, tag="9")]
    pub phone: ::prost::alloc::string::String,
    /// W3C Decentralized Identifiers (DIDs) associated to the user.
    #[prost(string, repeated, tag="10")]
    pub dids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// External system identifiers associated to the user. Ex UserID from the user's financial institution.
    #[prost(map="string, string", tag="11")]
    pub external_ids: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Additional data about the user such as account, address, branch, routing info, etc.
    #[prost(message, optional, tag="12")]
    pub metadata: ::core::option::Option<::prost_types::Struct>,
}
/// \[Example\]
/// {
/// "email": "example@gmail.com",
/// "first_name": "John", 
/// "last_name": "Doe",
/// "password": "PASSWORD",
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555" 
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserRequest {
    /// Email address of the user.
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    /// First name of the user.
    #[prost(string, tag="2")]
    pub first_name: ::prost::alloc::string::String,
    /// Last name of the user.
    #[prost(string, tag="3")]
    pub last_name: ::prost::alloc::string::String,
    /// Password of the user.
    #[prost(string, tag="5")]
    pub password: ::prost::alloc::string::String,
    /// Role of the user, either but not limited to Admin or User.
    #[prost(enumeration="Role", tag="6")]
    pub role: i32,
    /// Avatar URL of the user.
    #[prost(string, tag="7")]
    pub avatar: ::prost::alloc::string::String,
    /// Phone number of the user.
    #[prost(string, tag="8")]
    pub phone: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "token": "TOKEN", 
/// "token_type": "TOKEN_TYPE", 
/// "expires_in": 100, 
/// "refresh_token": "REFRESH_TOKEN"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthToken {
    /// OAuth2 RFC6749 Access Token.
    #[prost(string, tag="1")]
    pub token: ::prost::alloc::string::String,
    /// OAuth2 token type- Default Bearer- RFC6750.
    #[prost(string, tag="2")]
    pub token_type: ::prost::alloc::string::String,
    /// Validity time of the token in seconds.
    #[prost(int32, tag="3")]
    pub expires_in: i32,
    /// OAuth2 RFC6749 Refresh Token.
    #[prost(string, tag="4")]
    pub refresh_token: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": []
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserResponse {
    /// User object of the registeree.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
/// \[Example\]
/// {
/// "email_or_phone": "EMAIL_OR_PHONE", 
/// "token": "TOKEN"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmUserRequest {
    /// Confirmation method for user registration.
    #[prost(string, tag="1")]
    pub email_or_phone: ::prost::alloc::string::String,
    /// Token received from previous request.
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
}
/// \[Example\]
/// {} 
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmUserResponse {
}
/// \[Example\]
/// {
/// "provider": 0, 
/// "instance_name": "INSTANCE_NAME", 
/// "request_origin": "REQUEST_ORIGIN", 
/// "client_state": { } 
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnWithProviderRequest {
    /// Identity Provider to authenticate against via OIDC or SAML.
    #[prost(enumeration="Provider", tag="1")]
    pub provider: i32,
    /// Used by client to identify application instance initiating authentication.
    #[prost(string, tag="2")]
    pub instance_name: ::prost::alloc::string::String,
    /// Used by client to identify origin of the request initiating authentication.
    #[prost(string, tag="3")]
    pub request_origin: ::prost::alloc::string::String,
    /// Used by client to store state. 
    #[prost(message, optional, tag="4")]
    pub client_state: ::core::option::Option<::prost_types::Struct>,
}
/// \[Example\]
/// {
/// "provider_url": "PROVIDER_URL"
/// } 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnWithProviderResponse {
    /// Callback URL to be used for authentication.
    #[prost(string, tag="1")]
    pub provider_url: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "code": "CODE_PREVIOUS_RESPONSE", 
/// "state": "AL"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleOidcCallbackRequest {
    /// Code received in the previous response.
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
    /// Used to store state. 
    #[prost(string, tag="2")]
    pub state: ::prost::alloc::string::String,
}
/// \[Example\] 
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": []
/// }
/// },
/// "auth_token": { 
/// "token": "TOKEN", 
/// "token_type": "TOKEN_TYPE", 
/// "expires_in": 100, 
/// "refresh_token": "REFRESH_TOKEN"
/// }, 
/// "instance_name": "INSTANCE_NAME", 
/// "request_origin": "REQUEST_ORIGIN", 
/// "client_state": {} 
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleOidcCallbackResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access Token.
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
    /// Used by client to identify application instance initiating authentication.
    #[prost(string, tag="3")]
    pub instance_name: ::prost::alloc::string::String,
    /// Used by client to identify origin of the request initiating authentication.
    #[prost(string, tag="4")]
    pub request_origin: ::prost::alloc::string::String,
    /// Used by client to store state. 
    #[prost(message, optional, tag="5")]
    pub client_state: ::core::option::Option<::prost_types::Struct>,
}
/// \[Example\]
/// {
/// "email": "example@gmail.com"
/// } 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindByEmailRequest {
    /// Email address of the user.
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": []
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindByEmailResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
/// \[Example\]
/// {
/// "uuid": "UUID"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindByIdRequest {
    /// Unique identifier of the user for internal use.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": []
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindByIdResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
/// \[Example\]
/// {
/// "email": "example@gmail.com", 
/// "password": "PASSWORD"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnWithPasswordRequest {
    /// Email address of the user.
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    /// Password of the user.
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": []
/// }
/// }, 
/// "auth_token": {
/// "token": "TOKEN", 
/// "token_type": "TOKEN_TYPE", 
/// "expires_in": 100, 
/// "refresh_token": "REFRESH_TOKEN"
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnWithPasswordResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access Token.
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
}
/// \[Example\]
/// {
/// "refresh_token": "REFRESH_TOKEN"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshAccessTokenRequest {
    /// OAuth2 RFC6749 Refresh Token.
    #[prost(string, tag="1")]
    pub refresh_token: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": []
/// }
/// }, 
/// "auth_token": {
/// "token": "TOKEN", 
/// "token_type": "TOKEN_TYPE", 
/// "expires_in": 100, 
/// "refresh_token": "REFRESH_TOKEN"
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshAccessTokenResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access Token.
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMeRequest {
}
/// \[Example\]
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": []
/// }
/// }
/// } 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMeResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
/// \[Example\]
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": []
/// }
/// }, 
/// "auth_token": {
/// "token": "TOKEN", 
/// "token_type": "TOKEN_TYPE", 
/// "expires_in": 100, 
/// "refresh_token": "REFRESH_TOKEN"
/// }
/// } 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthTokenResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access Token.
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthnBrowserWithWalletChallengeRequest {
}
/// \[Example\]
/// {
/// "did_start": { 
/// "nonce": "NONCE" 
/// }
/// } 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthnBrowserWithWalletChallengeResponse {
    /// nonce OR auth_token - Expand for details.
    #[prost(oneof="create_authn_browser_with_wallet_challenge_response::DidStart", tags="1, 2")]
    pub did_start: ::core::option::Option<create_authn_browser_with_wallet_challenge_response::DidStart>,
}
/// Nested message and enum types in `CreateAuthnBrowserWithWalletChallengeResponse`.
pub mod create_authn_browser_with_wallet_challenge_response {
    /// nonce OR auth_token - Expand for details.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DidStart {
        /// Value used for the challenge.
        #[prost(string, tag="1")]
        Nonce(::prost::alloc::string::String),
        /// OAuth2 RFC6749 Access Token.
        #[prost(message, tag="2")]
        AuthToken(super::AuthTokenResponse),
    }
}
/// \[Example\]
/// {
/// "signature": "ByteLengthSignatureVariesByAlgo=", 
/// "nonce": "NONCE", 
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnBrowserWithWalletRequest {
    /// Signed value of the challenge by the wallet.
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Value used for the challenge.
    #[prost(string, tag="2")]
    pub nonce: ::prost::alloc::string::String,
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="3")]
    pub did: ::prost::alloc::string::String,
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnBrowserWithWalletResponse {
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRegisterWalletChallengeRequest {
}
/// \[Example\]
/// {
/// "registration_start": { 
/// "nonce": "NONCE" 
/// }
/// } 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRegisterWalletChallengeResponse {
    /// [nonce, ok]- Expand for details.
    #[prost(oneof="create_register_wallet_challenge_response::RegistrationStart", tags="1, 2")]
    pub registration_start: ::core::option::Option<create_register_wallet_challenge_response::RegistrationStart>,
}
/// Nested message and enum types in `CreateRegisterWalletChallengeResponse`.
pub mod create_register_wallet_challenge_response {
    /// [nonce, ok]- Expand for details.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RegistrationStart {
        /// Value used for the challenge.
        #[prost(string, tag="1")]
        Nonce(::prost::alloc::string::String),
        /// Confirmation of successful challenge.
        #[prost(string, tag="2")]
        Ok(::prost::alloc::string::String),
    }
}
/// \[Example\]
/// {
/// "signature": "ByteLengthSignatureVariesByAlgo=", 
/// "nonce": "NONCE", 
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterWalletRequest {
    /// Signed value of the challenge by the wallet.
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Value used for the challenge.
    #[prost(string, tag="2")]
    pub nonce: ::prost::alloc::string::String,
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="3")]
    pub did: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": []
/// }
/// }, 
/// "auth_token": {
/// "token": "TOKEN", 
/// "token_type": "TOKEN_TYPE", 
/// "expires_in": 100, 
/// "refresh_token": "REFRESH_TOKEN"
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterWalletResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access Token.
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthnWalletChallengeRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "nonce": "NONCE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthnWalletChallengeResponse {
    /// Value used for the challenge.
    #[prost(string, tag="1")]
    pub nonce: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "signature": "ByteLengthSignatureVariesByAlgo=", 
/// "nonce": "NONCE", 
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnWalletRequest {
    /// Signed value of the challenge by the wallet.
    #[prost(bytes="vec", tag="1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Value used for the challenge.
    #[prost(string, tag="2")]
    pub nonce: ::prost::alloc::string::String,
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="3")]
    pub did: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": []
/// }
/// }, 
/// "auth_token": {
/// "token": "TOKEN", 
/// "token_type": "TOKEN_TYPE", 
/// "expires_in": 100, 
/// "refresh_token": "REFRESH_TOKEN"
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnWalletResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access TokenUser user = 1;
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
}
/// \[Example\]
/// {
/// "saml_response": "SAML_RESPONSE", 
/// "relay_state": "RELAY_STATE"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleSamlCallbackRequest {
    /// SAML RFC7522 response from the identity provider.
    #[prost(string, tag="1")]
    pub saml_response: ::prost::alloc::string::String,
    /// URL for the identity provider to send the response.
    #[prost(string, tag="2")]
    pub relay_state: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": []
/// }
/// },
/// "auth_token": { 
/// "token": "TOKEN", 
/// "token_type": "TOKEN_TYPE", 
/// "expires_in": 100, 
/// "refresh_token": "REFRESH_TOKEN"
/// }, 
/// "instance_name": "INSTANCE_NAME", 
/// "request_origin": "REQUEST_ORIGIN"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleSamlCallbackResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access Token
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
    /// Used by client to identify application instance initiating authentication.
    #[prost(string, tag="3")]
    pub instance_name: ::prost::alloc::string::String,
    /// Used by client to identify origin of the request initiating authentication.
    #[prost(string, tag="4")]
    pub request_origin: ::prost::alloc::string::String,
}
/// \[Example\]
/// {} 
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamlspMetadataRequest {
}
/// \[Example\]
/// {
/// "sp_metadata": "SPMetadataVariesLen="
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamlspMetadataResponse {
    /// Metadata about the service provider using the identity provider.
    #[prost(bytes="vec", tag="1")]
    pub sp_metadata: ::prost::alloc::vec::Vec<u8>,
}
/// \[Example\]
/// {} 
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppSettingsRequest {
}
/// \[Example\]
/// {
/// "authn_providers": [
/// {
/// "provider": 0, 
/// "enabled": true
/// }, 
/// {
/// "provider": 1, 
/// "enabled": false
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppSettings {
    /// Supported identity providers used for authentication.
    #[prost(message, repeated, tag="1")]
    pub authn_providers: ::prost::alloc::vec::Vec<AuthnProvider>,
}
/// \[Example\]
/// {
/// "provider": 0, 
/// "enabled": true
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnProvider {
    /// Identity provider used for authentication.
    #[prost(enumeration="Provider", tag="1")]
    pub provider: i32,
    /// True or false.
    #[prost(bool, tag="2")]
    pub enabled: bool,
}
/// \[Example\]
/// {
/// "app_settings": {
/// "authn_providers": [
/// {
/// "provider": 0, 
/// "enabled": true
/// }, 
/// {
/// "provider": 1, 
/// "enabled": false
/// }
/// ]
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppSettingsResponse {
    /// AppSettings object configured in the environment.
    #[prost(message, optional, tag="1")]
    pub app_settings: ::core::option::Option<AppSettings>,
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// } 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByDidRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "user": {
/// "uuid": "UUID", 
/// "first_name": "John", 
/// "last_name": "Doe", 
/// "email": "example@gmail.com", 
/// "role": 0, 
/// "avatar": "AVATAR_URL", 
/// "phone": "+1 555 555 5555", 
/// "dids": [
/// "did:method-name:zDIDMultibase58Encoded", 
/// "did:method-name:zDIDMultibase58Encoded"
/// ], 
/// "external_ids": [
/// {
/// "key": "KEY1", 
/// "value": "VALUE1"
/// }, 
/// {
/// "key": "KEY2", 
/// "value": "VALUE2"
/// }
/// ], 
/// "metadata": { 
/// "fields": [
/// {
/// "key": "VALUE"
/// }
/// ]
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByDidResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// } 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateWalletRequest {
    /// W3C Decentralized Identifier (DID) of the wallet to be associated with the account in the bearer token.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
}
/// \[Example\]
/// {} 
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateWalletResponse {
}
/// \[Example\]
/// {
/// "did": "did:method-name:zDIDMultibase58Encoded"
/// } 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DissociateWalletRequest {
    /// W3C Decentralized Identifier (DID) of the wallet to be associated with the account in the bearer token.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
}
/// \[Example\]
/// {} 
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DissociateWalletResponse {
}
/// \[Example\]
/// {
/// "client_id": "CLIENT_ID",
/// "client_name": "CLIENT_NAME",
/// "api_key": "API_KEY",
/// "status": 0,
/// "expiry_date": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiKeyEntity {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub api_key: ::prost::alloc::string::String,
    #[prost(enumeration="ApiKeyStatus", tag="4")]
    pub status: i32,
    #[prost(message, optional, tag="5")]
    pub expiry_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// \[Example\]
/// {
/// "client_name": "CLIENT_NAME"
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiKeyRequest {
    #[prost(string, tag="1")]
    pub client_name: ::prost::alloc::string::String,
}
/// \[Example\]
/// {
/// "api_key_entity": {
/// "client_id": "CLIENT_ID",
/// "client_name": "CLIENT_NAME",
/// "api_key": "API_KEY",
/// "status": 0,
/// "expiry_date": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }
/// }
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiKeyResponse {
    #[prost(message, optional, tag="1")]
    pub api_key_entity: ::core::option::Option<ApiKeyEntity>,
}
/// \[Example\]
/// {
/// "client_id": "CLIENT_ID",
/// "api_key": "API_KEY",
/// "api_key_status": 0
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApiKeyStatusRequest {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub api_key: ::prost::alloc::string::String,
    #[prost(enumeration="ApiKeyStatus", tag="3")]
    pub api_key_status: i32,
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApiKeyStatusResponse {
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiKeysRequest {
}
/// \[Example\]
/// {
/// "api_key_entities": [
/// {
/// "client_id": "CLIENT_ID1",
/// "client_name": "CLIENT_NAME1",
/// "api_key": "API_KEY",
/// "status": 0,
/// "expiry_date": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }
/// },
/// {
/// "client_id": "CLIENT_ID2",
/// "client_name": "CLIENT_NAME2",
/// "api_key": "API_KEY",
/// "status": 0,
/// "expiry_date": { 
/// "seconds": 0, 
/// "nanos": 0 
/// }
/// }
/// ]
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiKeysResponse {
    #[prost(message, repeated, tag="1")]
    pub api_key_entities: ::prost::alloc::vec::Vec<ApiKeyEntity>,
}
/// \[Example\]
/// {}
///
/// Empty body.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckApiKeyRequest {
}
/// \[Example\]
/// {
/// "is_valid": true
/// }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckApiKeyResponse {
    /// Returns the validity of the API key
    #[prost(bool, tag="3")]
    pub is_valid: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Provider {
    Unspecified = 0,
    Github = 1,
    Facebook = 2,
    Google = 3,
    Saml = 4,
    Cognito = 5,
}
impl Provider {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Provider::Unspecified => "PROVIDER_UNSPECIFIED",
            Provider::Github => "PROVIDER_GITHUB",
            Provider::Facebook => "PROVIDER_FACEBOOK",
            Provider::Google => "PROVIDER_GOOGLE",
            Provider::Saml => "PROVIDER_SAML",
            Provider::Cognito => "PROVIDER_COGNITO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROVIDER_UNSPECIFIED" => Some(Self::Unspecified),
            "PROVIDER_GITHUB" => Some(Self::Github),
            "PROVIDER_FACEBOOK" => Some(Self::Facebook),
            "PROVIDER_GOOGLE" => Some(Self::Google),
            "PROVIDER_SAML" => Some(Self::Saml),
            "PROVIDER_COGNITO" => Some(Self::Cognito),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Role {
    Unspecified = 0,
    User = 1,
    Admin = 2,
}
impl Role {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Role::Unspecified => "ROLE_UNSPECIFIED",
            Role::User => "ROLE_USER",
            Role::Admin => "ROLE_ADMIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROLE_UNSPECIFIED" => Some(Self::Unspecified),
            "ROLE_USER" => Some(Self::User),
            "ROLE_ADMIN" => Some(Self::Admin),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ApiKeyStatus {
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
}
impl ApiKeyStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ApiKeyStatus::Unspecified => "API_KEY_STATUS_UNSPECIFIED",
            ApiKeyStatus::Active => "API_KEY_STATUS_ACTIVE",
            ApiKeyStatus::Inactive => "API_KEY_STATUS_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "API_KEY_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "API_KEY_STATUS_ACTIVE" => Some(Self::Active),
            "API_KEY_STATUS_INACTIVE" => Some(Self::Inactive),
            _ => None,
        }
    }
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
