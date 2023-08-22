// @generated
/// Generated client implementations.
pub mod issuer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Financial intermediary with a reserve account with the Authority. A role typically played by commercial banks and
 other depository institutions, it can issue, transfer, redeem digital banknotes and request an increase in issuance
 limits.  It holds digital banknotes in its own vault.
*/
    #[derive(Debug, Clone)]
    pub struct IssuerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IssuerServiceClient<tonic::transport::Channel> {
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
    impl<T> IssuerServiceClient<T>
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
        ) -> IssuerServiceClient<InterceptedService<T, F>>
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
            IssuerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /** Checks that the Verifier is authenticated, then stores and returns the challenge nonce.
*/
        pub async fn authenticate(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateRequest>,
        ) -> Result<tonic::Response<super::AuthenticateResponse>, tonic::Status> {
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
                "/issuer_api.v1.IssuerService/Authenticate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Issues digital banknotes against the corresponding issuance limit set by an Authority.  The Issuer must collect the
 Authenticate challenge signature, the Amount with a total amount, decimal place precision, and a currency code,
 along with the verifier of the Authority. Issued promissories are sent to the given recipient.
*/
        pub async fn issue(
            &mut self,
            request: impl tonic::IntoRequest<super::IssueRequest>,
        ) -> Result<tonic::Response<super::IssueResponse>, tonic::Status> {
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
                "/issuer_api.v1.IssuerService/Issue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Gets the roles configured for the Issuer- ex Currency limit for issuance.
*/
        pub async fn get_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRolesRequest>,
        ) -> Result<tonic::Response<super::GetRolesResponse>, tonic::Status> {
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
                "/issuer_api.v1.IssuerService/GetRoles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Sets the roles configured for the Issuer- Ex List of the currency limits.
*/
        pub async fn set_role(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRoleRequest>,
        ) -> Result<tonic::Response<super::SetRoleResponse>, tonic::Status> {
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
                "/issuer_api.v1.IssuerService/SetRole",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Sets the valid asset denominations that this Issuer will issuer.
*/
        pub async fn set_asset_denominations(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAssetDenominationsRequest>,
        ) -> Result<
            tonic::Response<super::SetAssetDenominationsResponse>,
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
                "/issuer_api.v1.IssuerService/SetAssetDenominations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Sets the authority for Issuer assets.
*/
        pub async fn set_asset_authorities(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAssetAuthoritiesRequest>,
        ) -> Result<tonic::Response<super::SetAssetAuthoritiesResponse>, tonic::Status> {
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
                "/issuer_api.v1.IssuerService/SetAssetAuthorities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod issuer_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with IssuerServiceServer.
    #[async_trait]
    pub trait IssuerService: Send + Sync + 'static {
        /** Checks that the Verifier is authenticated, then stores and returns the challenge nonce.
*/
        async fn authenticate(
            &self,
            request: tonic::Request<super::AuthenticateRequest>,
        ) -> Result<tonic::Response<super::AuthenticateResponse>, tonic::Status>;
        /** Issues digital banknotes against the corresponding issuance limit set by an Authority.  The Issuer must collect the
 Authenticate challenge signature, the Amount with a total amount, decimal place precision, and a currency code,
 along with the verifier of the Authority. Issued promissories are sent to the given recipient.
*/
        async fn issue(
            &self,
            request: tonic::Request<super::IssueRequest>,
        ) -> Result<tonic::Response<super::IssueResponse>, tonic::Status>;
        /** Gets the roles configured for the Issuer- ex Currency limit for issuance.
*/
        async fn get_roles(
            &self,
            request: tonic::Request<super::GetRolesRequest>,
        ) -> Result<tonic::Response<super::GetRolesResponse>, tonic::Status>;
        /** Sets the roles configured for the Issuer- Ex List of the currency limits.
*/
        async fn set_role(
            &self,
            request: tonic::Request<super::SetRoleRequest>,
        ) -> Result<tonic::Response<super::SetRoleResponse>, tonic::Status>;
        /** Sets the valid asset denominations that this Issuer will issuer.
*/
        async fn set_asset_denominations(
            &self,
            request: tonic::Request<super::SetAssetDenominationsRequest>,
        ) -> Result<
            tonic::Response<super::SetAssetDenominationsResponse>,
            tonic::Status,
        >;
        /** Sets the authority for Issuer assets.
*/
        async fn set_asset_authorities(
            &self,
            request: tonic::Request<super::SetAssetAuthoritiesRequest>,
        ) -> Result<tonic::Response<super::SetAssetAuthoritiesResponse>, tonic::Status>;
    }
    /** Financial intermediary with a reserve account with the Authority. A role typically played by commercial banks and
 other depository institutions, it can issue, transfer, redeem digital banknotes and request an increase in issuance
 limits.  It holds digital banknotes in its own vault.
*/
    #[derive(Debug)]
    pub struct IssuerServiceServer<T: IssuerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: IssuerService> IssuerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for IssuerServiceServer<T>
    where
        T: IssuerService,
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
                "/issuer_api.v1.IssuerService/Authenticate" => {
                    #[allow(non_camel_case_types)]
                    struct AuthenticateSvc<T: IssuerService>(pub Arc<T>);
                    impl<
                        T: IssuerService,
                    > tonic::server::UnaryService<super::AuthenticateRequest>
                    for AuthenticateSvc<T> {
                        type Response = super::AuthenticateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthenticateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).authenticate(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthenticateSvc(inner);
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
                "/issuer_api.v1.IssuerService/Issue" => {
                    #[allow(non_camel_case_types)]
                    struct IssueSvc<T: IssuerService>(pub Arc<T>);
                    impl<
                        T: IssuerService,
                    > tonic::server::UnaryService<super::IssueRequest> for IssueSvc<T> {
                        type Response = super::IssueResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IssueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).issue(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IssueSvc(inner);
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
                "/issuer_api.v1.IssuerService/GetRoles" => {
                    #[allow(non_camel_case_types)]
                    struct GetRolesSvc<T: IssuerService>(pub Arc<T>);
                    impl<
                        T: IssuerService,
                    > tonic::server::UnaryService<super::GetRolesRequest>
                    for GetRolesSvc<T> {
                        type Response = super::GetRolesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRolesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_roles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRolesSvc(inner);
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
                "/issuer_api.v1.IssuerService/SetRole" => {
                    #[allow(non_camel_case_types)]
                    struct SetRoleSvc<T: IssuerService>(pub Arc<T>);
                    impl<
                        T: IssuerService,
                    > tonic::server::UnaryService<super::SetRoleRequest>
                    for SetRoleSvc<T> {
                        type Response = super::SetRoleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetRoleSvc(inner);
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
                "/issuer_api.v1.IssuerService/SetAssetDenominations" => {
                    #[allow(non_camel_case_types)]
                    struct SetAssetDenominationsSvc<T: IssuerService>(pub Arc<T>);
                    impl<
                        T: IssuerService,
                    > tonic::server::UnaryService<super::SetAssetDenominationsRequest>
                    for SetAssetDenominationsSvc<T> {
                        type Response = super::SetAssetDenominationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetAssetDenominationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_asset_denominations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetAssetDenominationsSvc(inner);
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
                "/issuer_api.v1.IssuerService/SetAssetAuthorities" => {
                    #[allow(non_camel_case_types)]
                    struct SetAssetAuthoritiesSvc<T: IssuerService>(pub Arc<T>);
                    impl<
                        T: IssuerService,
                    > tonic::server::UnaryService<super::SetAssetAuthoritiesRequest>
                    for SetAssetAuthoritiesSvc<T> {
                        type Response = super::SetAssetAuthoritiesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetAssetAuthoritiesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_asset_authorities(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetAssetAuthoritiesSvc(inner);
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
    impl<T: IssuerService> Clone for IssuerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: IssuerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: IssuerService> tonic::server::NamedService for IssuerServiceServer<T> {
        const NAME: &'static str = "issuer_api.v1.IssuerService";
    }
}
