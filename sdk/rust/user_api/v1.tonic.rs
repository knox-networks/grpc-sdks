// @generated
/// Generated client implementations.
pub mod user_api_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** This service integrates with existing identity systems (Ex. Bank accounts) to bridge traditional account identifiers to DID/public key identifiers used in the Knox ecosytem. Leverages existing KYC/AML data to enable financial transaction compliance.  Supports OIDC, SAML and other traditional AuthN/Z standards.
*/
    #[derive(Debug, Clone)]
    pub struct UserApiServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserApiServiceClient<tonic::transport::Channel> {
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
    impl<T> UserApiServiceClient<T>
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
        ) -> UserApiServiceClient<InterceptedService<T, F>>
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
            UserApiServiceClient::new(InterceptedService::new(inner, interceptor))
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
                "/user_api.v1.UserApiService/AuthnWithPassword",
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
                "/user_api.v1.UserApiService/RefreshAccessToken",
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
                "/user_api.v1.UserApiService/AuthnWithProvider",
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
                "/user_api.v1.UserApiService/HandleOIDCCallback",
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
                "/user_api.v1.UserApiService/HandleSAMLCallback",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/SAMLSPMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/CreateAuthnBrowserWithWalletChallenge",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/AuthnBrowserWithWallet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/CreateAuthnWalletChallenge",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/AuthnWallet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/CreateRegisterWalletChallenge",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/RegisterWallet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/AssociateWallet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn dissociate_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::DissociateWalletRequest>,
        ) -> Result<tonic::Response<super::DissociateWalletResponse>, tonic::Status> {
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
                "/user_api.v1.UserApiService/DissociateWallet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/GetAppSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/RegisterUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/ConfirmUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/FindByEmail",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/FindByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/GetMe",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
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
                "/user_api.v1.UserApiService/GetUserByDID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_api_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UserApiServiceServer.
    #[async_trait]
    pub trait UserApiService: Send + Sync + 'static {
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
        async fn samlsp_metadata(
            &self,
            request: tonic::Request<super::SamlspMetadataRequest>,
        ) -> Result<tonic::Response<super::SamlspMetadataResponse>, tonic::Status>;
        /// Server streaming response type for the CreateAuthnBrowserWithWalletChallenge method.
        type CreateAuthnBrowserWithWalletChallengeStream: futures_core::Stream<
                Item = Result<
                    super::CreateAuthnBrowserWithWalletChallengeResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn create_authn_browser_with_wallet_challenge(
            &self,
            request: tonic::Request<super::CreateAuthnBrowserWithWalletChallengeRequest>,
        ) -> Result<
            tonic::Response<Self::CreateAuthnBrowserWithWalletChallengeStream>,
            tonic::Status,
        >;
        async fn authn_browser_with_wallet(
            &self,
            request: tonic::Request<super::AuthnBrowserWithWalletRequest>,
        ) -> Result<
            tonic::Response<super::AuthnBrowserWithWalletResponse>,
            tonic::Status,
        >;
        async fn create_authn_wallet_challenge(
            &self,
            request: tonic::Request<super::CreateAuthnWalletChallengeRequest>,
        ) -> Result<
            tonic::Response<super::CreateAuthnWalletChallengeResponse>,
            tonic::Status,
        >;
        async fn authn_wallet(
            &self,
            request: tonic::Request<super::AuthnWalletRequest>,
        ) -> Result<tonic::Response<super::AuthnWalletResponse>, tonic::Status>;
        /// Server streaming response type for the CreateRegisterWalletChallenge method.
        type CreateRegisterWalletChallengeStream: futures_core::Stream<
                Item = Result<
                    super::CreateRegisterWalletChallengeResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn create_register_wallet_challenge(
            &self,
            request: tonic::Request<super::CreateRegisterWalletChallengeRequest>,
        ) -> Result<
            tonic::Response<Self::CreateRegisterWalletChallengeStream>,
            tonic::Status,
        >;
        async fn register_wallet(
            &self,
            request: tonic::Request<super::RegisterWalletRequest>,
        ) -> Result<tonic::Response<super::RegisterWalletResponse>, tonic::Status>;
        async fn associate_wallet(
            &self,
            request: tonic::Request<super::AssociateWalletRequest>,
        ) -> Result<tonic::Response<super::AssociateWalletResponse>, tonic::Status>;
        async fn dissociate_wallet(
            &self,
            request: tonic::Request<super::DissociateWalletRequest>,
        ) -> Result<tonic::Response<super::DissociateWalletResponse>, tonic::Status>;
        async fn get_app_settings(
            &self,
            request: tonic::Request<super::GetAppSettingsRequest>,
        ) -> Result<tonic::Response<super::GetAppSettingsResponse>, tonic::Status>;
        async fn register_user(
            &self,
            request: tonic::Request<super::RegisterUserRequest>,
        ) -> Result<tonic::Response<super::RegisterUserResponse>, tonic::Status>;
        async fn confirm_user(
            &self,
            request: tonic::Request<super::ConfirmUserRequest>,
        ) -> Result<tonic::Response<super::ConfirmUserResponse>, tonic::Status>;
        async fn find_by_email(
            &self,
            request: tonic::Request<super::FindByEmailRequest>,
        ) -> Result<tonic::Response<super::FindByEmailResponse>, tonic::Status>;
        async fn find_by_id(
            &self,
            request: tonic::Request<super::FindByIdRequest>,
        ) -> Result<tonic::Response<super::FindByIdResponse>, tonic::Status>;
        async fn get_me(
            &self,
            request: tonic::Request<super::GetMeRequest>,
        ) -> Result<tonic::Response<super::GetMeResponse>, tonic::Status>;
        async fn get_user_by_did(
            &self,
            request: tonic::Request<super::GetUserByDidRequest>,
        ) -> Result<tonic::Response<super::GetUserByDidResponse>, tonic::Status>;
    }
    /** This service integrates with existing identity systems (Ex. Bank accounts) to bridge traditional account identifiers to DID/public key identifiers used in the Knox ecosytem. Leverages existing KYC/AML data to enable financial transaction compliance.  Supports OIDC, SAML and other traditional AuthN/Z standards.
*/
    #[derive(Debug)]
    pub struct UserApiServiceServer<T: UserApiService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UserApiService> UserApiServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UserApiServiceServer<T>
    where
        T: UserApiService,
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
                "/user_api.v1.UserApiService/AuthnWithPassword" => {
                    #[allow(non_camel_case_types)]
                    struct AuthnWithPasswordSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/RefreshAccessToken" => {
                    #[allow(non_camel_case_types)]
                    struct RefreshAccessTokenSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/AuthnWithProvider" => {
                    #[allow(non_camel_case_types)]
                    struct AuthnWithProviderSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/HandleOIDCCallback" => {
                    #[allow(non_camel_case_types)]
                    struct HandleOIDCCallbackSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/HandleSAMLCallback" => {
                    #[allow(non_camel_case_types)]
                    struct HandleSAMLCallbackSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/SAMLSPMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct SAMLSPMetadataSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/CreateAuthnBrowserWithWalletChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAuthnBrowserWithWalletChallengeSvc<T: UserApiService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/AuthnBrowserWithWallet" => {
                    #[allow(non_camel_case_types)]
                    struct AuthnBrowserWithWalletSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/CreateAuthnWalletChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAuthnWalletChallengeSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/AuthnWallet" => {
                    #[allow(non_camel_case_types)]
                    struct AuthnWalletSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/CreateRegisterWalletChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRegisterWalletChallengeSvc<T: UserApiService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/RegisterWallet" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterWalletSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/AssociateWallet" => {
                    #[allow(non_camel_case_types)]
                    struct AssociateWalletSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/DissociateWallet" => {
                    #[allow(non_camel_case_types)]
                    struct DissociateWalletSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
                    > tonic::server::UnaryService<super::DissociateWalletRequest>
                    for DissociateWalletSvc<T> {
                        type Response = super::DissociateWalletResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DissociateWalletRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).dissociate_wallet(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DissociateWalletSvc(inner);
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
                "/user_api.v1.UserApiService/GetAppSettings" => {
                    #[allow(non_camel_case_types)]
                    struct GetAppSettingsSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/RegisterUser" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterUserSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/ConfirmUser" => {
                    #[allow(non_camel_case_types)]
                    struct ConfirmUserSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/FindByEmail" => {
                    #[allow(non_camel_case_types)]
                    struct FindByEmailSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/FindByID" => {
                    #[allow(non_camel_case_types)]
                    struct FindByIDSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/GetMe" => {
                    #[allow(non_camel_case_types)]
                    struct GetMeSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
                "/user_api.v1.UserApiService/GetUserByDID" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserByDIDSvc<T: UserApiService>(pub Arc<T>);
                    impl<
                        T: UserApiService,
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
    impl<T: UserApiService> Clone for UserApiServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: UserApiService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserApiService> tonic::server::NamedService for UserApiServiceServer<T> {
        const NAME: &'static str = "user_api.v1.UserApiService";
    }
}
/// Generated client implementations.
pub mod api_key_manager_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct ApiKeyManagerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ApiKeyManagerServiceClient<tonic::transport::Channel> {
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
    impl<T> ApiKeyManagerServiceClient<T>
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
        ) -> ApiKeyManagerServiceClient<InterceptedService<T, F>>
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
            ApiKeyManagerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        ///
        pub async fn create_api_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiKeyRequest>,
        ) -> Result<tonic::Response<super::CreateApiKeyResponse>, tonic::Status> {
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
                "/user_api.v1.ApiKeyManagerService/CreateApiKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn update_api_key_status(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApiKeyStatusRequest>,
        ) -> Result<tonic::Response<super::UpdateApiKeyStatusResponse>, tonic::Status> {
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
                "/user_api.v1.ApiKeyManagerService/UpdateApiKeyStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn get_api_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiKeysRequest>,
        ) -> Result<tonic::Response<super::GetApiKeysResponse>, tonic::Status> {
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
                "/user_api.v1.ApiKeyManagerService/GetApiKeys",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        pub async fn check_api_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckApiKeyRequest>,
        ) -> Result<tonic::Response<super::CheckApiKeyResponse>, tonic::Status> {
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
                "/user_api.v1.ApiKeyManagerService/CheckApiKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod api_key_manager_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ApiKeyManagerServiceServer.
    #[async_trait]
    pub trait ApiKeyManagerService: Send + Sync + 'static {
        ///
        async fn create_api_key(
            &self,
            request: tonic::Request<super::CreateApiKeyRequest>,
        ) -> Result<tonic::Response<super::CreateApiKeyResponse>, tonic::Status>;
        ///
        async fn update_api_key_status(
            &self,
            request: tonic::Request<super::UpdateApiKeyStatusRequest>,
        ) -> Result<tonic::Response<super::UpdateApiKeyStatusResponse>, tonic::Status>;
        ///
        async fn get_api_keys(
            &self,
            request: tonic::Request<super::GetApiKeysRequest>,
        ) -> Result<tonic::Response<super::GetApiKeysResponse>, tonic::Status>;
        ///
        async fn check_api_key(
            &self,
            request: tonic::Request<super::CheckApiKeyRequest>,
        ) -> Result<tonic::Response<super::CheckApiKeyResponse>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct ApiKeyManagerServiceServer<T: ApiKeyManagerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ApiKeyManagerService> ApiKeyManagerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ApiKeyManagerServiceServer<T>
    where
        T: ApiKeyManagerService,
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
                "/user_api.v1.ApiKeyManagerService/CreateApiKey" => {
                    #[allow(non_camel_case_types)]
                    struct CreateApiKeySvc<T: ApiKeyManagerService>(pub Arc<T>);
                    impl<
                        T: ApiKeyManagerService,
                    > tonic::server::UnaryService<super::CreateApiKeyRequest>
                    for CreateApiKeySvc<T> {
                        type Response = super::CreateApiKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateApiKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_api_key(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateApiKeySvc(inner);
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
                "/user_api.v1.ApiKeyManagerService/UpdateApiKeyStatus" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateApiKeyStatusSvc<T: ApiKeyManagerService>(pub Arc<T>);
                    impl<
                        T: ApiKeyManagerService,
                    > tonic::server::UnaryService<super::UpdateApiKeyStatusRequest>
                    for UpdateApiKeyStatusSvc<T> {
                        type Response = super::UpdateApiKeyStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateApiKeyStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_api_key_status(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateApiKeyStatusSvc(inner);
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
                "/user_api.v1.ApiKeyManagerService/GetApiKeys" => {
                    #[allow(non_camel_case_types)]
                    struct GetApiKeysSvc<T: ApiKeyManagerService>(pub Arc<T>);
                    impl<
                        T: ApiKeyManagerService,
                    > tonic::server::UnaryService<super::GetApiKeysRequest>
                    for GetApiKeysSvc<T> {
                        type Response = super::GetApiKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetApiKeysRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_api_keys(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetApiKeysSvc(inner);
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
                "/user_api.v1.ApiKeyManagerService/CheckApiKey" => {
                    #[allow(non_camel_case_types)]
                    struct CheckApiKeySvc<T: ApiKeyManagerService>(pub Arc<T>);
                    impl<
                        T: ApiKeyManagerService,
                    > tonic::server::UnaryService<super::CheckApiKeyRequest>
                    for CheckApiKeySvc<T> {
                        type Response = super::CheckApiKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckApiKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).check_api_key(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CheckApiKeySvc(inner);
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
    impl<T: ApiKeyManagerService> Clone for ApiKeyManagerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ApiKeyManagerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ApiKeyManagerService> tonic::server::NamedService
    for ApiKeyManagerServiceServer<T> {
        const NAME: &'static str = "user_api.v1.ApiKeyManagerService";
    }
}
