"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...google.api import annotations_pb2 as google_dot_api_dot_annotations__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x12vc_api/v1/vc.proto\x12\tvc_api.v1\x1a\x1cgoogle/api/annotations.proto"y\n\x0fCredentialProof\x12\x0c\n\x04type\x18\x01 \x01(\t\x12\x0f\n\x07created\x18\x02 \x01(\t\x12\x1b\n\x13verification_method\x18\x03 \x01(\t\x12\x15\n\rproof_purpose\x18\x04 \x01(\t\x12\x13\n\x0bproof_value\x18\x05 \x01(\t"a\n\x1eCreateIssuanceChallengeRequest\x122\n\x0fcredential_type\x18\x01 \x01(\x0e2\x19.vc_api.v1.CredentialType\x12\x0b\n\x03did\x18\x02 \x01(\t"v\n\x1fCreateIssuanceChallengeResponse\x12\r\n\x05nonce\x18\x01 \x01(\t\x12\x10\n\x08endpoint\x18\x02 \x01(\t\x122\n\x0fcredential_type\x18\x03 \x01(\x0e2\x19.vc_api.v1.CredentialType"\x85\x01\n IssueVerifiableCredentialRequest\x122\n\x0fcredential_type\x18\x01 \x01(\x0e2\x19.vc_api.v1.CredentialType\x12\r\n\x05nonce\x18\x02 \x01(\t\x12\x11\n\tsignature\x18\x03 \x01(\x0c\x12\x0b\n\x03did\x18\x04 \x01(\t"7\n!IssueVerifiableCredentialResponse\x12\x12\n\ncredential\x18\x01 \x01(\t"Y\n"CreatePresentationChallengeRequest\x123\n\x10credential_types\x18\x01 \x03(\x0e2\x19.vc_api.v1.CredentialType"{\n#CreatePresentationChallengeResponse\x12\r\n\x05nonce\x18\x01 \x01(\t\x12\x10\n\x08endpoint\x18\x02 \x01(\t\x123\n\x10credential_types\x18\x03 \x03(\x0e2\x19.vc_api.v1.CredentialType"\x9d\x01\n"PresentVerifiableCredentialRequest\x12\x14\n\x0cpresentation\x18\x01 \x01(\t\x12\r\n\x05nonce\x18\x02 \x01(\t\x12\x11\n\tsignature\x18\x03 \x01(\x0c\x12\x0b\n\x03did\x18\x04 \x01(\t\x122\n\x0fcredential_type\x18\x05 \x03(\x0e2\x19.vc_api.v1.CredentialType"%\n#PresentVerifiableCredentialResponse")\n\x18WaitForCompletionRequest\x12\r\n\x05nonce\x18\x01 \x01(\t"6\n\x19WaitForCompletionResponse\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x0c\n\x04data\x18\x02 \x01(\t"\xb0\x01\n\'UpdateVerifiableCredentialStatusRequest\x122\n\x0fcredential_type\x18\x01 \x01(\x0e2\x19.vc_api.v1.CredentialType\x12\x0b\n\x03did\x18\x02 \x01(\t\x12D\n\x0bupdate_type\x18\x03 \x01(\x0e2/.vc_api.v1.UpdateVerifiableCredentialStatusType"*\n(UpdateVerifiableCredentialStatusResponse*\x9f\x01\n\x0eCredentialType\x12\x1f\n\x1bCREDENTIAL_TYPE_UNSPECIFIED\x10\x00\x12+\n\'CREDENTIAL_TYPE_PERMANENT_RESIDENT_CARD\x10\x01\x12\x1d\n\x19CREDENTIAL_TYPE_BANK_CARD\x10\x02\x12 \n\x1cCREDENTIAL_TYPE_BANK_ACCOUNT\x10\x03*\xd2\x01\n$UpdateVerifiableCredentialStatusType\x128\n4UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_UNSPECIFIED\x10\x00\x127\n3UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_REVOCATION\x10\x01\x127\n3UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_SUSPENSION\x10\x022\xef\x08\n\x18CredentialAdapterService\x12\xb8\x01\n\x17CreateIssuanceChallenge\x12).vc_api.v1.CreateIssuanceChallengeRequest\x1a*.vc_api.v1.CreateIssuanceChallengeResponse"F\x82\xd3\xe4\x93\x02@";/vc_api.v1.CredentialAdapterService/CreateIssuanceChallenge:\x01*\x12\xc0\x01\n\x19IssueVerifiableCredential\x12+.vc_api.v1.IssueVerifiableCredentialRequest\x1a,.vc_api.v1.IssueVerifiableCredentialResponse"H\x82\xd3\xe4\x93\x02B"=/vc_api.v1.CredentialAdapterService/IssueVerifiableCredential:\x01*\x12\xc8\x01\n\x1bCreatePresentationChallenge\x12-.vc_api.v1.CreatePresentationChallengeRequest\x1a..vc_api.v1.CreatePresentationChallengeResponse"J\x82\xd3\xe4\x93\x02D"?/vc_api.v1.CredentialAdapterService/CreatePresentationChallenge:\x01*\x12\xc8\x01\n\x1bPresentVerifiableCredential\x12-.vc_api.v1.PresentVerifiableCredentialRequest\x1a..vc_api.v1.PresentVerifiableCredentialResponse"J\x82\xd3\xe4\x93\x02D"?/vc_api.v1.CredentialAdapterService/PresentVerifiableCredential:\x01*\x12`\n\x11WaitForCompletion\x12#.vc_api.v1.WaitForCompletionRequest\x1a$.vc_api.v1.WaitForCompletionResponse0\x01\x12\xdc\x01\n UpdateVerifiableCredentialStatus\x122.vc_api.v1.UpdateVerifiableCredentialStatusRequest\x1a3.vc_api.v1.UpdateVerifiableCredentialStatusResponse"O\x82\xd3\xe4\x93\x02I"D/vc_api.v1.CredentialAdapterService/UpdateVerifiableCredentialStatus:\x01*B5Z3github.com/knox-networks/grpc-sdks/sdk/go/vc_api/v1b\x06proto3')
_CREDENTIALTYPE = DESCRIPTOR.enum_types_by_name['CredentialType']
CredentialType = enum_type_wrapper.EnumTypeWrapper(_CREDENTIALTYPE)
_UPDATEVERIFIABLECREDENTIALSTATUSTYPE = DESCRIPTOR.enum_types_by_name['UpdateVerifiableCredentialStatusType']
UpdateVerifiableCredentialStatusType = enum_type_wrapper.EnumTypeWrapper(_UPDATEVERIFIABLECREDENTIALSTATUSTYPE)
CREDENTIAL_TYPE_UNSPECIFIED = 0
CREDENTIAL_TYPE_PERMANENT_RESIDENT_CARD = 1
CREDENTIAL_TYPE_BANK_CARD = 2
CREDENTIAL_TYPE_BANK_ACCOUNT = 3
UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_UNSPECIFIED = 0
UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_REVOCATION = 1
UPDATE_VERIFIABLE_CREDENTIAL_STATUS_TYPE_SUSPENSION = 2
_CREDENTIALPROOF = DESCRIPTOR.message_types_by_name['CredentialProof']
_CREATEISSUANCECHALLENGEREQUEST = DESCRIPTOR.message_types_by_name['CreateIssuanceChallengeRequest']
_CREATEISSUANCECHALLENGERESPONSE = DESCRIPTOR.message_types_by_name['CreateIssuanceChallengeResponse']
_ISSUEVERIFIABLECREDENTIALREQUEST = DESCRIPTOR.message_types_by_name['IssueVerifiableCredentialRequest']
_ISSUEVERIFIABLECREDENTIALRESPONSE = DESCRIPTOR.message_types_by_name['IssueVerifiableCredentialResponse']
_CREATEPRESENTATIONCHALLENGEREQUEST = DESCRIPTOR.message_types_by_name['CreatePresentationChallengeRequest']
_CREATEPRESENTATIONCHALLENGERESPONSE = DESCRIPTOR.message_types_by_name['CreatePresentationChallengeResponse']
_PRESENTVERIFIABLECREDENTIALREQUEST = DESCRIPTOR.message_types_by_name['PresentVerifiableCredentialRequest']
_PRESENTVERIFIABLECREDENTIALRESPONSE = DESCRIPTOR.message_types_by_name['PresentVerifiableCredentialResponse']
_WAITFORCOMPLETIONREQUEST = DESCRIPTOR.message_types_by_name['WaitForCompletionRequest']
_WAITFORCOMPLETIONRESPONSE = DESCRIPTOR.message_types_by_name['WaitForCompletionResponse']
_UPDATEVERIFIABLECREDENTIALSTATUSREQUEST = DESCRIPTOR.message_types_by_name['UpdateVerifiableCredentialStatusRequest']
_UPDATEVERIFIABLECREDENTIALSTATUSRESPONSE = DESCRIPTOR.message_types_by_name['UpdateVerifiableCredentialStatusResponse']
CredentialProof = _reflection.GeneratedProtocolMessageType('CredentialProof', (_message.Message,), {'DESCRIPTOR': _CREDENTIALPROOF, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(CredentialProof)
CreateIssuanceChallengeRequest = _reflection.GeneratedProtocolMessageType('CreateIssuanceChallengeRequest', (_message.Message,), {'DESCRIPTOR': _CREATEISSUANCECHALLENGEREQUEST, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(CreateIssuanceChallengeRequest)
CreateIssuanceChallengeResponse = _reflection.GeneratedProtocolMessageType('CreateIssuanceChallengeResponse', (_message.Message,), {'DESCRIPTOR': _CREATEISSUANCECHALLENGERESPONSE, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(CreateIssuanceChallengeResponse)
IssueVerifiableCredentialRequest = _reflection.GeneratedProtocolMessageType('IssueVerifiableCredentialRequest', (_message.Message,), {'DESCRIPTOR': _ISSUEVERIFIABLECREDENTIALREQUEST, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(IssueVerifiableCredentialRequest)
IssueVerifiableCredentialResponse = _reflection.GeneratedProtocolMessageType('IssueVerifiableCredentialResponse', (_message.Message,), {'DESCRIPTOR': _ISSUEVERIFIABLECREDENTIALRESPONSE, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(IssueVerifiableCredentialResponse)
CreatePresentationChallengeRequest = _reflection.GeneratedProtocolMessageType('CreatePresentationChallengeRequest', (_message.Message,), {'DESCRIPTOR': _CREATEPRESENTATIONCHALLENGEREQUEST, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(CreatePresentationChallengeRequest)
CreatePresentationChallengeResponse = _reflection.GeneratedProtocolMessageType('CreatePresentationChallengeResponse', (_message.Message,), {'DESCRIPTOR': _CREATEPRESENTATIONCHALLENGERESPONSE, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(CreatePresentationChallengeResponse)
PresentVerifiableCredentialRequest = _reflection.GeneratedProtocolMessageType('PresentVerifiableCredentialRequest', (_message.Message,), {'DESCRIPTOR': _PRESENTVERIFIABLECREDENTIALREQUEST, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(PresentVerifiableCredentialRequest)
PresentVerifiableCredentialResponse = _reflection.GeneratedProtocolMessageType('PresentVerifiableCredentialResponse', (_message.Message,), {'DESCRIPTOR': _PRESENTVERIFIABLECREDENTIALRESPONSE, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(PresentVerifiableCredentialResponse)
WaitForCompletionRequest = _reflection.GeneratedProtocolMessageType('WaitForCompletionRequest', (_message.Message,), {'DESCRIPTOR': _WAITFORCOMPLETIONREQUEST, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(WaitForCompletionRequest)
WaitForCompletionResponse = _reflection.GeneratedProtocolMessageType('WaitForCompletionResponse', (_message.Message,), {'DESCRIPTOR': _WAITFORCOMPLETIONRESPONSE, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(WaitForCompletionResponse)
UpdateVerifiableCredentialStatusRequest = _reflection.GeneratedProtocolMessageType('UpdateVerifiableCredentialStatusRequest', (_message.Message,), {'DESCRIPTOR': _UPDATEVERIFIABLECREDENTIALSTATUSREQUEST, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(UpdateVerifiableCredentialStatusRequest)
UpdateVerifiableCredentialStatusResponse = _reflection.GeneratedProtocolMessageType('UpdateVerifiableCredentialStatusResponse', (_message.Message,), {'DESCRIPTOR': _UPDATEVERIFIABLECREDENTIALSTATUSRESPONSE, '__module__': 'vc_api.v1.vc_pb2'})
_sym_db.RegisterMessage(UpdateVerifiableCredentialStatusResponse)
_CREDENTIALADAPTERSERVICE = DESCRIPTOR.services_by_name['CredentialAdapterService']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z3github.com/knox-networks/grpc-sdks/sdk/go/vc_api/v1'
    _CREDENTIALADAPTERSERVICE.methods_by_name['CreateIssuanceChallenge']._options = None
    _CREDENTIALADAPTERSERVICE.methods_by_name['CreateIssuanceChallenge']._serialized_options = b'\x82\xd3\xe4\x93\x02@";/vc_api.v1.CredentialAdapterService/CreateIssuanceChallenge:\x01*'
    _CREDENTIALADAPTERSERVICE.methods_by_name['IssueVerifiableCredential']._options = None
    _CREDENTIALADAPTERSERVICE.methods_by_name['IssueVerifiableCredential']._serialized_options = b'\x82\xd3\xe4\x93\x02B"=/vc_api.v1.CredentialAdapterService/IssueVerifiableCredential:\x01*'
    _CREDENTIALADAPTERSERVICE.methods_by_name['CreatePresentationChallenge']._options = None
    _CREDENTIALADAPTERSERVICE.methods_by_name['CreatePresentationChallenge']._serialized_options = b'\x82\xd3\xe4\x93\x02D"?/vc_api.v1.CredentialAdapterService/CreatePresentationChallenge:\x01*'
    _CREDENTIALADAPTERSERVICE.methods_by_name['PresentVerifiableCredential']._options = None
    _CREDENTIALADAPTERSERVICE.methods_by_name['PresentVerifiableCredential']._serialized_options = b'\x82\xd3\xe4\x93\x02D"?/vc_api.v1.CredentialAdapterService/PresentVerifiableCredential:\x01*'
    _CREDENTIALADAPTERSERVICE.methods_by_name['UpdateVerifiableCredentialStatus']._options = None
    _CREDENTIALADAPTERSERVICE.methods_by_name['UpdateVerifiableCredentialStatus']._serialized_options = b'\x82\xd3\xe4\x93\x02I"D/vc_api.v1.CredentialAdapterService/UpdateVerifiableCredentialStatus:\x01*'
    _CREDENTIALTYPE._serialized_start = 1336
    _CREDENTIALTYPE._serialized_end = 1495
    _UPDATEVERIFIABLECREDENTIALSTATUSTYPE._serialized_start = 1498
    _UPDATEVERIFIABLECREDENTIALSTATUSTYPE._serialized_end = 1708
    _CREDENTIALPROOF._serialized_start = 63
    _CREDENTIALPROOF._serialized_end = 184
    _CREATEISSUANCECHALLENGEREQUEST._serialized_start = 186
    _CREATEISSUANCECHALLENGEREQUEST._serialized_end = 283
    _CREATEISSUANCECHALLENGERESPONSE._serialized_start = 285
    _CREATEISSUANCECHALLENGERESPONSE._serialized_end = 403
    _ISSUEVERIFIABLECREDENTIALREQUEST._serialized_start = 406
    _ISSUEVERIFIABLECREDENTIALREQUEST._serialized_end = 539
    _ISSUEVERIFIABLECREDENTIALRESPONSE._serialized_start = 541
    _ISSUEVERIFIABLECREDENTIALRESPONSE._serialized_end = 596
    _CREATEPRESENTATIONCHALLENGEREQUEST._serialized_start = 598
    _CREATEPRESENTATIONCHALLENGEREQUEST._serialized_end = 687
    _CREATEPRESENTATIONCHALLENGERESPONSE._serialized_start = 689
    _CREATEPRESENTATIONCHALLENGERESPONSE._serialized_end = 812
    _PRESENTVERIFIABLECREDENTIALREQUEST._serialized_start = 815
    _PRESENTVERIFIABLECREDENTIALREQUEST._serialized_end = 972
    _PRESENTVERIFIABLECREDENTIALRESPONSE._serialized_start = 974
    _PRESENTVERIFIABLECREDENTIALRESPONSE._serialized_end = 1011
    _WAITFORCOMPLETIONREQUEST._serialized_start = 1013
    _WAITFORCOMPLETIONREQUEST._serialized_end = 1054
    _WAITFORCOMPLETIONRESPONSE._serialized_start = 1056
    _WAITFORCOMPLETIONRESPONSE._serialized_end = 1110
    _UPDATEVERIFIABLECREDENTIALSTATUSREQUEST._serialized_start = 1113
    _UPDATEVERIFIABLECREDENTIALSTATUSREQUEST._serialized_end = 1289
    _UPDATEVERIFIABLECREDENTIALSTATUSRESPONSE._serialized_start = 1291
    _UPDATEVERIFIABLECREDENTIALSTATUSRESPONSE._serialized_end = 1333
    _CREDENTIALADAPTERSERVICE._serialized_start = 1711
    _CREDENTIALADAPTERSERVICE._serialized_end = 2846