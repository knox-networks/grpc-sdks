// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.4.0
// - protoc             v5.27.1
// source: gateway/v1/gateway.proto

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
	Gateway_Authenticate_FullMethodName   = "/gateway.Gateway/Authenticate"
	Gateway_Data_FullMethodName           = "/gateway.Gateway/Data"
	Gateway_GetConnections_FullMethodName = "/gateway.Gateway/GetConnections"
)

// GatewayClient is the client API for Gateway service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
//
// The Gateway service allows fast switching of streaming messages between senders and recipients identified by their
// public key, over authenticated connections. It exposes the APIs needed to connect, authenticate and send data.
type GatewayClient interface {
	// Start a new authenticated session and get a challenge nonce.
	Authenticate(ctx context.Context, in *AuthenticationRequest, opts ...grpc.CallOption) (*AuthenticationResponse, error)
	// Authenticates the connection and opens a bidirectional stream.
	Data(ctx context.Context, opts ...grpc.CallOption) (Gateway_DataClient, error)
	// Get a list of connections to the Gateway, including the active connections and connections that have been removed.
	GetConnections(ctx context.Context, in *ConnectionsRequest, opts ...grpc.CallOption) (*ConnectionsResponse, error)
}

type gatewayClient struct {
	cc grpc.ClientConnInterface
}

func NewGatewayClient(cc grpc.ClientConnInterface) GatewayClient {
	return &gatewayClient{cc}
}

func (c *gatewayClient) Authenticate(ctx context.Context, in *AuthenticationRequest, opts ...grpc.CallOption) (*AuthenticationResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(AuthenticationResponse)
	err := c.cc.Invoke(ctx, Gateway_Authenticate_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *gatewayClient) Data(ctx context.Context, opts ...grpc.CallOption) (Gateway_DataClient, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	stream, err := c.cc.NewStream(ctx, &Gateway_ServiceDesc.Streams[0], Gateway_Data_FullMethodName, cOpts...)
	if err != nil {
		return nil, err
	}
	x := &gatewayDataClient{ClientStream: stream}
	return x, nil
}

type Gateway_DataClient interface {
	Send(*Packet) error
	Recv() (*Packet, error)
	grpc.ClientStream
}

type gatewayDataClient struct {
	grpc.ClientStream
}

func (x *gatewayDataClient) Send(m *Packet) error {
	return x.ClientStream.SendMsg(m)
}

func (x *gatewayDataClient) Recv() (*Packet, error) {
	m := new(Packet)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func (c *gatewayClient) GetConnections(ctx context.Context, in *ConnectionsRequest, opts ...grpc.CallOption) (*ConnectionsResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(ConnectionsResponse)
	err := c.cc.Invoke(ctx, Gateway_GetConnections_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// GatewayServer is the server API for Gateway service.
// All implementations must embed UnimplementedGatewayServer
// for forward compatibility
//
// The Gateway service allows fast switching of streaming messages between senders and recipients identified by their
// public key, over authenticated connections. It exposes the APIs needed to connect, authenticate and send data.
type GatewayServer interface {
	// Start a new authenticated session and get a challenge nonce.
	Authenticate(context.Context, *AuthenticationRequest) (*AuthenticationResponse, error)
	// Authenticates the connection and opens a bidirectional stream.
	Data(Gateway_DataServer) error
	// Get a list of connections to the Gateway, including the active connections and connections that have been removed.
	GetConnections(context.Context, *ConnectionsRequest) (*ConnectionsResponse, error)
	mustEmbedUnimplementedGatewayServer()
}

// UnimplementedGatewayServer must be embedded to have forward compatible implementations.
type UnimplementedGatewayServer struct {
}

func (UnimplementedGatewayServer) Authenticate(context.Context, *AuthenticationRequest) (*AuthenticationResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Authenticate not implemented")
}
func (UnimplementedGatewayServer) Data(Gateway_DataServer) error {
	return status.Errorf(codes.Unimplemented, "method Data not implemented")
}
func (UnimplementedGatewayServer) GetConnections(context.Context, *ConnectionsRequest) (*ConnectionsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetConnections not implemented")
}
func (UnimplementedGatewayServer) mustEmbedUnimplementedGatewayServer() {}

// UnsafeGatewayServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to GatewayServer will
// result in compilation errors.
type UnsafeGatewayServer interface {
	mustEmbedUnimplementedGatewayServer()
}

func RegisterGatewayServer(s grpc.ServiceRegistrar, srv GatewayServer) {
	s.RegisterService(&Gateway_ServiceDesc, srv)
}

func _Gateway_Authenticate_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AuthenticationRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(GatewayServer).Authenticate(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Gateway_Authenticate_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(GatewayServer).Authenticate(ctx, req.(*AuthenticationRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _Gateway_Data_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(GatewayServer).Data(&gatewayDataServer{ServerStream: stream})
}

type Gateway_DataServer interface {
	Send(*Packet) error
	Recv() (*Packet, error)
	grpc.ServerStream
}

type gatewayDataServer struct {
	grpc.ServerStream
}

func (x *gatewayDataServer) Send(m *Packet) error {
	return x.ServerStream.SendMsg(m)
}

func (x *gatewayDataServer) Recv() (*Packet, error) {
	m := new(Packet)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

func _Gateway_GetConnections_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ConnectionsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(GatewayServer).GetConnections(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Gateway_GetConnections_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(GatewayServer).GetConnections(ctx, req.(*ConnectionsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Gateway_ServiceDesc is the grpc.ServiceDesc for Gateway service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Gateway_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "gateway.Gateway",
	HandlerType: (*GatewayServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Authenticate",
			Handler:    _Gateway_Authenticate_Handler,
		},
		{
			MethodName: "GetConnections",
			Handler:    _Gateway_GetConnections_Handler,
		},
	},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Data",
			Handler:       _Gateway_Data_Handler,
			ServerStreams: true,
			ClientStreams: true,
		},
	},
	Metadata: "gateway/v1/gateway.proto",
}
