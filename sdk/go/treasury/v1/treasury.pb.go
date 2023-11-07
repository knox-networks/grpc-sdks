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

// [Example]
// {
// "currency_code": "USD"
// }
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
		mi := &file_treasury_v1_treasury_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetSupplyRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetSupplyRequest) ProtoMessage() {}

func (x *GetSupplyRequest) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use GetSupplyRequest.ProtoReflect.Descriptor instead.
func (*GetSupplyRequest) Descriptor() ([]byte, []int) {
	return file_treasury_v1_treasury_proto_rawDescGZIP(), []int{0}
}

func (x *GetSupplyRequest) GetCurrencyCode() string {
	if x != nil {
		return x.CurrencyCode
	}
	return ""
}

// [Example]
// {
// "denomination": {
// "currency_code":"USD",
// "amount":2000,
// "decimals":2
// },
// "count": 5,
// "total": {
// "currency_code":"USD",
// "amount":10000,
// "decimals":2
// }
// }
type DenominationCount struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The amount of the denomination.
	Denomination *v1.Amount `protobuf:"bytes,1,opt,name=denomination,proto3" json:"denomination,omitempty"`
	// The number of digital banknotes of this amount held in supply.
	Count uint32 `protobuf:"varint,2,opt,name=count,proto3" json:"count,omitempty"`
	// The total value represented by digital banknotes of this denomination.
	Total *v1.Amount `protobuf:"bytes,3,opt,name=total,proto3" json:"total,omitempty"`
}

func (x *DenominationCount) Reset() {
	*x = DenominationCount{}
	if protoimpl.UnsafeEnabled {
		mi := &file_treasury_v1_treasury_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *DenominationCount) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DenominationCount) ProtoMessage() {}

func (x *DenominationCount) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use DenominationCount.ProtoReflect.Descriptor instead.
func (*DenominationCount) Descriptor() ([]byte, []int) {
	return file_treasury_v1_treasury_proto_rawDescGZIP(), []int{1}
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

// [Example]
// {
// "supply": [
// {
// "denomination": {
// "currency_code":"USD",
// "amount":2000,
// "decimals":2
// },
// "count": 5,
// "total": {
// "currency_code":"USD",
// "amount":10000, "decimals":2
// }
// },
// {
// "denomination": {
// "currency_code":"USD",
// "amount":1000,
// "decimals":2
// },
// "count": 5,
// "total": {
// "currency_code":"USD",
// "amount":5000,
// "decimals":2
// }
// }
// ],
// "total": {
// "currency_code":"USD",
// "amount": 15000,
// "decimals":2
// }
// }
type GetSupplyResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// A list of denomination counts of the digital banknotes held in supply.
	Supply []*DenominationCount `protobuf:"bytes,1,rep,name=supply,proto3" json:"supply,omitempty"`
	// grand total sum of all promissories
	Total *v1.Amount `protobuf:"bytes,2,opt,name=total,proto3" json:"total,omitempty"`
}

func (x *GetSupplyResponse) Reset() {
	*x = GetSupplyResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_treasury_v1_treasury_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *GetSupplyResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetSupplyResponse) ProtoMessage() {}

func (x *GetSupplyResponse) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use GetSupplyResponse.ProtoReflect.Descriptor instead.
func (*GetSupplyResponse) Descriptor() ([]byte, []int) {
	return file_treasury_v1_treasury_proto_rawDescGZIP(), []int{2}
}

func (x *GetSupplyResponse) GetSupply() []*DenominationCount {
	if x != nil {
		return x.Supply
	}
	return nil
}

func (x *GetSupplyResponse) GetTotal() *v1.Amount {
	if x != nil {
		return x.Total
	}
	return nil
}

// [Example]
// {
// "page": 0,
// "page_size": 10,
// "filters": {
// "filters": [
// {
// "field": "FIELD1",
// "operator": 0,
// "value": {
// "str_value": "STRINGVAL"
// }
// },
// {
// "field": "FIELD2",
// "operator": 0,
// "value": {
// "int_value": 10
// }
// },
// {
// "field": "FIELD3",
// "operator": 0,
// "value": {
// "bool_value": false
// }
// }
// ],
// "operator": 0
// },
// "sort_by": {
// "items": [
// {
// "field": "FIELD1",
// "direction": 0
// },
// {
// "field": "FIELD2",
// "direction": 0
// }
// ]
// }
// }
// }
type ListTransactionsRequest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Page     int32      `protobuf:"varint,1,opt,name=page,proto3" json:"page,omitempty"`
	PageSize int32      `protobuf:"varint,2,opt,name=page_size,json=pageSize,proto3" json:"page_size,omitempty"`
	SortBy   *v1.SortBy `protobuf:"bytes,3,opt,name=sort_by,json=sortBy,proto3" json:"sort_by,omitempty"`
	Filters  *v1.Filter `protobuf:"bytes,4,opt,name=filters,proto3" json:"filters,omitempty"`
}

