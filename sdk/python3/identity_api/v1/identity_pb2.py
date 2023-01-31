"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1eidentity_api/v1/identity.proto\x12\x08identity\x1a\x16common/v1/common.proto"1\n\x0fIdentityRequest\x12\x0f\n\x07subject\x18\x01 \x01(\x0c\x12\r\n\x05query\x18\x02 \x01(\t"A\n\x10IdentityResponse\x12\x0f\n\x07subject\x18\x01 \x01(\x0c\x12\x1c\n\x05media\x18\x02 \x03(\x0b2\r.common.Media"\x14\n\x12ConnectionIdentityB;Z9github.com/knox-networks/grpc-sdks/sdk/go/identity_api/v1b\x06proto3')
_IDENTITYREQUEST = DESCRIPTOR.message_types_by_name['IdentityRequest']
_IDENTITYRESPONSE = DESCRIPTOR.message_types_by_name['IdentityResponse']
_CONNECTIONIDENTITY = DESCRIPTOR.message_types_by_name['ConnectionIdentity']
IdentityRequest = _reflection.GeneratedProtocolMessageType('IdentityRequest', (_message.Message,), {'DESCRIPTOR': _IDENTITYREQUEST, '__module__': 'identity_api.v1.identity_pb2'})
_sym_db.RegisterMessage(IdentityRequest)
IdentityResponse = _reflection.GeneratedProtocolMessageType('IdentityResponse', (_message.Message,), {'DESCRIPTOR': _IDENTITYRESPONSE, '__module__': 'identity_api.v1.identity_pb2'})
_sym_db.RegisterMessage(IdentityResponse)
ConnectionIdentity = _reflection.GeneratedProtocolMessageType('ConnectionIdentity', (_message.Message,), {'DESCRIPTOR': _CONNECTIONIDENTITY, '__module__': 'identity_api.v1.identity_pb2'})
_sym_db.RegisterMessage(ConnectionIdentity)
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z9github.com/knox-networks/grpc-sdks/sdk/go/identity_api/v1'
    _IDENTITYREQUEST._serialized_start = 68
    _IDENTITYREQUEST._serialized_end = 117
    _IDENTITYRESPONSE._serialized_start = 119
    _IDENTITYRESPONSE._serialized_end = 184
    _CONNECTIONIDENTITY._serialized_start = 186
    _CONNECTIONIDENTITY._serialized_end = 206