// @generated
/// Generated client implementations.
pub mod issuer_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct IssuerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IssuerClient<tonic::transport::Channel> {
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
    impl<T> IssuerClient<T>
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
        ) -> IssuerClient<InterceptedService<T, F>>
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
            IssuerClient::new(InterceptedService::new(inner, interceptor))
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
                "/issuer_api.v1.Issuer/Authenticate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Sends a stream of digital banknotes to be authorized. The digital banknote is checked to see if it is transferred to the new owner, but not yet authorized. IT is then checked for double spends, signed and saved to check for future double spends. The newly authorized promissory files are returned as a stream.
*/
        pub async fn authorize(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::AuthorizeRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::AuthorizeResponse>>,
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
                "/issuer_api.v1.Issuer/Authorize",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /** Issues digital banknotes against the corresponding issuance limit set by an Authority.  The Issuer must collect the Authenticate challenge signature, the Amount with a total amount, decimal place precision, and a currency code, along with the verifier of the Authority.
*/
        pub async fn issue(
            &mut self,
            request: impl tonic::IntoRequest<super::IssueRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::IssueResponse>>,
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
                "/issuer_api.v1.Issuer/Issue",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
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
                "/issuer_api.v1.Issuer/GetRoles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Sets the roles configured for the Issuer- Ex list of the currency limits.
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
                "/issuer_api.v1.Issuer/SetRole",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod issuer_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with IssuerServer.
    #[async_trait]
    pub trait Issuer: Send + Sync + 'static {
        /** Checks that the Verifier is authenticated, then stores and returns the challenge nonce.
*/
        async fn authenticate(
            &self,
            request: tonic::Request<super::AuthenticateRequest>,
        ) -> Result<tonic::Response<super::AuthenticateResponse>, tonic::Status>;
        ///Server streaming response type for the Authorize method.
        type AuthorizeStream: futures_core::Stream<
                Item = Result<super::AuthorizeResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /** Sends a stream of digital banknotes to be authorized. The digital banknote is checked to see if it is transferred to the new owner, but not yet authorized. IT is then checked for double spends, signed and saved to check for future double spends. The newly authorized promissory files are returned as a stream.
*/
        async fn authorize(
            &self,
            request: tonic::Request<tonic::Streaming<super::AuthorizeRequest>>,
        ) -> Result<tonic::Response<Self::AuthorizeStream>, tonic::Status>;
        ///Server streaming response type for the Issue method.
        type IssueStream: futures_core::Stream<
                Item = Result<super::IssueResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /** Issues digital banknotes against the corresponding issuance limit set by an Authority.  The Issuer must collect the Authenticate challenge signature, the Amount with a total amount, decimal place precision, and a currency code, along with the verifier of the Authority.
*/
        async fn issue(
            &self,
            request: tonic::Request<super::IssueRequest>,
        ) -> Result<tonic::Response<Self::IssueStream>, tonic::Status>;
        /** Gets the roles configured for the Issuer- ex Currency limit for issuance.
*/
        async fn get_roles(
            &self,
            request: tonic::Request<super::GetRolesRequest>,
        ) -> Result<tonic::Response<super::GetRolesResponse>, tonic::Status>;
        /** Sets the roles configured for the Issuer- Ex list of the currency limits.
*/
        async fn set_role(
            &self,
            request: tonic::Request<super::SetRoleRequest>,
        ) -> Result<tonic::Response<super::SetRoleResponse>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct IssuerServer<T: Issuer> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Issuer> IssuerServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for IssuerServer<T>
    where
        T: Issuer,
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
                "/issuer_api.v1.Issuer/Authenticate" => {
                    #[allow(non_camel_case_types)]
                    struct AuthenticateSvc<T: Issuer>(pub Arc<T>);
                    impl<
                        T: Issuer,
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
                "/issuer_api.v1.Issuer/Authorize" => {
                    #[allow(non_camel_case_types)]
                    struct AuthorizeSvc<T: Issuer>(pub Arc<T>);
                    impl<
                        T: Issuer,
                    > tonic::server::StreamingService<super::AuthorizeRequest>
                    for AuthorizeSvc<T> {
                        type Response = super::AuthorizeResponse;
                        type ResponseStream = T::AuthorizeStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::AuthorizeRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).authorize(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthorizeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/issuer_api.v1.Issuer/Issue" => {
                    #[allow(non_camel_case_types)]
                    struct IssueSvc<T: Issuer>(pub Arc<T>);
                    impl<
                        T: Issuer,
                    > tonic::server::ServerStreamingService<super::IssueRequest>
                    for IssueSvc<T> {
                        type Response = super::IssueResponse;
                        type ResponseStream = T::IssueStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/issuer_api.v1.Issuer/GetRoles" => {
                    #[allow(non_camel_case_types)]
                    struct GetRolesSvc<T: Issuer>(pub Arc<T>);
                    impl<T: Issuer> tonic::server::UnaryService<super::GetRolesRequest>
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
                "/issuer_api.v1.Issuer/SetRole" => {
                    #[allow(non_camel_case_types)]
                    struct SetRoleSvc<T: Issuer>(pub Arc<T>);
                    impl<T: Issuer> tonic::server::UnaryService<super::SetRoleRequest>
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
    impl<T: Issuer> Clone for IssuerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Issuer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Issuer> tonic::server::NamedService for IssuerServer<T> {
        const NAME: &'static str = "issuer_api.v1.Issuer";
    }
}
