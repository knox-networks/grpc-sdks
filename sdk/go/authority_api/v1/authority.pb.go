// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.27.1
// 	protoc        v3.18.1
// source: authority_api/v1/authority.proto

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
// "limit": {
// "currency_code":"USD",
// "amount":100,
// "decimals":2
// },
// "issuer_public_key": "ISSUER_PUBLIC_KEY"
// }
type SetIssuerLimitRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Requested issuance amount limits including the limit and the currency code.
	Limit *v1.Amount `protobuf:"bytes,1,opt,name=limit,proto3" json:"limit,omitempty"`
	// Verifier of the issuer.
	IssuerPublicKey string `protobuf:"bytes,2,opt,name=issuer_public_key,json=issuerPublicKey,proto3" json:"issuer_public_key,omitempty"`
}

func (x *SetIssuerLimitRequest) Reset() {
	*x = SetIssuerLimitRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SetIssuerLimitRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SetIssuerLimitRequest) ProtoMessage() {}

func (x *SetIssuerLimitRequest) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SetIssuerLimitRequest.ProtoReflect.Descriptor instead.
func (*SetIssuerLimitRequest) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{0}
}

func (x *SetIssuerLimitRequest) GetLimit() *v1.Amount {
	if x != nil {
		return x.Limit
	}
	return nil
}

func (x *SetIssuerLimitRequest) GetIssuerPublicKey() string {
	if x != nil {
		return x.IssuerPublicKey
	}
	return ""
}

// [Example]
// {
// "limit": {
// "currency_code":"USD",
// "amount":100, "decimals":2
// },
// "signature": "ByteLengthSignatureVariesByAlgo="
// }
type SetIssuerLimitResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Approved issuance amount limit including the limit and the currency code.
	Limit *v1.Amount `protobuf:"bytes,1,opt,name=limit,proto3" json:"limit,omitempty"`
	// Hex encoded cryptographic signature of the issuer and limit by the Authority.
	Signature []byte `protobuf:"bytes,2,opt,name=signature,proto3" json:"signature,omitempty"`
}

func (x *SetIssuerLimitResponse) Reset() {
	*x = SetIssuerLimitResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SetIssuerLimitResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SetIssuerLimitResponse) ProtoMessage() {}

func (x *SetIssuerLimitResponse) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SetIssuerLimitResponse.ProtoReflect.Descriptor instead.
func (*SetIssuerLimitResponse) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{1}
}

func (x *SetIssuerLimitResponse) GetLimit() *v1.Amount {
	if x != nil {
		return x.Limit
	}
	return nil
}

func (x *SetIssuerLimitResponse) GetSignature() []byte {
	if x != nil {
		return x.Signature
	}
	return nil
}

// [Example]
// {
// "issuer_public_key": "ISSUER_PUBLIC_KEY"
// }
type GetIssuerLimitsRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Verifier of the Issuer.
	IssuerPublicKey string `protobuf:"bytes,2,opt,name=issuer_public_key,json=issuerPublicKey,proto3" json:"issuer_public_key,omitempty"`
}

func (x *GetIssuerLimitsRequest) Reset() {
	*x = GetIssuerLimitsRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetIssuerLimitsRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetIssuerLimitsRequest) ProtoMessage() {}

func (x *GetIssuerLimitsRequest) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetIssuerLimitsRequest.ProtoReflect.Descriptor instead.
func (*GetIssuerLimitsRequest) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{2}
}

func (x *GetIssuerLimitsRequest) GetIssuerPublicKey() string {
	if x != nil {
		return x.IssuerPublicKey
	}
	return ""
}

// [Example]
// {
// "limit": [
// {
// "currency_code":"USD",
// "amount": 100,
// "decimals":2
// }
// ],
// "issuer_public_key": "ISSUER_PUBLIC_KEY"
// }
type GetIssuerLimitsResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Approved issuance amount limit.
	Limit []*v1.Amount `protobuf:"bytes,1,rep,name=limit,proto3" json:"limit,omitempty"`
	// Verifier of the Issuer.
	IssuerPublicKey string `protobuf:"bytes,2,opt,name=issuer_public_key,json=issuerPublicKey,proto3" json:"issuer_public_key,omitempty"`
}

