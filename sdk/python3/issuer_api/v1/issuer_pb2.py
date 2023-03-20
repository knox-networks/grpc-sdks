"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1aissuer_api/v1/issuer.proto\x12\rissuer_api.v1\x1a\x16common/v1/common.proto"\xdf\x01\n\x0cIssueRequest\x12A\n\x18authentication_signature\x18\x01 \x01(\x0b2\x1f.common.AuthenticationSignature\x12\x1e\n\x06amount\x18\x02 \x01(\x0b2\x0e.common.Amount\x12*\n\tauthority\x18\x03 \x01(\x0b2\x17.common.DynamicVerifier\x12/\n\x0cdistribution\x18\x04 \x01(\x0b2\x14.common.DistributionH\x00\x88\x01\x01B\x0f\n\r_distribution"(\n\rIssueResponse\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c"+\n\x10AuthorizeRequest\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c",\n\x11AuthorizeResponse\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c"@\n\x13AuthenticateRequest\x12)\n\x08verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier")\n\x14AuthenticateResponse\x12\x11\n\tchallenge\x18\x01 \x01(\x0c"\x11\n\x0fGetRolesRequest"{\n\x10GetRolesResponse\x129\n\x05roles\x18\x01 \x03(\x0b2*.issuer_api.v1.GetRolesResponse.RolesEntry\x1a,\n\nRolesEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\t:\x028\x01",\n\x0eSetRoleRequest\x12\x1a\n\x04role\x18\x01 \x01(\x0b2\x0c.common.Role"\x11\n\x0fSetRoleResponse2\x92\x03\n\x06Issuer\x12W\n\x0cAuthenticate\x12".issuer_api.v1.AuthenticateRequest\x1a#.issuer_api.v1.AuthenticateResponse\x12R\n\tAuthorize\x12\x1f.issuer_api.v1.AuthorizeRequest\x1a .issuer_api.v1.AuthorizeResponse(\x010\x01\x12D\n\x05Issue\x12\x1b.issuer_api.v1.IssueRequest\x1a\x1c.issuer_api.v1.IssueResponse0\x01\x12K\n\x08GetRoles\x12\x1e.issuer_api.v1.GetRolesRequest\x1a\x1f.issuer_api.v1.GetRolesResponse\x12H\n\x07SetRole\x12\x1d.issuer_api.v1.SetRoleRequest\x1a\x1e.issuer_api.v1.SetRoleResponseB9Z7github.com/knox-networks/grpc-sdks/sdk/go/issuer_api/v1b\x06proto3')
_ISSUEREQUEST = DESCRIPTOR.message_types_by_name['IssueRequest']
_ISSUERESPONSE = DESCRIPTOR.message_types_by_name['IssueResponse']
_AUTHORIZEREQUEST = DESCRIPTOR.message_types_by_name['AuthorizeRequest']
_AUTHORIZERESPONSE = DESCRIPTOR.message_types_by_name['AuthorizeResponse']
_AUTHENTICATEREQUEST = DESCRIPTOR.message_types_by_name['AuthenticateRequest']
_AUTHENTICATERESPONSE = DESCRIPTOR.message_types_by_name['AuthenticateResponse']
_GETROLESREQUEST = DESCRIPTOR.message_types_by_name['GetRolesRequest']
_GETROLESRESPONSE = DESCRIPTOR.message_types_by_name['GetRolesResponse']
_GETROLESRESPONSE_ROLESENTRY = _GETROLESRESPONSE.nested_types_by_name['RolesEntry']
_SETROLEREQUEST = DESCRIPTOR.message_types_by_name['SetRoleRequest']
_SETROLERESPONSE = DESCRIPTOR.message_types_by_name['SetRoleResponse']
IssueRequest = _reflection.GeneratedProtocolMessageType('IssueRequest', (_message.Message,), {'DESCRIPTOR': _ISSUEREQUEST, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(IssueRequest)
IssueResponse = _reflection.GeneratedProtocolMessageType('IssueResponse', (_message.Message,), {'DESCRIPTOR': _ISSUERESPONSE, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(IssueResponse)
AuthorizeRequest = _reflection.GeneratedProtocolMessageType('AuthorizeRequest', (_message.Message,), {'DESCRIPTOR': _AUTHORIZEREQUEST, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(AuthorizeRequest)
AuthorizeResponse = _reflection.GeneratedProtocolMessageType('AuthorizeResponse', (_message.Message,), {'DESCRIPTOR': _AUTHORIZERESPONSE, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(AuthorizeResponse)
AuthenticateRequest = _reflection.GeneratedProtocolMessageType('AuthenticateRequest', (_message.Message,), {'DESCRIPTOR': _AUTHENTICATEREQUEST, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(AuthenticateRequest)
AuthenticateResponse = _reflection.GeneratedProtocolMessageType('AuthenticateResponse', (_message.Message,), {'DESCRIPTOR': _AUTHENTICATERESPONSE, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(AuthenticateResponse)
GetRolesRequest = _reflection.GeneratedProtocolMessageType('GetRolesRequest', (_message.Message,), {'DESCRIPTOR': _GETROLESREQUEST, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(GetRolesRequest)
GetRolesResponse = _reflection.GeneratedProtocolMessageType('GetRolesResponse', (_message.Message,), {'RolesEntry': _reflection.GeneratedProtocolMessageType('RolesEntry', (_message.Message,), {'DESCRIPTOR': _GETROLESRESPONSE_ROLESENTRY, '__module__': 'issuer_api.v1.issuer_pb2'}), 'DESCRIPTOR': _GETROLESRESPONSE, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(GetRolesResponse)
_sym_db.RegisterMessage(GetRolesResponse.RolesEntry)
SetRoleRequest = _reflection.GeneratedProtocolMessageType('SetRoleRequest', (_message.Message,), {'DESCRIPTOR': _SETROLEREQUEST, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(SetRoleRequest)
SetRoleResponse = _reflection.GeneratedProtocolMessageType('SetRoleResponse', (_message.Message,), {'DESCRIPTOR': _SETROLERESPONSE, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(SetRoleResponse)
_ISSUER = DESCRIPTOR.services_by_name['Issuer']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z7github.com/knox-networks/grpc-sdks/sdk/go/issuer_api/v1'
    _GETROLESRESPONSE_ROLESENTRY._options = None
    _GETROLESRESPONSE_ROLESENTRY._serialized_options = b'8\x01'
    _ISSUEREQUEST._serialized_start = 70
    _ISSUEREQUEST._serialized_end = 293
    _ISSUERESPONSE._serialized_start = 295
    _ISSUERESPONSE._serialized_end = 335
    _AUTHORIZEREQUEST._serialized_start = 337
    _AUTHORIZEREQUEST._serialized_end = 380
    _AUTHORIZERESPONSE._serialized_start = 382
    _AUTHORIZERESPONSE._serialized_end = 426
    _AUTHENTICATEREQUEST._serialized_start = 428
    _AUTHENTICATEREQUEST._serialized_end = 492
    _AUTHENTICATERESPONSE._serialized_start = 494
    _AUTHENTICATERESPONSE._serialized_end = 535
    _GETROLESREQUEST._serialized_start = 537
    _GETROLESREQUEST._serialized_end = 554
    _GETROLESRESPONSE._serialized_start = 556
    _GETROLESRESPONSE._serialized_end = 679
    _GETROLESRESPONSE_ROLESENTRY._serialized_start = 635
    _GETROLESRESPONSE_ROLESENTRY._serialized_end = 679
    _SETROLEREQUEST._serialized_start = 681
    _SETROLEREQUEST._serialized_end = 725
    _SETROLERESPONSE._serialized_start = 727
    _SETROLERESPONSE._serialized_end = 744
    _ISSUER._serialized_start = 747
    _ISSUER._serialized_end = 1149