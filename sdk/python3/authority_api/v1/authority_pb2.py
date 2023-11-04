"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n authority_api/v1/authority.proto\x12\x10authority_api.v1\x1a\x16common/v1/common.proto"Q\n\x15SetIssuerLimitRequest\x12\x1d\n\x05limit\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x19\n\x11issuer_public_key\x18\x02 \x01(\t"J\n\x16SetIssuerLimitResponse\x12\x1d\n\x05limit\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x11\n\tsignature\x18\x02 \x01(\x0c"3\n\x16GetIssuerLimitsRequest\x12\x19\n\x11issuer_public_key\x18\x02 \x01(\t"S\n\x17GetIssuerLimitsResponse\x12\x1d\n\x05limit\x18\x01 \x03(\x0b2\x0e.common.Amount\x12\x19\n\x11issuer_public_key\x18\x02 \x01(\t"+\n\x10AuthorizeRequest\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c",\n\x11AuthorizeResponse\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c":\n\rRedeemRequest\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c\x12\x10\n\x08verifier\x18\x02 \x01(\t"/\n\x0eRedeemResponse\x12\x1d\n\x05limit\x18\x01 \x01(\x0b2\x0e.common.Amount"\x17\n\x15GetNetworkInfoRequest"G\n\x16GetNetworkInfoResponse\x12\x10\n\x08emissary\x18\x01 \x01(\t\x12\x1b\n\x13transaction_manager\x18\x02 \x01(\t"$\n\x10GetNotaryRequest\x12\x10\n\x08verifier\x18\x01 \x01(\t"&\n\x11GetNotaryResponse\x12\x11\n\tsignature\x18\x01 \x01(\x0c"\x1b\n\x19ListGovernedAssetsRequest"\xba\x01\n\x1aListGovernedAssetsResponse\x12O\n\nasset_data\x18\x01 \x03(\x0b2;.authority_api.v1.ListGovernedAssetsResponse.AssetDataEntry\x1aK\n\x0eAssetDataEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12(\n\x05value\x18\x02 \x01(\x0b2\x19.common.AssetDenomination:\x028\x01"\xb6\x01\n\x18SetGovernedAssetsRequest\x12M\n\nasset_data\x18\x01 \x03(\x0b29.authority_api.v1.SetGovernedAssetsRequest.AssetDataEntry\x1aK\n\x0eAssetDataEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12(\n\x05value\x18\x02 \x01(\x0b2\x19.common.AssetDenomination:\x028\x01"\x1b\n\x19SetGovernedAssetsResponse2\xa0\x06\n\x10AuthorityService\x12c\n\x0eSetIssuerLimit\x12\'.authority_api.v1.SetIssuerLimitRequest\x1a(.authority_api.v1.SetIssuerLimitResponse\x12f\n\x0fGetIssuerLimits\x12(.authority_api.v1.GetIssuerLimitsRequest\x1a).authority_api.v1.GetIssuerLimitsResponse\x12X\n\tAuthorize\x12".authority_api.v1.AuthorizeRequest\x1a#.authority_api.v1.AuthorizeResponse(\x010\x01\x12K\n\x06Redeem\x12\x1f.authority_api.v1.RedeemRequest\x1a .authority_api.v1.RedeemResponse\x12c\n\x0eGetNetworkInfo\x12\'.authority_api.v1.GetNetworkInfoRequest\x1a(.authority_api.v1.GetNetworkInfoResponse\x12T\n\tGetNotary\x12".authority_api.v1.GetNotaryRequest\x1a#.authority_api.v1.GetNotaryResponse\x12o\n\x12ListGovernedAssets\x12+.authority_api.v1.ListGovernedAssetsRequest\x1a,.authority_api.v1.ListGovernedAssetsResponse\x12l\n\x11SetGovernedAssets\x12*.authority_api.v1.SetGovernedAssetsRequest\x1a+.authority_api.v1.SetGovernedAssetsResponseB<Z:github.com/knox-networks/grpc-sdks/sdk/go/authority_api/v1b\x06proto3')
_SETISSUERLIMITREQUEST = DESCRIPTOR.message_types_by_name['SetIssuerLimitRequest']
_SETISSUERLIMITRESPONSE = DESCRIPTOR.message_types_by_name['SetIssuerLimitResponse']
_GETISSUERLIMITSREQUEST = DESCRIPTOR.message_types_by_name['GetIssuerLimitsRequest']
_GETISSUERLIMITSRESPONSE = DESCRIPTOR.message_types_by_name['GetIssuerLimitsResponse']
_AUTHORIZEREQUEST = DESCRIPTOR.message_types_by_name['AuthorizeRequest']
_AUTHORIZERESPONSE = DESCRIPTOR.message_types_by_name['AuthorizeResponse']
_REDEEMREQUEST = DESCRIPTOR.message_types_by_name['RedeemRequest']
_REDEEMRESPONSE = DESCRIPTOR.message_types_by_name['RedeemResponse']
_GETNETWORKINFOREQUEST = DESCRIPTOR.message_types_by_name['GetNetworkInfoRequest']
_GETNETWORKINFORESPONSE = DESCRIPTOR.message_types_by_name['GetNetworkInfoResponse']
_GETNOTARYREQUEST = DESCRIPTOR.message_types_by_name['GetNotaryRequest']
_GETNOTARYRESPONSE = DESCRIPTOR.message_types_by_name['GetNotaryResponse']
_LISTGOVERNEDASSETSREQUEST = DESCRIPTOR.message_types_by_name['ListGovernedAssetsRequest']
_LISTGOVERNEDASSETSRESPONSE = DESCRIPTOR.message_types_by_name['ListGovernedAssetsResponse']
_LISTGOVERNEDASSETSRESPONSE_ASSETDATAENTRY = _LISTGOVERNEDASSETSRESPONSE.nested_types_by_name['AssetDataEntry']
_SETGOVERNEDASSETSREQUEST = DESCRIPTOR.message_types_by_name['SetGovernedAssetsRequest']
_SETGOVERNEDASSETSREQUEST_ASSETDATAENTRY = _SETGOVERNEDASSETSREQUEST.nested_types_by_name['AssetDataEntry']
_SETGOVERNEDASSETSRESPONSE = DESCRIPTOR.message_types_by_name['SetGovernedAssetsResponse']
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
GetNetworkInfoRequest = _reflection.GeneratedProtocolMessageType('GetNetworkInfoRequest', (_message.Message,), {'DESCRIPTOR': _GETNETWORKINFOREQUEST, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(GetNetworkInfoRequest)
GetNetworkInfoResponse = _reflection.GeneratedProtocolMessageType('GetNetworkInfoResponse', (_message.Message,), {'DESCRIPTOR': _GETNETWORKINFORESPONSE, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(GetNetworkInfoResponse)
GetNotaryRequest = _reflection.GeneratedProtocolMessageType('GetNotaryRequest', (_message.Message,), {'DESCRIPTOR': _GETNOTARYREQUEST, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(GetNotaryRequest)
GetNotaryResponse = _reflection.GeneratedProtocolMessageType('GetNotaryResponse', (_message.Message,), {'DESCRIPTOR': _GETNOTARYRESPONSE, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(GetNotaryResponse)
ListGovernedAssetsRequest = _reflection.GeneratedProtocolMessageType('ListGovernedAssetsRequest', (_message.Message,), {'DESCRIPTOR': _LISTGOVERNEDASSETSREQUEST, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(ListGovernedAssetsRequest)
ListGovernedAssetsResponse = _reflection.GeneratedProtocolMessageType('ListGovernedAssetsResponse', (_message.Message,), {'AssetDataEntry': _reflection.GeneratedProtocolMessageType('AssetDataEntry', (_message.Message,), {'DESCRIPTOR': _LISTGOVERNEDASSETSRESPONSE_ASSETDATAENTRY, '__module__': 'authority_api.v1.authority_pb2'}), 'DESCRIPTOR': _LISTGOVERNEDASSETSRESPONSE, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(ListGovernedAssetsResponse)
_sym_db.RegisterMessage(ListGovernedAssetsResponse.AssetDataEntry)
SetGovernedAssetsRequest = _reflection.GeneratedProtocolMessageType('SetGovernedAssetsRequest', (_message.Message,), {'AssetDataEntry': _reflection.GeneratedProtocolMessageType('AssetDataEntry', (_message.Message,), {'DESCRIPTOR': _SETGOVERNEDASSETSREQUEST_ASSETDATAENTRY, '__module__': 'authority_api.v1.authority_pb2'}), 'DESCRIPTOR': _SETGOVERNEDASSETSREQUEST, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(SetGovernedAssetsRequest)
_sym_db.RegisterMessage(SetGovernedAssetsRequest.AssetDataEntry)
SetGovernedAssetsResponse = _reflection.GeneratedProtocolMessageType('SetGovernedAssetsResponse', (_message.Message,), {'DESCRIPTOR': _SETGOVERNEDASSETSRESPONSE, '__module__': 'authority_api.v1.authority_pb2'})
_sym_db.RegisterMessage(SetGovernedAssetsResponse)
_AUTHORITYSERVICE = DESCRIPTOR.services_by_name['AuthorityService']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z:github.com/knox-networks/grpc-sdks/sdk/go/authority_api/v1'
    _LISTGOVERNEDASSETSRESPONSE_ASSETDATAENTRY._options = None
    _LISTGOVERNEDASSETSRESPONSE_ASSETDATAENTRY._serialized_options = b'8\x01'
    _SETGOVERNEDASSETSREQUEST_ASSETDATAENTRY._options = None
    _SETGOVERNEDASSETSREQUEST_ASSETDATAENTRY._serialized_options = b'8\x01'
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
    _GETNETWORKINFOREQUEST._serialized_start = 575
    _GETNETWORKINFOREQUEST._serialized_end = 598
    _GETNETWORKINFORESPONSE._serialized_start = 600
    _GETNETWORKINFORESPONSE._serialized_end = 671
    _GETNOTARYREQUEST._serialized_start = 673
    _GETNOTARYREQUEST._serialized_end = 709
    _GETNOTARYRESPONSE._serialized_start = 711
    _GETNOTARYRESPONSE._serialized_end = 749
    _LISTGOVERNEDASSETSREQUEST._serialized_start = 751
    _LISTGOVERNEDASSETSREQUEST._serialized_end = 778
    _LISTGOVERNEDASSETSRESPONSE._serialized_start = 781
    _LISTGOVERNEDASSETSRESPONSE._serialized_end = 967
    _LISTGOVERNEDASSETSRESPONSE_ASSETDATAENTRY._serialized_start = 892
    _LISTGOVERNEDASSETSRESPONSE_ASSETDATAENTRY._serialized_end = 967
    _SETGOVERNEDASSETSREQUEST._serialized_start = 970
    _SETGOVERNEDASSETSREQUEST._serialized_end = 1152
    _SETGOVERNEDASSETSREQUEST_ASSETDATAENTRY._serialized_start = 892
    _SETGOVERNEDASSETSREQUEST_ASSETDATAENTRY._serialized_end = 967
    _SETGOVERNEDASSETSRESPONSE._serialized_start = 1154
    _SETGOVERNEDASSETSRESPONSE._serialized_end = 1181
    _AUTHORITYSERVICE._serialized_start = 1184
    _AUTHORITYSERVICE._serialized_end = 1984