func (x *GetIssuerLimitsResponse) Reset() {
	*x = GetIssuerLimitsResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetIssuerLimitsResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetIssuerLimitsResponse) ProtoMessage() {}

func (x *GetIssuerLimitsResponse) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetIssuerLimitsResponse.ProtoReflect.Descriptor instead.
func (*GetIssuerLimitsResponse) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{3}
}

func (x *GetIssuerLimitsResponse) GetLimit() []*v1.Amount {
	if x != nil {
		return x.Limit
	}
	return nil
}

func (x *GetIssuerLimitsResponse) GetIssuerPublicKey() string {
	if x != nil {
		return x.IssuerPublicKey
	}
	return ""
}

// [Example]
// {
// "promissory_file": "LengthOfFileBytesVaries="
// }
type AuthorizeRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Digital banknote to authorize.
	PromissoryFile []byte `protobuf:"bytes,1,opt,name=promissory_file,json=promissoryFile,proto3" json:"promissory_file,omitempty"`
}

func (x *AuthorizeRequest) Reset() {
	*x = AuthorizeRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AuthorizeRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AuthorizeRequest) ProtoMessage() {}

func (x *AuthorizeRequest) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AuthorizeRequest.ProtoReflect.Descriptor instead.
func (*AuthorizeRequest) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{4}
}

func (x *AuthorizeRequest) GetPromissoryFile() []byte {
	if x != nil {
		return x.PromissoryFile
	}
	return nil
}

// [Example]
// {
// "promissory_file": "LengthOfFileBytesVaries="
// }
type AuthorizeResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Digital banknote with authorized signature.
	PromissoryFile []byte `protobuf:"bytes,1,opt,name=promissory_file,json=promissoryFile,proto3" json:"promissory_file,omitempty"`
}

func (x *AuthorizeResponse) Reset() {
	*x = AuthorizeResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AuthorizeResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AuthorizeResponse) ProtoMessage() {}

func (x *AuthorizeResponse) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AuthorizeResponse.ProtoReflect.Descriptor instead.
func (*AuthorizeResponse) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{5}
}

func (x *AuthorizeResponse) GetPromissoryFile() []byte {
	if x != nil {
		return x.PromissoryFile
	}
	return nil
}

// [Example]
// {
// "promissory_file": "LengthOfFileBytesVaries=",
// "verifier": "VERIFIER"
// }
type RedeemRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Digital banknote to redeem.
	PromissoryFile []byte `protobuf:"bytes,1,opt,name=promissory_file,json=promissoryFile,proto3" json:"promissory_file,omitempty"`
	// Verifier of the Issuer in multibase format.
	Verifier string `protobuf:"bytes,2,opt,name=verifier,proto3" json:"verifier,omitempty"`
}

func (x *RedeemRequest) Reset() {
	*x = RedeemRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *RedeemRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RedeemRequest) ProtoMessage() {}

func (x *RedeemRequest) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RedeemRequest.ProtoReflect.Descriptor instead.
func (*RedeemRequest) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{6}
}

func (x *RedeemRequest) GetPromissoryFile() []byte {
	if x != nil {
		return x.PromissoryFile
	}
	return nil
}

func (x *RedeemRequest) GetVerifier() string {
	if x != nil {
		return x.Verifier
	}
	return ""
}

// [Example]
// {
// "limit": {
// "currency_code":"USD",
// "amount":100,
// "decimals":2
// }
// }
type RedeemResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Updated issuance limit.
	Limit *v1.Amount `protobuf:"bytes,1,opt,name=limit,proto3" json:"limit,omitempty"`
}

func (x *RedeemResponse) Reset() {
	*x = RedeemResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[7]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *RedeemResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RedeemResponse) ProtoMessage() {}

