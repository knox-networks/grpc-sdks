"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1afaucet_api/v1/faucet.proto\x12\rfaucet_api.v1\x1a\x16common/v1/common.proto"\x84\x01\n\rFaucetRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x11\n\trecipient\x18\x02 \x01(\t\x12/\n\x0cdistribution\x18\x03 \x01(\x0b2\x14.common.DistributionH\x00\x88\x01\x01B\x0f\n\r_distribution".\n\x0eFaucetResponse\x12\x1c\n\x14num_promissory_files\x18\x01 \x01(\r2O\n\x06Faucet\x12E\n\x06Faucet\x12\x1c.faucet_api.v1.FaucetRequest\x1a\x1d.faucet_api.v1.FaucetResponseB9Z7github.com/knox-networks/grpc-sdks/sdk/go/faucet_api/v1b\x06proto3')
_FAUCETREQUEST = DESCRIPTOR.message_types_by_name['FaucetRequest']
_FAUCETRESPONSE = DESCRIPTOR.message_types_by_name['FaucetResponse']
FaucetRequest = _reflection.GeneratedProtocolMessageType('FaucetRequest', (_message.Message,), {'DESCRIPTOR': _FAUCETREQUEST, '__module__': 'faucet_api.v1.faucet_pb2'})
_sym_db.RegisterMessage(FaucetRequest)
FaucetResponse = _reflection.GeneratedProtocolMessageType('FaucetResponse', (_message.Message,), {'DESCRIPTOR': _FAUCETRESPONSE, '__module__': 'faucet_api.v1.faucet_pb2'})
_sym_db.RegisterMessage(FaucetResponse)
_FAUCET = DESCRIPTOR.services_by_name['Faucet']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z7github.com/knox-networks/grpc-sdks/sdk/go/faucet_api/v1'
    _FAUCETREQUEST._serialized_start = 70
    _FAUCETREQUEST._serialized_end = 202
    _FAUCETRESPONSE._serialized_start = 204
    _FAUCETRESPONSE._serialized_end = 250
    _FAUCET._serialized_start = 252
    _FAUCET._serialized_end = 331