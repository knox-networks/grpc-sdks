// @generated
/// Generated client implementations.
pub mod credential_adapter_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /**Enables existing systems to support W3C Verifiable Credential (VC) interactions between holders, VC issuers and verifiers as per https://www.w3.org/TR/vc-data-model/.  It is either used by credential providers (VC Issuer) or verifiers as required.  It provides predefined interfaces to expose (Example Bank customer info APIs) but allows for custom integrations as required.
*/
    #[derive(Debug, Clone)]
    pub struct CredentialAdapterClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CredentialAdapterClient<tonic::transport::Channel> {
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
    impl<T> CredentialAdapterClient<T>
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
        ) -> CredentialAdapterClient<InterceptedService<T, F>>
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
            CredentialAdapterClient::new(InterceptedService::new(inner, interceptor))
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
        /** Called by the holder to the VC issuer to initiate the challenge needed for credential issuance.
*/
        pub async fn create_issuance_challenge(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIssuanceChallengeRequest>,
        ) -> Result<
            tonic::Response<super::CreateIssuanceChallengeResponse>,
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
                "/vc_api.v1.CredentialAdapter/CreateIssuanceChallenge",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Called by the holder to the VC issuer to answer the challenge and receive the credential.
*/
        pub async fn issue_verifiable_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::IssueVerifiableCredentialRequest>,
        ) -> Result<
            tonic::Response<super::IssueVerifiableCredentialResponse>,
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
                "/vc_api.v1.CredentialAdapter/IssueVerifiableCredential",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Called by the holder to the verifier to initiate the challenge needed to verify the presentation with the verifier.
*/
        pub async fn create_presentation_challenge(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePresentationChallengeRequest>,
        ) -> Result<
            tonic::Response<super::CreatePresentationChallengeResponse>,
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
                "/vc_api.v1.CredentialAdapter/CreatePresentationChallenge",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Called by the holder to the verifier to verify the presentation with the verifier.
*/
        pub async fn present_verifiable_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::PresentVerifiableCredentialRequest>,
        ) -> Result<
            tonic::Response<super::PresentVerifiableCredentialResponse>,
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
                "/vc_api.v1.CredentialAdapter/PresentVerifiableCredential",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Called by the browser to initialize a stream with the server to listen for events such as challenge completion.