func (x *ListTransactionsRequest) Reset() {
	*x = ListTransactionsRequest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_treasury_v1_treasury_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListTransactionsRequest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListTransactionsRequest) ProtoMessage() {}

func (x *ListTransactionsRequest) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use ListTransactionsRequest.ProtoReflect.Descriptor instead.
func (*ListTransactionsRequest) Descriptor() ([]byte, []int) {
	return file_treasury_v1_treasury_proto_rawDescGZIP(), []int{3}
}

func (x *ListTransactionsRequest) GetPage() int32 {
	if x != nil {
		return x.Page
	}
	return 0
}

func (x *ListTransactionsRequest) GetPageSize() int32 {
	if x != nil {
		return x.PageSize
	}
	return 0
}

func (x *ListTransactionsRequest) GetSortBy() *v1.SortBy {
	if x != nil {
		return x.SortBy
	}
	return nil
}

func (x *ListTransactionsRequest) GetFilters() *v1.Filter {
	if x != nil {
		return x.Filters
	}
	return nil
}

type ContractSummary struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// UUID of the contract
	ContractId string `protobuf:"bytes,1,opt,name=contract_id,json=contractId,proto3" json:"contract_id,omitempty"`
	// Type of the contract
	ContractType string `protobuf:"bytes,2,opt,name=contract_type,json=contractType,proto3" json:"contract_type,omitempty"`
	// RFC3339 timestamp of the contract creation date.
	Created string `protobuf:"bytes,3,opt,name=created,proto3" json:"created,omitempty"`
	// List of participant public keys excluding self
	Counterparties []string `protobuf:"bytes,4,rep,name=counterparties,proto3" json:"counterparties,omitempty"`
	// Contract memo
	Memo string `protobuf:"bytes,6,opt,name=memo,proto3" json:"memo,omitempty"`
}

func (x *ContractSummary) Reset() {
	*x = ContractSummary{}
	if protoimpl.UnsafeEnabled {
		mi := &file_treasury_v1_treasury_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ContractSummary) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ContractSummary) ProtoMessage() {}

func (x *ContractSummary) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use ContractSummary.ProtoReflect.Descriptor instead.
func (*ContractSummary) Descriptor() ([]byte, []int) {
	return file_treasury_v1_treasury_proto_rawDescGZIP(), []int{4}
}

func (x *ContractSummary) GetContractId() string {
	if x != nil {
		return x.ContractId
	}
	return ""
}

func (x *ContractSummary) GetContractType() string {
	if x != nil {
		return x.ContractType
	}
	return ""
}

func (x *ContractSummary) GetCreated() string {
	if x != nil {
		return x.Created
	}
	return ""
}

func (x *ContractSummary) GetCounterparties() []string {
	if x != nil {
		return x.Counterparties
	}
	return nil
}

func (x *ContractSummary) GetMemo() string {
	if x != nil {
		return x.Memo
	}
	return ""
}

// [Example]
// {
// "contracts": [
// {
// "contract_id": "9628a2d9-f932-41f0-b2da-b9de3103d945",
// "contract_type": "Relay",
// "created": "2022-12-19T16:39:57-08:00",
// "counterparties": [
// "zCounterpartyPublicKeyMultibase58Encoded",
// "zCounterpartyPublicKey2Multibase58Encoded"
// ],
// "memo": "Payment"
// }
// ],
// "totalContracts": 1
// }
type ListTransactionsResponse struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// List of contracts with status metadata
	Contracts []*ContractSummary `protobuf:"bytes,1,rep,name=contracts,proto3" json:"contracts,omitempty"`
	// Total number of contracts in paginated set
	TotalContracts uint32 `protobuf:"varint,2,opt,name=totalContracts,proto3" json:"totalContracts,omitempty"`
}

func (x *ListTransactionsResponse) Reset() {
	*x = ListTransactionsResponse{}
	if protoimpl.UnsafeEnabled {
		mi := &file_treasury_v1_treasury_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ListTransactionsResponse) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ListTransactionsResponse) ProtoMessage() {}

