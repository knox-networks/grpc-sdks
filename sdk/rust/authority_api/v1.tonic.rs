// @generated
/// Generated client implementations.
pub mod authority_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Entity that is the approving monetary authority- Ex Central bank or commercial bank. It can create Issuers with limits, approve additional issuance limits, authorize issuing of digital banknotes, and replace digital banknotes in circulation.
*/
    #[derive(Debug, Clone)]
    pub struct AuthorityClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthorityClient<tonic::transport::Channel> {
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
    impl<T> AuthorityClient<T>
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
        ) -> AuthorityClient<InterceptedService<T, F>>
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
            AuthorityClient::new(InterceptedService::new(inner, interceptor))
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
        /** Sets the issuance limit for an Issuer to a specified value and returns the new limit.
*/
        pub async fn set_issuer_limit(
            &mut self,
            request: impl tonic::IntoRequest<super::SetIssuerLimitRequest>,
        ) -> Result<tonic::Response<super::SetIssuerLimitResponse>, tonic::Status> {
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
                "/authority_api.v1.Authority/SetIssuerLimit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Gets the the issuance limits for all the currencies for which the Issuer is allowed to issue digital banknotes.
*/
        pub async fn get_issuer_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIssuerLimitsRequest>,
        ) -> Result<tonic::Response<super::GetIssuerLimitsResponse>, tonic::Status> {
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
                "/authority_api.v1.Authority/GetIssuerLimits",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Accepts digital banknotes in a stream, authorizes them and returns authorized digital banknotes in a stream.
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
                "/authority_api.v1.Authority/Authorize",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /** Redeems a digital banknote in exchange for an increase in issuance limit for the same currency as the redeemed digital banknote.
*/
        pub async fn redeem(
            &mut self,
            request: impl tonic::IntoRequest<super::RedeemRequest>,
        ) -> Result<tonic::Response<super::RedeemResponse>, tonic::Status> {
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
                "/authority_api.v1.Authority/Redeem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod authority_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with AuthorityServer.
    #[async_trait]
    pub trait Authority: Send + Sync + 'static {
        /** Sets the issuance limit for an Issuer to a specified value and returns the new limit.
*/
        async fn set_issuer_limit(
            &self,
            request: tonic::Request<super::SetIssuerLimitRequest>,
        ) -> Result<tonic::Response<super::SetIssuerLimitResponse>, tonic::Status>;
        /** Gets the the issuance limits for all the currencies for which the Issuer is allowed to issue digital banknotes.
*/
        async fn get_issuer_limits(
            &self,
            request: tonic::Request<super::GetIssuerLimitsRequest>,
        ) -> Result<tonic::Response<super::GetIssuerLimitsResponse>, tonic::Status>;
        ///Server streaming response type for the Authorize method.
        type AuthorizeStream: futures_core::Stream<
                Item = Result<super::AuthorizeResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /** Accepts digital banknotes in a stream, authorizes them and returns authorized digital banknotes in a stream.
*/
        async fn authorize(
            &self,
            request: tonic::Request<tonic::Streaming<super::AuthorizeRequest>>,
        ) -> Result<tonic::Response<Self::AuthorizeStream>, tonic::Status>;
        /** Redeems a digital banknote in exchange for an increase in issuance limit for the same currency as the redeemed digital banknote.
*/
        async fn redeem(
            &self,
            request: tonic::Request<super::RedeemRequest>,
        ) -> Result<tonic::Response<super::RedeemResponse>, tonic::Status>;
    }
    /** Entity that is the approving monetary authority- Ex Central bank or commercial bank. It can create Issuers with limits, approve additional issuance limits, authorize issuing of digital banknotes, and replace digital banknotes in circulation.
*/
    #[derive(Debug)]
    pub struct AuthorityServer<T: Authority> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Authority> AuthorityServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuthorityServer<T>
    where
        T: Authority,
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
                "/authority_api.v1.Authority/SetIssuerLimit" => {
                    #[allow(non_camel_case_types)]
                    struct SetIssuerLimitSvc<T: Authority>(pub Arc<T>);
                    impl<
                        T: Authority,
                    > tonic::server::UnaryService<super::SetIssuerLimitRequest>
                    for SetIssuerLimitSvc<T> {
                        type Response = super::SetIssuerLimitResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetIssuerLimitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_issuer_limit(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetIssuerLimitSvc(inner);
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
                "/authority_api.v1.Authority/GetIssuerLimits" => {
                    #[allow(non_camel_case_types)]
                    struct GetIssuerLimitsSvc<T: Authority>(pub Arc<T>);
                    impl<
                        T: Authority,
                    > tonic::server::UnaryService<super::GetIssuerLimitsRequest>
                    for GetIssuerLimitsSvc<T> {
                        type Response = super::GetIssuerLimitsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetIssuerLimitsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_issuer_limits(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetIssuerLimitsSvc(inner);
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
                "/authority_api.v1.Authority/Authorize" => {
                    #[allow(non_camel_case_types)]
                    struct AuthorizeSvc<T: Authority>(pub Arc<T>);
                    impl<
                        T: Authority,
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
                "/authority_api.v1.Authority/Redeem" => {
                    #[allow(non_camel_case_types)]
                    struct RedeemSvc<T: Authority>(pub Arc<T>);
                    impl<T: Authority> tonic::server::UnaryService<super::RedeemRequest>
                    for RedeemSvc<T> {
                        type Response = super::RedeemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RedeemRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).redeem(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RedeemSvc(inner);
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
    impl<T: Authority> Clone for AuthorityServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Authority> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Authority> tonic::server::NamedService for AuthorityServer<T> {
        const NAME: &'static str = "authority_api.v1.Authority";
    }
}
