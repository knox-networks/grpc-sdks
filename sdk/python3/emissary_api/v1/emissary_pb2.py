"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import packet_pb2 as common_dot_v1_dot_packet__pb2
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1eemissary_api/v1/emissary.proto\x12\x08emissary\x1a\x16common/v1/packet.proto\x1a\x16common/v1/common.proto"t\n!EmissaryConnectionIdentityRequest\x12)\n\x08verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x12$\n\x07service\x18\x02 \x01(\x0e2\x13.common.ServiceType2<\n\x08Emissary\x120\n\nConnection\x12\x0e.common.Packet\x1a\x0e.common.Packet(\x010\x01B;Z9github.com/knox-networks/grpc-sdks/sdk/go/emissary_api/v1b\x06proto3')
_EMISSARYCONNECTIONIDENTITYREQUEST = DESCRIPTOR.message_types_by_name['EmissaryConnectionIdentityRequest']
EmissaryConnectionIdentityRequest = _reflection.GeneratedProtocolMessageType('EmissaryConnectionIdentityRequest', (_message.Message,), {'DESCRIPTOR': _EMISSARYCONNECTIONIDENTITYREQUEST, '__module__': 'emissary_api.v1.emissary_pb2'})
_sym_db.RegisterMessage(EmissaryConnectionIdentityRequest)
_EMISSARY = DESCRIPTOR.services_by_name['Emissary']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z9github.com/knox-networks/grpc-sdks/sdk/go/emissary_api/v1'
    _EMISSARYCONNECTIONIDENTITYREQUEST._serialized_start = 92
    _EMISSARYCONNECTIONIDENTITYREQUEST._serialized_end = 208
    _EMISSARY._serialized_start = 210
    _EMISSARY._serialized_end = 270