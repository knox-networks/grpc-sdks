// @generated
/// Generated client implementations.
pub mod authority_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Entity that is the approving monetary authority- Ex Central bank or commercial bank. It can create Issuers with
 limits, approve additional issuance limits, authorize issuing of digital banknotes, and replace digital banknotes in
 circulation.
*/
    #[derive(Debug, Clone)]
    pub struct AuthorityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthorityServiceClient<tonic::transport::Channel> {
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
    impl<T> AuthorityServiceClient<T>
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
        ) -> AuthorityServiceClient<InterceptedService<T, F>>
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
            AuthorityServiceClient::new(InterceptedService::new(inner, interceptor))
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
                "/authority_api.v1.AuthorityService/SetIssuerLimit",
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
                "/authority_api.v1.AuthorityService/GetIssuerLimits",
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
                "/authority_api.v1.AuthorityService/Authorize",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /** Redeems a digital banknote in exchange for an increase in issuance limit for the same currency as the redeemed
 digital banknote.
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
                "/authority_api.v1.AuthorityService/Redeem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Gets the verifier of the Emissary for this Authority.
*/
        pub async fn get_network_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNetworkInfoRequest>,
        ) -> Result<tonic::Response<super::GetNetworkInfoResponse>, tonic::Status> {
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
                "/authority_api.v1.AuthorityService/GetNetworkInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Gets the signature of the Notary for this Authority/Notary.
*/
        pub async fn get_notary(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotaryRequest>,
        ) -> Result<tonic::Response<super::GetNotaryResponse>, tonic::Status> {
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
                "/authority_api.v1.AuthorityService/GetNotary",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** List the Governed Assets for this Authority.
*/
        pub async fn list_governed_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGovernedAssetsRequest>,
        ) -> Result<tonic::Response<super::ListGovernedAssetsResponse>, tonic::Status> {
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
                "/authority_api.v1.AuthorityService/ListGovernedAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Set the Governed Assets for this Authority.
*/
        pub async fn set_governed_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::SetGovernedAssetsRequest>,
        ) -> Result<tonic::Response<super::SetGovernedAssetsResponse>, tonic::Status> {
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
                "/authority_api.v1.AuthorityService/SetGovernedAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod authority_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AuthorityServiceServer.
    #[async_trait]
    pub trait AuthorityService: Send + Sync + 'static {
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
        /// Server streaming response type for the Authorize method.
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
        /** Redeems a digital banknote in exchange for an increase in issuance limit for the same currency as the redeemed
 digital banknote.
*/
        async fn redeem(
            &self,
            request: tonic::Request<super::RedeemRequest>,
        ) -> Result<tonic::Response<super::RedeemResponse>, tonic::Status>;
        /** Gets the verifier of the Emissary for this Authority.
*/
        async fn get_network_info(
            &self,
            request: tonic::Request<super::GetNetworkInfoRequest>,
        ) -> Result<tonic::Response<super::GetNetworkInfoResponse>, tonic::Status>;
        /** Gets the signature of the Notary for this Authority/Notary.
*/
        async fn get_notary(
            &self,
            request: tonic::Request<super::GetNotaryRequest>,
        ) -> Result<tonic::Response<super::GetNotaryResponse>, tonic::Status>;
        /** List the Governed Assets for this Authority.
*/
        async fn list_governed_assets(
            &self,
            request: tonic::Request<super::ListGovernedAssetsRequest>,
        ) -> Result<tonic::Response<super::ListGovernedAssetsResponse>, tonic::Status>;
        /** Set the Governed Assets for this Authority.
*/
        async fn set_governed_assets(
            &self,
            request: tonic::Request<super::SetGovernedAssetsRequest>,
        ) -> Result<tonic::Response<super::SetGovernedAssetsResponse>, tonic::Status>;
    }
    /** Entity that is the approving monetary authority- Ex Central bank or commercial bank. It can create Issuers with
 limits, approve additional issuance limits, authorize issuing of digital banknotes, and replace digital banknotes in
 circulation.
*/
    #[derive(Debug)]
    pub struct AuthorityServiceServer<T: AuthorityService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AuthorityService> AuthorityServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuthorityServiceServer<T>
    where
        T: AuthorityService,
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
                "/authority_api.v1.AuthorityService/SetIssuerLimit" => {
                    #[allow(non_camel_case_types)]
                    struct SetIssuerLimitSvc<T: AuthorityService>(pub Arc<T>);
                    impl<
                        T: AuthorityService,
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
                "/authority_api.v1.AuthorityService/GetIssuerLimits" => {
                    #[allow(non_camel_case_types)]
                    struct GetIssuerLimitsSvc<T: AuthorityService>(pub Arc<T>);
                    impl<
                        T: AuthorityService,
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
                "/authority_api.v1.AuthorityService/Authorize" => {
                    #[allow(non_camel_case_types)]
                    struct AuthorizeSvc<T: AuthorityService>(pub Arc<T>);
                    impl<
                        T: AuthorityService,
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
                "/authority_api.v1.AuthorityService/Redeem" => {
                    #[allow(non_camel_case_types)]
                    struct RedeemSvc<T: AuthorityService>(pub Arc<T>);
                    impl<
                        T: AuthorityService,
                    > tonic::server::UnaryService<super::RedeemRequest>
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
                "/authority_api.v1.AuthorityService/GetNetworkInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetNetworkInfoSvc<T: AuthorityService>(pub Arc<T>);
                    impl<
                        T: AuthorityService,
                    > tonic::server::UnaryService<super::GetNetworkInfoRequest>
                    for GetNetworkInfoSvc<T> {
                        type Response = super::GetNetworkInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNetworkInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_network_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNetworkInfoSvc(inner);
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
                "/authority_api.v1.AuthorityService/GetNotary" => {
                    #[allow(non_camel_case_types)]
                    struct GetNotarySvc<T: AuthorityService>(pub Arc<T>);
                    impl<
                        T: AuthorityService,
                    > tonic::server::UnaryService<super::GetNotaryRequest>
                    for GetNotarySvc<T> {
                        type Response = super::GetNotaryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNotaryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_notary(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNotarySvc(inner);
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
                "/authority_api.v1.AuthorityService/ListGovernedAssets" => {
                    #[allow(non_camel_case_types)]
                    struct ListGovernedAssetsSvc<T: AuthorityService>(pub Arc<T>);
                    impl<
                        T: AuthorityService,
                    > tonic::server::UnaryService<super::ListGovernedAssetsRequest>
                    for ListGovernedAssetsSvc<T> {
                        type Response = super::ListGovernedAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListGovernedAssetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_governed_assets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListGovernedAssetsSvc(inner);
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
                "/authority_api.v1.AuthorityService/SetGovernedAssets" => {
                    #[allow(non_camel_case_types)]
                    struct SetGovernedAssetsSvc<T: AuthorityService>(pub Arc<T>);
                    impl<
                        T: AuthorityService,
                    > tonic::server::UnaryService<super::SetGovernedAssetsRequest>
                    for SetGovernedAssetsSvc<T> {
                        type Response = super::SetGovernedAssetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetGovernedAssetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_governed_assets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetGovernedAssetsSvc(inner);
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
    impl<T: AuthorityService> Clone for AuthorityServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AuthorityService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AuthorityService> tonic::server::NamedService for AuthorityServiceServer<T> {
        const NAME: &'static str = "authority_api.v1.AuthorityService";
    }
}
