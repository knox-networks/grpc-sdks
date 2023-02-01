// @generated
/// Generated client implementations.
pub mod user_management_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /**This service integrates with existing identity systems (Ex. Bank accounts) to bridge traditional account identifiers to DID/public key identifiers used in the Knox ecosystem. Leverages existing KYC/AML data to enable financial transaction compliance.  Supports OIDC, SAML and other traditional AuthN/Z standards.
*/
    #[derive(Debug, Clone)]
    pub struct UserManagementClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserManagementClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UserManagementClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UserManagementClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            UserManagementClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /** Login with username and password for admin and test users.
*/
        pub async fn authn_with_password(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthnWithPasswordRequest>,
        ) -> Result<tonic::Response<super::AuthnWithPasswordResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/AuthnWithPassword",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Login with refresh token and get a new access token.
*/
        pub async fn refresh_access_token(
            &mut self,
            request: impl tonic::IntoRequest<super::RefreshAccessTokenRequest>,
        ) -> Result<tonic::Response<super::RefreshAccessTokenResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/RefreshAccessToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Start login process against a selected identity provider. Supports OIDC and SAML.
*/
        pub async fn authn_with_provider(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthnWithProviderRequest>,
        ) -> Result<tonic::Response<super::AuthnWithProviderResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/AuthnWithProvider",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Complete login process with the information provided by the OIDC identity provider in previous step.
*/
        pub async fn handle_oidc_callback(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleOidcCallbackRequest>,
        ) -> Result<tonic::Response<super::HandleOidcCallbackResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/HandleOIDCCallback",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Complete login process with the information provided by the SAML identity provider in previous step.
*/
        pub async fn handle_saml_callback(
            &mut self,
            request: impl tonic::IntoRequest<super::HandleSamlCallbackRequest>,
        ) -> Result<tonic::Response<super::HandleSamlCallbackResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/HandleSAMLCallback",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Retrieve SAML service provider metadata.
*/
        pub async fn samlsp_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::SamlspMetadataRequest>,
        ) -> Result<tonic::Response<super::SamlspMetadataResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/SAMLSPMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Used for logging into the browser using a device wallet. Called from the browser, bearer token is received from the stream response once authentication is successful via the wallet.
*/
        pub async fn create_authn_browser_with_wallet_challenge(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateAuthnBrowserWithWalletChallengeRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::CreateAuthnBrowserWithWalletChallengeResponse,
                >,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/CreateAuthnBrowserWithWalletChallenge",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /** Called from the device wallet to authenticate the browser and bearer token is sent to the browser session.
*/
        pub async fn authn_browser_with_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthnBrowserWithWalletRequest>,
        ) -> Result<
            tonic::Response<super::AuthnBrowserWithWalletResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/AuthnBrowserWithWallet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Start login process on the device wallet.
*/
        pub async fn create_authn_wallet_challenge(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAuthnWalletChallengeRequest>,
        ) -> Result<
            tonic::Response<super::CreateAuthnWalletChallengeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/CreateAuthnWalletChallenge",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Finish login with cryptographic challenge and bearer token is returned to the caller.
*/
        pub async fn authn_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthnWalletRequest>,
        ) -> Result<tonic::Response<super::AuthnWalletResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/AuthnWallet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Called by the browser in an authenticated session to start a device wallet registration process. Acknowledgement is received from the stream response once registration is successful via the wallet.
*/
        pub async fn create_register_wallet_challenge(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRegisterWalletChallengeRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::CreateRegisterWalletChallengeResponse>,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/CreateRegisterWalletChallenge",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /** Called by the device wallet to register against the given authenticated account.
*/
        pub async fn register_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterWalletRequest>,
        ) -> Result<tonic::Response<super::RegisterWalletResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/RegisterWallet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Associate Wallet with the authenticated account in the bearer token.
*/
        pub async fn associate_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::AssociateWalletRequest>,
        ) -> Result<tonic::Response<super::AssociateWalletResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/AssociateWallet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Get application specific settings in a given environment.
*/
        pub async fn get_app_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAppSettingsRequest>,
        ) -> Result<tonic::Response<super::GetAppSettingsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/GetAppSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Admin API to register a user.
*/
        pub async fn register_user(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterUserRequest>,
        ) -> Result<tonic::Response<super::RegisterUserResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/RegisterUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Admin API to finish the user registration process.
*/
        pub async fn confirm_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfirmUserRequest>,
        ) -> Result<tonic::Response<super::ConfirmUserResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/ConfirmUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Admin API to retrieve User object via email.
*/
        pub async fn find_by_email(
            &mut self,
            request: impl tonic::IntoRequest<super::FindByEmailRequest>,
        ) -> Result<tonic::Response<super::FindByEmailResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/FindByEmail",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Admin API to retrieve User object via user ID.
