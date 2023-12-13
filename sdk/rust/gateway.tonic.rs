// @generated
/// Generated client implementations.
pub mod monetae_gateway_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** The Gateway service allows fast switching of streaming messages between senders and recipients identified by their
 public key, over authenticated connections. It exposes the APIs needed to connect, authenticate and send data.
*/
    #[derive(Debug, Clone)]
    pub struct MonetaeGatewayClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MonetaeGatewayClient<tonic::transport::Channel> {
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
    impl<T> MonetaeGatewayClient<T>
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
        ) -> MonetaeGatewayClient<InterceptedService<T, F>>
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
            MonetaeGatewayClient::new(InterceptedService::new(inner, interceptor))
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
        /** Start a new authenticated session and get a challenge nonce.
*/
        pub async fn authenticate(
            &mut self,
            request: impl tonic::IntoRequest<super::super::common::AuthenticationRequest>,
        ) -> Result<
            tonic::Response<super::super::common::AuthenticationResponse>,
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
                "/gateway.MonetaeGateway/Authenticate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Authenticates the connection and opens a bidirectional stream.
*/
        pub async fn data(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::common::Packet,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::common::Packet>>,
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
                "/gateway.MonetaeGateway/Data",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        /** Get a list of connections to the Gateway, including the active connections and connections that have been removed.
*/
        pub async fn get_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ConnectionsRequest>,
        ) -> Result<tonic::Response<super::ConnectionsResponse>, tonic::Status> {
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
                "/gateway.MonetaeGateway/GetConnections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod monetae_gateway_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MonetaeGatewayServer.
    #[async_trait]
    pub trait MonetaeGateway: Send + Sync + 'static {
        /** Start a new authenticated session and get a challenge nonce.
*/
        async fn authenticate(
            &self,
            request: tonic::Request<super::super::common::AuthenticationRequest>,
        ) -> Result<
            tonic::Response<super::super::common::AuthenticationResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the Data method.
        type DataStream: futures_core::Stream<
                Item = Result<super::super::common::Packet, tonic::Status>,
            >
            + Send
            + 'static;
        /** Authenticates the connection and opens a bidirectional stream.
*/
        async fn data(
            &self,
            request: tonic::Request<tonic::Streaming<super::super::common::Packet>>,
        ) -> Result<tonic::Response<Self::DataStream>, tonic::Status>;
        /** Get a list of connections to the Gateway, including the active connections and connections that have been removed.
*/
        async fn get_connections(
            &self,
            request: tonic::Request<super::ConnectionsRequest>,
        ) -> Result<tonic::Response<super::ConnectionsResponse>, tonic::Status>;
    }
    /** The Gateway service allows fast switching of streaming messages between senders and recipients identified by their
 public key, over authenticated connections. It exposes the APIs needed to connect, authenticate and send data.
*/
    #[derive(Debug)]
    pub struct MonetaeGatewayServer<T: MonetaeGateway> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MonetaeGateway> MonetaeGatewayServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MonetaeGatewayServer<T>
    where
        T: MonetaeGateway,
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
                "/gateway.MonetaeGateway/Authenticate" => {
                    #[allow(non_camel_case_types)]
                    struct AuthenticateSvc<T: MonetaeGateway>(pub Arc<T>);
                    impl<
                        T: MonetaeGateway,
                    > tonic::server::UnaryService<
                        super::super::common::AuthenticationRequest,
                    > for AuthenticateSvc<T> {
                        type Response = super::super::common::AuthenticationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::common::AuthenticationRequest,
                            >,
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
                "/gateway.MonetaeGateway/Data" => {
                    #[allow(non_camel_case_types)]
                    struct DataSvc<T: MonetaeGateway>(pub Arc<T>);
                    impl<
                        T: MonetaeGateway,
                    > tonic::server::StreamingService<super::super::common::Packet>
                    for DataSvc<T> {
                        type Response = super::super::common::Packet;
                        type ResponseStream = T::DataStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::super::common::Packet>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).data(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DataSvc(inner);
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
                "/gateway.MonetaeGateway/GetConnections" => {
                    #[allow(non_camel_case_types)]
                    struct GetConnectionsSvc<T: MonetaeGateway>(pub Arc<T>);
                    impl<
                        T: MonetaeGateway,
                    > tonic::server::UnaryService<super::ConnectionsRequest>
                    for GetConnectionsSvc<T> {
                        type Response = super::ConnectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConnectionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_connections(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetConnectionsSvc(inner);
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
    impl<T: MonetaeGateway> Clone for MonetaeGatewayServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MonetaeGateway> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MonetaeGateway> tonic::server::NamedService for MonetaeGatewayServer<T> {
        const NAME: &'static str = "gateway.MonetaeGateway";
    }
}