*/
        pub async fn wait_for_completion(
            &mut self,
            request: impl tonic::IntoRequest<super::WaitForCompletionRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::WaitForCompletionResponse>>,
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
                "/vc_api.v1.CredentialAdapter/WaitForCompletion",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod credential_adapter_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with CredentialAdapterServer.
    #[async_trait]
    pub trait CredentialAdapter: Send + Sync + 'static {
        /** Called by the holder to the VC issuer to initiate the challenge needed for credential issuance.
*/
        async fn create_issuance_challenge(
            &self,
            request: tonic::Request<super::CreateIssuanceChallengeRequest>,
        ) -> Result<
            tonic::Response<super::CreateIssuanceChallengeResponse>,
            tonic::Status,
        >;
        /** Called by the holder to the VC issuer to answer the challenge and receive the credential.
*/
        async fn issue_verifiable_credential(
            &self,
            request: tonic::Request<super::IssueVerifiableCredentialRequest>,
        ) -> Result<
            tonic::Response<super::IssueVerifiableCredentialResponse>,
            tonic::Status,
        >;
        /** Called by the holder to the verifier to initiate the challenge needed to verify the presentation with the verifier.
*/
        async fn create_presentation_challenge(
            &self,
            request: tonic::Request<super::CreatePresentationChallengeRequest>,
        ) -> Result<
            tonic::Response<super::CreatePresentationChallengeResponse>,
            tonic::Status,
        >;
        /** Called by the holder to the verifier to verify the presentation with the verifier.
*/
        async fn present_verifiable_credential(
            &self,
            request: tonic::Request<super::PresentVerifiableCredentialRequest>,
        ) -> Result<
            tonic::Response<super::PresentVerifiableCredentialResponse>,
            tonic::Status,
        >;
        ///Server streaming response type for the WaitForCompletion method.
        type WaitForCompletionStream: futures_core::Stream<
                Item = Result<super::WaitForCompletionResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /** Called by the browser to initialize a stream with the server to listen for events such as challenge completion.
*/
        async fn wait_for_completion(
            &self,
            request: tonic::Request<super::WaitForCompletionRequest>,
        ) -> Result<tonic::Response<Self::WaitForCompletionStream>, tonic::Status>;
    }
    /**Enables existing systems to support W3C Verifiable Credential (VC) interactions between holders, VC issuers and verifiers as per https://www.w3.org/TR/vc-data-model/.  It is either used by credential providers (VC Issuer) or verifiers as required.  It provides predefined interfaces to expose (Example Bank customer info APIs) but allows for custom integrations as required.
*/
    #[derive(Debug)]
    pub struct CredentialAdapterServer<T: CredentialAdapter> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CredentialAdapter> CredentialAdapterServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CredentialAdapterServer<T>
    where
        T: CredentialAdapter,
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
                "/vc_api.v1.CredentialAdapter/CreateIssuanceChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct CreateIssuanceChallengeSvc<T: CredentialAdapter>(pub Arc<T>);
                    impl<
                        T: CredentialAdapter,
                    > tonic::server::UnaryService<super::CreateIssuanceChallengeRequest>
                    for CreateIssuanceChallengeSvc<T> {
                        type Response = super::CreateIssuanceChallengeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateIssuanceChallengeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_issuance_challenge(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateIssuanceChallengeSvc(inner);
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
                "/vc_api.v1.CredentialAdapter/IssueVerifiableCredential" => {
                    #[allow(non_camel_case_types)]
                    struct IssueVerifiableCredentialSvc<T: CredentialAdapter>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CredentialAdapter,
                    > tonic::server::UnaryService<
                        super::IssueVerifiableCredentialRequest,
                    > for IssueVerifiableCredentialSvc<T> {
                        type Response = super::IssueVerifiableCredentialResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::IssueVerifiableCredentialRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).issue_verifiable_credential(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IssueVerifiableCredentialSvc(inner);
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
                "/vc_api.v1.CredentialAdapter/CreatePresentationChallenge" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePresentationChallengeSvc<T: CredentialAdapter>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CredentialAdapter,
                    > tonic::server::UnaryService<
                        super::CreatePresentationChallengeRequest,
                    > for CreatePresentationChallengeSvc<T> {
                        type Response = super::CreatePresentationChallengeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreatePresentationChallengeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_presentation_challenge(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreatePresentationChallengeSvc(inner);
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
                "/vc_api.v1.CredentialAdapter/PresentVerifiableCredential" => {
                    #[allow(non_camel_case_types)]
                    struct PresentVerifiableCredentialSvc<T: CredentialAdapter>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CredentialAdapter,
                    > tonic::server::UnaryService<
                        super::PresentVerifiableCredentialRequest,
                    > for PresentVerifiableCredentialSvc<T> {
                        type Response = super::PresentVerifiableCredentialResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PresentVerifiableCredentialRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).present_verifiable_credential(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PresentVerifiableCredentialSvc(inner);
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
                "/vc_api.v1.CredentialAdapter/WaitForCompletion" => {
                    #[allow(non_camel_case_types)]
                    struct WaitForCompletionSvc<T: CredentialAdapter>(pub Arc<T>);
                    impl<
                        T: CredentialAdapter,
                    > tonic::server::ServerStreamingService<
                        super::WaitForCompletionRequest,
                    > for WaitForCompletionSvc<T> {
                        type Response = super::WaitForCompletionResponse;
                        type ResponseStream = T::WaitForCompletionStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WaitForCompletionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).wait_for_completion(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WaitForCompletionSvc(inner);
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
    impl<T: CredentialAdapter> Clone for CredentialAdapterServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CredentialAdapter> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CredentialAdapter> tonic::server::NamedService
    for CredentialAdapterServer<T> {
        const NAME: &'static str = "vc_api.v1.CredentialAdapter";
    }
}
