"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1aissuer_api/v1/issuer.proto\x12\rissuer_api.v1\x1a\x16common/v1/common.proto"\x9c\x02\n\x0cIssueRequest\x12A\n\x18authentication_signature\x18\x01 \x01(\x0b2\x1f.common.AuthenticationSignature\x12\x1e\n\x06amount\x18\x02 \x01(\x0b2\x0e.common.Amount\x12*\n\tauthority\x18\x03 \x01(\x0b2\x17.common.DynamicVerifier\x12*\n\trecipient\x18\x05 \x01(\x0b2\x17.common.DynamicVerifier\x12/\n\x0cdistribution\x18\x04 \x01(\x0b2\x14.common.DistributionH\x00\x88\x01\x01\x12\x0f\n\x07session\x18\x06 \x01(\tB\x0f\n\r_distribution"-\n\rIssueResponse\x12\x1c\n\x14num_promissory_files\x18\x01 \x01(\r"@\n\x13AuthenticateRequest\x12)\n\x08verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier":\n\x14AuthenticateResponse\x12\x11\n\tchallenge\x18\x01 \x01(\x0c\x12\x0f\n\x07session\x18\x02 \x01(\t"\x11\n\x0fGetRolesRequest"{\n\x10GetRolesResponse\x129\n\x05roles\x18\x01 \x03(\x0b2*.issuer_api.v1.GetRolesResponse.RolesEntry\x1a,\n\nRolesEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\t:\x028\x01",\n\x0eSetRoleRequest\x12\x1a\n\x04role\x18\x01 \x01(\x0b2\x0c.common.Role"\x11\n\x0fSetRoleResponse"=\n\x11AssetDenomination\x12\x15\n\rdenominations\x18\x01 \x03(\x03\x12\x11\n\tprecision\x18\x02 \x01(\r"\xcd\x01\n\x1cSetAssetDenominationsRequest\x12U\n\rdenominations\x18\x01 \x03(\x0b2>.issuer_api.v1.SetAssetDenominationsRequest.DenominationsEntry\x1aV\n\x12DenominationsEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12/\n\x05value\x18\x02 \x01(\x0b2 .issuer_api.v1.AssetDenomination:\x028\x01"\x1f\n\x1dSetAssetDenominationsResponse"\xb9\x01\n\x1aSetAssetAuthoritiesRequest\x12O\n\x0bauthorities\x18\x01 \x03(\x0b2:.issuer_api.v1.SetAssetAuthoritiesRequest.AuthoritiesEntry\x1aJ\n\x10AuthoritiesEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12%\n\x05value\x18\x02 \x01(\x0b2\x16.common.AssetAuthority:\x028\x01"\x1d\n\x1bSetAssetAuthoritiesResponse2\xa5\x04\n\rIssuerService\x12W\n\x0cAuthenticate\x12".issuer_api.v1.AuthenticateRequest\x1a#.issuer_api.v1.AuthenticateResponse\x12B\n\x05Issue\x12\x1b.issuer_api.v1.IssueRequest\x1a\x1c.issuer_api.v1.IssueResponse\x12K\n\x08GetRoles\x12\x1e.issuer_api.v1.GetRolesRequest\x1a\x1f.issuer_api.v1.GetRolesResponse\x12H\n\x07SetRole\x12\x1d.issuer_api.v1.SetRoleRequest\x1a\x1e.issuer_api.v1.SetRoleResponse\x12r\n\x15SetAssetDenominations\x12+.issuer_api.v1.SetAssetDenominationsRequest\x1a,.issuer_api.v1.SetAssetDenominationsResponse\x12l\n\x13SetAssetAuthorities\x12).issuer_api.v1.SetAssetAuthoritiesRequest\x1a*.issuer_api.v1.SetAssetAuthoritiesResponseB9Z7github.com/knox-networks/grpc-sdks/sdk/go/issuer_api/v1b\x06proto3')
_ISSUEREQUEST = DESCRIPTOR.message_types_by_name['IssueRequest']
_ISSUERESPONSE = DESCRIPTOR.message_types_by_name['IssueResponse']
_AUTHENTICATEREQUEST = DESCRIPTOR.message_types_by_name['AuthenticateRequest']
_AUTHENTICATERESPONSE = DESCRIPTOR.message_types_by_name['AuthenticateResponse']
_GETROLESREQUEST = DESCRIPTOR.message_types_by_name['GetRolesRequest']
_GETROLESRESPONSE = DESCRIPTOR.message_types_by_name['GetRolesResponse']
_GETROLESRESPONSE_ROLESENTRY = _GETROLESRESPONSE.nested_types_by_name['RolesEntry']
_SETROLEREQUEST = DESCRIPTOR.message_types_by_name['SetRoleRequest']
_SETROLERESPONSE = DESCRIPTOR.message_types_by_name['SetRoleResponse']
_ASSETDENOMINATION = DESCRIPTOR.message_types_by_name['AssetDenomination']
_SETASSETDENOMINATIONSREQUEST = DESCRIPTOR.message_types_by_name['SetAssetDenominationsRequest']
_SETASSETDENOMINATIONSREQUEST_DENOMINATIONSENTRY = _SETASSETDENOMINATIONSREQUEST.nested_types_by_name['DenominationsEntry']
_SETASSETDENOMINATIONSRESPONSE = DESCRIPTOR.message_types_by_name['SetAssetDenominationsResponse']
_SETASSETAUTHORITIESREQUEST = DESCRIPTOR.message_types_by_name['SetAssetAuthoritiesRequest']
_SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY = _SETASSETAUTHORITIESREQUEST.nested_types_by_name['AuthoritiesEntry']
_SETASSETAUTHORITIESRESPONSE = DESCRIPTOR.message_types_by_name['SetAssetAuthoritiesResponse']
IssueRequest = _reflection.GeneratedProtocolMessageType('IssueRequest', (_message.Message,), {'DESCRIPTOR': _ISSUEREQUEST, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(IssueRequest)
IssueResponse = _reflection.GeneratedProtocolMessageType('IssueResponse', (_message.Message,), {'DESCRIPTOR': _ISSUERESPONSE, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(IssueResponse)
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
AssetDenomination = _reflection.GeneratedProtocolMessageType('AssetDenomination', (_message.Message,), {'DESCRIPTOR': _ASSETDENOMINATION, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(AssetDenomination)
SetAssetDenominationsRequest = _reflection.GeneratedProtocolMessageType('SetAssetDenominationsRequest', (_message.Message,), {'DenominationsEntry': _reflection.GeneratedProtocolMessageType('DenominationsEntry', (_message.Message,), {'DESCRIPTOR': _SETASSETDENOMINATIONSREQUEST_DENOMINATIONSENTRY, '__module__': 'issuer_api.v1.issuer_pb2'}), 'DESCRIPTOR': _SETASSETDENOMINATIONSREQUEST, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(SetAssetDenominationsRequest)
_sym_db.RegisterMessage(SetAssetDenominationsRequest.DenominationsEntry)
SetAssetDenominationsResponse = _reflection.GeneratedProtocolMessageType('SetAssetDenominationsResponse', (_message.Message,), {'DESCRIPTOR': _SETASSETDENOMINATIONSRESPONSE, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(SetAssetDenominationsResponse)
SetAssetAuthoritiesRequest = _reflection.GeneratedProtocolMessageType('SetAssetAuthoritiesRequest', (_message.Message,), {'AuthoritiesEntry': _reflection.GeneratedProtocolMessageType('AuthoritiesEntry', (_message.Message,), {'DESCRIPTOR': _SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY, '__module__': 'issuer_api.v1.issuer_pb2'}), 'DESCRIPTOR': _SETASSETAUTHORITIESREQUEST, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(SetAssetAuthoritiesRequest)
_sym_db.RegisterMessage(SetAssetAuthoritiesRequest.AuthoritiesEntry)
SetAssetAuthoritiesResponse = _reflection.GeneratedProtocolMessageType('SetAssetAuthoritiesResponse', (_message.Message,), {'DESCRIPTOR': _SETASSETAUTHORITIESRESPONSE, '__module__': 'issuer_api.v1.issuer_pb2'})
_sym_db.RegisterMessage(SetAssetAuthoritiesResponse)
_ISSUERSERVICE = DESCRIPTOR.services_by_name['IssuerService']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z7github.com/knox-networks/grpc-sdks/sdk/go/issuer_api/v1'
    _GETROLESRESPONSE_ROLESENTRY._options = None
    _GETROLESRESPONSE_ROLESENTRY._serialized_options = b'8\x01'
    _SETASSETDENOMINATIONSREQUEST_DENOMINATIONSENTRY._options = None
    _SETASSETDENOMINATIONSREQUEST_DENOMINATIONSENTRY._serialized_options = b'8\x01'
    _SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY._options = None
    _SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY._serialized_options = b'8\x01'
    _ISSUEREQUEST._serialized_start = 70
    _ISSUEREQUEST._serialized_end = 354
    _ISSUERESPONSE._serialized_start = 356
    _ISSUERESPONSE._serialized_end = 401
    _AUTHENTICATEREQUEST._serialized_start = 403
    _AUTHENTICATEREQUEST._serialized_end = 467
    _AUTHENTICATERESPONSE._serialized_start = 469
    _AUTHENTICATERESPONSE._serialized_end = 527
    _GETROLESREQUEST._serialized_start = 529
    _GETROLESREQUEST._serialized_end = 546
    _GETROLESRESPONSE._serialized_start = 548
    _GETROLESRESPONSE._serialized_end = 671
    _GETROLESRESPONSE_ROLESENTRY._serialized_start = 627
    _GETROLESRESPONSE_ROLESENTRY._serialized_end = 671
    _SETROLEREQUEST._serialized_start = 673
    _SETROLEREQUEST._serialized_end = 717
    _SETROLERESPONSE._serialized_start = 719
    _SETROLERESPONSE._serialized_end = 736
    _ASSETDENOMINATION._serialized_start = 738
    _ASSETDENOMINATION._serialized_end = 799
    _SETASSETDENOMINATIONSREQUEST._serialized_start = 802
    _SETASSETDENOMINATIONSREQUEST._serialized_end = 1007
    _SETASSETDENOMINATIONSREQUEST_DENOMINATIONSENTRY._serialized_start = 921
    _SETASSETDENOMINATIONSREQUEST_DENOMINATIONSENTRY._serialized_end = 1007
    _SETASSETDENOMINATIONSRESPONSE._serialized_start = 1009
    _SETASSETDENOMINATIONSRESPONSE._serialized_end = 1040
    _SETASSETAUTHORITIESREQUEST._serialized_start = 1043
    _SETASSETAUTHORITIESREQUEST._serialized_end = 1228
    _SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY._serialized_start = 1154
    _SETASSETAUTHORITIESREQUEST_AUTHORITIESENTRY._serialized_end = 1228
    _SETASSETAUTHORITIESRESPONSE._serialized_start = 1230
    _SETASSETAUTHORITIESRESPONSE._serialized_end = 1259
    _ISSUERSERVICE._serialized_start = 1262
    _ISSUERSERVICE._serialized_end = 1811