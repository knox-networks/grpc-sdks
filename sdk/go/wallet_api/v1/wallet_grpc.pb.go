// Code generated by protoc-gen-go-grpc. DO NOT EDIT.

package v1

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// WalletClient is the client API for Wallet service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type WalletClient interface {
	// Transfers digital banknotes from a custodial wallet to a given recipient public key.
	Transfer(ctx context.Context, in *TransferRequest, opts ...grpc.CallOption) (*TransferResponse, error)
	// Creates an owne and API key for access to future custodial wallet interactions.  Ex Could be a bank that may manage many customer custodial wallets.
	CreateOwner(ctx context.Context, in *CreateOwnerRequest, opts ...grpc.CallOption) (*CreateOwnerResponse, error)
	// Creates a wallet for a given owner, which can manage the wallet.
	CreateWallet(ctx context.Context, in *CreateWalletRequest, opts ...grpc.CallOption) (*CreateWalletResponse, error)
	// List all wallets for a given owner.
	ListWallets(ctx context.Context, in *ListWalletsRequest, opts ...grpc.CallOption) (*ListWalletsResponse, error)
	// List the digital banknotes for a wallet.
	ListFunds(ctx context.Context, in *ListFundsRequest, opts ...grpc.CallOption) (*ListFundsResponse, error)
	// List the balances of each currency in the wallet.
	ListWalletBalances(ctx context.Context, in *ListWalletBalancesRequest, opts ...grpc.CallOption) (*ListWalletBalancesResponse, error)
	// Sends digital banknotes from the Issuer wallet to the Authority for redemption of digital banknotes and increase issuance limit.
	Redeem(ctx context.Context, in *RedeemRequest, opts ...grpc.CallOption) (*RedeemResponse, error)
	// Lists the W3C Verifiable Credentials of the wallet subject.
	ListCredentials(ctx context.Context, in *ListCredentialsRequest, opts ...grpc.CallOption) (*ListCredentialsResponse, error)
	// Adds the specified W3C Verifiable Credential type for the wallet by calling out to the Credential Adapter. Based on the bearer JWT token provided by User Management to ensure the caller has been authenticated and authorized.
	AddCredential(ctx context.Context, in *AddCredentialRequest, opts ...grpc.CallOption) (*AddCredentialResponse, error)
	// Associates the wallet with the account specified in the bearer JWT token provided by User Management.
	AssociateWallet(ctx context.Context, in *AssociateWalletRequest, opts ...grpc.CallOption) (*AssociateWalletResponse, error)
	// Returns the JSON Verifiable Presentation containing the PII of the subject.
	RequestPresentation(ctx context.Context, in *RequestPresentationRequest, opts ...grpc.CallOption) (*RequestPresentationResponse, error)
	// Attempts to start a `StartSimplePaymentRequest`. The call immediately returns with a `status` and `uetr` that can be used to query more information about the transaction.
	PrepareSimplePayment(ctx context.Context, in *PrepareSimplePaymentRequest, opts ...grpc.CallOption) (*PrepareSimplePaymentResponse, error)
	// Attempts to start a funds change request. The call immediately returns with a `status` and `uetr` that can be used to query more information about the transaction.
	PrepareFundsChange(ctx context.Context, in *PrepareFundsChangeRequest, opts ...grpc.CallOption) (*PrepareFundsChangeResponse, error)
	// Obtains metadata about a transaction.
	GetTransaction(ctx context.Context, in *GetTransactionRequest, opts ...grpc.CallOption) (*GetTransactionResponse, error)
	// List all transactions of a given wallet.
	ListTransactions(ctx context.Context, in *ListTransactionsRequest, opts ...grpc.CallOption) (*ListTransactionsResponse, error)
	// Obtains current snapshot of a transaction.
	GetTransactionSnapshot(ctx context.Context, in *GetTransactionSnapshotRequest, opts ...grpc.CallOption) (*GetTransactionSnapshotResponse, error)
	// Obtains the history of a transaction.
	ListTransactionSnapshots(ctx context.Context, in *ListTransactionSnapshotsRequest, opts ...grpc.CallOption) (*ListTransactionSnapshotsResponse, error)
}

