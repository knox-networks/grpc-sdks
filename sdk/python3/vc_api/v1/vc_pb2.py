"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x12vc_api/v1/vc.proto\x12\tvc_api.v1"y\n\x0fCredentialProof\x12\x0c\n\x04type\x18\x01 \x01(\t\x12\x0f\n\x07created\x18\x02 \x01(\t\x12\x1b\n\x13verification_method\x18\x03 \x01(\t\x12\x15\n\rproof_purpose\x18\x04 \x01(\t\x12\x13\n\x0bproof_value\x18\x05 \x01(\t"a\n\x1eCreateIssuanceChallengeRequest\x122\n\x0fcredential_type\x18\x01 \x01(\x0e2\x19.vc_api.v1.CredentialType\x12\x0b\n\x03did\x18\x02 \x01(\t"v\n\x1fCreateIssuanceChallengeResponse\x12\r\n\x05nonce\x18\x01 \x01(\t\x12\x10\n\x08endpoint\x18\x02 \x01(\t\x122\n\x0fcredential_type\x18\x03 \x01(\x0e2\x19.vc_api.v1.CredentialType"\x85\x01\n IssueVerifiableCredentialRequest\x122\n\x0fcredential_type\x18\x01 \x01(\x0e2\x19.vc_api.v1.CredentialType\x12\r\n\x05nonce\x18\x02 \x01(\t\x12\x11\n\tsignature\x18\x03 \x01(\x0c\x12\x0b\n\x03did\x18\x04 \x01(\t"7\n!IssueVerifiableCredentialResponse\x12\x12\n\ncredential\x18\x01 \x01(\t"Y\n"CreatePresentationChallengeRequest\x123\n\x10credential_types\x18\x01 \x03(\x0e2\x19.vc_api.v1.CredentialType"{\n#CreatePresentationChallengeResponse\x12\r\n\x05nonce\x18\x01 \x01(\t\x12\x10\n\x08endpoint\x18\x02 \x01(\t\x123\n\x10credential_types\x18\x03 \x03(\x0e2\x19.vc_api.v1.CredentialType"\x9d\x01\n"PresentVerifiableCredentialRequest\x12\x14\n\x0cpresentation\x18\x01 \x01(\t\x12\r\n\x05nonce\x18\x02 \x01(\t\x12\x11\n\tsignature\x18\x03 \x01(\x0c\x12\x0b\n\x03did\x18\x04 \x01(\t\x122\n\x0fcredential_type\x18\x05 \x03(\x0e2\x19.vc_api.v1.CredentialType"%\n#PresentVerifiableCredentialResponse")\n\x18WaitForCompletionRequest\x12\r\n\x05nonce\x18\x01 \x01(\t"6\n\x19WaitForCompletionResponse\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x0c\n\x04data\x18\x02 \x01(\t*\x9f\x01\n\x0eCredentialType\x12\x1f\n\x1bCREDENTIAL_TYPE_UNSPECIFIED\x10\x00\x12+\n\'CREDENTIAL_TYPE_PERMANENT_RESIDENT_CARD\x10\x01\x12\x1d\n\x19CREDENTIAL_TYPE_BANK_CARD\x10\x02\x12 \n\x1cCREDENTIAL_TYPE_BANK_ACCOUNT\x10\x032\xdb\x04\n\x11CredentialAdapter\x12p\n\x17CreateIssuanceChallenge\x12).vc_api.v1.CreateIssuanceChallengeRequest\x1a*.vc_api.v1.CreateIssuanceChallengeResponse\x12v\n\x19IssueVerifiableCredential\x12+.vc_api.v1.IssueVerifiableCredentialRequest\x1a,.vc_api.v1.IssueVerifiableCredentialResponse\x12|\n\x1bCreatePresentationChallenge\x12-.vc_api.v1.CreatePresentationChallengeRequest\x1a..vc_api.v1.CreatePresentationChallengeResponse\x12|\n\x1bPresentVerifiableCredential\x12-.vc_api.v1.PresentVerifiableCredentialRequest\x1a..vc_api.v1.PresentVerifiableCredentialResponse\x12`\n\x11WaitForCompletion\x12#.vc_api.v1.WaitForCompletionRequest\x1a$.vc_api.v1.WaitForCompletionResponse0\x01B5Z3github.com/knox-networks/grpc-sdks/sdk/go/vc_api/v1b\x06proto3')
_CREDENTIALTYPE = DESCRIPTOR.enum_types_by_name['CredentialType']
CredentialType = enum_type_wrapper.EnumTypeWrapper(_CREDENTIALTYPE)
CREDENTIAL_TYPE_UNSPECIFIED = 0
CREDENTIAL_TYPE_PERMANENT_RESIDENT_CARD = 1
CREDENTIAL_TYPE_BANK_CARD = 2
CREDENTIAL_TYPE_BANK_ACCOUNT = 3
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
_CREDENTIALADAPTER = DESCRIPTOR.services_by_name['CredentialAdapter']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z3github.com/knox-networks/grpc-sdks/sdk/go/vc_api/v1'
    _CREDENTIALTYPE._serialized_start = 1083
    _CREDENTIALTYPE._serialized_end = 1242
    _CREDENTIALPROOF._serialized_start = 33
    _CREDENTIALPROOF._serialized_end = 154
    _CREATEISSUANCECHALLENGEREQUEST._serialized_start = 156
    _CREATEISSUANCECHALLENGEREQUEST._serialized_end = 253
    _CREATEISSUANCECHALLENGERESPONSE._serialized_start = 255
    _CREATEISSUANCECHALLENGERESPONSE._serialized_end = 373
    _ISSUEVERIFIABLECREDENTIALREQUEST._serialized_start = 376
    _ISSUEVERIFIABLECREDENTIALREQUEST._serialized_end = 509
    _ISSUEVERIFIABLECREDENTIALRESPONSE._serialized_start = 511
    _ISSUEVERIFIABLECREDENTIALRESPONSE._serialized_end = 566
    _CREATEPRESENTATIONCHALLENGEREQUEST._serialized_start = 568
    _CREATEPRESENTATIONCHALLENGEREQUEST._serialized_end = 657
    _CREATEPRESENTATIONCHALLENGERESPONSE._serialized_start = 659
    _CREATEPRESENTATIONCHALLENGERESPONSE._serialized_end = 782
    _PRESENTVERIFIABLECREDENTIALREQUEST._serialized_start = 785
    _PRESENTVERIFIABLECREDENTIALREQUEST._serialized_end = 942
    _PRESENTVERIFIABLECREDENTIALRESPONSE._serialized_start = 944
    _PRESENTVERIFIABLECREDENTIALRESPONSE._serialized_end = 981
    _WAITFORCOMPLETIONREQUEST._serialized_start = 983
    _WAITFORCOMPLETIONREQUEST._serialized_end = 1024
    _WAITFORCOMPLETIONRESPONSE._serialized_start = 1026
    _WAITFORCOMPLETIONRESPONSE._serialized_end = 1080
    _CREDENTIALADAPTER._serialized_start = 1245
    _CREDENTIALADAPTER._serialized_end = 1848