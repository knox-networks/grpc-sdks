// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.27.1
// 	protoc        v3.18.1
// source: issuer_api/v1/issuer.proto

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
// "authentication_signature": {
// "signature": "ByteLengthSignatureVariesByAlgo=",
// "verifier": {
// "signature_system": 1,
// "verifier": "VerifierBytesLengthVariesByAlgo="
// }
// },
// "amount": {
// "currency_code":"USD",
// "amount":100,
// "decimals":2
// },
// "authority": {
// "signature_system": 1,
// "verifier": "VerifierBytesLengthVariesByAlgo="
// }
// }
type IssueRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Authentication challenge signature.
	AuthenticationSignature *v1.AuthenticationSignature `protobuf:"bytes,1,opt,name=authentication_signature,json=authenticationSignature,proto3" json:"authentication_signature,omitempty"`
	// Total amount to be issued.
	Amount *v1.Amount `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
	// Verifier of the Authority.
	Authority *v1.DynamicVerifier `protobuf:"bytes,3,opt,name=authority,proto3" json:"authority,omitempty"`
	// sum must match corresponding amount.amount
	Distribution *v1.Distribution `protobuf:"bytes,4,opt,name=distribution,proto3,oneof" json:"distribution,omitempty"`
}

func (x *IssueRequest) Reset() {
	*x = IssueRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_issuer_api_v1_issuer_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *IssueRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*IssueRequest) ProtoMessage() {}

func (x *IssueRequest) ProtoReflect() protoreflect.Message {
	mi := &file_issuer_api_v1_issuer_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use IssueRequest.ProtoReflect.Descriptor instead.
func (*IssueRequest) Descriptor() ([]byte, []int) {
	return file_issuer_api_v1_issuer_proto_rawDescGZIP(), []int{0}
}

func (x *IssueRequest) GetAuthenticationSignature() *v1.AuthenticationSignature {
	if x != nil {
		return x.AuthenticationSignature
	}
	return nil
}

func (x *IssueRequest) GetAmount() *v1.Amount {
	if x != nil {
		return x.Amount
	}
	return nil
}

func (x *IssueRequest) GetAuthority() *v1.DynamicVerifier {
	if x != nil {
		return x.Authority
	}
	return nil
}

func (x *IssueRequest) GetDistribution() *v1.Distribution {
	if x != nil {
		return x.Distribution
	}
	return nil
}

// [Example]
// {
// "promissory_file": "PROMISSORYFILEBYTES="
// }
type IssueResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Stream of a digital banknote.
	PromissoryFile []byte `protobuf:"bytes,1,opt,name=promissory_file,json=promissoryFile,proto3" json:"promissory_file,omitempty"`
}

func (x *IssueResponse) Reset() {
	*x = IssueResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_issuer_api_v1_issuer_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *IssueResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*IssueResponse) ProtoMessage() {}

func (x *IssueResponse) ProtoReflect() protoreflect.Message {
	mi := &file_issuer_api_v1_issuer_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use IssueResponse.ProtoReflect.Descriptor instead.
func (*IssueResponse) Descriptor() ([]byte, []int) {
	return file_issuer_api_v1_issuer_proto_rawDescGZIP(), []int{1}
}

func (x *IssueResponse) GetPromissoryFile() []byte {
	if x != nil {
		return x.PromissoryFile
	}
	return nil
}

// [Example]
// {
// "promissory_file": "PROMISSORYFILEBYTES="
// }
type AuthorizeRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Stream of a digital banknote to authorize.
	PromissoryFile []byte `protobuf:"bytes,1,opt,name=promissory_file,json=promissoryFile,proto3" json:"promissory_file,omitempty"`
}

func (x *AuthorizeRequest) Reset() {
	*x = AuthorizeRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_issuer_api_v1_issuer_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AuthorizeRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AuthorizeRequest) ProtoMessage() {}

func (x *AuthorizeRequest) ProtoReflect() protoreflect.Message {
	mi := &file_issuer_api_v1_issuer_proto_msgTypes[2]
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
	return file_issuer_api_v1_issuer_proto_rawDescGZIP(), []int{2}
}

func (x *AuthorizeRequest) GetPromissoryFile() []byte {
	if x != nil {
		return x.PromissoryFile
	}
	return nil
}

// [Example]
// {
// "promissory_file": "PROMISSORYFILEBYTES="
// }
type AuthorizeResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Stream of a digital banknote with authorized signature.
	PromissoryFile []byte `protobuf:"bytes,1,opt,name=promissory_file,json=promissoryFile,proto3" json:"promissory_file,omitempty"`
}

func (x *AuthorizeResponse) Reset() {
	*x = AuthorizeResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_issuer_api_v1_issuer_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AuthorizeResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AuthorizeResponse) ProtoMessage() {}

func (x *AuthorizeResponse) ProtoReflect() protoreflect.Message {
	mi := &file_issuer_api_v1_issuer_proto_msgTypes[3]
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
	return file_issuer_api_v1_issuer_proto_rawDescGZIP(), []int{3}
}

func (x *AuthorizeResponse) GetPromissoryFile() []byte {
	if x != nil {
		return x.PromissoryFile
	}
	return nil
}

// [Example]
// {
// "verifier": {
// "signature_system": 1,
// "verifier": "VerifierBytesLengthVariesByAlgo="
// }
// }
type AuthenticateRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Verifier of the requester.
	Verifier *v1.DynamicVerifier `protobuf:"bytes,1,opt,name=verifier,proto3" json:"verifier,omitempty"`
}

func (x *AuthenticateRequest) Reset() {
	*x = AuthenticateRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_issuer_api_v1_issuer_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AuthenticateRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AuthenticateRequest) ProtoMessage() {}

func (x *AuthenticateRequest) ProtoReflect() protoreflect.Message {
	mi := &file_issuer_api_v1_issuer_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AuthenticateRequest.ProtoReflect.Descriptor instead.
func (*AuthenticateRequest) Descriptor() ([]byte, []int) {
	return file_issuer_api_v1_issuer_proto_rawDescGZIP(), []int{4}
}

func (x *AuthenticateRequest) GetVerifier() *v1.DynamicVerifier {
	if x != nil {
		return x.Verifier
	}
	return nil
}

// [Example]
// {
// "challenge": "ByteLengthChallengeVariesByAlgo="
// }
type AuthenticateResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Random unique string to be signed by the authorized key.
	Challenge []byte `protobuf:"bytes,1,opt,name=challenge,proto3" json:"challenge,omitempty"`
}

func (x *AuthenticateResponse) Reset() {
	*x = AuthenticateResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_issuer_api_v1_issuer_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AuthenticateResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AuthenticateResponse) ProtoMessage() {}

func (x *AuthenticateResponse) ProtoReflect() protoreflect.Message {
	mi := &file_issuer_api_v1_issuer_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AuthenticateResponse.ProtoReflect.Descriptor instead.
func (*AuthenticateResponse) Descriptor() ([]byte, []int) {
	return file_issuer_api_v1_issuer_proto_rawDescGZIP(), []int{5}
}

func (x *AuthenticateResponse) GetChallenge() []byte {
	if x != nil {
		return x.Challenge
	}
	return nil
}

// [Example]
// {}
type GetRolesRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *GetRolesRequest) Reset() {
	*x = GetRolesRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_issuer_api_v1_issuer_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetRolesRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetRolesRequest) ProtoMessage() {}

func (x *GetRolesRequest) ProtoReflect() protoreflect.Message {
	mi := &file_issuer_api_v1_issuer_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetRolesRequest.ProtoReflect.Descriptor instead.
func (*GetRolesRequest) Descriptor() ([]byte, []int) {
	return file_issuer_api_v1_issuer_proto_rawDescGZIP(), []int{6}
}

// [Example]
// {
// "roles": [
// {
// "key": "KEY1",
// "value": "VALUE1"
// },
// {
// "key": "KEY2",
// "value": "VALUE2"
// }
// ]
// }
type GetRolesResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Key is the verifier and the Value is <authorized/unauthorized>: <currency limits>.
	Roles map[string]string `protobuf:"bytes,1,rep,name=roles,proto3" json:"roles,omitempty" protobuf_key:"bytes,1,opt,name=key,proto3" protobuf_val:"bytes,2,opt,name=value,proto3"`
}

func (x *GetRolesResponse) Reset() {
	*x = GetRolesResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_issuer_api_v1_issuer_proto_msgTypes[7]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetRolesResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetRolesResponse) ProtoMessage() {}

func (x *GetRolesResponse) ProtoReflect() protoreflect.Message {
	mi := &file_issuer_api_v1_issuer_proto_msgTypes[7]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetRolesResponse.ProtoReflect.Descriptor instead.
func (*GetRolesResponse) Descriptor() ([]byte, []int) {
	return file_issuer_api_v1_issuer_proto_rawDescGZIP(), []int{7}
}

func (x *GetRolesResponse) GetRoles() map[string]string {
	if x != nil {
		return x.Roles
	}
	return nil
}

// [Example]
// {
// "role": {
// "verifier": "VERIFIER",
// "role_enum": {
// "issue_permissions": {
// "inner": [
// {
// "currency_code":"USD",
// "amount":100,
// "decimals":2
// },
// {
// "currency_code":"USD",
// "amount":500,
// "decimals":2
// }
// ]
// }
// }
// }
// }
type SetRoleRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Configured roles set in the Issuer.
	Role *v1.Role `protobuf:"bytes,1,opt,name=role,proto3" json:"role,omitempty"`
}

func (x *SetRoleRequest) Reset() {
	*x = SetRoleRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_issuer_api_v1_issuer_proto_msgTypes[8]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SetRoleRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SetRoleRequest) ProtoMessage() {}

func (x *SetRoleRequest) ProtoReflect() protoreflect.Message {
	mi := &file_issuer_api_v1_issuer_proto_msgTypes[8]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SetRoleRequest.ProtoReflect.Descriptor instead.
func (*SetRoleRequest) Descriptor() ([]byte, []int) {
	return file_issuer_api_v1_issuer_proto_rawDescGZIP(), []int{8}
}

func (x *SetRoleRequest) GetRole() *v1.Role {
	if x != nil {
		return x.Role
	}
	return nil
}

// [Example]
// {}
type SetRoleResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *SetRoleResponse) Reset() {
	*x = SetRoleResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_issuer_api_v1_issuer_proto_msgTypes[9]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *SetRoleResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*SetRoleResponse) ProtoMessage() {}

func (x *SetRoleResponse) ProtoReflect() protoreflect.Message {
	mi := &file_issuer_api_v1_issuer_proto_msgTypes[9]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use SetRoleResponse.ProtoReflect.Descriptor instead.
func (*SetRoleResponse) Descriptor() ([]byte, []int) {
	return file_issuer_api_v1_issuer_proto_rawDescGZIP(), []int{9}
}

var File_issuer_api_v1_issuer_proto protoreflect.FileDescriptor

var file_issuer_api_v1_issuer_proto_rawDesc = []byte{
	0x0a, 0x1a, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f,
	0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x69, 0x73,
	0x73, 0x75, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x1a, 0x16, 0x63, 0x6f, 0x6d,
	0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x22, 0x99, 0x02, 0x0a, 0x0c, 0x49, 0x73, 0x73, 0x75, 0x65, 0x52, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x12, 0x5a, 0x0a, 0x18, 0x61, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69,
	0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
	0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x69,
	0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x52, 0x17, 0x61, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74,
	0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65,
	0x12, 0x26, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74,
	0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x35, 0x0a, 0x09, 0x61, 0x75, 0x74, 0x68,
	0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x63, 0x6f,
	0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x56, 0x65, 0x72, 0x69,
	0x66, 0x69, 0x65, 0x72, 0x52, 0x09, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x12,
	0x3d, 0x0a, 0x0c, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x18,
	0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x44,
	0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x0c, 0x64,
	0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x88, 0x01, 0x01, 0x42, 0x0f,
	0x0a, 0x0d, 0x5f, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x22,
	0x38, 0x0a, 0x0d, 0x49, 0x73, 0x73, 0x75, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
	0x12, 0x27, 0x0a, 0x0f, 0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72, 0x79, 0x5f, 0x66,
	0x69, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0e, 0x70, 0x72, 0x6f, 0x6d, 0x69,
	0x73, 0x73, 0x6f, 0x72, 0x79, 0x46, 0x69, 0x6c, 0x65, 0x22, 0x3b, 0x0a, 0x10, 0x41, 0x75, 0x74,
	0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x27, 0x0a,
	0x0f, 0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72, 0x79, 0x5f, 0x66, 0x69, 0x6c, 0x65,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0e, 0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f,
	0x72, 0x79, 0x46, 0x69, 0x6c, 0x65, 0x22, 0x3c, 0x0a, 0x11, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72,
	0x69, 0x7a, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x70,
	0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72, 0x79, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0c, 0x52, 0x0e, 0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72, 0x79,
	0x46, 0x69, 0x6c, 0x65, 0x22, 0x4a, 0x0a, 0x13, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69,
	0x63, 0x61, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x33, 0x0a, 0x08, 0x76,
	0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e,
	0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x56, 0x65,
	0x72, 0x69, 0x66, 0x69, 0x65, 0x72, 0x52, 0x08, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x72,
	0x22, 0x34, 0x0a, 0x14, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x65,
	0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1c, 0x0a, 0x09, 0x63, 0x68, 0x61, 0x6c,
	0x6c, 0x65, 0x6e, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x63, 0x68, 0x61,
	0x6c, 0x6c, 0x65, 0x6e, 0x67, 0x65, 0x22, 0x11, 0x0a, 0x0f, 0x47, 0x65, 0x74, 0x52, 0x6f, 0x6c,
	0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x8e, 0x01, 0x0a, 0x10, 0x47, 0x65,
	0x74, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x40,
	0x0a, 0x05, 0x72, 0x6f, 0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2a, 0x2e,
	0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65,
	0x74, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x52,
	0x6f, 0x6c, 0x65, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x05, 0x72, 0x6f, 0x6c, 0x65, 0x73,
	0x1a, 0x38, 0x0a, 0x0a, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10,
	0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79,
	0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
	0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x22, 0x32, 0x0a, 0x0e, 0x53, 0x65,
	0x74, 0x52, 0x6f, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x20, 0x0a, 0x04,
	0x72, 0x6f, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x63, 0x6f, 0x6d,
	0x6d, 0x6f, 0x6e, 0x2e, 0x52, 0x6f, 0x6c, 0x65, 0x52, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x22, 0x11,
	0x0a, 0x0f, 0x53, 0x65, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
	0x65, 0x32, 0x99, 0x03, 0x0a, 0x0d, 0x49, 0x73, 0x73, 0x75, 0x65, 0x72, 0x53, 0x65, 0x72, 0x76,
	0x69, 0x63, 0x65, 0x12, 0x57, 0x0a, 0x0c, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63,
	0x61, 0x74, 0x65, 0x12, 0x22, 0x2e, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69,
	0x2e, 0x76, 0x31, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x65,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x23, 0x2e, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72,
	0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69,
	0x63, 0x61, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x52, 0x0a, 0x09,
	0x41, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x12, 0x1f, 0x2e, 0x69, 0x73, 0x73, 0x75,
	0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x6f, 0x72,
	0x69, 0x7a, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x20, 0x2e, 0x69, 0x73, 0x73,
	0x75, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x75, 0x74, 0x68, 0x6f,
	0x72, 0x69, 0x7a, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x28, 0x01, 0x30, 0x01,
	0x12, 0x44, 0x0a, 0x05, 0x49, 0x73, 0x73, 0x75, 0x65, 0x12, 0x1b, 0x2e, 0x69, 0x73, 0x73, 0x75,
	0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x49, 0x73, 0x73, 0x75, 0x65, 0x52,
	0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1c, 0x2e, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x5f,
	0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x49, 0x73, 0x73, 0x75, 0x65, 0x52, 0x65, 0x73, 0x70,
	0x6f, 0x6e, 0x73, 0x65, 0x30, 0x01, 0x12, 0x4b, 0x0a, 0x08, 0x47, 0x65, 0x74, 0x52, 0x6f, 0x6c,
	0x65, 0x73, 0x12, 0x1e, 0x2e, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2e,
	0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
	0x73, 0x74, 0x1a, 0x1f, 0x2e, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2e,
	0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x12, 0x48, 0x0a, 0x07, 0x53, 0x65, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x12, 0x1d,
	0x2e, 0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x53,
	0x65, 0x74, 0x52, 0x6f, 0x6c, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1e, 0x2e,
	0x69, 0x73, 0x73, 0x75, 0x65, 0x72, 0x5f, 0x61, 0x70, 0x69, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x65,
	0x74, 0x52, 0x6f, 0x6c, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x39, 0x5a,
	0x37, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6b, 0x6e, 0x6f, 0x78,
	0x2d, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x2d, 0x73,
	0x64, 0x6b, 0x73, 0x2f, 0x73, 0x64, 0x6b, 0x2f, 0x67, 0x6f, 0x2f, 0x69, 0x73, 0x73, 0x75, 0x65,
	0x72, 0x5f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_issuer_api_v1_issuer_proto_rawDescOnce sync.Once
	file_issuer_api_v1_issuer_proto_rawDescData = file_issuer_api_v1_issuer_proto_rawDesc
)

func file_issuer_api_v1_issuer_proto_rawDescGZIP() []byte {
	file_issuer_api_v1_issuer_proto_rawDescOnce.Do(func() {
		file_issuer_api_v1_issuer_proto_rawDescData = protoimpl.X.CompressGZIP(file_issuer_api_v1_issuer_proto_rawDescData)
	})
	return file_issuer_api_v1_issuer_proto_rawDescData
}

var file_issuer_api_v1_issuer_proto_msgTypes = make([]protoimpl.MessageInfo, 11)
var file_issuer_api_v1_issuer_proto_goTypes = []interface{}{
	(*IssueRequest)(nil),               // 0: issuer_api.v1.IssueRequest
	(*IssueResponse)(nil),              // 1: issuer_api.v1.IssueResponse
	(*AuthorizeRequest)(nil),           // 2: issuer_api.v1.AuthorizeRequest
	(*AuthorizeResponse)(nil),          // 3: issuer_api.v1.AuthorizeResponse
	(*AuthenticateRequest)(nil),        // 4: issuer_api.v1.AuthenticateRequest
	(*AuthenticateResponse)(nil),       // 5: issuer_api.v1.AuthenticateResponse
	(*GetRolesRequest)(nil),            // 6: issuer_api.v1.GetRolesRequest
	(*GetRolesResponse)(nil),           // 7: issuer_api.v1.GetRolesResponse
	(*SetRoleRequest)(nil),             // 8: issuer_api.v1.SetRoleRequest
	(*SetRoleResponse)(nil),            // 9: issuer_api.v1.SetRoleResponse
	nil,                                // 10: issuer_api.v1.GetRolesResponse.RolesEntry
	(*v1.AuthenticationSignature)(nil), // 11: common.AuthenticationSignature
	(*v1.Amount)(nil),                  // 12: common.Amount
	(*v1.DynamicVerifier)(nil),         // 13: common.DynamicVerifier
	(*v1.Distribution)(nil),            // 14: common.Distribution
	(*v1.Role)(nil),                    // 15: common.Role
}
var file_issuer_api_v1_issuer_proto_depIdxs = []int32{
	11, // 0: issuer_api.v1.IssueRequest.authentication_signature:type_name -> common.AuthenticationSignature
	12, // 1: issuer_api.v1.IssueRequest.amount:type_name -> common.Amount
	13, // 2: issuer_api.v1.IssueRequest.authority:type_name -> common.DynamicVerifier
	14, // 3: issuer_api.v1.IssueRequest.distribution:type_name -> common.Distribution
	13, // 4: issuer_api.v1.AuthenticateRequest.verifier:type_name -> common.DynamicVerifier
	10, // 5: issuer_api.v1.GetRolesResponse.roles:type_name -> issuer_api.v1.GetRolesResponse.RolesEntry
	15, // 6: issuer_api.v1.SetRoleRequest.role:type_name -> common.Role
	4,  // 7: issuer_api.v1.IssuerService.Authenticate:input_type -> issuer_api.v1.AuthenticateRequest
	2,  // 8: issuer_api.v1.IssuerService.Authorize:input_type -> issuer_api.v1.AuthorizeRequest
	0,  // 9: issuer_api.v1.IssuerService.Issue:input_type -> issuer_api.v1.IssueRequest
	6,  // 10: issuer_api.v1.IssuerService.GetRoles:input_type -> issuer_api.v1.GetRolesRequest
	8,  // 11: issuer_api.v1.IssuerService.SetRole:input_type -> issuer_api.v1.SetRoleRequest
	5,  // 12: issuer_api.v1.IssuerService.Authenticate:output_type -> issuer_api.v1.AuthenticateResponse
	3,  // 13: issuer_api.v1.IssuerService.Authorize:output_type -> issuer_api.v1.AuthorizeResponse
	1,  // 14: issuer_api.v1.IssuerService.Issue:output_type -> issuer_api.v1.IssueResponse
	7,  // 15: issuer_api.v1.IssuerService.GetRoles:output_type -> issuer_api.v1.GetRolesResponse
	9,  // 16: issuer_api.v1.IssuerService.SetRole:output_type -> issuer_api.v1.SetRoleResponse
	12, // [12:17] is the sub-list for method output_type
	7,  // [7:12] is the sub-list for method input_type
	7,  // [7:7] is the sub-list for extension type_name
	7,  // [7:7] is the sub-list for extension extendee
	0,  // [0:7] is the sub-list for field type_name
}

func init() { file_issuer_api_v1_issuer_proto_init() }
func file_issuer_api_v1_issuer_proto_init() {
	if File_issuer_api_v1_issuer_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_issuer_api_v1_issuer_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*IssueRequest); i {
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
		file_issuer_api_v1_issuer_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*IssueResponse); i {
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
		file_issuer_api_v1_issuer_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
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
		file_issuer_api_v1_issuer_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
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
		file_issuer_api_v1_issuer_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AuthenticateRequest); i {
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
		file_issuer_api_v1_issuer_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AuthenticateResponse); i {
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
		file_issuer_api_v1_issuer_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetRolesRequest); i {
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
		file_issuer_api_v1_issuer_proto_msgTypes[7].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetRolesResponse); i {
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
		file_issuer_api_v1_issuer_proto_msgTypes[8].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SetRoleRequest); i {
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
		file_issuer_api_v1_issuer_proto_msgTypes[9].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*SetRoleResponse); i {
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
	file_issuer_api_v1_issuer_proto_msgTypes[0].OneofWrappers = []interface{}{}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_issuer_api_v1_issuer_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   11,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_issuer_api_v1_issuer_proto_goTypes,
		DependencyIndexes: file_issuer_api_v1_issuer_proto_depIdxs,
		MessageInfos:      file_issuer_api_v1_issuer_proto_msgTypes,
	}.Build()
	File_issuer_api_v1_issuer_proto = out.File
	file_issuer_api_v1_issuer_proto_rawDesc = nil
	file_issuer_api_v1_issuer_proto_goTypes = nil
	file_issuer_api_v1_issuer_proto_depIdxs = nil
}
