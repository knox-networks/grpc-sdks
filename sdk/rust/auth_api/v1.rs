// @generated
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
    #[prost(string, tag="7")]
    pub role: ::prost::alloc::string::String,
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
    #[prost(string, tag="6")]
    pub role: ::prost::alloc::string::String,
    /// Avatar URL of the user.
    #[prost(string, tag="7")]
    pub avatar: ::prost::alloc::string::String,
    /// Phone number of the user.
    #[prost(string, tag="8")]
    pub phone: ::prost::alloc::string::String,
}
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserResponse {
    /// User object of the registeree.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmUserRequest {
    /// Confirmation method for user registration.
    #[prost(string, tag="1")]
    pub email_or_phone: ::prost::alloc::string::String,
    /// Token received from previous request.
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
}
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfirmUserResponse {
}
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnWithProviderResponse {
    /// Callback URL to be used for authentication.
    #[prost(string, tag="1")]
    pub provider_url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleOidcCallbackRequest {
    /// Code received in the previous response.
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
    /// Used to store state. 
    #[prost(string, tag="2")]
    pub state: ::prost::alloc::string::String,
}
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindByEmailRequest {
    /// Email address of the user.
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindByEmailResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindByIdRequest {
    /// Unique identifier of the user for internal use.
    #[prost(string, tag="1")]
    pub uuid: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindByIdResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnWithPasswordRequest {
    /// Email address of the user.
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    /// Password of the user.
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnWithPasswordResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access Token.
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshAccessTokenRequest {
    /// OAuth2 RFC6749 Refresh Token.
    #[prost(string, tag="1")]
    pub refresh_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshAccessTokenResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access Token.
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
}
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMeRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMeResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthTokenResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access Token.
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
}
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthnBrowserWithWalletChallengeRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthnBrowserWithWalletChallengeResponse {
    /// Expand for details.
    #[prost(oneof="create_authn_browser_with_wallet_challenge_response::DidStart", tags="1, 2")]
    pub did_start: ::core::option::Option<create_authn_browser_with_wallet_challenge_response::DidStart>,
}
/// Nested message and enum types in `CreateAuthnBrowserWithWalletChallengeResponse`.
pub mod create_authn_browser_with_wallet_challenge_response {
    /// Expand for details.
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
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnBrowserWithWalletResponse {
}
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRegisterWalletChallengeRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRegisterWalletChallengeResponse {
    /// Expand for details.
    #[prost(oneof="create_register_wallet_challenge_response::RegistrationStart", tags="1, 2")]
    pub registration_start: ::core::option::Option<create_register_wallet_challenge_response::RegistrationStart>,
}
/// Nested message and enum types in `CreateRegisterWalletChallengeResponse`.
pub mod create_register_wallet_challenge_response {
    /// Expand for details.
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterWalletResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access Token.
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthnWalletChallengeRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthnWalletChallengeResponse {
    /// Value used for the challenge.
    #[prost(string, tag="1")]
    pub nonce: ::prost::alloc::string::String,
}
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnWalletResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    /// OAuth2 RFC6749 Access TokenUser user = 1;
    #[prost(message, optional, tag="2")]
    pub auth_token: ::core::option::Option<AuthToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HandleSamlCallbackRequest {
    /// SAML RFC7522 response from the identity provider.
    #[prost(string, tag="1")]
    pub saml_response: ::prost::alloc::string::String,
    /// URL for the identity provider to send the response.
    #[prost(string, tag="2")]
    pub relay_state: ::prost::alloc::string::String,
}
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
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamlspMetadataRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamlspMetadataResponse {
    /// Metadata about the service provider using the identity provider.
    #[prost(bytes="vec", tag="1")]
    pub sp_metadata: ::prost::alloc::vec::Vec<u8>,
}
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppSettingsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppSettings {
    /// Supported identity providers used for authentication.
    #[prost(message, repeated, tag="1")]
    pub authn_providers: ::prost::alloc::vec::Vec<AuthnProvider>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthnProvider {
    /// Identity provider used for authentication.
    #[prost(enumeration="Provider", tag="1")]
    pub provider: i32,
    /// True or false.
    #[prost(bool, tag="2")]
    pub enabled: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppSettingsResponse {
    /// Appsettings object configured in the environment.
    #[prost(message, optional, tag="1")]
    pub app_settings: ::core::option::Option<AppSettings>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByDidRequest {
    /// W3C Decentralized Identifier (DID) of the wallet.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserByDidResponse {
    /// User object of the subject.
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateWalletRequest {
    /// W3C Decentralized Identifier (DID) of the wallet to be associated with the account in the bearer token.
    #[prost(string, tag="1")]
    pub did: ::prost::alloc::string::String,
}
/// Empty body.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateWalletResponse {
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
}
include!("v1.tonic.rs");
// @@protoc_insertion_point(module)
