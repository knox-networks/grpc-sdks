// Code generated by protoc-gen-go-grpc. DO NOT EDIT.

package v1

import (
	context "context"
	v1 "github.com/knox-networks/grpc-sdks/sdk/go/common/v1"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// EmissaryClient is the client API for Emissary service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type EmissaryClient interface {
	// Establish an emissary connection to the gateway.
	Connection(ctx context.Context, opts ...grpc.CallOption) (Emissary_ConnectionClient, error)
}

type emissaryClient struct {
	cc grpc.ClientConnInterface
}

func NewEmissaryClient(cc grpc.ClientConnInterface) EmissaryClient {
	return &emissaryClient{cc}
}

func (c *emissaryClient) Connection(ctx context.Context, opts ...grpc.CallOption) (Emissary_ConnectionClient, error) {
	stream, err := c.cc.NewStream(ctx, &Emissary_ServiceDesc.Streams[0], "/emissary.Emissary/Connection", opts...)
	if err != nil {
		return nil, err
	}
	x := &emissaryConnectionClient{stream}
	return x, nil
}

type Emissary_ConnectionClient interface {
	Send(*v1.Packet) error
	Recv() (*v1.Packet, error)
	grpc.ClientStream
}

type emissaryConnectionClient struct {
	grpc.ClientStream
}

func (x *emissaryConnectionClient) Send(m *v1.Packet) error {
	return x.ClientStream.SendMsg(m)
}

func (x *emissaryConnectionClient) Recv() (*v1.Packet, error) {
	m := new(v1.Packet)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// EmissaryServer is the server API for Emissary service.
// All implementations must embed UnimplementedEmissaryServer
// for forward compatibility
type EmissaryServer interface {
	// Establish an emissary connection to the gateway.
	Connection(Emissary_ConnectionServer) error
	mustEmbedUnimplementedEmissaryServer()
}

// UnimplementedEmissaryServer must be embedded to have forward compatible implementations.
type UnimplementedEmissaryServer struct {
}

func (UnimplementedEmissaryServer) Connection(Emissary_ConnectionServer) error {
	return status.Errorf(codes.Unimplemented, "method Connection not implemented")
}
func (UnimplementedEmissaryServer) mustEmbedUnimplementedEmissaryServer() {}

// UnsafeEmissaryServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to EmissaryServer will
// result in compilation errors.
type UnsafeEmissaryServer interface {
	mustEmbedUnimplementedEmissaryServer()
}

func RegisterEmissaryServer(s grpc.ServiceRegistrar, srv EmissaryServer) {
	s.RegisterService(&Emissary_ServiceDesc, srv)
}

func _Emissary_Connection_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(EmissaryServer).Connection(&emissaryConnectionServer{stream})
}

type Emissary_ConnectionServer interface {
	Send(*v1.Packet) error
	Recv() (*v1.Packet, error)
	grpc.ServerStream
}

type emissaryConnectionServer struct {
	grpc.ServerStream
}

func (x *emissaryConnectionServer) Send(m *v1.Packet) error {
	return x.ServerStream.SendMsg(m)
}

func (x *emissaryConnectionServer) Recv() (*v1.Packet, error) {
	m := new(v1.Packet)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// Emissary_ServiceDesc is the grpc.ServiceDesc for Emissary service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Emissary_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "emissary.Emissary",
	HandlerType: (*EmissaryServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "Connection",
			Handler:       _Emissary_Connection_Handler,
			ServerStreams: true,
			ClientStreams: true,
		},
	},
	Metadata: "emissary_api/v1/emissary.proto",
}
