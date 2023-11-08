// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.27.1
// 	protoc        v3.18.1
// source: faucet_api/v1/faucet.proto

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

// [Example]
// {
// "amount": {
// "currency_code":"USD",
// "amount":100,
// "decimals":2
// },
// "recipient": "zRecipientPublicKeyMultibase58Encoded",
// "distribution": {
// "value": [
// {
// "key": 20,
// "value": 2
// },
// {
// "key": 50,
// "value": 1
// },
// {
// "key": 10,
// "value": 1
// }
// ]
// }
// }
type FaucetRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Total amount to be issued.
	Amount *v1.Amount `protobuf:"bytes,1,opt,name=amount,proto3" json:"amount,omitempty"`
	// Verifier public key of the intended recipient of the issued promissory
	// files (multibase encoding).
	Recipient string `protobuf:"bytes,2,opt,name=recipient,proto3" json:"recipient,omitempty"`
	// sum must match corresponding amount.amount
	Distribution *v1.Distribution `protobuf:"bytes,3,opt,name=distribution,proto3,oneof" json:"distribution,omitempty"`
}

func (x *FaucetRequest) Reset() {
	*x = FaucetRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_faucet_api_v1_faucet_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *FaucetRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*FaucetRequest) ProtoMessage() {}

func (x *FaucetRequest) ProtoReflect() protoreflect.Message {
	mi := &file_faucet_api_v1_faucet_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use FaucetRequest.ProtoReflect.Descriptor instead.
func (*FaucetRequest) Descriptor() ([]byte, []int) {
	return file_faucet_api_v1_faucet_proto_rawDescGZIP(), []int{0}
}

func (x *FaucetRequest) GetAmount() *v1.Amount {
	if x != nil {
		return x.Amount
	}
	return nil
}

func (x *FaucetRequest) GetRecipient() string {
	if x != nil {
		return x.Recipient
	}
	return ""
}

func (x *FaucetRequest) GetDistribution() *v1.Distribution {
	if x != nil {
		return x.Distribution
	}
	return nil
}

// [Example]
// {
// "num_promissory_files": 10
// }
type FaucetResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Number of Promissory Files Returned.
	NumPromissoryFiles uint32 `protobuf:"varint,1,opt,name=num_promissory_files,json=numPromissoryFiles,proto3" json:"num_promissory_files,omitempty"`
}

func (x *FaucetResponse) Reset() {
	*x = FaucetResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_faucet_api_v1_faucet_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *FaucetResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*FaucetResponse) ProtoMessage() {}

