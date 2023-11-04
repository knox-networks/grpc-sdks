// @generated
/// Generated client implementations.
pub mod business_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Business Service.
*/
    #[derive(Debug, Clone)]
    pub struct BusinessServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BusinessServiceClient<tonic::transport::Channel> {
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
    impl<T> BusinessServiceClient<T>
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
        ) -> BusinessServiceClient<InterceptedService<T, F>>
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
            BusinessServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /** List.
*/
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRequest>,
        ) -> Result<tonic::Response<super::ListResponse>, tonic::Status> {
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
                "/business_api.v1.BusinessService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Clear Request.
*/
        pub async fn clear(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearRequest>,
        ) -> Result<tonic::Response<super::ClearResponse>, tonic::Status> {
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
                "/business_api.v1.BusinessService/Clear",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Send Payload.
*/
        pub async fn send_payload(
            &mut self,
            request: impl tonic::IntoRequest<super::SendPayloadRequest>,
        ) -> Result<tonic::Response<super::SendPayloadResponse>, tonic::Status> {
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
                "/business_api.v1.BusinessService/SendPayload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Send New Transaction.
*/
        pub async fn send_new_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::SendNewTransactionRequest>,
        ) -> Result<tonic::Response<super::SendNewTransactionResponse>, tonic::Status> {
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
                "/business_api.v1.BusinessService/SendNewTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Send Proposal Acked.
*/
        pub async fn send_proposal_acked(
            &mut self,
            request: impl tonic::IntoRequest<super::SendProposalAckedRequest>,
        ) -> Result<tonic::Response<super::SendProposalAckedResponse>, tonic::Status> {
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
                "/business_api.v1.BusinessService/SendProposalAcked",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Send Proposal Nacked.
*/
        pub async fn send_proposal_nacked(
            &mut self,
            request: impl tonic::IntoRequest<super::SendProposalNackedRequest>,
        ) -> Result<tonic::Response<super::SendProposalNackedResponse>, tonic::Status> {
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
                "/business_api.v1.BusinessService/SendProposalNacked",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Send Secret Revealed.
*/
        pub async fn send_secret_revealed(
            &mut self,
            request: impl tonic::IntoRequest<super::SendSecretRevealedRequest>,
        ) -> Result<tonic::Response<super::SendSecretRevealedResponse>, tonic::Status> {
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
                "/business_api.v1.BusinessService/SendSecretRevealed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod business_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BusinessServiceServer.
    #[async_trait]
    pub trait BusinessService: Send + Sync + 'static {
        /** List.
*/
        async fn list(
            &self,
            request: tonic::Request<super::ListRequest>,
        ) -> Result<tonic::Response<super::ListResponse>, tonic::Status>;
        /** Clear Request.
*/
        async fn clear(
            &self,
            request: tonic::Request<super::ClearRequest>,
        ) -> Result<tonic::Response<super::ClearResponse>, tonic::Status>;
        /** Send Payload.
*/
        async fn send_payload(
            &self,
            request: tonic::Request<super::SendPayloadRequest>,
        ) -> Result<tonic::Response<super::SendPayloadResponse>, tonic::Status>;
        /** Send New Transaction.
*/
        async fn send_new_transaction(
            &self,
            request: tonic::Request<super::SendNewTransactionRequest>,
        ) -> Result<tonic::Response<super::SendNewTransactionResponse>, tonic::Status>;
        /** Send Proposal Acked.
*/
        async fn send_proposal_acked(
            &self,
            request: tonic::Request<super::SendProposalAckedRequest>,
        ) -> Result<tonic::Response<super::SendProposalAckedResponse>, tonic::Status>;
        /** Send Proposal Nacked.
*/
        async fn send_proposal_nacked(
            &self,
            request: tonic::Request<super::SendProposalNackedRequest>,
        ) -> Result<tonic::Response<super::SendProposalNackedResponse>, tonic::Status>;
        /** Send Secret Revealed.
*/
        async fn send_secret_revealed(
            &self,
            request: tonic::Request<super::SendSecretRevealedRequest>,
        ) -> Result<tonic::Response<super::SendSecretRevealedResponse>, tonic::Status>;
    }
    /** Business Service.
*/
    #[derive(Debug)]
    pub struct BusinessServiceServer<T: BusinessService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BusinessService> BusinessServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BusinessServiceServer<T>
    where
        T: BusinessService,
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
                "/business_api.v1.BusinessService/List" => {
                    #[allow(non_camel_case_types)]
                    struct ListSvc<T: BusinessService>(pub Arc<T>);
                    impl<
                        T: BusinessService,
                    > tonic::server::UnaryService<super::ListRequest> for ListSvc<T> {
                        type Response = super::ListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSvc(inner);
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
                "/business_api.v1.BusinessService/Clear" => {
                    #[allow(non_camel_case_types)]
                    struct ClearSvc<T: BusinessService>(pub Arc<T>);
                    impl<
                        T: BusinessService,
                    > tonic::server::UnaryService<super::ClearRequest> for ClearSvc<T> {
                        type Response = super::ClearResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).clear(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClearSvc(inner);
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
                "/business_api.v1.BusinessService/SendPayload" => {
                    #[allow(non_camel_case_types)]
                    struct SendPayloadSvc<T: BusinessService>(pub Arc<T>);
                    impl<
                        T: BusinessService,
                    > tonic::server::UnaryService<super::SendPayloadRequest>
                    for SendPayloadSvc<T> {
                        type Response = super::SendPayloadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendPayloadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).send_payload(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendPayloadSvc(inner);
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
                "/business_api.v1.BusinessService/SendNewTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct SendNewTransactionSvc<T: BusinessService>(pub Arc<T>);
                    impl<
                        T: BusinessService,
                    > tonic::server::UnaryService<super::SendNewTransactionRequest>
                    for SendNewTransactionSvc<T> {
                        type Response = super::SendNewTransactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendNewTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).send_new_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendNewTransactionSvc(inner);
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
                "/business_api.v1.BusinessService/SendProposalAcked" => {
                    #[allow(non_camel_case_types)]
                    struct SendProposalAckedSvc<T: BusinessService>(pub Arc<T>);
                    impl<
                        T: BusinessService,
                    > tonic::server::UnaryService<super::SendProposalAckedRequest>
                    for SendProposalAckedSvc<T> {
                        type Response = super::SendProposalAckedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendProposalAckedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).send_proposal_acked(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendProposalAckedSvc(inner);
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
                "/business_api.v1.BusinessService/SendProposalNacked" => {
                    #[allow(non_camel_case_types)]
                    struct SendProposalNackedSvc<T: BusinessService>(pub Arc<T>);
                    impl<
                        T: BusinessService,
                    > tonic::server::UnaryService<super::SendProposalNackedRequest>
                    for SendProposalNackedSvc<T> {
                        type Response = super::SendProposalNackedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendProposalNackedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).send_proposal_nacked(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendProposalNackedSvc(inner);
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
                "/business_api.v1.BusinessService/SendSecretRevealed" => {
                    #[allow(non_camel_case_types)]
                    struct SendSecretRevealedSvc<T: BusinessService>(pub Arc<T>);
                    impl<
                        T: BusinessService,
                    > tonic::server::UnaryService<super::SendSecretRevealedRequest>
                    for SendSecretRevealedSvc<T> {
                        type Response = super::SendSecretRevealedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendSecretRevealedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).send_secret_revealed(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendSecretRevealedSvc(inner);
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
    impl<T: BusinessService> Clone for BusinessServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: BusinessService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BusinessService> tonic::server::NamedService for BusinessServiceServer<T> {
        const NAME: &'static str = "business_api.v1.BusinessService";
    }
}
