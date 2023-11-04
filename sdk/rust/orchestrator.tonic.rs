// @generated
/// Generated client implementations.
pub mod monetae_overload_orchestrator_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Orchestrator Service for Managing Load Tests.
*/
    #[derive(Debug, Clone)]
    pub struct MonetaeOverloadOrchestratorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MonetaeOverloadOrchestratorClient<tonic::transport::Channel> {
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
    impl<T> MonetaeOverloadOrchestratorClient<T>
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
        ) -> MonetaeOverloadOrchestratorClient<InterceptedService<T, F>>
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
            MonetaeOverloadOrchestratorClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /** Broadcasts overload agent connections to all agents
*/
        pub async fn broadcast_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::DynamicVerifier>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::AgentConnection>>,
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
                "/orchestrator.MonetaeOverloadOrchestrator/BroadcastAgent",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /** Execute the Overload Agent.
*/
        pub async fn execute(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteRequest>,
        ) -> Result<tonic::Response<super::ExecuteResponse>, tonic::Status> {
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
                "/orchestrator.MonetaeOverloadOrchestrator/Execute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Shutdown all Overload Agents.
*/
        pub async fn shutdown(
            &mut self,
            request: impl tonic::IntoRequest<super::ShutdownRequest>,
        ) -> Result<tonic::Response<super::ShutdownResponse>, tonic::Status> {
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
                "/orchestrator.MonetaeOverloadOrchestrator/Shutdown",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Commands Overload Agents to run test setup.
*/
        pub async fn setup(
            &mut self,
            request: impl tonic::IntoRequest<super::SetupRequest>,
        ) -> Result<tonic::Response<super::SetupResponse>, tonic::Status> {
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
                "/orchestrator.MonetaeOverloadOrchestrator/Setup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod monetae_overload_orchestrator_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MonetaeOverloadOrchestratorServer.
    #[async_trait]
    pub trait MonetaeOverloadOrchestrator: Send + Sync + 'static {
        /// Server streaming response type for the BroadcastAgent method.
        type BroadcastAgentStream: futures_core::Stream<
                Item = Result<super::AgentConnection, tonic::Status>,
            >
            + Send
            + 'static;
        /** Broadcasts overload agent connections to all agents
*/
        async fn broadcast_agent(
            &self,
            request: tonic::Request<super::super::common::DynamicVerifier>,
        ) -> Result<tonic::Response<Self::BroadcastAgentStream>, tonic::Status>;
        /** Execute the Overload Agent.
*/
        async fn execute(
            &self,
            request: tonic::Request<super::ExecuteRequest>,
        ) -> Result<tonic::Response<super::ExecuteResponse>, tonic::Status>;
        /** Shutdown all Overload Agents.
*/
        async fn shutdown(
            &self,
            request: tonic::Request<super::ShutdownRequest>,
        ) -> Result<tonic::Response<super::ShutdownResponse>, tonic::Status>;
        /** Commands Overload Agents to run test setup.
*/
        async fn setup(
            &self,
            request: tonic::Request<super::SetupRequest>,
        ) -> Result<tonic::Response<super::SetupResponse>, tonic::Status>;
    }
    /** Orchestrator Service for Managing Load Tests.
*/
    #[derive(Debug)]
    pub struct MonetaeOverloadOrchestratorServer<T: MonetaeOverloadOrchestrator> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MonetaeOverloadOrchestrator> MonetaeOverloadOrchestratorServer<T> {
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
    for MonetaeOverloadOrchestratorServer<T>
    where
        T: MonetaeOverloadOrchestrator,
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
                "/orchestrator.MonetaeOverloadOrchestrator/BroadcastAgent" => {
                    #[allow(non_camel_case_types)]
                    struct BroadcastAgentSvc<T: MonetaeOverloadOrchestrator>(pub Arc<T>);
                    impl<
                        T: MonetaeOverloadOrchestrator,
                    > tonic::server::ServerStreamingService<
                        super::super::common::DynamicVerifier,
                    > for BroadcastAgentSvc<T> {
                        type Response = super::AgentConnection;
                        type ResponseStream = T::BroadcastAgentStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::common::DynamicVerifier,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).broadcast_agent(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BroadcastAgentSvc(inner);
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
                "/orchestrator.MonetaeOverloadOrchestrator/Execute" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteSvc<T: MonetaeOverloadOrchestrator>(pub Arc<T>);
                    impl<
                        T: MonetaeOverloadOrchestrator,
                    > tonic::server::UnaryService<super::ExecuteRequest>
                    for ExecuteSvc<T> {
                        type Response = super::ExecuteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).execute(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ExecuteSvc(inner);
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
                "/orchestrator.MonetaeOverloadOrchestrator/Shutdown" => {
                    #[allow(non_camel_case_types)]
                    struct ShutdownSvc<T: MonetaeOverloadOrchestrator>(pub Arc<T>);
                    impl<
                        T: MonetaeOverloadOrchestrator,
                    > tonic::server::UnaryService<super::ShutdownRequest>
                    for ShutdownSvc<T> {
                        type Response = super::ShutdownResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ShutdownRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).shutdown(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ShutdownSvc(inner);
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
                "/orchestrator.MonetaeOverloadOrchestrator/Setup" => {
                    #[allow(non_camel_case_types)]
                    struct SetupSvc<T: MonetaeOverloadOrchestrator>(pub Arc<T>);
                    impl<
                        T: MonetaeOverloadOrchestrator,
                    > tonic::server::UnaryService<super::SetupRequest> for SetupSvc<T> {
                        type Response = super::SetupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).setup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetupSvc(inner);
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
    impl<T: MonetaeOverloadOrchestrator> Clone for MonetaeOverloadOrchestratorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MonetaeOverloadOrchestrator> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MonetaeOverloadOrchestrator> tonic::server::NamedService
    for MonetaeOverloadOrchestratorServer<T> {
        const NAME: &'static str = "orchestrator.MonetaeOverloadOrchestrator";
    }
}