type walletClient struct {
	cc grpc.ClientConnInterface
}

func NewWalletClient(cc grpc.ClientConnInterface) WalletClient {
	return &walletClient{cc}
}

func (c *walletClient) Transfer(ctx context.Context, in *TransferRequest, opts ...grpc.CallOption) (*TransferResponse, error) {
	out := new(TransferResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/Transfer", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) CreateOwner(ctx context.Context, in *CreateOwnerRequest, opts ...grpc.CallOption) (*CreateOwnerResponse, error) {
	out := new(CreateOwnerResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/CreateOwner", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) CreateWallet(ctx context.Context, in *CreateWalletRequest, opts ...grpc.CallOption) (*CreateWalletResponse, error) {
	out := new(CreateWalletResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/CreateWallet", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) ListWallets(ctx context.Context, in *ListWalletsRequest, opts ...grpc.CallOption) (*ListWalletsResponse, error) {
	out := new(ListWalletsResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/ListWallets", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) ListFunds(ctx context.Context, in *ListFundsRequest, opts ...grpc.CallOption) (*ListFundsResponse, error) {
	out := new(ListFundsResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/ListFunds", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) ListWalletBalances(ctx context.Context, in *ListWalletBalancesRequest, opts ...grpc.CallOption) (*ListWalletBalancesResponse, error) {
	out := new(ListWalletBalancesResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/ListWalletBalances", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) Redeem(ctx context.Context, in *RedeemRequest, opts ...grpc.CallOption) (*RedeemResponse, error) {
	out := new(RedeemResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/Redeem", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) ListCredentials(ctx context.Context, in *ListCredentialsRequest, opts ...grpc.CallOption) (*ListCredentialsResponse, error) {
	out := new(ListCredentialsResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/ListCredentials", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) AddCredential(ctx context.Context, in *AddCredentialRequest, opts ...grpc.CallOption) (*AddCredentialResponse, error) {
	out := new(AddCredentialResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/AddCredential", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) AssociateWallet(ctx context.Context, in *AssociateWalletRequest, opts ...grpc.CallOption) (*AssociateWalletResponse, error) {
	out := new(AssociateWalletResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/AssociateWallet", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) RequestPresentation(ctx context.Context, in *RequestPresentationRequest, opts ...grpc.CallOption) (*RequestPresentationResponse, error) {
	out := new(RequestPresentationResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/RequestPresentation", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) PrepareSimplePayment(ctx context.Context, in *PrepareSimplePaymentRequest, opts ...grpc.CallOption) (*PrepareSimplePaymentResponse, error) {
	out := new(PrepareSimplePaymentResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/PrepareSimplePayment", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) PrepareFundsChange(ctx context.Context, in *PrepareFundsChangeRequest, opts ...grpc.CallOption) (*PrepareFundsChangeResponse, error) {
	out := new(PrepareFundsChangeResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/PrepareFundsChange", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) GetTransaction(ctx context.Context, in *GetTransactionRequest, opts ...grpc.CallOption) (*GetTransactionResponse, error) {
	out := new(GetTransactionResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/GetTransaction", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) ListTransactions(ctx context.Context, in *ListTransactionsRequest, opts ...grpc.CallOption) (*ListTransactionsResponse, error) {
	out := new(ListTransactionsResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/ListTransactions", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) GetTransactionSnapshot(ctx context.Context, in *GetTransactionSnapshotRequest, opts ...grpc.CallOption) (*GetTransactionSnapshotResponse, error) {
	out := new(GetTransactionSnapshotResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/GetTransactionSnapshot", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *walletClient) ListTransactionSnapshots(ctx context.Context, in *ListTransactionSnapshotsRequest, opts ...grpc.CallOption) (*ListTransactionSnapshotsResponse, error) {
	out := new(ListTransactionSnapshotsResponse)
	err := c.cc.Invoke(ctx, "/wallet_api.v1.Wallet/ListTransactionSnapshots", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// WalletServer is the server API for Wallet service.
// All implementations must embed UnimplementedWalletServer
// for forward compatibility
type WalletServer interface {
	// Transfers digital banknotes from a custodial wallet to a given recipient public key.
	Transfer(context.Context, *TransferRequest) (*TransferResponse, error)
	// Creates an owne and API key for access to future custodial wallet interactions.  Ex Could be a bank that may manage many customer custodial wallets.
	CreateOwner(context.Context, *CreateOwnerRequest) (*CreateOwnerResponse, error)
	// Creates a wallet for a given owner, which can manage the wallet.
	CreateWallet(context.Context, *CreateWalletRequest) (*CreateWalletResponse, error)
	// List all wallets for a given owner.
	ListWallets(context.Context, *ListWalletsRequest) (*ListWalletsResponse, error)
	// List the digital banknotes for a wallet.
	ListFunds(context.Context, *ListFundsRequest) (*ListFundsResponse, error)
	// List the balances of each currency in the wallet.
	ListWalletBalances(context.Context, *ListWalletBalancesRequest) (*ListWalletBalancesResponse, error)
	// Sends digital banknotes from the Issuer wallet to the Authority for redemption of digital banknotes and increase issuance limit.
	Redeem(context.Context, *RedeemRequest) (*RedeemResponse, error)
	// Lists the W3C Verifiable Credentials of the wallet subject.
	ListCredentials(context.Context, *ListCredentialsRequest) (*ListCredentialsResponse, error)
	// Adds the specified W3C Verifiable Credential type for the wallet by calling out to the Credential Adapter. Based on the bearer JWT token provided by User Management to ensure the caller has been authenticated and authorized.
	AddCredential(context.Context, *AddCredentialRequest) (*AddCredentialResponse, error)
	// Associates the wallet with the account specified in the bearer JWT token provided by User Management.
	AssociateWallet(context.Context, *AssociateWalletRequest) (*AssociateWalletResponse, error)
	// Returns the JSON Verifiable Presentation containing the PII of the subject.
	RequestPresentation(context.Context, *RequestPresentationRequest) (*RequestPresentationResponse, error)
	// Attempts to start a `StartSimplePaymentRequest`. The call immediately returns with a `status` and `uetr` that can be used to query more information about the transaction.
	PrepareSimplePayment(context.Context, *PrepareSimplePaymentRequest) (*PrepareSimplePaymentResponse, error)
	// Attempts to start a funds change request. The call immediately returns with a `status` and `uetr` that can be used to query more information about the transaction.
	PrepareFundsChange(context.Context, *PrepareFundsChangeRequest) (*PrepareFundsChangeResponse, error)
	// Obtains metadata about a transaction.
	GetTransaction(context.Context, *GetTransactionRequest) (*GetTransactionResponse, error)
	// List all transactions of a given wallet.
	ListTransactions(context.Context, *ListTransactionsRequest) (*ListTransactionsResponse, error)
	// Obtains current snapshot of a transaction.
	GetTransactionSnapshot(context.Context, *GetTransactionSnapshotRequest) (*GetTransactionSnapshotResponse, error)
	// Obtains the history of a transaction.
	ListTransactionSnapshots(context.Context, *ListTransactionSnapshotsRequest) (*ListTransactionSnapshotsResponse, error)
	mustEmbedUnimplementedWalletServer()
}

// UnimplementedWalletServer must be embedded to have forward compatible implementations.
type UnimplementedWalletServer struct {
}

func (UnimplementedWalletServer) Transfer(context.Context, *TransferRequest) (*TransferResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Transfer not implemented")
}
func (UnimplementedWalletServer) CreateOwner(context.Context, *CreateOwnerRequest) (*CreateOwnerResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CreateOwner not implemented")
}
func (UnimplementedWalletServer) CreateWallet(context.Context, *CreateWalletRequest) (*CreateWalletResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CreateWallet not implemented")
}
func (UnimplementedWalletServer) ListWallets(context.Context, *ListWalletsRequest) (*ListWalletsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListWallets not implemented")
}
func (UnimplementedWalletServer) ListFunds(context.Context, *ListFundsRequest) (*ListFundsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListFunds not implemented")
}
func (UnimplementedWalletServer) ListWalletBalances(context.Context, *ListWalletBalancesRequest) (*ListWalletBalancesResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListWalletBalances not implemented")
}
func (UnimplementedWalletServer) Redeem(context.Context, *RedeemRequest) (*RedeemResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Redeem not implemented")
}
func (UnimplementedWalletServer) ListCredentials(context.Context, *ListCredentialsRequest) (*ListCredentialsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListCredentials not implemented")
}
func (UnimplementedWalletServer) AddCredential(context.Context, *AddCredentialRequest) (*AddCredentialResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AddCredential not implemented")
}
func (UnimplementedWalletServer) AssociateWallet(context.Context, *AssociateWalletRequest) (*AssociateWalletResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AssociateWallet not implemented")
}
func (UnimplementedWalletServer) RequestPresentation(context.Context, *RequestPresentationRequest) (*RequestPresentationResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method RequestPresentation not implemented")
}
func (UnimplementedWalletServer) PrepareSimplePayment(context.Context, *PrepareSimplePaymentRequest) (*PrepareSimplePaymentResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method PrepareSimplePayment not implemented")
}
func (UnimplementedWalletServer) PrepareFundsChange(context.Context, *PrepareFundsChangeRequest) (*PrepareFundsChangeResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method PrepareFundsChange not implemented")
}
func (UnimplementedWalletServer) GetTransaction(context.Context, *GetTransactionRequest) (*GetTransactionResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetTransaction not implemented")
}
func (UnimplementedWalletServer) ListTransactions(context.Context, *ListTransactionsRequest) (*ListTransactionsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListTransactions not implemented")
}
func (UnimplementedWalletServer) GetTransactionSnapshot(context.Context, *GetTransactionSnapshotRequest) (*GetTransactionSnapshotResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetTransactionSnapshot not implemented")
}
func (UnimplementedWalletServer) ListTransactionSnapshots(context.Context, *ListTransactionSnapshotsRequest) (*ListTransactionSnapshotsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListTransactionSnapshots not implemented")
}
func (UnimplementedWalletServer) mustEmbedUnimplementedWalletServer() {}

// UnsafeWalletServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to WalletServer will
// result in compilation errors.
type UnsafeWalletServer interface {
	mustEmbedUnimplementedWalletServer()
}

func RegisterWalletServer(s grpc.ServiceRegistrar, srv WalletServer) {
	s.RegisterService(&Wallet_ServiceDesc, srv)
}

func _Wallet_Transfer_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(TransferRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).Transfer(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/Transfer",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).Transfer(ctx, req.(*TransferRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_CreateOwner_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CreateOwnerRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).CreateOwner(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/CreateOwner",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).CreateOwner(ctx, req.(*CreateOwnerRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_CreateWallet_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CreateWalletRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).CreateWallet(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/CreateWallet",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).CreateWallet(ctx, req.(*CreateWalletRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_ListWallets_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListWalletsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).ListWallets(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/ListWallets",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).ListWallets(ctx, req.(*ListWalletsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_ListFunds_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListFundsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).ListFunds(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/ListFunds",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).ListFunds(ctx, req.(*ListFundsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_ListWalletBalances_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListWalletBalancesRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).ListWalletBalances(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/ListWalletBalances",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).ListWalletBalances(ctx, req.(*ListWalletBalancesRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_Redeem_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RedeemRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).Redeem(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/Redeem",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).Redeem(ctx, req.(*RedeemRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_ListCredentials_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListCredentialsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).ListCredentials(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/ListCredentials",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).ListCredentials(ctx, req.(*ListCredentialsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_AddCredential_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AddCredentialRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).AddCredential(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/AddCredential",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).AddCredential(ctx, req.(*AddCredentialRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_AssociateWallet_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AssociateWalletRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).AssociateWallet(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/AssociateWallet",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).AssociateWallet(ctx, req.(*AssociateWalletRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_RequestPresentation_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RequestPresentationRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).RequestPresentation(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/RequestPresentation",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).RequestPresentation(ctx, req.(*RequestPresentationRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_PrepareSimplePayment_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PrepareSimplePaymentRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).PrepareSimplePayment(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/PrepareSimplePayment",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).PrepareSimplePayment(ctx, req.(*PrepareSimplePaymentRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_PrepareFundsChange_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PrepareFundsChangeRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).PrepareFundsChange(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/PrepareFundsChange",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).PrepareFundsChange(ctx, req.(*PrepareFundsChangeRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_GetTransaction_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetTransactionRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).GetTransaction(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/GetTransaction",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).GetTransaction(ctx, req.(*GetTransactionRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_ListTransactions_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListTransactionsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).ListTransactions(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/ListTransactions",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).ListTransactions(ctx, req.(*ListTransactionsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_GetTransactionSnapshot_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetTransactionSnapshotRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).GetTransactionSnapshot(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/GetTransactionSnapshot",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).GetTransactionSnapshot(ctx, req.(*GetTransactionSnapshotRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Wallet_ListTransactionSnapshots_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListTransactionSnapshotsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WalletServer).ListTransactionSnapshots(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/wallet_api.v1.Wallet/ListTransactionSnapshots",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WalletServer).ListTransactionSnapshots(ctx, req.(*ListTransactionSnapshotsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Wallet_ServiceDesc is the grpc.ServiceDesc for Wallet service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Wallet_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "wallet_api.v1.Wallet",
	HandlerType: (*WalletServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Transfer",
			Handler:    _Wallet_Transfer_Handler,
		},
		{
			MethodName: "CreateOwner",
			Handler:    _Wallet_CreateOwner_Handler,
		},
		{
			MethodName: "CreateWallet",
			Handler:    _Wallet_CreateWallet_Handler,
		},
		{
			MethodName: "ListWallets",
			Handler:    _Wallet_ListWallets_Handler,
		},
		{
			MethodName: "ListFunds",
			Handler:    _Wallet_ListFunds_Handler,
		},
		{
			MethodName: "ListWalletBalances",
			Handler:    _Wallet_ListWalletBalances_Handler,
		},
		{
			MethodName: "Redeem",
			Handler:    _Wallet_Redeem_Handler,
		},
		{
			MethodName: "ListCredentials",
			Handler:    _Wallet_ListCredentials_Handler,
		},
		{
			MethodName: "AddCredential",
			Handler:    _Wallet_AddCredential_Handler,
		},
		{
			MethodName: "AssociateWallet",
			Handler:    _Wallet_AssociateWallet_Handler,
		},
		{
			MethodName: "RequestPresentation",
			Handler:    _Wallet_RequestPresentation_Handler,
		},
		{
			MethodName: "PrepareSimplePayment",
			Handler:    _Wallet_PrepareSimplePayment_Handler,
		},
		{
			MethodName: "PrepareFundsChange",
			Handler:    _Wallet_PrepareFundsChange_Handler,
		},
		{
			MethodName: "GetTransaction",
			Handler:    _Wallet_GetTransaction_Handler,
		},
		{
			MethodName: "ListTransactions",
			Handler:    _Wallet_ListTransactions_Handler,
		},
		{
			MethodName: "GetTransactionSnapshot",
			Handler:    _Wallet_GetTransactionSnapshot_Handler,
		},
		{
			MethodName: "ListTransactionSnapshots",
			Handler:    _Wallet_ListTransactionSnapshots_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "wallet_api/v1/wallet.proto",
}
