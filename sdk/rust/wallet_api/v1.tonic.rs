// @generated
/// Generated client implementations.
pub mod wallet_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Transacts and holds digital banknotes and bank customer information required for financial compliance, in the form of
 a W3C Verifiable Credential (https://www.w3.org/TR/vc-data-model).  This is a server side wallet solution in lieu of
 a mobile wallet, hosted by the bank on behalf of the customer who opts in to do so. Can also be used by institutions,
 e.g. a financial institution vault.
*/
    #[derive(Debug, Clone)]
    pub struct WalletServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletServiceClient<tonic::transport::Channel> {
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
    impl<T> WalletServiceClient<T>
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
        ) -> WalletServiceClient<InterceptedService<T, F>>
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
            WalletServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /** Transfers digital banknotes from a custodial wallet to a given recipient public key.
*/
        pub async fn transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferRequest>,
        ) -> Result<tonic::Response<super::TransferResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/Transfer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Creates an owned and API key for access to future custodial wallet interactions.  Ex Could be a bank that may
 manage many customer custodial wallets.
*/
        pub async fn create_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOwnerRequest>,
        ) -> Result<tonic::Response<super::CreateOwnerResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/CreateOwner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Creates a wallet for a given owner, which can manage the wallet.
*/
        pub async fn create_wallet(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWalletRequest>,
        ) -> Result<tonic::Response<super::CreateWalletResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/CreateWallet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** List all wallets for a given owner.
*/
        pub async fn list_wallets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWalletsRequest>,
        ) -> Result<tonic::Response<super::ListWalletsResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/ListWallets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** List the digital banknotes for a wallet.
*/
        pub async fn list_asset_files(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetFilesRequest>,
        ) -> Result<tonic::Response<super::ListAssetFilesResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/ListAssetFiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** List the balances of each currency in the wallet.
*/
        pub async fn list_wallet_balances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWalletBalancesRequest>,
        ) -> Result<tonic::Response<super::ListWalletBalancesResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/ListWalletBalances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Sends digital banknotes from the Issuer wallet to the Authority for redemption of digital banknotes and increase
 issuance limit.
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
                "/wallet_api.v1.WalletService/Redeem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Lists the W3C Verifiable Credentials of the wallet subject.
*/
        pub async fn list_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCredentialsRequest>,
        ) -> Result<tonic::Response<super::ListCredentialsResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/ListCredentials",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Adds the specified W3C Verifiable Credential type for the wallet by calling out to the Credential Adapter. Based on
 the bearer JWT token provided by User Management to ensure the caller has been authenticated and authorized.
*/
        pub async fn add_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCredentialRequest>,
        ) -> Result<tonic::Response<super::AddCredentialResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/AddCredential",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Associates the wallet with the account specified in the bearer JWT token provided by User Management.
*/
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
                "/wallet_api.v1.WalletService/AssociateWallet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Returns the JSON Verifiable Presentation containing the PII of the subject.
*/
        pub async fn request_presentation(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestPresentationRequest>,
        ) -> Result<tonic::Response<super::RequestPresentationResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/RequestPresentation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Lists more details about balances in a wallet such as how much funds are available vs reserved.
*/
        pub async fn list_wallet_balance_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWalletBalanceDetailsRequest>,
        ) -> Result<
            tonic::Response<super::ListWalletBalanceDetailsResponse>,
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
                "/wallet_api.v1.WalletService/ListWalletBalanceDetails",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** List transactions
*/
        pub async fn list_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransactionsRequest>,
        ) -> Result<tonic::Response<super::ListTransactionsResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/ListTransactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Get detailed information about a transaction.
*/
        pub async fn get_transaction_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionDetailRequest>,
        ) -> Result<
            tonic::Response<super::GetTransactionDetailResponse>,
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
                "/wallet_api.v1.WalletService/GetTransactionDetail",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Create a new transaction
*/
        pub async fn prepare_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::PrepareTransactionRequest>,
        ) -> Result<tonic::Response<super::PrepareTransactionResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/PrepareTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Attempts to start an atomic asset swap. The call immediately returns with a `status` and `uetr` that can
 be used to query more information about the transaction.
