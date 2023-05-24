// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.27.1
// 	protoc        v3.18.1
// source: gateway/v1/gateway.proto

package v1

import (
	v1 "github.com/knox-networks/grpc-sdks/sdk/go/common/v1"
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// A log entry for a Gateway connection event.
// [Example]{"verifier": "VERIFIER", "note": "NOTE", "timestamp": "TIMESTAMP", "origin": "ORIGIN", "connection_id": "CONNECTION_ID_UUID"}
type LogEntry struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Public Key
	Verifier string `protobuf:"bytes,1,opt,name=verifier,proto3" json:"verifier,omitempty"`
	// Specific action being recorded.
	Note string `protobuf:"bytes,2,opt,name=note,proto3" json:"note,omitempty"`
	// RFC3339 timestamp of the log entry.
	Timestamp string `protobuf:"bytes,3,opt,name=timestamp,proto3" json:"timestamp,omitempty"`
	Origin    string `protobuf:"bytes,4,opt,name=origin,proto3" json:"origin,omitempty"`
	// UUID representing a unique client connection
	ConnectionId string `protobuf:"bytes,5,opt,name=connection_id,json=connectionId,proto3" json:"connection_id,omitempty"`
}

func (x *LogEntry) Reset() {
	*x = LogEntry{}
	if protoimpl.UnsafeEnabled {
		mi := &file_gateway_v1_gateway_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *LogEntry) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*LogEntry) ProtoMessage() {}

func (x *LogEntry) ProtoReflect() protoreflect.Message {
	mi := &file_gateway_v1_gateway_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use LogEntry.ProtoReflect.Descriptor instead.
func (*LogEntry) Descriptor() ([]byte, []int) {
	return file_gateway_v1_gateway_proto_rawDescGZIP(), []int{0}
}

func (x *LogEntry) GetVerifier() string {
	if x != nil {
		return x.Verifier
	}
	return ""
}

func (x *LogEntry) GetNote() string {
	if x != nil {
		return x.Note
	}
	return ""
}

func (x *LogEntry) GetTimestamp() string {
	if x != nil {
		return x.Timestamp
	}
	return ""
}

func (x *LogEntry) GetOrigin() string {
	if x != nil {
		return x.Origin
	}
	return ""
}

func (x *LogEntry) GetConnectionId() string {
	if x != nil {
		return x.ConnectionId
	}
	return ""
}

// Requests a list of connections known to the Gateway.
// [Example]{ }
type ConnectionsRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *ConnectionsRequest) Reset() {
	*x = ConnectionsRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_gateway_v1_gateway_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ConnectionsRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ConnectionsRequest) ProtoMessage() {}

func (x *ConnectionsRequest) ProtoReflect() protoreflect.Message {
	mi := &file_gateway_v1_gateway_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ConnectionsRequest.ProtoReflect.Descriptor instead.
func (*ConnectionsRequest) Descriptor() ([]byte, []int) {
	return file_gateway_v1_gateway_proto_rawDescGZIP(), []int{1}
}

// Contains a list of active connections, as well as lists of log entries for all created connections and for all
// removed connections.
// [Example]{"active": ["ActiveConnection1", "ActiveConnection2"], "created": [{"verifier": "VERIFIER", "note": "CreatedConnection1", "timestamp": "TIMESTAMP", "origin": "ORIGIN", "connection_id": "CONNECTION_ID_UUID"}, {"verifier": "VERIFIER", "note": "CreatedConnection2", "timestamp": "TIMESTAMP", "origin": "ORIGIN", "connection_id": "CONNECTION_ID_UUID"}], "removed": [{"verifier": "VERIFIER", "note": "RemovedConnection1", "timestamp": "TIMESTAMP", "origin": "ORIGIN", "connection_id": "CONNECTION_ID_UUID"}, {"verifier": "VERIFIER", "note": "RemovedConnection2", "timestamp": "TIMESTAMP", "origin": "ORIGIN", "connection_id": "CONNECTION_ID_UUID"}]}
type ConnectionsResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Currently active connections.
	Active []string `protobuf:"bytes,1,rep,name=active,proto3" json:"active,omitempty"`
	// The number of connections that have been created.
	Created []*LogEntry `protobuf:"bytes,2,rep,name=created,proto3" json:"created,omitempty"`
	// The number of connections that have been removed.
	Removed []*LogEntry `protobuf:"bytes,3,rep,name=removed,proto3" json:"removed,omitempty"`
}

