// @generated
/// Generated client implementations.
pub mod webhook_manager_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** The Service Manages the Lifecycle of Webhooks.
*/
    #[derive(Debug, Clone)]
    pub struct WebhookManagerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WebhookManagerServiceClient<tonic::transport::Channel> {
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
    impl<T> WebhookManagerServiceClient<T>
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
        ) -> WebhookManagerServiceClient<InterceptedService<T, F>>
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
            WebhookManagerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /** Creates a Webhook.
*/
        pub async fn create_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWebhookRequest>,
        ) -> Result<tonic::Response<super::CreateWebhookResponse>, tonic::Status> {
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
                "/webhook_api.v1.WebhookManagerService/CreateWebhook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Lists Webhooks.
*/
        pub async fn list_webhooks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWebhooksRequest>,
        ) -> Result<tonic::Response<super::ListWebhooksResponse>, tonic::Status> {
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
                "/webhook_api.v1.WebhookManagerService/ListWebhooks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Deletes a Webhook.
*/
        pub async fn delete_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWebhookRequest>,
        ) -> Result<tonic::Response<super::DeleteWebhookResponse>, tonic::Status> {
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
                "/webhook_api.v1.WebhookManagerService/DeleteWebhook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Lists Webhook Events.
*/
        pub async fn list_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEventsRequest>,
        ) -> Result<tonic::Response<super::ListEventsResponse>, tonic::Status> {
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
                "/webhook_api.v1.WebhookManagerService/ListEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Retries a Webhook Event.
*/
        pub async fn retry_event(
            &mut self,
            request: impl tonic::IntoRequest<super::RetryEventRequest>,
        ) -> Result<tonic::Response<super::RetryEventResponse>, tonic::Status> {
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
                "/webhook_api.v1.WebhookManagerService/RetryEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Ping a Webhook.
*/
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<super::PingRequest>,
        ) -> Result<tonic::Response<super::PingResponse>, tonic::Status> {
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
                "/webhook_api.v1.WebhookManagerService/Ping",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Lists Delivery History.
*/
        pub async fn list_delivery_history(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeliveryHistoryRequest>,
        ) -> Result<tonic::Response<super::ListDeliveryHistoryResponse>, tonic::Status> {
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
                "/webhook_api.v1.WebhookManagerService/ListDeliveryHistory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Creates a Scheduled Reaction.
*/
        pub async fn create_scheduled_reaction(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateScheduledReactionRequest>,
        ) -> Result<
            tonic::Response<super::CreateScheduledReactionResponse>,
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
                "/webhook_api.v1.WebhookManagerService/CreateScheduledReaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Lists Scheduled Reactions.
*/
        pub async fn list_scheduled_reactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListScheduledReactionsRequest>,
        ) -> Result<
            tonic::Response<super::ListScheduledReactionsResponse>,
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
                "/webhook_api.v1.WebhookManagerService/ListScheduledReactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Deletes a Scheduled Reaction.
*/
        pub async fn delete_scheduled_reaction(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteScheduledReactionRequest>,
        ) -> Result<
            tonic::Response<super::DeleteScheduledReactionResponse>,
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
                "/webhook_api.v1.WebhookManagerService/DeleteScheduledReaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Create Prevalidation
*/
        pub async fn create_prevalidation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePrevalidationRequest>,
        ) -> Result<tonic::Response<super::CreatePrevalidationResponse>, tonic::Status> {
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
                "/webhook_api.v1.WebhookManagerService/CreatePrevalidation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** List Prevalidations
*/
        pub async fn list_prevalidations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPrevalidationsRequest>,
        ) -> Result<tonic::Response<super::ListPrevalidationsResponse>, tonic::Status> {
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
                "/webhook_api.v1.WebhookManagerService/ListPrevalidations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Delete Prevalidation
*/
        pub async fn delete_prevalidation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePrevalidationRequest>,
        ) -> Result<tonic::Response<super::DeletePrevalidationResponse>, tonic::Status> {
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
                "/webhook_api.v1.WebhookManagerService/DeletePrevalidation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod webhook_manager_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with WebhookManagerServiceServer.
    #[async_trait]
    pub trait WebhookManagerService: Send + Sync + 'static {
        /** Creates a Webhook.
*/
        async fn create_webhook(
            &self,
            request: tonic::Request<super::CreateWebhookRequest>,
        ) -> Result<tonic::Response<super::CreateWebhookResponse>, tonic::Status>;
        /** Lists Webhooks.
*/
        async fn list_webhooks(
            &self,
            request: tonic::Request<super::ListWebhooksRequest>,
        ) -> Result<tonic::Response<super::ListWebhooksResponse>, tonic::Status>;
        /** Deletes a Webhook.
*/
        async fn delete_webhook(
            &self,
            request: tonic::Request<super::DeleteWebhookRequest>,
        ) -> Result<tonic::Response<super::DeleteWebhookResponse>, tonic::Status>;
        /** Lists Webhook Events.
*/
        async fn list_events(
            &self,
            request: tonic::Request<super::ListEventsRequest>,
        ) -> Result<tonic::Response<super::ListEventsResponse>, tonic::Status>;
        /** Retries a Webhook Event.
*/
        async fn retry_event(
            &self,
            request: tonic::Request<super::RetryEventRequest>,
        ) -> Result<tonic::Response<super::RetryEventResponse>, tonic::Status>;
        /** Ping a Webhook.
*/
        async fn ping(
            &self,
            request: tonic::Request<super::PingRequest>,
        ) -> Result<tonic::Response<super::PingResponse>, tonic::Status>;
        /** Lists Delivery History.
*/
        async fn list_delivery_history(
            &self,
            request: tonic::Request<super::ListDeliveryHistoryRequest>,
        ) -> Result<tonic::Response<super::ListDeliveryHistoryResponse>, tonic::Status>;
        /** Creates a Scheduled Reaction.
*/
        async fn create_scheduled_reaction(
            &self,
            request: tonic::Request<super::CreateScheduledReactionRequest>,
        ) -> Result<
            tonic::Response<super::CreateScheduledReactionResponse>,
            tonic::Status,
        >;
        /** Lists Scheduled Reactions.
*/
        async fn list_scheduled_reactions(
            &self,
            request: tonic::Request<super::ListScheduledReactionsRequest>,
        ) -> Result<
            tonic::Response<super::ListScheduledReactionsResponse>,
            tonic::Status,
        >;
        /** Deletes a Scheduled Reaction.
*/
        async fn delete_scheduled_reaction(
            &self,
            request: tonic::Request<super::DeleteScheduledReactionRequest>,
        ) -> Result<
            tonic::Response<super::DeleteScheduledReactionResponse>,
            tonic::Status,
        >;
        /** Create Prevalidation
*/
        async fn create_prevalidation(
            &self,
            request: tonic::Request<super::CreatePrevalidationRequest>,
        ) -> Result<tonic::Response<super::CreatePrevalidationResponse>, tonic::Status>;
        /** List Prevalidations
*/
        async fn list_prevalidations(
            &self,
            request: tonic::Request<super::ListPrevalidationsRequest>,
        ) -> Result<tonic::Response<super::ListPrevalidationsResponse>, tonic::Status>;
        /** Delete Prevalidation
*/
        async fn delete_prevalidation(
            &self,
            request: tonic::Request<super::DeletePrevalidationRequest>,
        ) -> Result<tonic::Response<super::DeletePrevalidationResponse>, tonic::Status>;
    }
    /** The Service Manages the Lifecycle of Webhooks.
*/
    #[derive(Debug)]
    pub struct WebhookManagerServiceServer<T: WebhookManagerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WebhookManagerService> WebhookManagerServiceServer<T> {
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
    for WebhookManagerServiceServer<T>
    where
        T: WebhookManagerService,
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
                "/webhook_api.v1.WebhookManagerService/CreateWebhook" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWebhookSvc<T: WebhookManagerService>(pub Arc<T>);
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::CreateWebhookRequest>
                    for CreateWebhookSvc<T> {
                        type Response = super::CreateWebhookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWebhookRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_webhook(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateWebhookSvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/ListWebhooks" => {
                    #[allow(non_camel_case_types)]
                    struct ListWebhooksSvc<T: WebhookManagerService>(pub Arc<T>);
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::ListWebhooksRequest>
                    for ListWebhooksSvc<T> {
                        type Response = super::ListWebhooksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListWebhooksRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_webhooks(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListWebhooksSvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/DeleteWebhook" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteWebhookSvc<T: WebhookManagerService>(pub Arc<T>);
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::DeleteWebhookRequest>
                    for DeleteWebhookSvc<T> {
                        type Response = super::DeleteWebhookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteWebhookRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_webhook(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteWebhookSvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/ListEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ListEventsSvc<T: WebhookManagerService>(pub Arc<T>);
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::ListEventsRequest>
                    for ListEventsSvc<T> {
                        type Response = super::ListEventsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_events(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListEventsSvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/RetryEvent" => {
                    #[allow(non_camel_case_types)]
                    struct RetryEventSvc<T: WebhookManagerService>(pub Arc<T>);
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::RetryEventRequest>
                    for RetryEventSvc<T> {
                        type Response = super::RetryEventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetryEventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).retry_event(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RetryEventSvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/Ping" => {
                    #[allow(non_camel_case_types)]
                    struct PingSvc<T: WebhookManagerService>(pub Arc<T>);
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::PingRequest> for PingSvc<T> {
                        type Response = super::PingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).ping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PingSvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/ListDeliveryHistory" => {
                    #[allow(non_camel_case_types)]
                    struct ListDeliveryHistorySvc<T: WebhookManagerService>(pub Arc<T>);
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::ListDeliveryHistoryRequest>
                    for ListDeliveryHistorySvc<T> {
                        type Response = super::ListDeliveryHistoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDeliveryHistoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_delivery_history(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListDeliveryHistorySvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/CreateScheduledReaction" => {
                    #[allow(non_camel_case_types)]
                    struct CreateScheduledReactionSvc<T: WebhookManagerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::CreateScheduledReactionRequest>
                    for CreateScheduledReactionSvc<T> {
                        type Response = super::CreateScheduledReactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateScheduledReactionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_scheduled_reaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateScheduledReactionSvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/ListScheduledReactions" => {
                    #[allow(non_camel_case_types)]
                    struct ListScheduledReactionsSvc<T: WebhookManagerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::ListScheduledReactionsRequest>
                    for ListScheduledReactionsSvc<T> {
                        type Response = super::ListScheduledReactionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListScheduledReactionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_scheduled_reactions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListScheduledReactionsSvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/DeleteScheduledReaction" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteScheduledReactionSvc<T: WebhookManagerService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::DeleteScheduledReactionRequest>
                    for DeleteScheduledReactionSvc<T> {
                        type Response = super::DeleteScheduledReactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteScheduledReactionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_scheduled_reaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteScheduledReactionSvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/CreatePrevalidation" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePrevalidationSvc<T: WebhookManagerService>(pub Arc<T>);
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::CreatePrevalidationRequest>
                    for CreatePrevalidationSvc<T> {
                        type Response = super::CreatePrevalidationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePrevalidationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_prevalidation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreatePrevalidationSvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/ListPrevalidations" => {
                    #[allow(non_camel_case_types)]
                    struct ListPrevalidationsSvc<T: WebhookManagerService>(pub Arc<T>);
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::ListPrevalidationsRequest>
                    for ListPrevalidationsSvc<T> {
                        type Response = super::ListPrevalidationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPrevalidationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_prevalidations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListPrevalidationsSvc(inner);
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
                "/webhook_api.v1.WebhookManagerService/DeletePrevalidation" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePrevalidationSvc<T: WebhookManagerService>(pub Arc<T>);
                    impl<
                        T: WebhookManagerService,
                    > tonic::server::UnaryService<super::DeletePrevalidationRequest>
                    for DeletePrevalidationSvc<T> {
                        type Response = super::DeletePrevalidationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePrevalidationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_prevalidation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeletePrevalidationSvc(inner);
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
    impl<T: WebhookManagerService> Clone for WebhookManagerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WebhookManagerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WebhookManagerService> tonic::server::NamedService
    for WebhookManagerServiceServer<T> {
        const NAME: &'static str = "webhook_api.v1.WebhookManagerService";
    }
}
/// Generated client implementations.
pub mod event_consumer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** This service is implemented by the consumer service receiving the webhook event
*/
    #[derive(Debug, Clone)]
    pub struct EventConsumerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EventConsumerServiceClient<tonic::transport::Channel> {
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
    impl<T> EventConsumerServiceClient<T>
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
        ) -> EventConsumerServiceClient<InterceptedService<T, F>>
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
            EventConsumerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /** Webhook Event.
*/
        pub async fn event(
            &mut self,
            request: impl tonic::IntoRequest<super::EventRequest>,
        ) -> Result<tonic::Response<super::EventResponse>, tonic::Status> {
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
                "/webhook_api.v1.EventConsumerService/Event",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod event_consumer_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EventConsumerServiceServer.
    #[async_trait]
    pub trait EventConsumerService: Send + Sync + 'static {
        /** Webhook Event.
*/
        async fn event(
            &self,
            request: tonic::Request<super::EventRequest>,
        ) -> Result<tonic::Response<super::EventResponse>, tonic::Status>;
    }
    /** This service is implemented by the consumer service receiving the webhook event
*/
    #[derive(Debug)]
    pub struct EventConsumerServiceServer<T: EventConsumerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EventConsumerService> EventConsumerServiceServer<T> {
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
    for EventConsumerServiceServer<T>
    where
        T: EventConsumerService,
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
                "/webhook_api.v1.EventConsumerService/Event" => {
                    #[allow(non_camel_case_types)]
                    struct EventSvc<T: EventConsumerService>(pub Arc<T>);
                    impl<
                        T: EventConsumerService,
                    > tonic::server::UnaryService<super::EventRequest> for EventSvc<T> {
                        type Response = super::EventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).event(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EventSvc(inner);
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
    impl<T: EventConsumerService> Clone for EventConsumerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: EventConsumerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EventConsumerService> tonic::server::NamedService
    for EventConsumerServiceServer<T> {
        const NAME: &'static str = "webhook_api.v1.EventConsumerService";
    }
}
