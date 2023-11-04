"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1atreasury/v1/treasury.proto\x12\x08treasury\x1a\x16common/v1/common.proto")\n\x10GetSupplyRequest\x12\x15\n\rcurrency_code\x18\x01 \x01(\t"g\n\x11DenominationCount\x12$\n\x0cdenomination\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\r\n\x05count\x18\x02 \x01(\r\x12\x1d\n\x05total\x18\x03 \x01(\x0b2\x0e.common.Amount"_\n\x11GetSupplyResponse\x12+\n\x06supply\x18\x01 \x03(\x0b2\x1b.treasury.DenominationCount\x12\x1d\n\x05total\x18\x02 \x01(\x0b2\x0e.common.Amount"|\n\x17ListTransactionsRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x1f\n\x07sort_by\x18\x03 \x01(\x0b2\x0e.common.SortBy\x12\x1f\n\x07filters\x18\x04 \x01(\x0b2\x0e.common.Filter"t\n\x0fContractSummary\x12\x13\n\x0bcontract_id\x18\x01 \x01(\t\x12\x15\n\rcontract_type\x18\x02 \x01(\t\x12\x0f\n\x07created\x18\x03 \x01(\t\x12\x16\n\x0ecounterparties\x18\x04 \x03(\t\x12\x0c\n\x04memo\x18\x06 \x01(\t"`\n\x18ListTransactionsResponse\x12,\n\tcontracts\x18\x01 \x03(\x0b2\x19.treasury.ContractSummary\x12\x16\n\x0etotalContracts\x18\x02 \x01(\r2\xb2\x01\n\x0fMonetaeTreasury\x12D\n\tGetSupply\x12\x1a.treasury.GetSupplyRequest\x1a\x1b.treasury.GetSupplyResponse\x12Y\n\x10ListTransactions\x12!.treasury.ListTransactionsRequest\x1a".treasury.ListTransactionsResponseB7Z5github.com/knox-networks/grpc-sdks/sdk/go/treasury/v1b\x06proto3')
_GETSUPPLYREQUEST = DESCRIPTOR.message_types_by_name['GetSupplyRequest']
_DENOMINATIONCOUNT = DESCRIPTOR.message_types_by_name['DenominationCount']
_GETSUPPLYRESPONSE = DESCRIPTOR.message_types_by_name['GetSupplyResponse']
_LISTTRANSACTIONSREQUEST = DESCRIPTOR.message_types_by_name['ListTransactionsRequest']
_CONTRACTSUMMARY = DESCRIPTOR.message_types_by_name['ContractSummary']
_LISTTRANSACTIONSRESPONSE = DESCRIPTOR.message_types_by_name['ListTransactionsResponse']
GetSupplyRequest = _reflection.GeneratedProtocolMessageType('GetSupplyRequest', (_message.Message,), {'DESCRIPTOR': _GETSUPPLYREQUEST, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(GetSupplyRequest)
DenominationCount = _reflection.GeneratedProtocolMessageType('DenominationCount', (_message.Message,), {'DESCRIPTOR': _DENOMINATIONCOUNT, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(DenominationCount)
GetSupplyResponse = _reflection.GeneratedProtocolMessageType('GetSupplyResponse', (_message.Message,), {'DESCRIPTOR': _GETSUPPLYRESPONSE, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(GetSupplyResponse)
ListTransactionsRequest = _reflection.GeneratedProtocolMessageType('ListTransactionsRequest', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSREQUEST, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(ListTransactionsRequest)
ContractSummary = _reflection.GeneratedProtocolMessageType('ContractSummary', (_message.Message,), {'DESCRIPTOR': _CONTRACTSUMMARY, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(ContractSummary)
ListTransactionsResponse = _reflection.GeneratedProtocolMessageType('ListTransactionsResponse', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSRESPONSE, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(ListTransactionsResponse)
_MONETAETREASURY = DESCRIPTOR.services_by_name['MonetaeTreasury']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z5github.com/knox-networks/grpc-sdks/sdk/go/treasury/v1'
    _GETSUPPLYREQUEST._serialized_start = 64
    _GETSUPPLYREQUEST._serialized_end = 105
    _DENOMINATIONCOUNT._serialized_start = 107
    _DENOMINATIONCOUNT._serialized_end = 210
    _GETSUPPLYRESPONSE._serialized_start = 212
    _GETSUPPLYRESPONSE._serialized_end = 307
    _LISTTRANSACTIONSREQUEST._serialized_start = 309
    _LISTTRANSACTIONSREQUEST._serialized_end = 433
    _CONTRACTSUMMARY._serialized_start = 435
    _CONTRACTSUMMARY._serialized_end = 551
    _LISTTRANSACTIONSRESPONSE._serialized_start = 553
    _LISTTRANSACTIONSRESPONSE._serialized_end = 649
    _MONETAETREASURY._serialized_start = 652
    _MONETAETREASURY._serialized_end = 830