*/
        pub async fn prepare_atomic_asset_swap(
            &mut self,
            request: impl tonic::IntoRequest<super::PrepareAtomicAssetSwapRequest>,
        ) -> Result<
            tonic::Response<super::PrepareAtomicAssetSwapResponse>,
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
                "/wallet_api.v1.WalletService/PrepareAtomicAssetSwap",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Attempts to start a cross border payment. The call immediately returns with a `status` and `uetr` that can
 be used to query more information about the transaction.
*/
        pub async fn prepare_cross_border_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::PrepareCrossBorderPaymentRequest>,
        ) -> Result<
            tonic::Response<super::PrepareCrossBorderPaymentResponse>,
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
                "/wallet_api.v1.WalletService/PrepareCrossBorderPayment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Attempts to start a funds change request. The call immediately returns with a `status` and `uetr` that can be used
 to query more information about the transaction.
*/
        pub async fn prepare_funds_change(
            &mut self,
            request: impl tonic::IntoRequest<super::PrepareFundsChangeRequest>,
        ) -> Result<tonic::Response<super::PrepareFundsChangeResponse>, tonic::Status> {
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
                "/wallet_api.v1.WalletService/PrepareFundsChange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod wallet_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with WalletServiceServer.
    #[async_trait]
    pub trait WalletService: Send + Sync + 'static {
        /** Transfers digital banknotes from a custodial wallet to a given recipient public key.
*/
        async fn transfer(
            &self,
            request: tonic::Request<super::TransferRequest>,
        ) -> Result<tonic::Response<super::TransferResponse>, tonic::Status>;
        /** Creates an owned and API key for access to future custodial wallet interactions.  Ex Could be a bank that may
 manage many customer custodial wallets.
*/
        async fn create_owner(
            &self,
            request: tonic::Request<super::CreateOwnerRequest>,
        ) -> Result<tonic::Response<super::CreateOwnerResponse>, tonic::Status>;
        /** Creates a wallet for a given owner, which can manage the wallet.
*/
        async fn create_wallet(
            &self,
            request: tonic::Request<super::CreateWalletRequest>,
        ) -> Result<tonic::Response<super::CreateWalletResponse>, tonic::Status>;
        /** List all wallets for a given owner.
*/
        async fn list_wallets(
            &self,
            request: tonic::Request<super::ListWalletsRequest>,
        ) -> Result<tonic::Response<super::ListWalletsResponse>, tonic::Status>;
        /** List the digital banknotes for a wallet.
*/
        async fn list_asset_files(
            &self,
            request: tonic::Request<super::ListAssetFilesRequest>,
        ) -> Result<tonic::Response<super::ListAssetFilesResponse>, tonic::Status>;
        /** List the balances of each currency in the wallet.
*/
        async fn list_wallet_balances(
            &self,
            request: tonic::Request<super::ListWalletBalancesRequest>,
        ) -> Result<tonic::Response<super::ListWalletBalancesResponse>, tonic::Status>;
        /** Sends digital banknotes from the Issuer wallet to the Authority for redemption of digital banknotes and increase
 issuance limit.
*/
        async fn redeem(
            &self,
            request: tonic::Request<super::RedeemRequest>,
        ) -> Result<tonic::Response<super::RedeemResponse>, tonic::Status>;
        /** Lists the W3C Verifiable Credentials of the wallet subject.
*/
        async fn list_credentials(
            &self,
            request: tonic::Request<super::ListCredentialsRequest>,
        ) -> Result<tonic::Response<super::ListCredentialsResponse>, tonic::Status>;
        /** Adds the specified W3C Verifiable Credential type for the wallet by calling out to the Credential Adapter. Based on
 the bearer JWT token provided by User Management to ensure the caller has been authenticated and authorized.
*/
        async fn add_credential(
            &self,
            request: tonic::Request<super::AddCredentialRequest>,
        ) -> Result<tonic::Response<super::AddCredentialResponse>, tonic::Status>;
        /** Associates the wallet with the account specified in the bearer JWT token provided by User Management.
*/
        async fn associate_wallet(
            &self,
            request: tonic::Request<super::AssociateWalletRequest>,
        ) -> Result<tonic::Response<super::AssociateWalletResponse>, tonic::Status>;
        /** Returns the JSON Verifiable Presentation containing the PII of the subject.
*/
        async fn request_presentation(
            &self,
            request: tonic::Request<super::RequestPresentationRequest>,
        ) -> Result<tonic::Response<super::RequestPresentationResponse>, tonic::Status>;
        /** Lists more details about balances in a wallet such as how much funds are available vs reserved.
*/
        async fn list_wallet_balance_details(
            &self,
            request: tonic::Request<super::ListWalletBalanceDetailsRequest>,
        ) -> Result<
            tonic::Response<super::ListWalletBalanceDetailsResponse>,
            tonic::Status,
        >;
        /** List transactions
*/
        async fn list_transactions(
            &self,
            request: tonic::Request<super::ListTransactionsRequest>,
        ) -> Result<tonic::Response<super::ListTransactionsResponse>, tonic::Status>;
        /** Get detailed information about a transaction.
*/
        async fn get_transaction_detail(
            &self,
            request: tonic::Request<super::GetTransactionDetailRequest>,
        ) -> Result<tonic::Response<super::GetTransactionDetailResponse>, tonic::Status>;
        /** Create a new transaction
*/
        async fn prepare_transaction(
            &self,
            request: tonic::Request<super::PrepareTransactionRequest>,
        ) -> Result<tonic::Response<super::PrepareTransactionResponse>, tonic::Status>;
        /** Attempts to start an atomic asset swap. The call immediately returns with a `status` and `uetr` that can
 be used to query more information about the transaction.
*/
        async fn prepare_atomic_asset_swap(
            &self,
            request: tonic::Request<super::PrepareAtomicAssetSwapRequest>,
        ) -> Result<
            tonic::Response<super::PrepareAtomicAssetSwapResponse>,
            tonic::Status,
        >;
        /** Attempts to start a cross border payment. The call immediately returns with a `status` and `uetr` that can
 be used to query more information about the transaction.
*/
        async fn prepare_cross_border_payment(
            &self,
            request: tonic::Request<super::PrepareCrossBorderPaymentRequest>,
        ) -> Result<
            tonic::Response<super::PrepareCrossBorderPaymentResponse>,
            tonic::Status,
        >;
        /** Attempts to start a funds change request. The call immediately returns with a `status` and `uetr` that can be used
 to query more information about the transaction.
*/
        async fn prepare_funds_change(
            &self,
            request: tonic::Request<super::PrepareFundsChangeRequest>,
        ) -> Result<tonic::Response<super::PrepareFundsChangeResponse>, tonic::Status>;
    }
    /** Transacts and holds digital banknotes and bank customer information required for financial compliance, in the form of
 a W3C Verifiable Credential (https://www.w3.org/TR/vc-data-model).  This is a server side wallet solution in lieu of
 a mobile wallet, hosted by the bank on behalf of the customer who opts in to do so. Can also be used by institutions,
 e.g. a financial institution vault.
*/
    #[derive(Debug)]
    pub struct WalletServiceServer<T: WalletService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WalletService> WalletServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WalletServiceServer<T>
    where
        T: WalletService,
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
                "/wallet_api.v1.WalletService/Transfer" => {
                    #[allow(non_camel_case_types)]
                    struct TransferSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::TransferRequest>
                    for TransferSvc<T> {
                        type Response = super::TransferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).transfer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferSvc(inner);
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
                "/wallet_api.v1.WalletService/CreateOwner" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOwnerSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::CreateOwnerRequest>
                    for CreateOwnerSvc<T> {
                        type Response = super::CreateOwnerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOwnerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_owner(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateOwnerSvc(inner);
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
                "/wallet_api.v1.WalletService/CreateWallet" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWalletSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::CreateWalletRequest>
                    for CreateWalletSvc<T> {
                        type Response = super::CreateWalletResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWalletRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_wallet(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateWalletSvc(inner);
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
                "/wallet_api.v1.WalletService/ListWallets" => {
                    #[allow(non_camel_case_types)]
                    struct ListWalletsSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::ListWalletsRequest>
                    for ListWalletsSvc<T> {
                        type Response = super::ListWalletsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListWalletsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_wallets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListWalletsSvc(inner);
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
                "/wallet_api.v1.WalletService/ListAssetFiles" => {
                    #[allow(non_camel_case_types)]
                    struct ListAssetFilesSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::ListAssetFilesRequest>
                    for ListAssetFilesSvc<T> {
                        type Response = super::ListAssetFilesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAssetFilesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_asset_files(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListAssetFilesSvc(inner);
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
                "/wallet_api.v1.WalletService/ListWalletBalances" => {
                    #[allow(non_camel_case_types)]
                    struct ListWalletBalancesSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::ListWalletBalancesRequest>
                    for ListWalletBalancesSvc<T> {
                        type Response = super::ListWalletBalancesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListWalletBalancesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_wallet_balances(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListWalletBalancesSvc(inner);
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
                "/wallet_api.v1.WalletService/Redeem" => {
                    #[allow(non_camel_case_types)]
                    struct RedeemSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
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
                "/wallet_api.v1.WalletService/ListCredentials" => {
                    #[allow(non_camel_case_types)]
                    struct ListCredentialsSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::ListCredentialsRequest>
                    for ListCredentialsSvc<T> {
                        type Response = super::ListCredentialsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCredentialsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_credentials(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListCredentialsSvc(inner);
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
                "/wallet_api.v1.WalletService/AddCredential" => {
                    #[allow(non_camel_case_types)]
                    struct AddCredentialSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::AddCredentialRequest>
                    for AddCredentialSvc<T> {
                        type Response = super::AddCredentialResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddCredentialRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_credential(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddCredentialSvc(inner);
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
                "/wallet_api.v1.WalletService/AssociateWallet" => {
                    #[allow(non_camel_case_types)]
                    struct AssociateWalletSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
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
                "/wallet_api.v1.WalletService/RequestPresentation" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPresentationSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::RequestPresentationRequest>
                    for RequestPresentationSvc<T> {
                        type Response = super::RequestPresentationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestPresentationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).request_presentation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RequestPresentationSvc(inner);
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
                "/wallet_api.v1.WalletService/ListWalletBalanceDetails" => {
                    #[allow(non_camel_case_types)]
                    struct ListWalletBalanceDetailsSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::ListWalletBalanceDetailsRequest>
                    for ListWalletBalanceDetailsSvc<T> {
                        type Response = super::ListWalletBalanceDetailsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListWalletBalanceDetailsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_wallet_balance_details(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListWalletBalanceDetailsSvc(inner);
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
                "/wallet_api.v1.WalletService/ListTransactions" => {
                    #[allow(non_camel_case_types)]
                    struct ListTransactionsSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::ListTransactionsRequest>
                    for ListTransactionsSvc<T> {
                        type Response = super::ListTransactionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTransactionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_transactions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListTransactionsSvc(inner);
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
                "/wallet_api.v1.WalletService/GetTransactionDetail" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionDetailSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::GetTransactionDetailRequest>
                    for GetTransactionDetailSvc<T> {
                        type Response = super::GetTransactionDetailResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionDetailRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_transaction_detail(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionDetailSvc(inner);
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
                "/wallet_api.v1.WalletService/PrepareTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareTransactionSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::PrepareTransactionRequest>
                    for PrepareTransactionSvc<T> {
                        type Response = super::PrepareTransactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PrepareTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).prepare_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrepareTransactionSvc(inner);
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
                "/wallet_api.v1.WalletService/PrepareAtomicAssetSwap" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareAtomicAssetSwapSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::PrepareAtomicAssetSwapRequest>
                    for PrepareAtomicAssetSwapSvc<T> {
                        type Response = super::PrepareAtomicAssetSwapResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PrepareAtomicAssetSwapRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).prepare_atomic_asset_swap(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrepareAtomicAssetSwapSvc(inner);
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
                "/wallet_api.v1.WalletService/PrepareCrossBorderPayment" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareCrossBorderPaymentSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<
                        super::PrepareCrossBorderPaymentRequest,
                    > for PrepareCrossBorderPaymentSvc<T> {
                        type Response = super::PrepareCrossBorderPaymentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::PrepareCrossBorderPaymentRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).prepare_cross_border_payment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrepareCrossBorderPaymentSvc(inner);
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
                "/wallet_api.v1.WalletService/PrepareFundsChange" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareFundsChangeSvc<T: WalletService>(pub Arc<T>);
                    impl<
                        T: WalletService,
                    > tonic::server::UnaryService<super::PrepareFundsChangeRequest>
                    for PrepareFundsChangeSvc<T> {
                        type Response = super::PrepareFundsChangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PrepareFundsChangeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).prepare_funds_change(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrepareFundsChangeSvc(inner);
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
    impl<T: WalletService> Clone for WalletServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: WalletService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WalletService> tonic::server::NamedService for WalletServiceServer<T> {
        const NAME: &'static str = "wallet_api.v1.WalletService";
    }
}