func (x *ConnectionsResponse) Reset() {
	*x = ConnectionsResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_gateway_v1_gateway_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ConnectionsResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ConnectionsResponse) ProtoMessage() {}

func (x *ConnectionsResponse) ProtoReflect() protoreflect.Message {
	mi := &file_gateway_v1_gateway_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ConnectionsResponse.ProtoReflect.Descriptor instead.
func (*ConnectionsResponse) Descriptor() ([]byte, []int) {
	return file_gateway_v1_gateway_proto_rawDescGZIP(), []int{2}
}

func (x *ConnectionsResponse) GetActive() []string {
	if x != nil {
		return x.Active
	}
	return nil
}

func (x *ConnectionsResponse) GetCreated() []*LogEntry {
	if x != nil {
		return x.Created
	}
	return nil
}

func (x *ConnectionsResponse) GetRemoved() []*LogEntry {
	if x != nil {
		return x.Removed
	}
	return nil
}

var File_gateway_v1_gateway_proto protoreflect.FileDescriptor

var file_gateway_v1_gateway_proto_rawDesc = []byte{
	0x0a, 0x18, 0x67, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x2f, 0x76, 0x31, 0x2f, 0x67, 0x61, 0x74,
	0x65, 0x77, 0x61, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x07, 0x67, 0x61, 0x74, 0x65,
	0x77, 0x61, 0x79, 0x1a, 0x16, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x63,
	0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x16, 0x63, 0x6f, 0x6d,
	0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x22, 0x95, 0x01, 0x0a, 0x08, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79,
	0x12, 0x1a, 0x0a, 0x08, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x08, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04,
	0x6e, 0x6f, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x6f, 0x74, 0x65,
	0x12, 0x1c, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x03, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x16,
	0x0a, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06,
	0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
	0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x63,
	0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x14, 0x0a, 0x12, 0x43,
	0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
	0x74, 0x22, 0x87, 0x01, 0x0a, 0x13, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
	0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x63, 0x74,
	0x69, 0x76, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x06, 0x61, 0x63, 0x74, 0x69, 0x76,
	0x65, 0x12, 0x2b, 0x0a, 0x07, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x18, 0x02, 0x20, 0x03,
	0x28, 0x0b, 0x32, 0x11, 0x2e, 0x67, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x2e, 0x4c, 0x6f, 0x67,
	0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x07, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x12, 0x2b,
	0x0a, 0x07, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32,
	0x11, 0x2e, 0x67, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x2e, 0x4c, 0x6f, 0x67, 0x45, 0x6e, 0x74,
	0x72, 0x79, 0x52, 0x07, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x32, 0xd8, 0x01, 0x0a, 0x0e,
	0x4d, 0x6f, 0x6e, 0x65, 0x74, 0x61, 0x65, 0x47, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x12, 0x4d,
	0x0a, 0x0c, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x65, 0x12, 0x1d,
	0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69,
	0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1e, 0x2e,
	0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63,
	0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2a, 0x0a,
	0x04, 0x44, 0x61, 0x74, 0x61, 0x12, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x50,
	0x61, 0x63, 0x6b, 0x65, 0x74, 0x1a, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x50,
	0x61, 0x63, 0x6b, 0x65, 0x74, 0x28, 0x01, 0x30, 0x01, 0x12, 0x4b, 0x0a, 0x0e, 0x47, 0x65, 0x74,
	0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x1b, 0x2e, 0x67, 0x61,
	0x74, 0x65, 0x77, 0x61, 0x79, 0x2e, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e,
	0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1c, 0x2e, 0x67, 0x61, 0x74, 0x65, 0x77,
	0x61, 0x79, 0x2e, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65,
	0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x36, 0x5a, 0x34, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
	0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6b, 0x6e, 0x6f, 0x78, 0x2d, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72,
	0x6b, 0x73, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x2d, 0x73, 0x64, 0x6b, 0x73, 0x2f, 0x73, 0x64, 0x6b,
	0x2f, 0x67, 0x6f, 0x2f, 0x67, 0x61, 0x74, 0x65, 0x77, 0x61, 0x79, 0x2f, 0x76, 0x31, 0x62, 0x06,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_gateway_v1_gateway_proto_rawDescOnce sync.Once
	file_gateway_v1_gateway_proto_rawDescData = file_gateway_v1_gateway_proto_rawDesc
)

func file_gateway_v1_gateway_proto_rawDescGZIP() []byte {
	file_gateway_v1_gateway_proto_rawDescOnce.Do(func() {
		file_gateway_v1_gateway_proto_rawDescData = protoimpl.X.CompressGZIP(file_gateway_v1_gateway_proto_rawDescData)
	})
	return file_gateway_v1_gateway_proto_rawDescData
}

var file_gateway_v1_gateway_proto_msgTypes = make([]protoimpl.MessageInfo, 3)
var file_gateway_v1_gateway_proto_goTypes = []interface{}{
	(*LogEntry)(nil),                  // 0: gateway.LogEntry
	(*ConnectionsRequest)(nil),        // 1: gateway.ConnectionsRequest
	(*ConnectionsResponse)(nil),       // 2: gateway.ConnectionsResponse
	(*v1.AuthenticationRequest)(nil),  // 3: common.AuthenticationRequest
	(*v1.Packet)(nil),                 // 4: common.Packet
	(*v1.AuthenticationResponse)(nil), // 5: common.AuthenticationResponse
}
var file_gateway_v1_gateway_proto_depIdxs = []int32{
	0, // 0: gateway.ConnectionsResponse.created:type_name -> gateway.LogEntry
	0, // 1: gateway.ConnectionsResponse.removed:type_name -> gateway.LogEntry
	3, // 2: gateway.MonetaeGateway.Authenticate:input_type -> common.AuthenticationRequest
	4, // 3: gateway.MonetaeGateway.Data:input_type -> common.Packet
	1, // 4: gateway.MonetaeGateway.GetConnections:input_type -> gateway.ConnectionsRequest
	5, // 5: gateway.MonetaeGateway.Authenticate:output_type -> common.AuthenticationResponse
	4, // 6: gateway.MonetaeGateway.Data:output_type -> common.Packet
	2, // 7: gateway.MonetaeGateway.GetConnections:output_type -> gateway.ConnectionsResponse
	5, // [5:8] is the sub-list for method output_type
	2, // [2:5] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_gateway_v1_gateway_proto_init() }
func file_gateway_v1_gateway_proto_init() {
	if File_gateway_v1_gateway_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_gateway_v1_gateway_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*LogEntry); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_gateway_v1_gateway_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ConnectionsRequest); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_gateway_v1_gateway_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ConnectionsResponse); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_gateway_v1_gateway_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   3,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_gateway_v1_gateway_proto_goTypes,
		DependencyIndexes: file_gateway_v1_gateway_proto_depIdxs,
		MessageInfos:      file_gateway_v1_gateway_proto_msgTypes,
	}.Build()
	File_gateway_v1_gateway_proto = out.File
	file_gateway_v1_gateway_proto_rawDesc = nil
	file_gateway_v1_gateway_proto_goTypes = nil
	file_gateway_v1_gateway_proto_depIdxs = nil
}
