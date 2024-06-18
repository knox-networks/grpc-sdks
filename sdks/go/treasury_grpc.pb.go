// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.4.0
// - protoc             v5.27.1
// source: treasury/v1/treasury.proto

package __

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.62.0 or later.
const _ = grpc.SupportPackageIsVersion8

const (
	Treasury_GetSupply_FullMethodName        = "/treasury.Treasury/GetSupply"
	Treasury_ListTransactions_FullMethodName = "/treasury.Treasury/ListTransactions"
)

// TreasuryClient is the client API for Treasury service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
//
// The Treasury application implements value added functions that may be offered by the financial institution. APIs
// defined here are limited to those that are exclusively exposed via the Treasury service, e.g. APIs to manage currency
// supply of the treasury and APIs for remittance. Where functions are offered as part of a Transaction service, the
// APIs can be found under the Transaction Service.
type TreasuryClient interface {
	// Get a list of denomination counts of the digital banknotes held in the Treasury service's vault.
	GetSupply(ctx context.Context, in *GetSupplyRequest, opts ...grpc.CallOption) (*GetSupplyResponse, error)
	// Get a list of past contract transactions.
	ListTransactions(ctx context.Context, in *ListTransactionsRequest, opts ...grpc.CallOption) (*ListTransactionsResponse, error)
}

type treasuryClient struct {
	cc grpc.ClientConnInterface
}

func NewTreasuryClient(cc grpc.ClientConnInterface) TreasuryClient {
	return &treasuryClient{cc}
}

func (c *treasuryClient) GetSupply(ctx context.Context, in *GetSupplyRequest, opts ...grpc.CallOption) (*GetSupplyResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(GetSupplyResponse)
	err := c.cc.Invoke(ctx, Treasury_GetSupply_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *treasuryClient) ListTransactions(ctx context.Context, in *ListTransactionsRequest, opts ...grpc.CallOption) (*ListTransactionsResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(ListTransactionsResponse)
	err := c.cc.Invoke(ctx, Treasury_ListTransactions_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// TreasuryServer is the server API for Treasury service.
// All implementations must embed UnimplementedTreasuryServer
// for forward compatibility
//
// The Treasury application implements value added functions that may be offered by the financial institution. APIs
// defined here are limited to those that are exclusively exposed via the Treasury service, e.g. APIs to manage currency
// supply of the treasury and APIs for remittance. Where functions are offered as part of a Transaction service, the
// APIs can be found under the Transaction Service.
type TreasuryServer interface {
	// Get a list of denomination counts of the digital banknotes held in the Treasury service's vault.
	GetSupply(context.Context, *GetSupplyRequest) (*GetSupplyResponse, error)
	// Get a list of past contract transactions.
	ListTransactions(context.Context, *ListTransactionsRequest) (*ListTransactionsResponse, error)
	mustEmbedUnimplementedTreasuryServer()
}

// UnimplementedTreasuryServer must be embedded to have forward compatible implementations.
type UnimplementedTreasuryServer struct {
}

func (UnimplementedTreasuryServer) GetSupply(context.Context, *GetSupplyRequest) (*GetSupplyResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetSupply not implemented")
}
func (UnimplementedTreasuryServer) ListTransactions(context.Context, *ListTransactionsRequest) (*ListTransactionsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListTransactions not implemented")
}
func (UnimplementedTreasuryServer) mustEmbedUnimplementedTreasuryServer() {}

// UnsafeTreasuryServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to TreasuryServer will
// result in compilation errors.
type UnsafeTreasuryServer interface {
	mustEmbedUnimplementedTreasuryServer()
}

func RegisterTreasuryServer(s grpc.ServiceRegistrar, srv TreasuryServer) {
	s.RegisterService(&Treasury_ServiceDesc, srv)
}

func _Treasury_GetSupply_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetSupplyRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(TreasuryServer).GetSupply(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Treasury_GetSupply_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(TreasuryServer).GetSupply(ctx, req.(*GetSupplyRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Treasury_ListTransactions_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListTransactionsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(TreasuryServer).ListTransactions(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Treasury_ListTransactions_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(TreasuryServer).ListTransactions(ctx, req.(*ListTransactionsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Treasury_ServiceDesc is the grpc.ServiceDesc for Treasury service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Treasury_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "treasury.Treasury",
	HandlerType: (*TreasuryServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "GetSupply",
			Handler:    _Treasury_GetSupply_Handler,
		},
		{
			MethodName: "ListTransactions",
			Handler:    _Treasury_ListTransactions_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "treasury/v1/treasury.proto",
}