func (x *RedeemResponse) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[7]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RedeemResponse.ProtoReflect.Descriptor instead.
func (*RedeemResponse) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{7}
}

func (x *RedeemResponse) GetLimit() *v1.Amount {
	if x != nil {
		return x.Limit
	}
	return nil
}

// [Example]
// {}
type GetEmissaryRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *GetEmissaryRequest) Reset() {
	*x = GetEmissaryRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[8]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetEmissaryRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetEmissaryRequest) ProtoMessage() {}

func (x *GetEmissaryRequest) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[8]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetEmissaryRequest.ProtoReflect.Descriptor instead.
func (*GetEmissaryRequest) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{8}
}

// [Example]
// {
// "verifier": "VERIFIER"
// }
type GetEmissaryResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Verifier of the Emissary in multibase format.
	Verifier string `protobuf:"bytes,1,opt,name=verifier,proto3" json:"verifier,omitempty"`
}

func (x *GetEmissaryResponse) Reset() {
	*x = GetEmissaryResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[9]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetEmissaryResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetEmissaryResponse) ProtoMessage() {}

func (x *GetEmissaryResponse) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[9]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetEmissaryResponse.ProtoReflect.Descriptor instead.
func (*GetEmissaryResponse) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{9}
}

func (x *GetEmissaryResponse) GetVerifier() string {
	if x != nil {
		return x.Verifier
	}
	return ""
}

// [Example]
// {
// "verifier": "VERIFIER"
// }
type GetNotaryRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The public key of the Notary
	Verifier string `protobuf:"bytes,1,opt,name=verifier,proto3" json:"verifier,omitempty"`
}

func (x *GetNotaryRequest) Reset() {
	*x = GetNotaryRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[10]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetNotaryRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetNotaryRequest) ProtoMessage() {}

func (x *GetNotaryRequest) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[10]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetNotaryRequest.ProtoReflect.Descriptor instead.
func (*GetNotaryRequest) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{10}
}

func (x *GetNotaryRequest) GetVerifier() string {
	if x != nil {
		return x.Verifier
	}
	return ""
}

// [Example]
// {
// "signature": "ByteLengthSignatureVariesByAlgo="
// }
type GetNotaryResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The signature of the Notary created by signing the Notary Public key with the Authority keypair
	Signature []byte `protobuf:"bytes,1,opt,name=signature,proto3" json:"signature,omitempty"`
}

func (x *GetNotaryResponse) Reset() {
	*x = GetNotaryResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_authority_api_v1_authority_proto_msgTypes[11]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetNotaryResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetNotaryResponse) ProtoMessage() {}

