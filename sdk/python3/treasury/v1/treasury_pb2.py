"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1atreasury/v1/treasury.proto\x12\x08treasury\x1a\x16common/v1/common.proto")\n\x10GetSupplyRequest\x12\x15\n\rcurrency_code\x18\x01 \x01(\t"g\n\x11DenominationCount\x12$\n\x0cdenomination\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\r\n\x05count\x18\x02 \x01(\r\x12\x1d\n\x05total\x18\x03 \x01(\x0b2\x0e.common.Amount"_\n\x11GetSupplyResponse\x12+\n\x06supply\x18\x01 \x03(\x0b2\x1b.treasury.DenominationCount\x12\x1d\n\x05total\x18\x02 \x01(\x0b2\x0e.common.Amount2W\n\x0fMonetaeTreasury\x12D\n\tGetSupply\x12\x1a.treasury.GetSupplyRequest\x1a\x1b.treasury.GetSupplyResponseB7Z5github.com/knox-networks/grpc-sdks/sdk/go/treasury/v1b\x06proto3')
_GETSUPPLYREQUEST = DESCRIPTOR.message_types_by_name['GetSupplyRequest']
_DENOMINATIONCOUNT = DESCRIPTOR.message_types_by_name['DenominationCount']
_GETSUPPLYRESPONSE = DESCRIPTOR.message_types_by_name['GetSupplyResponse']
GetSupplyRequest = _reflection.GeneratedProtocolMessageType('GetSupplyRequest', (_message.Message,), {'DESCRIPTOR': _GETSUPPLYREQUEST, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(GetSupplyRequest)
DenominationCount = _reflection.GeneratedProtocolMessageType('DenominationCount', (_message.Message,), {'DESCRIPTOR': _DENOMINATIONCOUNT, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(DenominationCount)
GetSupplyResponse = _reflection.GeneratedProtocolMessageType('GetSupplyResponse', (_message.Message,), {'DESCRIPTOR': _GETSUPPLYRESPONSE, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(GetSupplyResponse)
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
    _MONETAETREASURY._serialized_start = 309
    _MONETAETREASURY._serialized_end = 396