func (x *ListTransactionsResponse) ProtoReflect() protoreflect.Message {
	mi := &file_treasury_v1_treasury_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ListTransactionsResponse.ProtoReflect.Descriptor instead.
func (*ListTransactionsResponse) Descriptor() ([]byte, []int) {
	return file_treasury_v1_treasury_proto_rawDescGZIP(), []int{5}
}

func (x *ListTransactionsResponse) GetContracts() []*ContractSummary {
	if x != nil {
		return x.Contracts
	}
	return nil
}

func (x *ListTransactionsResponse) GetTotalContracts() uint32 {
	if x != nil {
		return x.TotalContracts
	}
	return 0
}

var File_treasury_v1_treasury_proto protoreflect.FileDescriptor

var file_treasury_v1_treasury_proto_rawDesc = []byte{
	0x0a, 0x1a, 0x74, 0x72, 0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x72,
	0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x74, 0x72,
	0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x1a, 0x16, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76,
	0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x37,
	0x0a, 0x10, 0x47, 0x65, 0x74, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x52, 0x65, 0x71, 0x75, 0x65,
	0x73, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x63, 0x79, 0x5f, 0x63,
	0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x63, 0x75, 0x72, 0x72, 0x65,
	0x6e, 0x63, 0x79, 0x43, 0x6f, 0x64, 0x65, 0x22, 0x83, 0x01, 0x0a, 0x11, 0x44, 0x65, 0x6e, 0x6f,
	0x6d, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x32, 0x0a,
	0x0c, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x41, 0x6d, 0x6f,
	0x75, 0x6e, 0x74, 0x52, 0x0c, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x12, 0x14, 0x0a, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
	0x52, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x24, 0x0a, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c,
	0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
	0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x22, 0x6e, 0x0a,
	0x11, 0x47, 0x65, 0x74, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
	0x73, 0x65, 0x12, 0x33, 0x0a, 0x06, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x18, 0x01, 0x20, 0x03,
	0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x74, 0x72, 0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2e, 0x44, 0x65,
	0x6e, 0x6f, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x52,
	0x06, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x12, 0x24, 0x0a, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
	0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x52, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x22, 0x9d, 0x01,
	0x0a, 0x17, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
	0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x61, 0x67,
	0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x05, 0x52, 0x04, 0x70, 0x61, 0x67, 0x65, 0x12, 0x1b, 0x0a,
	0x09, 0x70, 0x61, 0x67, 0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x05,
	0x52, 0x08, 0x70, 0x61, 0x67, 0x65, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x27, 0x0a, 0x07, 0x73, 0x6f,
	0x72, 0x74, 0x5f, 0x62, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63, 0x6f,
	0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x53, 0x6f, 0x72, 0x74, 0x42, 0x79, 0x52, 0x06, 0x73, 0x6f, 0x72,
	0x74, 0x42, 0x79, 0x12, 0x28, 0x0a, 0x07, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x18, 0x04,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x46, 0x69,
	0x6c, 0x74, 0x65, 0x72, 0x52, 0x07, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x22, 0xad, 0x01,
	0x0a, 0x0f, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x53, 0x75, 0x6d, 0x6d, 0x61, 0x72,
	0x79, 0x12, 0x1f, 0x0a, 0x0b, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x5f, 0x69, 0x64,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74,
	0x49, 0x64, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x5f, 0x74,
	0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x63, 0x6f, 0x6e, 0x74, 0x72,
	0x61, 0x63, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x72, 0x65, 0x61, 0x74,
	0x65, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65,
	0x64, 0x12, 0x26, 0x0a, 0x0e, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x61, 0x72, 0x74,
	0x69, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0e, 0x63, 0x6f, 0x75, 0x6e, 0x74,
	0x65, 0x72, 0x70, 0x61, 0x72, 0x74, 0x69, 0x65, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x6d, 0x65, 0x6d,
	0x6f, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6d, 0x65, 0x6d, 0x6f, 0x22, 0x7b, 0x0a,
	0x18, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
	0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x37, 0x0a, 0x09, 0x63, 0x6f, 0x6e,
	0x74, 0x72, 0x61, 0x63, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x74,
	0x72, 0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74,
	0x53, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x52, 0x09, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63,
	0x74, 0x73, 0x12, 0x26, 0x0a, 0x0e, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x43, 0x6f, 0x6e, 0x74, 0x72,
	0x61, 0x63, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0e, 0x74, 0x6f, 0x74, 0x61,
	0x6c, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x73, 0x32, 0xb2, 0x01, 0x0a, 0x0f, 0x4d,
	0x6f, 0x6e, 0x65, 0x74, 0x61, 0x65, 0x54, 0x72, 0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x12, 0x44,
	0x0a, 0x09, 0x47, 0x65, 0x74, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x12, 0x1a, 0x2e, 0x74, 0x72,
	0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x79,
	0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x1b, 0x2e, 0x74, 0x72, 0x65, 0x61, 0x73, 0x75,
	0x72, 0x79, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x52, 0x65, 0x73, 0x70,
	0x6f, 0x6e, 0x73, 0x65, 0x12, 0x59, 0x0a, 0x10, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x72, 0x61, 0x6e,
	0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x21, 0x2e, 0x74, 0x72, 0x65, 0x61, 0x73,
	0x75, 0x72, 0x79, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
	0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x22, 0x2e, 0x74, 0x72,
	0x65, 0x61, 0x73, 0x75, 0x72, 0x79, 0x2e, 0x4c, 0x69, 0x73, 0x74, 0x54, 0x72, 0x61, 0x6e, 0x73,
	0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42,
	0x37, 0x5a, 0x35, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6b, 0x6e,
	0x6f, 0x78, 0x2d, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x2f, 0x67, 0x72, 0x70, 0x63,
	0x2d, 0x73, 0x64, 0x6b, 0x73, 0x2f, 0x73, 0x64, 0x6b, 0x2f, 0x67, 0x6f, 0x2f, 0x74, 0x72, 0x65,
	0x61, 0x73, 0x75, 0x72, 0x79, 0x2f, 0x76, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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

var file_treasury_v1_treasury_proto_msgTypes = make([]protoimpl.MessageInfo, 6)
var file_treasury_v1_treasury_proto_goTypes = []interface{}{
	(*GetSupplyRequest)(nil),         // 0: treasury.GetSupplyRequest
	(*DenominationCount)(nil),        // 1: treasury.DenominationCount
	(*GetSupplyResponse)(nil),        // 2: treasury.GetSupplyResponse
	(*ListTransactionsRequest)(nil),  // 3: treasury.ListTransactionsRequest
	(*ContractSummary)(nil),          // 4: treasury.ContractSummary
	(*ListTransactionsResponse)(nil), // 5: treasury.ListTransactionsResponse
	(*v1.Amount)(nil),                // 6: common.Amount
	(*v1.SortBy)(nil),                // 7: common.SortBy
	(*v1.Filter)(nil),                // 8: common.Filter
}
var file_treasury_v1_treasury_proto_depIdxs = []int32{
	6, // 0: treasury.DenominationCount.denomination:type_name -> common.Amount
	6, // 1: treasury.DenominationCount.total:type_name -> common.Amount
	1, // 2: treasury.GetSupplyResponse.supply:type_name -> treasury.DenominationCount
	6, // 3: treasury.GetSupplyResponse.total:type_name -> common.Amount
	7, // 4: treasury.ListTransactionsRequest.sort_by:type_name -> common.SortBy
	8, // 5: treasury.ListTransactionsRequest.filters:type_name -> common.Filter
	4, // 6: treasury.ListTransactionsResponse.contracts:type_name -> treasury.ContractSummary
	0, // 7: treasury.MonetaeTreasury.GetSupply:input_type -> treasury.GetSupplyRequest
	3, // 8: treasury.MonetaeTreasury.ListTransactions:input_type -> treasury.ListTransactionsRequest
	2, // 9: treasury.MonetaeTreasury.GetSupply:output_type -> treasury.GetSupplyResponse
	5, // 10: treasury.MonetaeTreasury.ListTransactions:output_type -> treasury.ListTransactionsResponse
	9, // [9:11] is the sub-list for method output_type
	7, // [7:9] is the sub-list for method input_type
	7, // [7:7] is the sub-list for extension type_name
	7, // [7:7] is the sub-list for extension extendee
	0, // [0:7] is the sub-list for field type_name
}

func init() { file_treasury_v1_treasury_proto_init() }
func file_treasury_v1_treasury_proto_init() {
	if File_treasury_v1_treasury_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_treasury_v1_treasury_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
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
		file_treasury_v1_treasury_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
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
		file_treasury_v1_treasury_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
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
		file_treasury_v1_treasury_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ListTransactionsRequest); i {
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
			switch v := v.(*ContractSummary); i {
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
		file_treasury_v1_treasury_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ListTransactionsResponse); i {
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
			NumMessages:   6,
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
