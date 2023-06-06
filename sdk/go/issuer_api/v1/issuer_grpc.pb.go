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

// IssuerServiceClient is the client API for IssuerService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type IssuerServiceClient interface {
	// Checks that the Verifier is authenticated, then stores and returns the challenge nonce.
	Authenticate(ctx context.Context, in *AuthenticateRequest, opts ...grpc.CallOption) (*AuthenticateResponse, error)
	// Issues digital banknotes against the corresponding issuance limit set by an Authority.  The Issuer must collect the
	// Authenticate challenge signature, the Amount with a total amount, decimal place precision, and a currency code,
	// along with the verifier of the Authority. Issued promissories are sent to the given recipient.
	Issue(ctx context.Context, in *IssueRequest, opts ...grpc.CallOption) (*IssueResponse, error)
	// Gets the roles configured for the Issuer- ex Currency limit for issuance.
	GetRoles(ctx context.Context, in *GetRolesRequest, opts ...grpc.CallOption) (*GetRolesResponse, error)
	// Sets the roles configured for the Issuer- Ex List of the currency limits.
	SetRole(ctx context.Context, in *SetRoleRequest, opts ...grpc.CallOption) (*SetRoleResponse, error)
}

type issuerServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewIssuerServiceClient(cc grpc.ClientConnInterface) IssuerServiceClient {
	return &issuerServiceClient{cc}
}

func (c *issuerServiceClient) Authenticate(ctx context.Context, in *AuthenticateRequest, opts ...grpc.CallOption) (*AuthenticateResponse, error) {
	out := new(AuthenticateResponse)
	err := c.cc.Invoke(ctx, "/issuer_api.v1.IssuerService/Authenticate", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *issuerServiceClient) Issue(ctx context.Context, in *IssueRequest, opts ...grpc.CallOption) (*IssueResponse, error) {
	out := new(IssueResponse)
	err := c.cc.Invoke(ctx, "/issuer_api.v1.IssuerService/Issue", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *issuerServiceClient) GetRoles(ctx context.Context, in *GetRolesRequest, opts ...grpc.CallOption) (*GetRolesResponse, error) {
	out := new(GetRolesResponse)
	err := c.cc.Invoke(ctx, "/issuer_api.v1.IssuerService/GetRoles", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *issuerServiceClient) SetRole(ctx context.Context, in *SetRoleRequest, opts ...grpc.CallOption) (*SetRoleResponse, error) {
	out := new(SetRoleResponse)
	err := c.cc.Invoke(ctx, "/issuer_api.v1.IssuerService/SetRole", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// IssuerServiceServer is the server API for IssuerService service.
// All implementations must embed UnimplementedIssuerServiceServer
// for forward compatibility
type IssuerServiceServer interface {
	// Checks that the Verifier is authenticated, then stores and returns the challenge nonce.
	Authenticate(context.Context, *AuthenticateRequest) (*AuthenticateResponse, error)
	// Issues digital banknotes against the corresponding issuance limit set by an Authority.  The Issuer must collect the
	// Authenticate challenge signature, the Amount with a total amount, decimal place precision, and a currency code,
	// along with the verifier of the Authority. Issued promissories are sent to the given recipient.
	Issue(context.Context, *IssueRequest) (*IssueResponse, error)
	// Gets the roles configured for the Issuer- ex Currency limit for issuance.
	GetRoles(context.Context, *GetRolesRequest) (*GetRolesResponse, error)
	// Sets the roles configured for the Issuer- Ex List of the currency limits.
	SetRole(context.Context, *SetRoleRequest) (*SetRoleResponse, error)
	mustEmbedUnimplementedIssuerServiceServer()
}

// UnimplementedIssuerServiceServer must be embedded to have forward compatible implementations.
type UnimplementedIssuerServiceServer struct {
}

func (UnimplementedIssuerServiceServer) Authenticate(context.Context, *AuthenticateRequest) (*AuthenticateResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Authenticate not implemented")
}
func (UnimplementedIssuerServiceServer) Issue(context.Context, *IssueRequest) (*IssueResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Issue not implemented")
}
func (UnimplementedIssuerServiceServer) GetRoles(context.Context, *GetRolesRequest) (*GetRolesResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetRoles not implemented")
}
func (UnimplementedIssuerServiceServer) SetRole(context.Context, *SetRoleRequest) (*SetRoleResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SetRole not implemented")
}
func (UnimplementedIssuerServiceServer) mustEmbedUnimplementedIssuerServiceServer() {}

// UnsafeIssuerServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to IssuerServiceServer will
// result in compilation errors.
type UnsafeIssuerServiceServer interface {
	mustEmbedUnimplementedIssuerServiceServer()
}

func RegisterIssuerServiceServer(s grpc.ServiceRegistrar, srv IssuerServiceServer) {
	s.RegisterService(&IssuerService_ServiceDesc, srv)
}

func _IssuerService_Authenticate_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AuthenticateRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(IssuerServiceServer).Authenticate(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/issuer_api.v1.IssuerService/Authenticate",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(IssuerServiceServer).Authenticate(ctx, req.(*AuthenticateRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _IssuerService_Issue_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(IssueRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(IssuerServiceServer).Issue(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/issuer_api.v1.IssuerService/Issue",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(IssuerServiceServer).Issue(ctx, req.(*IssueRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _IssuerService_GetRoles_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetRolesRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(IssuerServiceServer).GetRoles(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/issuer_api.v1.IssuerService/GetRoles",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(IssuerServiceServer).GetRoles(ctx, req.(*GetRolesRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _IssuerService_SetRole_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(SetRoleRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(IssuerServiceServer).SetRole(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/issuer_api.v1.IssuerService/SetRole",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(IssuerServiceServer).SetRole(ctx, req.(*SetRoleRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// IssuerService_ServiceDesc is the grpc.ServiceDesc for IssuerService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var IssuerService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "issuer_api.v1.IssuerService",
	HandlerType: (*IssuerServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Authenticate",
			Handler:    _IssuerService_Authenticate_Handler,
		},
		{
			MethodName: "Issue",
			Handler:    _IssuerService_Issue_Handler,
		},
		{
			MethodName: "GetRoles",
			Handler:    _IssuerService_GetRoles_Handler,
		},
		{
			MethodName: "SetRole",
			Handler:    _IssuerService_SetRole_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "issuer_api/v1/issuer.proto",
}
