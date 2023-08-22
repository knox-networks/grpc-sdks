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

// WebhookManagerServiceClient is the client API for WebhookManagerService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type WebhookManagerServiceClient interface {
	// Creates a Webhook.
	CreateWebhook(ctx context.Context, in *CreateWebhookRequest, opts ...grpc.CallOption) (*CreateWebhookResponse, error)
	// Lists Webhooks.
	ListWebhooks(ctx context.Context, in *ListWebhooksRequest, opts ...grpc.CallOption) (*ListWebhooksResponse, error)
	// Deletes a Webhook.
	DeleteWebhook(ctx context.Context, in *DeleteWebhookRequest, opts ...grpc.CallOption) (*DeleteWebhookResponse, error)
	// Lists Webhook Events.
	ListEvents(ctx context.Context, in *ListEventsRequest, opts ...grpc.CallOption) (*ListEventsResponse, error)
	// Retries a Webhook Event.
	RetryEvent(ctx context.Context, in *RetryEventRequest, opts ...grpc.CallOption) (*RetryEventResponse, error)
	// Ping a Webhook.
	Ping(ctx context.Context, in *PingRequest, opts ...grpc.CallOption) (*PingResponse, error)
	// Lists Delivery History.
	ListDeliveryHistory(ctx context.Context, in *ListDeliveryHistoryRequest, opts ...grpc.CallOption) (*ListDeliveryHistoryResponse, error)
}

type webhookManagerServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewWebhookManagerServiceClient(cc grpc.ClientConnInterface) WebhookManagerServiceClient {
	return &webhookManagerServiceClient{cc}
}

