// @generated
/// Generated client implementations.
pub mod wallet_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Transacts and holds digital banknotes and bank customer information required for financial compliance, in the form of
 a W3C Verifiable Credential (https://www.w3.org/TR/vc-data-model).  This is a server side wallet solution in lieu of
 a mobile wallet, hosted by the bank on behalf of the customer who opts in to do so. Can also be used by institutions,
 e.g. a financial institution vault.
*/
    #[derive(Debug, Clone)]
    pub struct WalletClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WalletClient<tonic::transport::Channel> {
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
    impl<T> WalletClient<T>
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
        ) -> WalletClient<InterceptedService<T, F>>
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
            WalletClient::new(InterceptedService::new(inner, interceptor))
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
                "/wallet_api.v1.Wallet/Transfer",
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
                "/wallet_api.v1.Wallet/CreateOwner",
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
                "/wallet_api.v1.Wallet/CreateWallet",
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
                "/wallet_api.v1.Wallet/ListWallets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** List the digital banknotes for a wallet.
*/
        pub async fn list_funds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFundsRequest>,
        ) -> Result<tonic::Response<super::ListFundsResponse>, tonic::Status> {
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
                "/wallet_api.v1.Wallet/ListFunds",
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
                "/wallet_api.v1.Wallet/ListWalletBalances",
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
                "/wallet_api.v1.Wallet/Redeem",
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
                "/wallet_api.v1.Wallet/ListCredentials",
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
                "/wallet_api.v1.Wallet/AddCredential",
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
                "/wallet_api.v1.Wallet/AssociateWallet",
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
                "/wallet_api.v1.Wallet/RequestPresentation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Attempts to start a `StartSimplePaymentRequest`. The call immediately returns with a `status` and `uetr` that can
 be used to query more information about the transaction.
*/
        pub async fn prepare_simple_payment(
            &mut self,
            request: impl tonic::IntoRequest<super::PrepareSimplePaymentRequest>,
        ) -> Result<
            tonic::Response<super::PrepareSimplePaymentResponse>,
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
                "/wallet_api.v1.Wallet/PrepareSimplePayment",
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
                "/wallet_api.v1.Wallet/PrepareFundsChange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Obtains metadata about a transaction.
*/
        pub async fn get_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status> {
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
                "/wallet_api.v1.Wallet/GetTransaction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** List all transactions of a given wallet.
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
                "/wallet_api.v1.Wallet/ListTransactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Obtains current snapshot of a transaction.
*/
        pub async fn get_transaction_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransactionSnapshotRequest>,
        ) -> Result<
            tonic::Response<super::GetTransactionSnapshotResponse>,
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
                "/wallet_api.v1.Wallet/GetTransactionSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /** Obtains the history of a transaction.
*/
        pub async fn list_transaction_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransactionSnapshotsRequest>,
        ) -> Result<
            tonic::Response<super::ListTransactionSnapshotsResponse>,
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
                "/wallet_api.v1.Wallet/ListTransactionSnapshots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod wallet_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with WalletServer.
    #[async_trait]
    pub trait Wallet: Send + Sync + 'static {
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
        async fn list_funds(
            &self,
            request: tonic::Request<super::ListFundsRequest>,
        ) -> Result<tonic::Response<super::ListFundsResponse>, tonic::Status>;
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
        /** Attempts to start a `StartSimplePaymentRequest`. The call immediately returns with a `status` and `uetr` that can
 be used to query more information about the transaction.
*/
        async fn prepare_simple_payment(
            &self,
            request: tonic::Request<super::PrepareSimplePaymentRequest>,
        ) -> Result<tonic::Response<super::PrepareSimplePaymentResponse>, tonic::Status>;
        /** Attempts to start a funds change request. The call immediately returns with a `status` and `uetr` that can be used
 to query more information about the transaction.
*/
        async fn prepare_funds_change(
            &self,
            request: tonic::Request<super::PrepareFundsChangeRequest>,
        ) -> Result<tonic::Response<super::PrepareFundsChangeResponse>, tonic::Status>;
        /** Obtains metadata about a transaction.
*/
        async fn get_transaction(
            &self,
            request: tonic::Request<super::GetTransactionRequest>,
        ) -> Result<tonic::Response<super::GetTransactionResponse>, tonic::Status>;
        /** List all transactions of a given wallet.
*/
        async fn list_transactions(
            &self,
            request: tonic::Request<super::ListTransactionsRequest>,
        ) -> Result<tonic::Response<super::ListTransactionsResponse>, tonic::Status>;
        /** Obtains current snapshot of a transaction.
*/
        async fn get_transaction_snapshot(
            &self,
            request: tonic::Request<super::GetTransactionSnapshotRequest>,
        ) -> Result<
            tonic::Response<super::GetTransactionSnapshotResponse>,
            tonic::Status,
        >;
        /** Obtains the history of a transaction.
*/
        async fn list_transaction_snapshots(
            &self,
            request: tonic::Request<super::ListTransactionSnapshotsRequest>,
        ) -> Result<
            tonic::Response<super::ListTransactionSnapshotsResponse>,
            tonic::Status,
        >;
    }
    /** Transacts and holds digital banknotes and bank customer information required for financial compliance, in the form of
 a W3C Verifiable Credential (https://www.w3.org/TR/vc-data-model).  This is a server side wallet solution in lieu of
 a mobile wallet, hosted by the bank on behalf of the customer who opts in to do so. Can also be used by institutions,
 e.g. a financial institution vault.
*/
    #[derive(Debug)]
    pub struct WalletServer<T: Wallet> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Wallet> WalletServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WalletServer<T>
    where
        T: Wallet,
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
                "/wallet_api.v1.Wallet/Transfer" => {
                    #[allow(non_camel_case_types)]
                    struct TransferSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::TransferRequest>
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
                "/wallet_api.v1.Wallet/CreateOwner" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOwnerSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
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
                "/wallet_api.v1.Wallet/CreateWallet" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWalletSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
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
                "/wallet_api.v1.Wallet/ListWallets" => {
                    #[allow(non_camel_case_types)]
                    struct ListWalletsSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
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
                "/wallet_api.v1.Wallet/ListFunds" => {
                    #[allow(non_camel_case_types)]
                    struct ListFundsSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::ListFundsRequest>
                    for ListFundsSvc<T> {
                        type Response = super::ListFundsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFundsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_funds(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListFundsSvc(inner);
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
                "/wallet_api.v1.Wallet/ListWalletBalances" => {
                    #[allow(non_camel_case_types)]
                    struct ListWalletBalancesSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
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
                "/wallet_api.v1.Wallet/Redeem" => {
                    #[allow(non_camel_case_types)]
                    struct RedeemSvc<T: Wallet>(pub Arc<T>);
                    impl<T: Wallet> tonic::server::UnaryService<super::RedeemRequest>
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
                "/wallet_api.v1.Wallet/ListCredentials" => {
                    #[allow(non_camel_case_types)]
                    struct ListCredentialsSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
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
                "/wallet_api.v1.Wallet/AddCredential" => {
                    #[allow(non_camel_case_types)]
                    struct AddCredentialSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
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
                "/wallet_api.v1.Wallet/AssociateWallet" => {
                    #[allow(non_camel_case_types)]
                    struct AssociateWalletSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
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
                "/wallet_api.v1.Wallet/RequestPresentation" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPresentationSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
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
                "/wallet_api.v1.Wallet/PrepareSimplePayment" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareSimplePaymentSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::PrepareSimplePaymentRequest>
                    for PrepareSimplePaymentSvc<T> {
                        type Response = super::PrepareSimplePaymentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PrepareSimplePaymentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).prepare_simple_payment(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PrepareSimplePaymentSvc(inner);
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
                "/wallet_api.v1.Wallet/PrepareFundsChange" => {
                    #[allow(non_camel_case_types)]
                    struct PrepareFundsChangeSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
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
                "/wallet_api.v1.Wallet/GetTransaction" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::GetTransactionRequest>
                    for GetTransactionSvc<T> {
                        type Response = super::GetTransactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_transaction(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionSvc(inner);
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
                "/wallet_api.v1.Wallet/ListTransactions" => {
                    #[allow(non_camel_case_types)]
                    struct ListTransactionsSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
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
                "/wallet_api.v1.Wallet/GetTransactionSnapshot" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransactionSnapshotSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::GetTransactionSnapshotRequest>
                    for GetTransactionSnapshotSvc<T> {
                        type Response = super::GetTransactionSnapshotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransactionSnapshotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_transaction_snapshot(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTransactionSnapshotSvc(inner);
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
                "/wallet_api.v1.Wallet/ListTransactionSnapshots" => {
                    #[allow(non_camel_case_types)]
                    struct ListTransactionSnapshotsSvc<T: Wallet>(pub Arc<T>);
                    impl<
                        T: Wallet,
                    > tonic::server::UnaryService<super::ListTransactionSnapshotsRequest>
                    for ListTransactionSnapshotsSvc<T> {
                        type Response = super::ListTransactionSnapshotsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListTransactionSnapshotsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_transaction_snapshots(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListTransactionSnapshotsSvc(inner);
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
    impl<T: Wallet> Clone for WalletServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Wallet> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Wallet> tonic::server::NamedService for WalletServer<T> {
        const NAME: &'static str = "wallet_api.v1.Wallet";
    }
}
