"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1atreasury/v1/treasury.proto\x12\x08treasury\x1a\x16common/v1/common.proto"\x90\x01\n\x11RemittanceRequest\x12*\n\trecipient\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x12\x1e\n\x06amount\x18\x02 \x01(\x0b2\x0e.common.Amount\x12\x18\n\x10promissory_files\x18\x03 \x03(\x0c\x12\x15\n\rexchange_rate\x18\x04 \x01(\x01"\x96\x01\n\x12RemittanceResponse\x12"\n\x1aexchanged_promissory_files\x18\x01 \x03(\x0c\x12"\n\nfee_amount\x18\x02 \x01(\x0b2\x0e.common.Amount\x12\x15\n\rexchange_rate\x18\x03 \x01(\x01\x12!\n\x19refunded_promissory_files\x18\x04 \x03(\x0c")\n\x10GetSupplyRequest\x12\x15\n\rcurrency_code\x18\x01 \x01(\t"g\n\x11DenominationCount\x12$\n\x0cdenomination\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\r\n\x05count\x18\x02 \x01(\r\x12\x1d\n\x05total\x18\x03 \x01(\x0b2\x0e.common.Amount"@\n\x11GetSupplyResponse\x12+\n\x06supply\x18\x01 \x03(\x0b2\x1b.treasury.DenominationCount2\x98\x01\n\x08Treasury\x12F\n\x05Remit\x12\x1b.treasury.RemittanceRequest\x1a\x1c.treasury.RemittanceResponse(\x010\x01\x12D\n\tGetSupply\x12\x1a.treasury.GetSupplyRequest\x1a\x1b.treasury.GetSupplyResponseB7Z5github.com/knox-networks/grpc-sdks/sdk/go/treasury/v1b\x06proto3')
_REMITTANCEREQUEST = DESCRIPTOR.message_types_by_name['RemittanceRequest']
_REMITTANCERESPONSE = DESCRIPTOR.message_types_by_name['RemittanceResponse']
_GETSUPPLYREQUEST = DESCRIPTOR.message_types_by_name['GetSupplyRequest']
_DENOMINATIONCOUNT = DESCRIPTOR.message_types_by_name['DenominationCount']
_GETSUPPLYRESPONSE = DESCRIPTOR.message_types_by_name['GetSupplyResponse']
RemittanceRequest = _reflection.GeneratedProtocolMessageType('RemittanceRequest', (_message.Message,), {'DESCRIPTOR': _REMITTANCEREQUEST, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(RemittanceRequest)
RemittanceResponse = _reflection.GeneratedProtocolMessageType('RemittanceResponse', (_message.Message,), {'DESCRIPTOR': _REMITTANCERESPONSE, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(RemittanceResponse)
GetSupplyRequest = _reflection.GeneratedProtocolMessageType('GetSupplyRequest', (_message.Message,), {'DESCRIPTOR': _GETSUPPLYREQUEST, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(GetSupplyRequest)
DenominationCount = _reflection.GeneratedProtocolMessageType('DenominationCount', (_message.Message,), {'DESCRIPTOR': _DENOMINATIONCOUNT, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(DenominationCount)
GetSupplyResponse = _reflection.GeneratedProtocolMessageType('GetSupplyResponse', (_message.Message,), {'DESCRIPTOR': _GETSUPPLYRESPONSE, '__module__': 'treasury.v1.treasury_pb2'})
_sym_db.RegisterMessage(GetSupplyResponse)
_TREASURY = DESCRIPTOR.services_by_name['Treasury']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z5github.com/knox-networks/grpc-sdks/sdk/go/treasury/v1'
    _REMITTANCEREQUEST._serialized_start = 65
    _REMITTANCEREQUEST._serialized_end = 209
    _REMITTANCERESPONSE._serialized_start = 212
    _REMITTANCERESPONSE._serialized_end = 362
    _GETSUPPLYREQUEST._serialized_start = 364
    _GETSUPPLYREQUEST._serialized_end = 405
    _DENOMINATIONCOUNT._serialized_start = 407
    _DENOMINATIONCOUNT._serialized_end = 510
    _GETSUPPLYRESPONSE._serialized_start = 512
    _GETSUPPLYRESPONSE._serialized_end = 576
    _TREASURY._serialized_start = 579
    _TREASURY._serialized_end = 731