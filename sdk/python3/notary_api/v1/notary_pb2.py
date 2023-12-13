"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1anotary_api/v1/notary.proto\x12\x06notary\x1a\x16common/v1/common.proto"\xb2\x01\n\x1aSetAssetAuthoritiesRequest\x12H\n\x0bauthorities\x18\x01 \x03(\x0b23.notary.SetAssetAuthoritiesRequest.AuthoritiesEntry\x1aJ\n\x10AuthoritiesEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12%\n\x05value\x18\x02 \x01(\x0b2\x16.common.AssetAuthority:\x028\x01"\x1d\n\x1bSetAssetAuthoritiesResponse2o\n\rMonetaeNotary\x12^\n\x13SetAssetAuthorities\x12".notary.SetAssetAuthoritiesRequest\x1a#.notary.SetAssetAuthoritiesResponseB9Z7github.com/knox-networks/grpc-sdks/sdk/go/notary_api/v1b\x06proto3')
_SETASSETAUTHORITIESREQUEST = DESCRIPTOR.message_types_by_name['SetAssetAuthoritiesRequest']
_SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY = _SETASSETAUTHORITIESREQUEST.nested_types_by_name['AuthoritiesEntry']
_SETASSETAUTHORITIESRESPONSE = DESCRIPTOR.message_types_by_name['SetAssetAuthoritiesResponse']
SetAssetAuthoritiesRequest = _reflection.GeneratedProtocolMessageType('SetAssetAuthoritiesRequest', (_message.Message,), {'AuthoritiesEntry': _reflection.GeneratedProtocolMessageType('AuthoritiesEntry', (_message.Message,), {'DESCRIPTOR': _SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY, '__module__': 'notary_api.v1.notary_pb2'}), 'DESCRIPTOR': _SETASSETAUTHORITIESREQUEST, '__module__': 'notary_api.v1.notary_pb2'})
_sym_db.RegisterMessage(SetAssetAuthoritiesRequest)
_sym_db.RegisterMessage(SetAssetAuthoritiesRequest.AuthoritiesEntry)
SetAssetAuthoritiesResponse = _reflection.GeneratedProtocolMessageType('SetAssetAuthoritiesResponse', (_message.Message,), {'DESCRIPTOR': _SETASSETAUTHORITIESRESPONSE, '__module__': 'notary_api.v1.notary_pb2'})
_sym_db.RegisterMessage(SetAssetAuthoritiesResponse)
_MONETAENOTARY = DESCRIPTOR.services_by_name['MonetaeNotary']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z7github.com/knox-networks/grpc-sdks/sdk/go/notary_api/v1'
    _SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY._options = None
    _SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY._serialized_options = b'8\x01'
    _SETASSETAUTHORITIESREQUEST._serialized_start = 63
    _SETASSETAUTHORITIESREQUEST._serialized_end = 241
    _SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY._serialized_start = 167
    _SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY._serialized_end = 241
    _SETASSETAUTHORITIESRESPONSE._serialized_start = 243
    _SETASSETAUTHORITIESRESPONSE._serialized_end = 272
    _MONETAENOTARY._serialized_start = 274
    _MONETAENOTARY._serialized_end = 385