func (c *webhookManagerServiceClient) CreateWebhook(ctx context.Context, in *CreateWebhookRequest, opts ...grpc.CallOption) (*CreateWebhookResponse, error) {
	out := new(CreateWebhookResponse)
	err := c.cc.Invoke(ctx, "/webhook_api.v1.WebhookManagerService/CreateWebhook", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *webhookManagerServiceClient) ListWebhooks(ctx context.Context, in *ListWebhooksRequest, opts ...grpc.CallOption) (*ListWebhooksResponse, error) {
	out := new(ListWebhooksResponse)
	err := c.cc.Invoke(ctx, "/webhook_api.v1.WebhookManagerService/ListWebhooks", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *webhookManagerServiceClient) DeleteWebhook(ctx context.Context, in *DeleteWebhookRequest, opts ...grpc.CallOption) (*DeleteWebhookResponse, error) {
	out := new(DeleteWebhookResponse)
	err := c.cc.Invoke(ctx, "/webhook_api.v1.WebhookManagerService/DeleteWebhook", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *webhookManagerServiceClient) ListEvents(ctx context.Context, in *ListEventsRequest, opts ...grpc.CallOption) (*ListEventsResponse, error) {
	out := new(ListEventsResponse)
	err := c.cc.Invoke(ctx, "/webhook_api.v1.WebhookManagerService/ListEvents", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *webhookManagerServiceClient) RetryEvent(ctx context.Context, in *RetryEventRequest, opts ...grpc.CallOption) (*RetryEventResponse, error) {
	out := new(RetryEventResponse)
	err := c.cc.Invoke(ctx, "/webhook_api.v1.WebhookManagerService/RetryEvent", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *webhookManagerServiceClient) Ping(ctx context.Context, in *PingRequest, opts ...grpc.CallOption) (*PingResponse, error) {
	out := new(PingResponse)
	err := c.cc.Invoke(ctx, "/webhook_api.v1.WebhookManagerService/Ping", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *webhookManagerServiceClient) ListDeliveryHistory(ctx context.Context, in *ListDeliveryHistoryRequest, opts ...grpc.CallOption) (*ListDeliveryHistoryResponse, error) {
	out := new(ListDeliveryHistoryResponse)
	err := c.cc.Invoke(ctx, "/webhook_api.v1.WebhookManagerService/ListDeliveryHistory", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// WebhookManagerServiceServer is the server API for WebhookManagerService service.
// All implementations must embed UnimplementedWebhookManagerServiceServer
// for forward compatibility
type WebhookManagerServiceServer interface {
	// Creates a Webhook.
	CreateWebhook(context.Context, *CreateWebhookRequest) (*CreateWebhookResponse, error)
	// Lists Webhooks.
	ListWebhooks(context.Context, *ListWebhooksRequest) (*ListWebhooksResponse, error)
	// Deletes a Webhook.
	DeleteWebhook(context.Context, *DeleteWebhookRequest) (*DeleteWebhookResponse, error)
	// Lists Webhook Events.
	ListEvents(context.Context, *ListEventsRequest) (*ListEventsResponse, error)
	// Retries a Webhook Event.
	RetryEvent(context.Context, *RetryEventRequest) (*RetryEventResponse, error)
	// Ping a Webhook.
	Ping(context.Context, *PingRequest) (*PingResponse, error)
	// Lists Delivery History.
	ListDeliveryHistory(context.Context, *ListDeliveryHistoryRequest) (*ListDeliveryHistoryResponse, error)
	mustEmbedUnimplementedWebhookManagerServiceServer()
}

// UnimplementedWebhookManagerServiceServer must be embedded to have forward compatible implementations.
type UnimplementedWebhookManagerServiceServer struct {
}

func (UnimplementedWebhookManagerServiceServer) CreateWebhook(context.Context, *CreateWebhookRequest) (*CreateWebhookResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method CreateWebhook not implemented")
}
func (UnimplementedWebhookManagerServiceServer) ListWebhooks(context.Context, *ListWebhooksRequest) (*ListWebhooksResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListWebhooks not implemented")
}
func (UnimplementedWebhookManagerServiceServer) DeleteWebhook(context.Context, *DeleteWebhookRequest) (*DeleteWebhookResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method DeleteWebhook not implemented")
}
func (UnimplementedWebhookManagerServiceServer) ListEvents(context.Context, *ListEventsRequest) (*ListEventsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListEvents not implemented")
}
func (UnimplementedWebhookManagerServiceServer) RetryEvent(context.Context, *RetryEventRequest) (*RetryEventResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method RetryEvent not implemented")
}
func (UnimplementedWebhookManagerServiceServer) Ping(context.Context, *PingRequest) (*PingResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Ping not implemented")
}
func (UnimplementedWebhookManagerServiceServer) ListDeliveryHistory(context.Context, *ListDeliveryHistoryRequest) (*ListDeliveryHistoryResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListDeliveryHistory not implemented")
}
func (UnimplementedWebhookManagerServiceServer) mustEmbedUnimplementedWebhookManagerServiceServer() {}

// UnsafeWebhookManagerServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to WebhookManagerServiceServer will
// result in compilation errors.
type UnsafeWebhookManagerServiceServer interface {
	mustEmbedUnimplementedWebhookManagerServiceServer()
}

func RegisterWebhookManagerServiceServer(s grpc.ServiceRegistrar, srv WebhookManagerServiceServer) {
	s.RegisterService(&WebhookManagerService_ServiceDesc, srv)
}

func _WebhookManagerService_CreateWebhook_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CreateWebhookRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WebhookManagerServiceServer).CreateWebhook(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/webhook_api.v1.WebhookManagerService/CreateWebhook",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WebhookManagerServiceServer).CreateWebhook(ctx, req.(*CreateWebhookRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _WebhookManagerService_ListWebhooks_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListWebhooksRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WebhookManagerServiceServer).ListWebhooks(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/webhook_api.v1.WebhookManagerService/ListWebhooks",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WebhookManagerServiceServer).ListWebhooks(ctx, req.(*ListWebhooksRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _WebhookManagerService_DeleteWebhook_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(DeleteWebhookRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WebhookManagerServiceServer).DeleteWebhook(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/webhook_api.v1.WebhookManagerService/DeleteWebhook",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WebhookManagerServiceServer).DeleteWebhook(ctx, req.(*DeleteWebhookRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _WebhookManagerService_ListEvents_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListEventsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WebhookManagerServiceServer).ListEvents(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/webhook_api.v1.WebhookManagerService/ListEvents",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WebhookManagerServiceServer).ListEvents(ctx, req.(*ListEventsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _WebhookManagerService_RetryEvent_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RetryEventRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WebhookManagerServiceServer).RetryEvent(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/webhook_api.v1.WebhookManagerService/RetryEvent",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WebhookManagerServiceServer).RetryEvent(ctx, req.(*RetryEventRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _WebhookManagerService_Ping_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(PingRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WebhookManagerServiceServer).Ping(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/webhook_api.v1.WebhookManagerService/Ping",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WebhookManagerServiceServer).Ping(ctx, req.(*PingRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _WebhookManagerService_ListDeliveryHistory_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListDeliveryHistoryRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(WebhookManagerServiceServer).ListDeliveryHistory(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/webhook_api.v1.WebhookManagerService/ListDeliveryHistory",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(WebhookManagerServiceServer).ListDeliveryHistory(ctx, req.(*ListDeliveryHistoryRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// WebhookManagerService_ServiceDesc is the grpc.ServiceDesc for WebhookManagerService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var WebhookManagerService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "webhook_api.v1.WebhookManagerService",
	HandlerType: (*WebhookManagerServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "CreateWebhook",
			Handler:    _WebhookManagerService_CreateWebhook_Handler,
		},
		{
			MethodName: "ListWebhooks",
			Handler:    _WebhookManagerService_ListWebhooks_Handler,
		},
		{
			MethodName: "DeleteWebhook",
			Handler:    _WebhookManagerService_DeleteWebhook_Handler,
		},
		{
			MethodName: "ListEvents",
			Handler:    _WebhookManagerService_ListEvents_Handler,
		},
		{
			MethodName: "RetryEvent",
			Handler:    _WebhookManagerService_RetryEvent_Handler,
		},
		{
			MethodName: "Ping",
			Handler:    _WebhookManagerService_Ping_Handler,
		},
		{
			MethodName: "ListDeliveryHistory",
			Handler:    _WebhookManagerService_ListDeliveryHistory_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "webhook_api/v1/webhook.proto",
}

// EventConsumerServiceClient is the client API for EventConsumerService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type EventConsumerServiceClient interface {
	// Webhook Event.
	Event(ctx context.Context, in *EventRequest, opts ...grpc.CallOption) (*EventResponse, error)
}

type eventConsumerServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewEventConsumerServiceClient(cc grpc.ClientConnInterface) EventConsumerServiceClient {
	return &eventConsumerServiceClient{cc}
}

func (c *eventConsumerServiceClient) Event(ctx context.Context, in *EventRequest, opts ...grpc.CallOption) (*EventResponse, error) {
	out := new(EventResponse)
	err := c.cc.Invoke(ctx, "/webhook_api.v1.EventConsumerService/Event", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// EventConsumerServiceServer is the server API for EventConsumerService service.
// All implementations must embed UnimplementedEventConsumerServiceServer
// for forward compatibility
type EventConsumerServiceServer interface {
	// Webhook Event.
	Event(context.Context, *EventRequest) (*EventResponse, error)
	mustEmbedUnimplementedEventConsumerServiceServer()
}

// UnimplementedEventConsumerServiceServer must be embedded to have forward compatible implementations.
type UnimplementedEventConsumerServiceServer struct {
}

func (UnimplementedEventConsumerServiceServer) Event(context.Context, *EventRequest) (*EventResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Event not implemented")
}
func (UnimplementedEventConsumerServiceServer) mustEmbedUnimplementedEventConsumerServiceServer() {}

// UnsafeEventConsumerServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to EventConsumerServiceServer will
// result in compilation errors.
type UnsafeEventConsumerServiceServer interface {
	mustEmbedUnimplementedEventConsumerServiceServer()
}

func RegisterEventConsumerServiceServer(s grpc.ServiceRegistrar, srv EventConsumerServiceServer) {
	s.RegisterService(&EventConsumerService_ServiceDesc, srv)
}

func _EventConsumerService_Event_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(EventRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(EventConsumerServiceServer).Event(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/webhook_api.v1.EventConsumerService/Event",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(EventConsumerServiceServer).Event(ctx, req.(*EventRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// EventConsumerService_ServiceDesc is the grpc.ServiceDesc for EventConsumerService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var EventConsumerService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "webhook_api.v1.EventConsumerService",
	HandlerType: (*EventConsumerServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Event",
			Handler:    _EventConsumerService_Event_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "webhook_api/v1/webhook.proto",
}
