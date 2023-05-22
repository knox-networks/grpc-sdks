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

// MonetaeTreasuryClient is the client API for MonetaeTreasury service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type MonetaeTreasuryClient interface {
	// Start a new remittance, passing in a streamed request and gets back a streamed response.
	Remit(ctx context.Context, opts ...grpc.CallOption) (MonetaeTreasury_RemitClient, error)
	// Get a list of denomination counts of the digital banknotes held in the Treasury service's vault.
	GetSupply(ctx context.Context, in *GetSupplyRequest, opts ...grpc.CallOption) (*GetSupplyResponse, error)
}

type monetaeTreasuryClient struct {
	cc grpc.ClientConnInterface
}

func NewMonetaeTreasuryClient(cc grpc.ClientConnInterface) MonetaeTreasuryClient {
	return &monetaeTreasuryClient{cc}
}

func (c *monetaeTreasuryClient) Remit(ctx context.Context, opts ...grpc.CallOption) (MonetaeTreasury_RemitClient, error) {
	stream, err := c.cc.NewStream(ctx, &MonetaeTreasury_ServiceDesc.Streams[0], "/treasury.MonetaeTreasury/Remit", opts...)
	if err != nil {
		return nil, err
	}
	x := &monetaeTreasuryRemitClient{stream}
	return x, nil
}

type MonetaeTreasury_RemitClient interface {
	Send(*RemittanceRequest) error
	Recv() (*RemittanceResponse, error)
	grpc.ClientStream
}

type monetaeTreasuryRemitClient struct {
	grpc.ClientStream
}

func (x *monetaeTreasuryRemitClient) Send(m *RemittanceRequest) error {
	return x.ClientStream.SendMsg(m)
}

func (x *monetaeTreasuryRemitClient) Recv() (*RemittanceResponse, error) {
	m := new(RemittanceResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func (c *monetaeTreasuryClient) GetSupply(ctx context.Context, in *GetSupplyRequest, opts ...grpc.CallOption) (*GetSupplyResponse, error) {
	out := new(GetSupplyResponse)
	err := c.cc.Invoke(ctx, "/treasury.MonetaeTreasury/GetSupply", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// MonetaeTreasuryServer is the server API for MonetaeTreasury service.
// All implementations must embed UnimplementedMonetaeTreasuryServer
// for forward compatibility
type MonetaeTreasuryServer interface {
	// Start a new remittance, passing in a streamed request and gets back a streamed response.
	Remit(MonetaeTreasury_RemitServer) error
	// Get a list of denomination counts of the digital banknotes held in the Treasury service's vault.
	GetSupply(context.Context, *GetSupplyRequest) (*GetSupplyResponse, error)
	mustEmbedUnimplementedMonetaeTreasuryServer()
}

// UnimplementedMonetaeTreasuryServer must be embedded to have forward compatible implementations.
type UnimplementedMonetaeTreasuryServer struct {
}

func (UnimplementedMonetaeTreasuryServer) Remit(MonetaeTreasury_RemitServer) error {
	return status.Errorf(codes.Unimplemented, "method Remit not implemented")
}
func (UnimplementedMonetaeTreasuryServer) GetSupply(context.Context, *GetSupplyRequest) (*GetSupplyResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetSupply not implemented")
}
func (UnimplementedMonetaeTreasuryServer) mustEmbedUnimplementedMonetaeTreasuryServer() {}

// UnsafeMonetaeTreasuryServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to MonetaeTreasuryServer will
// result in compilation errors.
type UnsafeMonetaeTreasuryServer interface {
	mustEmbedUnimplementedMonetaeTreasuryServer()
}

func RegisterMonetaeTreasuryServer(s grpc.ServiceRegistrar, srv MonetaeTreasuryServer) {
	s.RegisterService(&MonetaeTreasury_ServiceDesc, srv)
}

func _MonetaeTreasury_Remit_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(MonetaeTreasuryServer).Remit(&monetaeTreasuryRemitServer{stream})
}

type MonetaeTreasury_RemitServer interface {
	Send(*RemittanceResponse) error
	Recv() (*RemittanceRequest, error)
	grpc.ServerStream
}

type monetaeTreasuryRemitServer struct {
	grpc.ServerStream
}

func (x *monetaeTreasuryRemitServer) Send(m *RemittanceResponse) error {
	return x.ServerStream.SendMsg(m)
}

func (x *monetaeTreasuryRemitServer) Recv() (*RemittanceRequest, error) {
	m := new(RemittanceRequest)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func _MonetaeTreasury_GetSupply_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetSupplyRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MonetaeTreasuryServer).GetSupply(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/treasury.MonetaeTreasury/GetSupply",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MonetaeTreasuryServer).GetSupply(ctx, req.(*GetSupplyRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// MonetaeTreasury_ServiceDesc is the grpc.ServiceDesc for MonetaeTreasury service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var MonetaeTreasury_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "treasury.MonetaeTreasury",
	HandlerType: (*MonetaeTreasuryServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "GetSupply",
			Handler:    _MonetaeTreasury_GetSupply_Handler,
		},
	},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Remit",
			Handler:       _MonetaeTreasury_Remit_Handler,
			ServerStreams: true,
			ClientStreams: true,
		},
	},
	Metadata: "treasury/v1/treasury.proto",
}
