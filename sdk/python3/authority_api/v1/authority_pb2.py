"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n authority_api/v1/authority.proto\x12\x10authority_api.v1\x1a\x16common/v1/common.proto"Q\n\x15SetIssuerLimitRequest\x12\x1d\n\x05limit\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x19\n\x11issuer_public_key\x18\x02 \x01(\t"J\n\x16SetIssuerLimitResponse\x12\x1d\n\x05limit\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x11\n\tsignature\x18\x02 \x01(\x0c"3\n\x16GetIssuerLimitsRequest\x12\x19\n\x11issuer_public_key\x18\x02 \x01(\t"S\n\x17GetIssuerLimitsResponse\x12\x1d\n\x05limit\x18\x01 \x03(\x0b2\x0e.common.Amount\x12\x19\n\x11issuer_public_key\x18\x02 \x01(\t"+\n\x10AuthorizeRequest\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c",\n\x11AuthorizeResponse\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c":\n\rRedeemRequest\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c\x12\x10\n\x08verifier\x18\x02 \x01(\t"/\n\x0eRedeemResponse\x12\x1d\n\x05limit\x18\x01 \x01(\x0b2\x0e.common.Amount"\x14\n\x12GetEmissaryRequest"\'\n\x13GetEmissaryResponse\x12\x10\n\x08verifier\x18\x01 \x01(\t"$\n\x10GetNotaryRequest\x12\x10\n\x08verifier\x18\x01 \x01(\t"&\n\x11GetNotaryResponse\x12\x11\n\tsignature\x18\x01 \x01(\x0c2\xb1\x04\n\tAuthority\x12c\n\x0eSetIssuerLimit\x12\'.authority_api.v1.SetIssuerLimitRequest\x1a(.authority_api.v1.SetIssuerLimitResponse\x12f\n\x0fGetIssuerLimits\x12(.authority_api.v1.GetIssuerLimitsRequest\x1a).authority_api.v1.GetIssuerLimitsResponse\x12X\n\tAuthorize\x12".authority_api.v1.AuthorizeRequest\x1a#.authority_api.v1.AuthorizeResponse(\x010\x01\x12K\n\x06Redeem\x12\x1f.authority_api.v1.RedeemRequest\x1a .authority_api.v1.RedeemResponse\x12Z\n\x0bGetEmissary\x12$.authority_api.v1.GetEmissaryRequest\x1a%.authority_api.v1.GetEmissaryResponse\x12T\n\tGetNotary\x12".authority_api.v1.GetNotaryRequest\x1a#.authority_api.v1.GetNotaryResponseB<Z:github.com/knox-networks/grpc-sdks/sdk/go/authority_api/v1b\x06proto3')
_SETISSUERLIMITREQUEST = DESCRIPTOR.message_types_by_name['SetIssuerLimitRequest']
_SETISSUERLIMITRESPONSE = DESCRIPTOR.message_types_by_name['SetIssuerLimitResponse']
_GETISSUERLIMITSREQUEST = DESCRIPTOR.message_types_by_name['GetIssuerLimitsRequest']
_GETISSUERLIMITSRESPONSE = DESCRIPTOR.message_types_by_name['GetIssuerLimitsResponse']
_AUTHORIZEREQUEST = DESCRIPTOR.message_types_by_name['AuthorizeRequest']
_AUTHORIZERESPONSE = DESCRIPTOR.message_types_by_name['AuthorizeResponse']
_REDEEMREQUEST = DESCRIPTOR.message_types_by_name['RedeemRequest']
_REDEEMRESPONSE = DESCRIPTOR.message_types_by_name['RedeemResponse']
_GETEMISSARYREQUEST = DESCRIPTOR.message_types_by_name['GetEmissaryRequest']
_GETEMISSARYRESPONSE = DESCRIPTOR.message_types_by_name['GetEmissaryResponse']
_GETNOTARYREQUEST = DESCRIPTOR.message_types_by_name['GetNotaryRequest']
_GETNOTARYRESPONSE = DESCRIPTOR.message_types_by_name['GetNotaryResponse']
SetIssuerLimitRequest = _reflection.GeneratedProtocolMessageType('SetIssuerLimitRequest', (_message.Message,), {'DESCRIPTOR': _SETISSUERLIMITREQUEST, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(SetIssuerLimitRequest)
SetIssuerLimitResponse = _reflection.GeneratedProtocolMessageType('SetIssuerLimitResponse', (_message.Message,), {'DESCRIPTOR': _SETISSUERLIMITRESPONSE, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(SetIssuerLimitResponse)
GetIssuerLimitsRequest = _reflection.GeneratedProtocolMessageType('GetIssuerLimitsRequest', (_message.Message,), {'DESCRIPTOR': _GETISSUERLIMITSREQUEST, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(GetIssuerLimitsRequest)
GetIssuerLimitsResponse = _reflection.GeneratedProtocolMessageType('GetIssuerLimitsResponse', (_message.Message,), {'DESCRIPTOR': _GETISSUERLIMITSRESPONSE, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(GetIssuerLimitsResponse)
AuthorizeRequest = _reflection.GeneratedProtocolMessageType('AuthorizeRequest', (_message.Message,), {'DESCRIPTOR': _AUTHORIZEREQUEST, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(AuthorizeRequest)
AuthorizeResponse = _reflection.GeneratedProtocolMessageType('AuthorizeResponse', (_message.Message,), {'DESCRIPTOR': _AUTHORIZERESPONSE, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(AuthorizeResponse)
RedeemRequest = _reflection.GeneratedProtocolMessageType('RedeemRequest', (_message.Message,), {'DESCRIPTOR': _REDEEMREQUEST, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(RedeemRequest)
RedeemResponse = _reflection.GeneratedProtocolMessageType('RedeemResponse', (_message.Message,), {'DESCRIPTOR': _REDEEMRESPONSE, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(RedeemResponse)
GetEmissaryRequest = _reflection.GeneratedProtocolMessageType('GetEmissaryRequest', (_message.Message,), {'DESCRIPTOR': _GETEMISSARYREQUEST, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(GetEmissaryRequest)
GetEmissaryResponse = _reflection.GeneratedProtocolMessageType('GetEmissaryResponse', (_message.Message,), {'DESCRIPTOR': _GETEMISSARYRESPONSE, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(GetEmissaryResponse)
GetNotaryRequest = _reflection.GeneratedProtocolMessageType('GetNotaryRequest', (_message.Message,), {'DESCRIPTOR': _GETNOTARYREQUEST, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(GetNotaryRequest)
GetNotaryResponse = _reflection.GeneratedProtocolMessageType('GetNotaryResponse', (_message.Message,), {'DESCRIPTOR': _GETNOTARYRESPONSE, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(GetNotaryResponse)
_AUTHORITY = DESCRIPTOR.services_by_name['Authority']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z:github.com/knox-networks/grpc-sdks/sdk/go/authority_api/v1'
    _SETISSUERLIMITREQUEST._serialized_start = 78
    _SETISSUERLIMITREQUEST._serialized_end = 159
    _SETISSUERLIMITRESPONSE._serialized_start = 161
    _SETISSUERLIMITRESPONSE._serialized_end = 235
    _GETISSUERLIMITSREQUEST._serialized_start = 237
    _GETISSUERLIMITSREQUEST._serialized_end = 288
    _GETISSUERLIMITSRESPONSE._serialized_start = 290
    _GETISSUERLIMITSRESPONSE._serialized_end = 373
    _AUTHORIZEREQUEST._serialized_start = 375
    _AUTHORIZEREQUEST._serialized_end = 418
    _AUTHORIZERESPONSE._serialized_start = 420
    _AUTHORIZERESPONSE._serialized_end = 464
    _REDEEMREQUEST._serialized_start = 466
    _REDEEMREQUEST._serialized_end = 524
    _REDEEMRESPONSE._serialized_start = 526
    _REDEEMRESPONSE._serialized_end = 573
    _GETEMISSARYREQUEST._serialized_start = 575
    _GETEMISSARYREQUEST._serialized_end = 595
    _GETEMISSARYRESPONSE._serialized_start = 597
    _GETEMISSARYRESPONSE._serialized_end = 636
    _GETNOTARYREQUEST._serialized_start = 638
    _GETNOTARYREQUEST._serialized_end = 674
    _GETNOTARYRESPONSE._serialized_start = 676
    _GETNOTARYRESPONSE._serialized_end = 714
    _AUTHORITY._serialized_start = 717
    _AUTHORITY._serialized_end = 1278