"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1eregistry_api/v1/registry.proto\x12\x0fregistry_api.v1\x1a\x1fgoogle/protobuf/timestamp.proto".\n\rCreateRequest\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x10\n\x08document\x18\x02 \x01(\t"\x10\n\x0eCreateResponse"\x1a\n\x0bReadRequest\x12\x0b\n\x03did\x18\x01 \x01(\t"d\n\x0cReadResponse\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x10\n\x08document\x18\x02 \x01(\t\x125\n\x08metadata\x18\x03 \x01(\x0b2#.registry_api.v1.ResolutionMetadata"\x83\x01\n\x12ResolutionMetadata\x12+\n\x07created\x18\x01 \x01(\x0b2\x1a.google.protobuf.Timestamp\x12+\n\x07updated\x18\x02 \x01(\x0b2\x1a.google.protobuf.Timestamp\x12\x13\n\x0bcontentType\x18\x03 \x01(\t".\n\rUpdateRequest\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x10\n\x08document\x18\x02 \x01(\t"\x10\n\x0eUpdateResponse"\x1c\n\rRevokeRequest\x12\x0b\n\x03did\x18\x01 \x01(\t"\x10\n\x0eRevokeResponse2\xb0\x02\n\x08Registry\x12I\n\x06Create\x12\x1e.registry_api.v1.CreateRequest\x1a\x1f.registry_api.v1.CreateResponse\x12C\n\x04Read\x12\x1c.registry_api.v1.ReadRequest\x1a\x1d.registry_api.v1.ReadResponse\x12I\n\x06Update\x12\x1e.registry_api.v1.UpdateRequest\x1a\x1f.registry_api.v1.UpdateResponse\x12I\n\x06Revoke\x12\x1e.registry_api.v1.RevokeRequest\x1a\x1f.registry_api.v1.RevokeResponseB;Z9github.com/knox-networks/grpc-sdks/sdk/go/registry_api/v1b\x06proto3')
_CREATEREQUEST = DESCRIPTOR.message_types_by_name['CreateRequest']
_CREATERESPONSE = DESCRIPTOR.message_types_by_name['CreateResponse']
_READREQUEST = DESCRIPTOR.message_types_by_name['ReadRequest']
_READRESPONSE = DESCRIPTOR.message_types_by_name['ReadResponse']
_RESOLUTIONMETADATA = DESCRIPTOR.message_types_by_name['ResolutionMetadata']
_UPDATEREQUEST = DESCRIPTOR.message_types_by_name['UpdateRequest']
_UPDATERESPONSE = DESCRIPTOR.message_types_by_name['UpdateResponse']
_REVOKEREQUEST = DESCRIPTOR.message_types_by_name['RevokeRequest']
_REVOKERESPONSE = DESCRIPTOR.message_types_by_name['RevokeResponse']
CreateRequest = _reflection.GeneratedProtocolMessageType('CreateRequest', (_message.Message,), {'DESCRIPTOR': _CREATEREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(CreateRequest)
CreateResponse = _reflection.GeneratedProtocolMessageType('CreateResponse', (_message.Message,), {'DESCRIPTOR': _CREATERESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(CreateResponse)
ReadRequest = _reflection.GeneratedProtocolMessageType('ReadRequest', (_message.Message,), {'DESCRIPTOR': _READREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(ReadRequest)
ReadResponse = _reflection.GeneratedProtocolMessageType('ReadResponse', (_message.Message,), {'DESCRIPTOR': _READRESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(ReadResponse)
ResolutionMetadata = _reflection.GeneratedProtocolMessageType('ResolutionMetadata', (_message.Message,), {'DESCRIPTOR': _RESOLUTIONMETADATA, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(ResolutionMetadata)
UpdateRequest = _reflection.GeneratedProtocolMessageType('UpdateRequest', (_message.Message,), {'DESCRIPTOR': _UPDATEREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(UpdateRequest)
UpdateResponse = _reflection.GeneratedProtocolMessageType('UpdateResponse', (_message.Message,), {'DESCRIPTOR': _UPDATERESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(UpdateResponse)
RevokeRequest = _reflection.GeneratedProtocolMessageType('RevokeRequest', (_message.Message,), {'DESCRIPTOR': _REVOKEREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(RevokeRequest)
RevokeResponse = _reflection.GeneratedProtocolMessageType('RevokeResponse', (_message.Message,), {'DESCRIPTOR': _REVOKERESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(RevokeResponse)
_REGISTRY = DESCRIPTOR.services_by_name['Registry']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z9github.com/knox-networks/grpc-sdks/sdk/go/registry_api/v1'
    _CREATEREQUEST._serialized_start = 84
    _CREATEREQUEST._serialized_end = 130
    _CREATERESPONSE._serialized_start = 132
    _CREATERESPONSE._serialized_end = 148
    _READREQUEST._serialized_start = 150
    _READREQUEST._serialized_end = 176
    _READRESPONSE._serialized_start = 178
    _READRESPONSE._serialized_end = 278
    _RESOLUTIONMETADATA._serialized_start = 281
    _RESOLUTIONMETADATA._serialized_end = 412
    _UPDATEREQUEST._serialized_start = 414
    _UPDATEREQUEST._serialized_end = 460
    _UPDATERESPONSE._serialized_start = 462
    _UPDATERESPONSE._serialized_end = 478
    _REVOKEREQUEST._serialized_start = 480
    _REVOKEREQUEST._serialized_end = 508
    _REVOKERESPONSE._serialized_start = 510
    _REVOKERESPONSE._serialized_end = 526
    _REGISTRY._serialized_start = 529
    _REGISTRY._serialized_end = 833