*/
        pub async fn find_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::FindByIdRequest>,
        ) -> Result<tonic::Response<super::FindByIdResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/FindByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Get current authenticated User by bearer token.
*/
        pub async fn get_me(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMeRequest>,
        ) -> Result<tonic::Response<super::GetMeResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/GetMe",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** M2M helper API to retrieve User object given a Decentralized Identifier (DID). API Key required for this call.
*/
        pub async fn get_user_by_did(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserByDidRequest>,
        ) -> Result<tonic::Response<super::GetUserByDidResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/auth_api.v1.UserManagement/GetUserByDID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_management_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with UserManagementServer.
    #[async_trait]
    pub trait UserManagement: Send + Sync + 'static {
        /** Login with username and password for admin and test users.
*/
        async fn authn_with_password(
            &self,
            request: tonic::Request<super::AuthnWithPasswordRequest>,
        ) -> Result<tonic::Response<super::AuthnWithPasswordResponse>, tonic::Status>;
        /** Login with refresh token and get a new access token.
*/
        async fn refresh_access_token(
            &self,
            request: tonic::Request<super::RefreshAccessTokenRequest>,
        ) -> Result<tonic::Response<super::RefreshAccessTokenResponse>, tonic::Status>;
        /** Start login process against a selected identity provider. Supports OIDC and SAML.
*/
        async fn authn_with_provider(
            &self,
            request: tonic::Request<super::AuthnWithProviderRequest>,
        ) -> Result<tonic::Response<super::AuthnWithProviderResponse>, tonic::Status>;
        /** Complete login process with the information provided by the OIDC identity provider in previous step.
*/
        async fn handle_oidc_callback(
            &self,
            request: tonic::Request<super::HandleOidcCallbackRequest>,
        ) -> Result<tonic::Response<super::HandleOidcCallbackResponse>, tonic::Status>;
        /** Complete login process with the information provided by the SAML identity provider in previous step.
*/
        async fn handle_saml_callback(
            &self,
            request: tonic::Request<super::HandleSamlCallbackRequest>,
        ) -> Result<tonic::Response<super::HandleSamlCallbackResponse>, tonic::Status>;
        /** Retrieve SAML service provider metadata.
*/
        async fn samlsp_metadata(
            &self,
            request: tonic::Request<super::SamlspMetadataRequest>,
        ) -> Result<tonic::Response<super::SamlspMetadataResponse>, tonic::Status>;
        ///Server streaming response type for the CreateAuthnBrowserWithWalletChallenge method.
        type CreateAuthnBrowserWithWalletChallengeStream: futures_core::Stream<
                Item = Result<
                    super::CreateAuthnBrowserWithWalletChallengeResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /** Used for logging into the browser using a device wallet. Called from the browser, bearer token is received from the stream response once authentication is successful via the wallet.
*/
        async fn create_authn_browser_with_wallet_challenge(
            &self,
            request: tonic::Request<super::CreateAuthnBrowserWithWalletChallengeRequest>,
        ) -> Result<
            tonic::Response<Self::CreateAuthnBrowserWithWalletChallengeStream>,
            tonic::Status,
        >;
        /** Called from the device wallet to authenticate the browser and bearer token is sent to the browser session.
*/
        async fn authn_browser_with_wallet(
            &self,
            request: tonic::Request<super::AuthnBrowserWithWalletRequest>,
        ) -> Result<
            tonic::Response<super::AuthnBrowserWithWalletResponse>,
            tonic::Status,
        >;
        /** Start login process on the device wallet.
*/
        async fn create_authn_wallet_challenge(
            &self,
            request: tonic::Request<super::CreateAuthnWalletChallengeRequest>,
        ) -> Result<
            tonic::Response<super::CreateAuthnWalletChallengeResponse>,
            tonic::Status,
        >;
        /** Finish login with cryptographic challenge and bearer token is returned to the caller.
*/
        async fn authn_wallet(
            &self,
            request: tonic::Request<super::AuthnWalletRequest>,
        ) -> Result<tonic::Response<super::AuthnWalletResponse>, tonic::Status>;
        ///Server streaming response type for the CreateRegisterWalletChallenge method.
        type CreateRegisterWalletChallengeStream: futures_core::Stream<
                Item = Result<
                    super::CreateRegisterWalletChallengeResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /** Called by the browser in an authenticated session to start a device wallet registration process. Acknowledgement is received from the stream response once registration is successful via the wallet.
*/
        async fn create_register_wallet_challenge(
            &self,
            request: tonic::Request<super::CreateRegisterWalletChallengeRequest>,
        ) -> Result<
            tonic::Response<Self::CreateRegisterWalletChallengeStream>,
            tonic::Status,
        >;
        /** Called by the device wallet to register against the given authenticated account.
*/
        async fn register_wallet(
            &self,
            request: tonic::Request<super::RegisterWalletRequest>,
        ) -> Result<tonic::Response<super::RegisterWalletResponse>, tonic::Status>;
        /** Associate Wallet with the authenticated account in the bearer token.
*/
        async fn associate_wallet(
            &self,
            request: tonic::Request<super::AssociateWalletRequest>,
        ) -> Result<tonic::Response<super::AssociateWalletResponse>, tonic::Status>;
        /** Get application specific settings in a given environment.
*/
        async fn get_app_settings(
            &self,
            request: tonic::Request<super::GetAppSettingsRequest>,
        ) -> Result<tonic::Response<super::GetAppSettingsResponse>, tonic::Status>;
        /** Admin API to register a user.
*/
        async fn register_user(
            &self,
            request: tonic::Request<super::RegisterUserRequest>,
        ) -> Result<tonic::Response<super::RegisterUserResponse>, tonic::Status>;
        /** Admin API to finish the user registration process.
*/
        async fn confirm_user(
            &self,
            request: tonic::Request<super::ConfirmUserRequest>,
        ) -> Result<tonic::Response<super::ConfirmUserResponse>, tonic::Status>;
        /** Admin API to retrieve User object via email.
*/
        async fn find_by_email(
            &self,
            request: tonic::Request<super::FindByEmailRequest>,
        ) -> Result<tonic::Response<super::FindByEmailResponse>, tonic::Status>;
        /** Admin API to retrieve User object via user ID.
*/
        async fn find_by_id(
            &self,
            request: tonic::Request<super::FindByIdRequest>,
        ) -> Result<tonic::Response<super::FindByIdResponse>, tonic::Status>;
        /** Get current authenticated User by bearer token.
*/
        async fn get_me(
            &self,
            request: tonic::Request<super::GetMeRequest>,
        ) -> Result<tonic::Response<super::GetMeResponse>, tonic::Status>;
        /** M2M helper API to retrieve User object given a Decentralized Identifier (DID). API Key required for this call.
*/
        async fn get_user_by_did(
            &self,
            request: tonic::Request<super::GetUserByDidRequest>,
        ) -> Result<tonic::Response<super::GetUserByDidResponse>, tonic::Status>;
    }
    /**This service integrates with existing identity systems (Ex. Bank accounts) to bridge traditional account identifiers to DID/public key identifiers used in the Knox ecosystem. Leverages existing KYC/AML data to enable financial transaction compliance.  Supports OIDC, SAML and other traditional AuthN/Z standards.
*/
    #[derive(Debug)]
    pub struct UserManagementServer<T: UserManagement> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UserManagement> UserManagementServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UserManagementServer<T>
    where
        T: UserManagement,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/auth_api.v1.UserManagement/AuthnWithPassword" => {
                    #[allow(non_camel_case_types)]
                    struct AuthnWithPasswordSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::AuthnWithPasswordRequest>
                    for AuthnWithPasswordSvc<T> {
                        type Response = super::AuthnWithPasswordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthnWithPasswordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).authn_with_password(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthnWithPasswordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/RefreshAccessToken" => {
                    #[allow(non_camel_case_types)]
                    struct RefreshAccessTokenSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::RefreshAccessTokenRequest>
                    for RefreshAccessTokenSvc<T> {
                        type Response = super::RefreshAccessTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RefreshAccessTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).refresh_access_token(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RefreshAccessTokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/AuthnWithProvider" => {
                    #[allow(non_camel_case_types)]
                    struct AuthnWithProviderSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::AuthnWithProviderRequest>
                    for AuthnWithProviderSvc<T> {
                        type Response = super::AuthnWithProviderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthnWithProviderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).authn_with_provider(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthnWithProviderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/HandleOIDCCallback" => {
                    #[allow(non_camel_case_types)]
                    struct HandleOIDCCallbackSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::HandleOidcCallbackRequest>
                    for HandleOIDCCallbackSvc<T> {
                        type Response = super::HandleOidcCallbackResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleOidcCallbackRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).handle_oidc_callback(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HandleOIDCCallbackSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/HandleSAMLCallback" => {
                    #[allow(non_camel_case_types)]
                    struct HandleSAMLCallbackSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::HandleSamlCallbackRequest>
                    for HandleSAMLCallbackSvc<T> {
                        type Response = super::HandleSamlCallbackResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HandleSamlCallbackRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).handle_saml_callback(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HandleSAMLCallbackSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/SAMLSPMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct SAMLSPMetadataSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::SamlspMetadataRequest>
                    for SAMLSPMetadataSvc<T> {
                        type Response = super::SamlspMetadataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SamlspMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).samlsp_metadata(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SAMLSPMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/CreateAuthnBrowserWithWalletChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAuthnBrowserWithWalletChallengeSvc<T: UserManagement>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserManagement,
                    > tonic::server::ServerStreamingService<
                        super::CreateAuthnBrowserWithWalletChallengeRequest,
                    > for CreateAuthnBrowserWithWalletChallengeSvc<T> {
                        type Response = super::CreateAuthnBrowserWithWalletChallengeResponse;
                        type ResponseStream = T::CreateAuthnBrowserWithWalletChallengeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateAuthnBrowserWithWalletChallengeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .create_authn_browser_with_wallet_challenge(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAuthnBrowserWithWalletChallengeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/AuthnBrowserWithWallet" => {
                    #[allow(non_camel_case_types)]
                    struct AuthnBrowserWithWalletSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::AuthnBrowserWithWalletRequest>
                    for AuthnBrowserWithWalletSvc<T> {
                        type Response = super::AuthnBrowserWithWalletResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthnBrowserWithWalletRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).authn_browser_with_wallet(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthnBrowserWithWalletSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/CreateAuthnWalletChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAuthnWalletChallengeSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<
                        super::CreateAuthnWalletChallengeRequest,
                    > for CreateAuthnWalletChallengeSvc<T> {
                        type Response = super::CreateAuthnWalletChallengeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateAuthnWalletChallengeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_authn_wallet_challenge(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAuthnWalletChallengeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/AuthnWallet" => {
                    #[allow(non_camel_case_types)]
                    struct AuthnWalletSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::AuthnWalletRequest>
                    for AuthnWalletSvc<T> {
                        type Response = super::AuthnWalletResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthnWalletRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).authn_wallet(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthnWalletSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/CreateRegisterWalletChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRegisterWalletChallengeSvc<T: UserManagement>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserManagement,
                    > tonic::server::ServerStreamingService<
                        super::CreateRegisterWalletChallengeRequest,
                    > for CreateRegisterWalletChallengeSvc<T> {
                        type Response = super::CreateRegisterWalletChallengeResponse;
                        type ResponseStream = T::CreateRegisterWalletChallengeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateRegisterWalletChallengeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_register_wallet_challenge(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateRegisterWalletChallengeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/RegisterWallet" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterWalletSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::RegisterWalletRequest>
                    for RegisterWalletSvc<T> {
                        type Response = super::RegisterWalletResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterWalletRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).register_wallet(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterWalletSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/AssociateWallet" => {
                    #[allow(non_camel_case_types)]
                    struct AssociateWalletSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::AssociateWalletRequest>
                    for AssociateWalletSvc<T> {
                        type Response = super::AssociateWalletResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssociateWalletRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).associate_wallet(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssociateWalletSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/GetAppSettings" => {
                    #[allow(non_camel_case_types)]
                    struct GetAppSettingsSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::GetAppSettingsRequest>
                    for GetAppSettingsSvc<T> {
                        type Response = super::GetAppSettingsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAppSettingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_app_settings(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAppSettingsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/RegisterUser" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterUserSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::RegisterUserRequest>
                    for RegisterUserSvc<T> {
                        type Response = super::RegisterUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).register_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/ConfirmUser" => {
                    #[allow(non_camel_case_types)]
                    struct ConfirmUserSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::ConfirmUserRequest>
                    for ConfirmUserSvc<T> {
                        type Response = super::ConfirmUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfirmUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).confirm_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConfirmUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/FindByEmail" => {
                    #[allow(non_camel_case_types)]
                    struct FindByEmailSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::FindByEmailRequest>
                    for FindByEmailSvc<T> {
                        type Response = super::FindByEmailResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindByEmailRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).find_by_email(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FindByEmailSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/FindByID" => {
                    #[allow(non_camel_case_types)]
                    struct FindByIDSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::FindByIdRequest>
                    for FindByIDSvc<T> {
                        type Response = super::FindByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).find_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FindByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/GetMe" => {
                    #[allow(non_camel_case_types)]
                    struct GetMeSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::GetMeRequest> for GetMeSvc<T> {
                        type Response = super::GetMeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_me(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/auth_api.v1.UserManagement/GetUserByDID" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserByDIDSvc<T: UserManagement>(pub Arc<T>);
                    impl<
                        T: UserManagement,
                    > tonic::server::UnaryService<super::GetUserByDidRequest>
                    for GetUserByDIDSvc<T> {
                        type Response = super::GetUserByDidResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserByDidRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_by_did(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserByDIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: UserManagement> Clone for UserManagementServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: UserManagement> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserManagement> tonic::server::NamedService for UserManagementServer<T> {
        const NAME: &'static str = "auth_api.v1.UserManagement";
    }
}