func (x *FaucetResponse) ProtoReflect() protoreflect.Message {
	mi := &file_faucet_api_v1_faucet_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use FaucetResponse.ProtoReflect.Descriptor instead.
func (*FaucetResponse) Descriptor() ([]byte, []int) {
	return file_faucet_api_v1_faucet_proto_rawDescGZIP(), []int{1}
}

func (x *FaucetResponse) GetNumPromissoryFiles() uint32 {
	if x != nil {
		return x.NumPromissoryFiles
	}
	return 0
}

var File_faucet_api_v1_faucet_proto protoreflect.FileDescriptor

var file_faucet_api_v1_faucet_proto_rawDesc = []byte{
	0x0a, 0x1a, 0x66, 0x61, 0x75, 0x63, 0x65, 0x74, 0x5f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f,
	0x66, 0x61, 0x75, 0x63, 0x65, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x66, 0x61,
	0x75, 0x63, 0x65, 0x74, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x1a, 0x16, 0x63, 0x6f, 0x6d,
	0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x22, 0xa5, 0x01, 0x0a, 0x0d, 0x46, 0x61, 0x75, 0x63, 0x65, 0x74, 0x52, 0x65,
	0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x26, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41,
	0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x1c, 0x0a,
	0x09, 0x72, 0x65, 0x63, 0x69, 0x70, 0x69, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x09, 0x72, 0x65, 0x63, 0x69, 0x70, 0x69, 0x65, 0x6e, 0x74, 0x12, 0x3d, 0x0a, 0x0c, 0x64,
	0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x14, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x44, 0x69, 0x73, 0x74, 0x72,
	0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x0c, 0x64, 0x69, 0x73, 0x74, 0x72,
	0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x88, 0x01, 0x01, 0x42, 0x0f, 0x0a, 0x0d, 0x5f, 0x64,
	0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x42, 0x0a, 0x0e, 0x46,
	0x61, 0x75, 0x63, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x30, 0x0a,
	0x14, 0x6e, 0x75, 0x6d, 0x5f, 0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72, 0x79, 0x5f,
	0x66, 0x69, 0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x12, 0x6e, 0x75, 0x6d,
	0x50, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72, 0x79, 0x46, 0x69, 0x6c, 0x65, 0x73, 0x32,
	0x5d, 0x0a, 0x14, 0x4d, 0x6f, 0x6e, 0x65, 0x74, 0x61, 0x65, 0x46, 0x61, 0x75, 0x63, 0x65, 0x74,
	0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x45, 0x0a, 0x06, 0x46, 0x61, 0x75, 0x63, 0x65,
	0x74, 0x12, 0x1c, 0x2e, 0x66, 0x61, 0x75, 0x63, 0x65, 0x74, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76,
	0x31, 0x2e, 0x46, 0x61, 0x75, 0x63, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
	0x1d, 0x2e, 0x66, 0x61, 0x75, 0x63, 0x65, 0x74, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e,
	0x46, 0x61, 0x75, 0x63, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x39,
	0x5a, 0x37, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6b, 0x6e, 0x6f,
	0x78, 0x2d, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x2d,
	0x73, 0x64, 0x6b, 0x73, 0x2f, 0x73, 0x64, 0x6b, 0x2f, 0x67, 0x6f, 0x2f, 0x66, 0x61, 0x75, 0x63,
	0x65, 0x74, 0x5f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x33,
}

var (
	file_faucet_api_v1_faucet_proto_rawDescOnce sync.Once
	file_faucet_api_v1_faucet_proto_rawDescData = file_faucet_api_v1_faucet_proto_rawDesc
)

func file_faucet_api_v1_faucet_proto_rawDescGZIP() []byte {
	file_faucet_api_v1_faucet_proto_rawDescOnce.Do(func() {
		file_faucet_api_v1_faucet_proto_rawDescData = protoimpl.X.CompressGZIP(file_faucet_api_v1_faucet_proto_rawDescData)
	})
	return file_faucet_api_v1_faucet_proto_rawDescData
}

var file_faucet_api_v1_faucet_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_faucet_api_v1_faucet_proto_goTypes = []interface{}{
	(*FaucetRequest)(nil),   // 0: faucet_api.v1.FaucetRequest
	(*FaucetResponse)(nil),  // 1: faucet_api.v1.FaucetResponse
	(*v1.Amount)(nil),       // 2: common.Amount
	(*v1.Distribution)(nil), // 3: common.Distribution
}
var file_faucet_api_v1_faucet_proto_depIdxs = []int32{
	2, // 0: faucet_api.v1.FaucetRequest.amount:type_name -> common.Amount
	3, // 1: faucet_api.v1.FaucetRequest.distribution:type_name -> common.Distribution
	0, // 2: faucet_api.v1.MonetaeFaucetService.Faucet:input_type -> faucet_api.v1.FaucetRequest
	1, // 3: faucet_api.v1.MonetaeFaucetService.Faucet:output_type -> faucet_api.v1.FaucetResponse
	3, // [3:4] is the sub-list for method output_type
	2, // [2:3] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_faucet_api_v1_faucet_proto_init() }
func file_faucet_api_v1_faucet_proto_init() {
	if File_faucet_api_v1_faucet_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_faucet_api_v1_faucet_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*FaucetRequest); i {
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
		file_faucet_api_v1_faucet_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*FaucetResponse); i {
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
	file_faucet_api_v1_faucet_proto_msgTypes[0].OneofWrappers = []interface{}{}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_faucet_api_v1_faucet_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_faucet_api_v1_faucet_proto_goTypes,
		DependencyIndexes: file_faucet_api_v1_faucet_proto_depIdxs,
		MessageInfos:      file_faucet_api_v1_faucet_proto_msgTypes,
	}.Build()
	File_faucet_api_v1_faucet_proto = out.File
	file_faucet_api_v1_faucet_proto_rawDesc = nil
	file_faucet_api_v1_faucet_proto_goTypes = nil
	file_faucet_api_v1_faucet_proto_depIdxs = nil
}