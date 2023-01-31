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

// Requests the issuance limit be set to the value specified in `Amount` for the Issuer identified by its public key (aka Verifier).
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

// Contains the newly set issuance limit. The `Amount` indicates both the numeric value of the limit and the currency code. The Authority signs the amount and the public key (aka Verifier) of the issuer, and the signature is included in the response.
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

// Requests the issuance limits associated with the issuer for all currencies that the Issuer is allowed to issue. The Issuer is identified by its public key (aka Verifier).
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

// Responds with the limits applicable to the Issuer for various currencies. The Issuer is identified by its public key (aka Verifier).
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

// A digital banknote that is to be authorized. Severel such requests may be sent in a stream.
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

// A digital banknote that has been authorized. Several such responses may be returned in a stream.
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

// Requests redemption of a digital banknote in exchange for an increase in the issuance limit for an Issuer idetified by its Verifier (aka public key).
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

// Returns the new issuance limit for the Issuer after redeeming a digital banknote.
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
	0x74, 0x32, 0xff, 0x02, 0x0a, 0x09, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x12,
	0x63, 0x0a, 0x0e, 0x53, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69,
	0x74, 0x12, 0x27, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70,
	0x69, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x72, 0x4c, 0x69,
	0x6d, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x28, 0x2e, 0x61, 0x75, 0x74,
	0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x65,
	0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70,
	0x6f, 0x6e, 0x73, 0x65, 0x12, 0x66, 0x0a, 0x0f, 0x47, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65,
	0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x12, 0x28, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72,
	0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x49, 0x73,
	0x73, 0x75, 0x65, 0x72, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
	0x74, 0x1a, 0x29, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70,
	0x69, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x49, 0x73, 0x73, 0x75, 0x65, 0x72, 0x4c, 0x69,
	0x6d, 0x69, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x58, 0x0a, 0x09,
	0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x12, 0x22, 0x2e, 0x61, 0x75, 0x74, 0x68,
	0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x75, 0x74,
	0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x23, 0x2e,
	0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31,
	0x2e, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
	0x73, 0x65, 0x28, 0x01, 0x30, 0x01, 0x12, 0x4b, 0x0a, 0x06, 0x52, 0x65, 0x64, 0x65, 0x65, 0x6d,
	0x12, 0x1f, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69,
	0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x64, 0x65, 0x65, 0x6d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
	0x74, 0x1a, 0x20, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70,
	0x69, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x64, 0x65, 0x65, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x42, 0x3c, 0x5a, 0x3a, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
	0x6d, 0x2f, 0x6b, 0x6e, 0x6f, 0x78, 0x2d, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x2f,
	0x67, 0x72, 0x70, 0x63, 0x2d, 0x73, 0x64, 0x6b, 0x73, 0x2f, 0x73, 0x64, 0x6b, 0x2f, 0x67, 0x6f,
	0x2f, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x2f, 0x76,
	0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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

var file_authority_api_v1_authority_proto_msgTypes = make([]protoimpl.MessageInfo, 8)
var file_authority_api_v1_authority_proto_goTypes = []interface{}{
	(*SetIssuerLimitRequest)(nil),   // 0: authority_api.v1.SetIssuerLimitRequest
	(*SetIssuerLimitResponse)(nil),  // 1: authority_api.v1.SetIssuerLimitResponse
	(*GetIssuerLimitsRequest)(nil),  // 2: authority_api.v1.GetIssuerLimitsRequest
	(*GetIssuerLimitsResponse)(nil), // 3: authority_api.v1.GetIssuerLimitsResponse
	(*AuthorizeRequest)(nil),        // 4: authority_api.v1.AuthorizeRequest
	(*AuthorizeResponse)(nil),       // 5: authority_api.v1.AuthorizeResponse
	(*RedeemRequest)(nil),           // 6: authority_api.v1.RedeemRequest
	(*RedeemResponse)(nil),          // 7: authority_api.v1.RedeemResponse
	(*v1.Amount)(nil),               // 8: common.Amount
}
var file_authority_api_v1_authority_proto_depIdxs = []int32{
	8, // 0: authority_api.v1.SetIssuerLimitRequest.limit:type_name -> common.Amount
	8, // 1: authority_api.v1.SetIssuerLimitResponse.limit:type_name -> common.Amount
	8, // 2: authority_api.v1.GetIssuerLimitsResponse.limit:type_name -> common.Amount
	8, // 3: authority_api.v1.RedeemResponse.limit:type_name -> common.Amount
	0, // 4: authority_api.v1.Authority.SetIssuerLimit:input_type -> authority_api.v1.SetIssuerLimitRequest
	2, // 5: authority_api.v1.Authority.GetIssuerLimits:input_type -> authority_api.v1.GetIssuerLimitsRequest
	4, // 6: authority_api.v1.Authority.Authorize:input_type -> authority_api.v1.AuthorizeRequest
	6, // 7: authority_api.v1.Authority.Redeem:input_type -> authority_api.v1.RedeemRequest
	1, // 8: authority_api.v1.Authority.SetIssuerLimit:output_type -> authority_api.v1.SetIssuerLimitResponse
	3, // 9: authority_api.v1.Authority.GetIssuerLimits:output_type -> authority_api.v1.GetIssuerLimitsResponse
	5, // 10: authority_api.v1.Authority.Authorize:output_type -> authority_api.v1.AuthorizeResponse
	7, // 11: authority_api.v1.Authority.Redeem:output_type -> authority_api.v1.RedeemResponse
	8, // [8:12] is the sub-list for method output_type
	4, // [4:8] is the sub-list for method input_type
	4, // [4:4] is the sub-list for extension type_name
	4, // [4:4] is the sub-list for extension extendee
	0, // [0:4] is the sub-list for field type_name
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
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_authority_api_v1_authority_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   8,
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
