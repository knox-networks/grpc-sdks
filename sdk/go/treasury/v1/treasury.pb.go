// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.27.1
// 	protoc        v3.18.1
// source: treasury/v1/treasury.proto

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

// RemittanceRequest defines the input to a remittance operation in same or different currencies, including the recipient, the amount inclusive of currency in which remittance is desired, the promissory files to support the remittance and an optional exchange rate.
type RemittanceRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Recipient public key to remit the promissory files to.
	Recipient *v1.DynamicVerifier `protobuf:"bytes,1,opt,name=recipient,proto3" json:"recipient,omitempty"`
	// Total amount to remit, residual batch amounts returned to sender.
	Amount *v1.Amount `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
	// Batch of promissory files to be remitted; Promissory files need to be signed to treasury service. Promissory files of a batch must be of the same currency.
	PromissoryFiles [][]byte `protobuf:"bytes,3,rep,name=promissory_files,json=promissoryFiles,proto3" json:"promissory_files,omitempty"`
	// Optional exchange rate required to perform conversion, otherwise will use spot rate If the batch of promissory files is of a different currency than the currency of the remittance request, then the exchange rate is required to convert the promissory files to the currency of the remittance request.
	ExchangeRate float64 `protobuf:"fixed64,4,opt,name=exchange_rate,json=exchangeRate,proto3" json:"exchange_rate,omitempty"`
}

func (x *RemittanceRequest) Reset() {
	*x = RemittanceRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_treasury_v1_treasury_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *RemittanceRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RemittanceRequest) ProtoMessage() {}

func (x *RemittanceRequest) ProtoReflect() protoreflect.Message {
	mi := &file_treasury_v1_treasury_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RemittanceRequest.ProtoReflect.Descriptor instead.
func (*RemittanceRequest) Descriptor() ([]byte, []int) {
	return file_treasury_v1_treasury_proto_rawDescGZIP(), []int{0}
}

func (x *RemittanceRequest) GetRecipient() *v1.DynamicVerifier {
	if x != nil {
		return x.Recipient
	}
	return nil
}

func (x *RemittanceRequest) GetAmount() *v1.Amount {
	if x != nil {
		return x.Amount
	}
	return nil
}

func (x *RemittanceRequest) GetPromissoryFiles() [][]byte {
	if x != nil {
		return x.PromissoryFiles
	}
	return nil
}

func (x *RemittanceRequest) GetExchangeRate() float64 {
	if x != nil {
		return x.ExchangeRate
	}
	return 0
}

// RemittanceResponse is returned in response to a RemittanceRequest and includes the files sent to recipient at the applicable exchange rate, files representing a residual amount  left after the conversion if any and a fee.
type RemittanceResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Batch of exchanged promissory files remitted to the recipient of the request.
	ExchangedPromissoryFiles [][]byte `protobuf:"bytes,1,rep,name=exchanged_promissory_files,json=exchangedPromissoryFiles,proto3" json:"exchanged_promissory_files,omitempty"`
	// Transaction fee amount associated with remittance.
	FeeAmount *v1.Amount `protobuf:"bytes,2,opt,name=fee_amount,json=feeAmount,proto3" json:"fee_amount,omitempty"`
	// Exchange rate used to perform conversion, if foreign currency transaction.
	ExchangeRate float64 `protobuf:"fixed64,3,opt,name=exchange_rate,json=exchangeRate,proto3" json:"exchange_rate,omitempty"`
	// Batch of promissory files returned to the sender for (if any) residual amounts.
	RefundedPromissoryFiles [][]byte `protobuf:"bytes,4,rep,name=refunded_promissory_files,json=refundedPromissoryFiles,proto3" json:"refunded_promissory_files,omitempty"`
}

func (x *RemittanceResponse) Reset() {
	*x = RemittanceResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_treasury_v1_treasury_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *RemittanceResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*RemittanceResponse) ProtoMessage() {}

func (x *RemittanceResponse) ProtoReflect() protoreflect.Message {
	mi := &file_treasury_v1_treasury_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use RemittanceResponse.ProtoReflect.Descriptor instead.
func (*RemittanceResponse) Descriptor() ([]byte, []int) {
	return file_treasury_v1_treasury_proto_rawDescGZIP(), []int{1}
}

func (x *RemittanceResponse) GetExchangedPromissoryFiles() [][]byte {
	if x != nil {
		return x.ExchangedPromissoryFiles
	}
	return nil
}

func (x *RemittanceResponse) GetFeeAmount() *v1.Amount {
	if x != nil {
		return x.FeeAmount
	}
	return nil
}

func (x *RemittanceResponse) GetExchangeRate() float64 {
	if x != nil {
		return x.ExchangeRate
	}
	return 0
}

func (x *RemittanceResponse) GetRefundedPromissoryFiles() [][]byte {
	if x != nil {
		return x.RefundedPromissoryFiles
	}
	return nil
}

// GetSupplyRequest requests the count of files of various denominations for a given currency.
type GetSupplyRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The currency code of the requested supply.
	CurrencyCode string `protobuf:"bytes,1,opt,name=currency_code,json=currencyCode,proto3" json:"currency_code,omitempty"`
}

func (x *GetSupplyRequest) Reset() {
	*x = GetSupplyRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_treasury_v1_treasury_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetSupplyRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetSupplyRequest) ProtoMessage() {}

func (x *GetSupplyRequest) ProtoReflect() protoreflect.Message {
	mi := &file_treasury_v1_treasury_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetSupplyRequest.ProtoReflect.Descriptor instead.
func (*GetSupplyRequest) Descriptor() ([]byte, []int) {
	return file_treasury_v1_treasury_proto_rawDescGZIP(), []int{2}
}

func (x *GetSupplyRequest) GetCurrencyCode() string {
	if x != nil {
		return x.CurrencyCode
	}
	return ""
}

// The Denomination count object lists the count of the files of specific denominations held in the treasury vault, and the total value held in files of that denomination.
type DenominationCount struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The amount of the denomination.
	Denomination *v1.Amount `protobuf:"bytes,1,opt,name=denomination,proto3" json:"denomination,omitempty"`
	// The number of promissories of this amount held in supply.
	Count uint32 `protobuf:"varint,2,opt,name=count,proto3" json:"count,omitempty"`
	// The total value represented by promissories of this denomination.
	Total *v1.Amount `protobuf:"bytes,3,opt,name=total,proto3" json:"total,omitempty"`
}

func (x *DenominationCount) Reset() {
	*x = DenominationCount{}
	if protoimpl.UnsafeEnabled {
		mi := &file_treasury_v1_treasury_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *DenominationCount) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DenominationCount) ProtoMessage() {}

func (x *DenominationCount) ProtoReflect() protoreflect.Message {
	mi := &file_treasury_v1_treasury_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DenominationCount.ProtoReflect.Descriptor instead.
func (*DenominationCount) Descriptor() ([]byte, []int) {
	return file_treasury_v1_treasury_proto_rawDescGZIP(), []int{3}
}

func (x *DenominationCount) GetDenomination() *v1.Amount {
	if x != nil {
		return x.Denomination
	}
	return nil
}

func (x *DenominationCount) GetCount() uint32 {
	if x != nil {
		return x.Count
	}
	return 0
}

func (x *DenominationCount) GetTotal() *v1.Amount {
	if x != nil {
		return x.Total
	}
	return nil
}

// GetSupplyResponse provides a list of file denominations in the Treasury service's Vault.
type GetSupplyResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// A list of denomination counts of the promissories held in supply.
	Supply []*DenominationCount `protobuf:"bytes,1,rep,name=supply,proto3" json:"supply,omitempty"`
}

func (x *GetSupplyResponse) Reset() {
	*x = GetSupplyResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_treasury_v1_treasury_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetSupplyResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetSupplyResponse) ProtoMessage() {}

func (x *GetSupplyResponse) ProtoReflect() protoreflect.Message {
	mi := &file_treasury_v1_treasury_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use GetSupplyResponse.ProtoReflect.Descriptor instead.
func (*GetSupplyResponse) Descriptor() ([]byte, []int) {
	return file_treasury_v1_treasury_proto_rawDescGZIP(), []int{4}
}

func (x *GetSupplyResponse) GetSupply() []*DenominationCount {
	if x != nil {
		return x.Supply
	}
	return nil
}

var File_treasury_v1_treasury_proto protoreflect.FileDescriptor

var file_treasury_v1_treasury_proto_rawDesc = []byte{
	0x0a, 0x1a, 0x74, 0x72, 0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x72,
	0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x74, 0x72,
	0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x1a, 0x16, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76,
	0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc2,
	0x01, 0x0a, 0x11, 0x52, 0x65, 0x6d, 0x69, 0x74, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x52, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x12, 0x35, 0x0a, 0x09, 0x72, 0x65, 0x63, 0x69, 0x70, 0x69, 0x65, 0x6e,
	0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e,
	0x2e, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x56, 0x65, 0x72, 0x69, 0x66, 0x69, 0x65, 0x72,
	0x52, 0x09, 0x72, 0x65, 0x63, 0x69, 0x70, 0x69, 0x65, 0x6e, 0x74, 0x12, 0x26, 0x0a, 0x06, 0x61,
	0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63, 0x6f,
	0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x06, 0x61, 0x6d, 0x6f,
	0x75, 0x6e, 0x74, 0x12, 0x29, 0x0a, 0x10, 0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72,
	0x79, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x0f, 0x70,
	0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f, 0x72, 0x79, 0x46, 0x69, 0x6c, 0x65, 0x73, 0x12, 0x23,
	0x0a, 0x0d, 0x65, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x18,
	0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0c, 0x65, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x52,
	0x61, 0x74, 0x65, 0x22, 0xe2, 0x01, 0x0a, 0x12, 0x52, 0x65, 0x6d, 0x69, 0x74, 0x74, 0x61, 0x6e,
	0x63, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3c, 0x0a, 0x1a, 0x65, 0x78,
	0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73, 0x6f,
	0x72, 0x79, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x18,
	0x65, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x50, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73,
	0x6f, 0x72, 0x79, 0x46, 0x69, 0x6c, 0x65, 0x73, 0x12, 0x2d, 0x0a, 0x0a, 0x66, 0x65, 0x65, 0x5f,
	0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63,
	0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x09, 0x66, 0x65,
	0x65, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x65, 0x78, 0x63, 0x68, 0x61,
	0x6e, 0x67, 0x65, 0x5f, 0x72, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x52, 0x0c,
	0x65, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x61, 0x74, 0x65, 0x12, 0x3a, 0x0a, 0x19,
	0x72, 0x65, 0x66, 0x75, 0x6e, 0x64, 0x65, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73,
	0x6f, 0x72, 0x79, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0c, 0x52,
	0x17, 0x72, 0x65, 0x66, 0x75, 0x6e, 0x64, 0x65, 0x64, 0x50, 0x72, 0x6f, 0x6d, 0x69, 0x73, 0x73,
	0x6f, 0x72, 0x79, 0x46, 0x69, 0x6c, 0x65, 0x73, 0x22, 0x37, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x53,
	0x75, 0x70, 0x70, 0x6c, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x23, 0x0a, 0x0d,
	0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x63, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x0c, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x43, 0x6f, 0x64,
	0x65, 0x22, 0x83, 0x01, 0x0a, 0x11, 0x44, 0x65, 0x6e, 0x6f, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x69,
	0x6f, 0x6e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x32, 0x0a, 0x0c, 0x64, 0x65, 0x6e, 0x6f, 0x6d,
	0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e,
	0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x0c, 0x64,
	0x65, 0x6e, 0x6f, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x63,
	0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x05, 0x63, 0x6f, 0x75, 0x6e,
	0x74, 0x12, 0x24, 0x0a, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74,
	0x52, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x22, 0x48, 0x0a, 0x11, 0x47, 0x65, 0x74, 0x53, 0x75,
	0x70, 0x70, 0x6c, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x33, 0x0a, 0x06,
	0x73, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x74,
	0x72, 0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2e, 0x44, 0x65, 0x6e, 0x6f, 0x6d, 0x69, 0x6e, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x06, 0x73, 0x75, 0x70, 0x70, 0x6c,
	0x79, 0x32, 0x98, 0x01, 0x0a, 0x08, 0x54, 0x72, 0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x12, 0x46,
	0x0a, 0x05, 0x52, 0x65, 0x6d, 0x69, 0x74, 0x12, 0x1b, 0x2e, 0x74, 0x72, 0x65, 0x61, 0x73, 0x75,
	0x72, 0x79, 0x2e, 0x52, 0x65, 0x6d, 0x69, 0x74, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x52, 0x65, 0x71,
	0x75, 0x65, 0x73, 0x74, 0x1a, 0x1c, 0x2e, 0x74, 0x72, 0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2e,
	0x52, 0x65, 0x6d, 0x69, 0x74, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
	0x73, 0x65, 0x28, 0x01, 0x30, 0x01, 0x12, 0x44, 0x0a, 0x09, 0x47, 0x65, 0x74, 0x53, 0x75, 0x70,
	0x70, 0x6c, 0x79, 0x12, 0x1a, 0x2e, 0x74, 0x72, 0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2e, 0x47,
	0x65, 0x74, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
	0x1b, 0x2e, 0x74, 0x72, 0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x75,
	0x70, 0x70, 0x6c, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x37, 0x5a, 0x35,
	0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6b, 0x6e, 0x6f, 0x78, 0x2d,
	0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x2d, 0x73, 0x64,
	0x6b, 0x73, 0x2f, 0x73, 0x64, 0x6b, 0x2f, 0x67, 0x6f, 0x2f, 0x74, 0x72, 0x65, 0x61, 0x73, 0x75,
	0x72, 0x79, 0x2f, 0x76, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_treasury_v1_treasury_proto_rawDescOnce sync.Once
	file_treasury_v1_treasury_proto_rawDescData = file_treasury_v1_treasury_proto_rawDesc
)

func file_treasury_v1_treasury_proto_rawDescGZIP() []byte {
	file_treasury_v1_treasury_proto_rawDescOnce.Do(func() {
		file_treasury_v1_treasury_proto_rawDescData = protoimpl.X.CompressGZIP(file_treasury_v1_treasury_proto_rawDescData)
	})
	return file_treasury_v1_treasury_proto_rawDescData
}

var file_treasury_v1_treasury_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_treasury_v1_treasury_proto_goTypes = []interface{}{
	(*RemittanceRequest)(nil),  // 0: treasury.RemittanceRequest
	(*RemittanceResponse)(nil), // 1: treasury.RemittanceResponse
	(*GetSupplyRequest)(nil),   // 2: treasury.GetSupplyRequest
	(*DenominationCount)(nil),  // 3: treasury.DenominationCount
	(*GetSupplyResponse)(nil),  // 4: treasury.GetSupplyResponse
	(*v1.DynamicVerifier)(nil), // 5: common.DynamicVerifier
	(*v1.Amount)(nil),          // 6: common.Amount
}
var file_treasury_v1_treasury_proto_depIdxs = []int32{
	5, // 0: treasury.RemittanceRequest.recipient:type_name -> common.DynamicVerifier
	6, // 1: treasury.RemittanceRequest.amount:type_name -> common.Amount
	6, // 2: treasury.RemittanceResponse.fee_amount:type_name -> common.Amount
	6, // 3: treasury.DenominationCount.denomination:type_name -> common.Amount
	6, // 4: treasury.DenominationCount.total:type_name -> common.Amount
	3, // 5: treasury.GetSupplyResponse.supply:type_name -> treasury.DenominationCount
	0, // 6: treasury.Treasury.Remit:input_type -> treasury.RemittanceRequest
	2, // 7: treasury.Treasury.GetSupply:input_type -> treasury.GetSupplyRequest
	1, // 8: treasury.Treasury.Remit:output_type -> treasury.RemittanceResponse
	4, // 9: treasury.Treasury.GetSupply:output_type -> treasury.GetSupplyResponse
	8, // [8:10] is the sub-list for method output_type
	6, // [6:8] is the sub-list for method input_type
	6, // [6:6] is the sub-list for extension type_name
	6, // [6:6] is the sub-list for extension extendee
	0, // [0:6] is the sub-list for field type_name
}

func init() { file_treasury_v1_treasury_proto_init() }
func file_treasury_v1_treasury_proto_init() {
	if File_treasury_v1_treasury_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_treasury_v1_treasury_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*RemittanceRequest); i {
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
		file_treasury_v1_treasury_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*RemittanceResponse); i {
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
		file_treasury_v1_treasury_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetSupplyRequest); i {
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
		file_treasury_v1_treasury_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*DenominationCount); i {
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
		file_treasury_v1_treasury_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*GetSupplyResponse); i {
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
			RawDescriptor: file_treasury_v1_treasury_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_treasury_v1_treasury_proto_goTypes,
		DependencyIndexes: file_treasury_v1_treasury_proto_depIdxs,
		MessageInfos:      file_treasury_v1_treasury_proto_msgTypes,
	}.Build()
	File_treasury_v1_treasury_proto = out.File
	file_treasury_v1_treasury_proto_rawDesc = nil
	file_treasury_v1_treasury_proto_goTypes = nil
	file_treasury_v1_treasury_proto_depIdxs = nil
}