func (x *GetNotaryResponse) ProtoReflect() protoreflect.Message {
	mi := &file_authority_api_v1_authority_proto_msgTypes[11]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetNotaryResponse.ProtoReflect.Descriptor instead.
func (*GetNotaryResponse) Descriptor() ([]byte, []int) {
	return file_authority_api_v1_authority_proto_rawDescGZIP(), []int{11}
}

func (x *GetNotaryResponse) GetSignature() []byte {
	if x != nil {
		return x.Signature
	}
	return nil
}

var File_authority_api_v1_authority_proto protoreflect.FileDescriptor

var file_authority_api_v1_authority_proto_rawDesc = []byte{
	0x0a, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2f,
	0x76, 0x31, 0x2f, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x12, 0x10, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70,
	0x69, 0x2e, 0x76, 0x31, 0x1a, 0x16, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f,
	0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x69, 0x0a, 0x15,
	0x53, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x52, 0x65,
	0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x24, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41, 0x6d,
	0x6f, 0x75, 0x6e, 0x74, 0x52, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x2a, 0x0a, 0x11, 0x69,
	0x73, 0x73, 0x75, 0x65, 0x72, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x50, 0x75,
	0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x22, 0x5c, 0x0a, 0x16, 0x53, 0x65, 0x74, 0x49, 0x73,
	0x73, 0x75, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
	0x65, 0x12, 0x24, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74,
	0x52, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61,
	0x74, 0x75, 0x72, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x73, 0x69, 0x67, 0x6e,
	0x61, 0x74, 0x75, 0x72, 0x65, 0x22, 0x44, 0x0a, 0x16, 0x47, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75,
	0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
	0x2a, 0x0a, 0x11, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63,
	0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x69, 0x73, 0x73, 0x75,
	0x65, 0x72, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x22, 0x6b, 0x0a, 0x17, 0x47,
	0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x52, 0x65,
	0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x24, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18,
	0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41,
	0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x2a, 0x0a, 0x11,
	0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65,
	0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x50,
	0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x22, 0x3b, 0x0a, 0x10, 0x41, 0x75, 0x74, 0x68,
	0x6f, 0x72, 0x69, 0x7a, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x27, 0x0a, 0x0f,
	0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72, 0x79, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0e, 0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72,
	0x79, 0x46, 0x69, 0x6c, 0x65, 0x22, 0x3c, 0x0a, 0x11, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69,
	0x7a, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x72,
	0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72, 0x79, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x0c, 0x52, 0x0e, 0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72, 0x79, 0x46,
	0x69, 0x6c, 0x65, 0x22, 0x54, 0x0a, 0x0d, 0x52, 0x65, 0x64, 0x65, 0x65, 0x6d, 0x52, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x12, 0x27, 0x0a, 0x0f, 0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f,
	0x72, 0x79, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0e, 0x70,
	0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72, 0x79, 0x46, 0x69, 0x6c, 0x65, 0x12, 0x1a, 0x0a,
	0x08, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x08, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x72, 0x22, 0x36, 0x0a, 0x0e, 0x52, 0x65, 0x64,
	0x65, 0x65, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x24, 0x0a, 0x05, 0x6c,
	0x69, 0x6d, 0x69, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63, 0x6f, 0x6d,
	0x6d, 0x6f, 0x6e, 0x2e, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x05, 0x6c, 0x69, 0x6d, 0x69,
	0x74, 0x22, 0x14, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x45, 0x6d, 0x69, 0x73, 0x73, 0x61, 0x72, 0x79,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x31, 0x0a, 0x13, 0x47, 0x65, 0x74, 0x45, 0x6d,
	0x69, 0x73, 0x73, 0x61, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1a,
	0x0a, 0x08, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x08, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x72, 0x22, 0x2e, 0x0a, 0x10, 0x47, 0x65,
	0x74, 0x4e, 0x6f, 0x74, 0x61, 0x72, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1a,
	0x0a, 0x08, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x08, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x72, 0x22, 0x31, 0x0a, 0x11, 0x47, 0x65,
	0x74, 0x4e, 0x6f, 0x74, 0x61, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
	0x1c, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x0c, 0x52, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x32, 0xb8, 0x04,
	0x0a, 0x10, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x53, 0x65, 0x72, 0x76, 0x69,
	0x63, 0x65, 0x12, 0x63, 0x0a, 0x0e, 0x53, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x72, 0x4c,
	0x69, 0x6d, 0x69, 0x74, 0x12, 0x27, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79,
	0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65,
	0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x28, 0x2e,
	0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31,
	0x2e, 0x53, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x52,
	0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x66, 0x0a, 0x0f, 0x47, 0x65, 0x74, 0x49, 0x73,
	0x73, 0x75, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x12, 0x28, 0x2e, 0x61, 0x75, 0x74,
	0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65,
	0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x52, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x1a, 0x29, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79,
	0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65,
	0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
	0x58, 0x0a, 0x09, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x12, 0x22, 0x2e, 0x61,
	0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e,
	0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
	0x1a, 0x23, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69,
	0x2e, 0x76, 0x31, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x52, 0x65, 0x73,
	0x70, 0x6f, 0x6e, 0x73, 0x65, 0x28, 0x01, 0x30, 0x01, 0x12, 0x4b, 0x0a, 0x06, 0x52, 0x65, 0x64,
	0x65, 0x65, 0x6d, 0x12, 0x1f, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f,
	0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x64, 0x65, 0x65, 0x6d, 0x52, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x1a, 0x20, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79,
	0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x64, 0x65, 0x65, 0x6d, 0x52, 0x65,
	0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x5a, 0x0a, 0x0b, 0x47, 0x65, 0x74, 0x45, 0x6d, 0x69,
	0x73, 0x73, 0x61, 0x72, 0x79, 0x12, 0x24, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74,
	0x79, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x45, 0x6d, 0x69, 0x73,
	0x73, 0x61, 0x72, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x25, 0x2e, 0x61, 0x75,
	0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x47,
	0x65, 0x74, 0x45, 0x6d, 0x69, 0x73, 0x73, 0x61, 0x72, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
	0x73, 0x65, 0x12, 0x54, 0x0a, 0x09, 0x47, 0x65, 0x74, 0x4e, 0x6f, 0x74, 0x61, 0x72, 0x79, 0x12,
	0x22, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2e,
	0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x4e, 0x6f, 0x74, 0x61, 0x72, 0x79, 0x52, 0x65, 0x71, 0x75,
	0x65, 0x73, 0x74, 0x1a, 0x23, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f,
	0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x4e, 0x6f, 0x74, 0x61, 0x72, 0x79,
	0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x3c, 0x5a, 0x3a, 0x67, 0x69, 0x74, 0x68,
	0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6b, 0x6e, 0x6f, 0x78, 0x2d, 0x6e, 0x65, 0x74, 0x77,
	0x6f, 0x72, 0x6b, 0x73, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x2d, 0x73, 0x64, 0x6b, 0x73, 0x2f, 0x73,
	0x64, 0x6b, 0x2f, 0x67, 0x6f, 0x2f, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f,
	0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_authority_api_v1_authority_proto_rawDescOnce sync.Once
	file_authority_api_v1_authority_proto_rawDescData = file_authority_api_v1_authority_proto_rawDesc
)

func file_authority_api_v1_authority_proto_rawDescGZIP() []byte {
	file_authority_api_v1_authority_proto_rawDescOnce.Do(func() {
		file_authority_api_v1_authority_proto_rawDescData = protoimpl.X.CompressGZIP(file_authority_api_v1_authority_proto_rawDescData)
	})
	return file_authority_api_v1_authority_proto_rawDescData
}

var file_authority_api_v1_authority_proto_msgTypes = make([]protoimpl.MessageInfo, 12)
var file_authority_api_v1_authority_proto_goTypes = []interface{}{
	(*SetIssuerLimitRequest)(nil),   // 0: authority_api.v1.SetIssuerLimitRequest
	(*SetIssuerLimitResponse)(nil),  // 1: authority_api.v1.SetIssuerLimitResponse
	(*GetIssuerLimitsRequest)(nil),  // 2: authority_api.v1.GetIssuerLimitsRequest
	(*GetIssuerLimitsResponse)(nil), // 3: authority_api.v1.GetIssuerLimitsResponse
	(*AuthorizeRequest)(nil),        // 4: authority_api.v1.AuthorizeRequest
	(*AuthorizeResponse)(nil),       // 5: authority_api.v1.AuthorizeResponse
	(*RedeemRequest)(nil),           // 6: authority_api.v1.RedeemRequest
	(*RedeemResponse)(nil),          // 7: authority_api.v1.RedeemResponse
	(*GetEmissaryRequest)(nil),      // 8: authority_api.v1.GetEmissaryRequest
	(*GetEmissaryResponse)(nil),     // 9: authority_api.v1.GetEmissaryResponse
	(*GetNotaryRequest)(nil),        // 10: authority_api.v1.GetNotaryRequest
	(*GetNotaryResponse)(nil),       // 11: authority_api.v1.GetNotaryResponse
	(*v1.Amount)(nil),               // 12: common.Amount
}
var file_authority_api_v1_authority_proto_depIdxs = []int32{
	12, // 0: authority_api.v1.SetIssuerLimitRequest.limit:type_name -> common.Amount
	12, // 1: authority_api.v1.SetIssuerLimitResponse.limit:type_name -> common.Amount
	12, // 2: authority_api.v1.GetIssuerLimitsResponse.limit:type_name -> common.Amount
	12, // 3: authority_api.v1.RedeemResponse.limit:type_name -> common.Amount
	0,  // 4: authority_api.v1.AuthorityService.SetIssuerLimit:input_type -> authority_api.v1.SetIssuerLimitRequest
	2,  // 5: authority_api.v1.AuthorityService.GetIssuerLimits:input_type -> authority_api.v1.GetIssuerLimitsRequest
	4,  // 6: authority_api.v1.AuthorityService.Authorize:input_type -> authority_api.v1.AuthorizeRequest
	6,  // 7: authority_api.v1.AuthorityService.Redeem:input_type -> authority_api.v1.RedeemRequest
	8,  // 8: authority_api.v1.AuthorityService.GetEmissary:input_type -> authority_api.v1.GetEmissaryRequest
	10, // 9: authority_api.v1.AuthorityService.GetNotary:input_type -> authority_api.v1.GetNotaryRequest
	1,  // 10: authority_api.v1.AuthorityService.SetIssuerLimit:output_type -> authority_api.v1.SetIssuerLimitResponse
	3,  // 11: authority_api.v1.AuthorityService.GetIssuerLimits:output_type -> authority_api.v1.GetIssuerLimitsResponse
	5,  // 12: authority_api.v1.AuthorityService.Authorize:output_type -> authority_api.v1.AuthorizeResponse
	7,  // 13: authority_api.v1.AuthorityService.Redeem:output_type -> authority_api.v1.RedeemResponse
	9,  // 14: authority_api.v1.AuthorityService.GetEmissary:output_type -> authority_api.v1.GetEmissaryResponse
	11, // 15: authority_api.v1.AuthorityService.GetNotary:output_type -> authority_api.v1.GetNotaryResponse
	10, // [10:16] is the sub-list for method output_type
	4,  // [4:10] is the sub-list for method input_type
	4,  // [4:4] is the sub-list for extension type_name
	4,  // [4:4] is the sub-list for extension extendee
	0,  // [0:4] is the sub-list for field type_name
}

func init() { file_authority_api_v1_authority_proto_init() }
func file_authority_api_v1_authority_proto_init() {
	if File_authority_api_v1_authority_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_authority_api_v1_authority_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SetIssuerLimitRequest); i {
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
		file_authority_api_v1_authority_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SetIssuerLimitResponse); i {
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
		file_authority_api_v1_authority_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetIssuerLimitsRequest); i {
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
		file_authority_api_v1_authority_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetIssuerLimitsResponse); i {
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
		file_authority_api_v1_authority_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AuthorizeRequest); i {
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
		file_authority_api_v1_authority_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AuthorizeResponse); i {
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
		file_authority_api_v1_authority_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*RedeemRequest); i {
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
		file_authority_api_v1_authority_proto_msgTypes[7].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*RedeemResponse); i {
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
		file_authority_api_v1_authority_proto_msgTypes[8].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetEmissaryRequest); i {
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
		file_authority_api_v1_authority_proto_msgTypes[9].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetEmissaryResponse); i {
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
		file_authority_api_v1_authority_proto_msgTypes[10].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetNotaryRequest); i {
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
		file_authority_api_v1_authority_proto_msgTypes[11].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetNotaryResponse); i {
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
			RawDescriptor: file_authority_api_v1_authority_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   12,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_authority_api_v1_authority_proto_goTypes,
		DependencyIndexes: file_authority_api_v1_authority_proto_depIdxs,
		MessageInfos:      file_authority_api_v1_authority_proto_msgTypes,
	}.Build()
	File_authority_api_v1_authority_proto = out.File
	file_authority_api_v1_authority_proto_rawDesc = nil
	file_authority_api_v1_authority_proto_goTypes = nil
	file_authority_api_v1_authority_proto_depIdxs = nil
}
