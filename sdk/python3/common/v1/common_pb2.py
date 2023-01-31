"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x16common/v1/common.proto\x12\x06common"V\n\x0fDynamicVerifier\x121\n\x10signature_system\x18\x01 \x01(\x0e2\x17.common.SignatureSystem\x12\x10\n\x08verifier\x18\x02 \x01(\x0c"A\n\x06Amount\x12\x15\n\rcurrency_code\x18\x01 \x01(\t\x12\x0e\n\x06amount\x18\x02 \x01(\x03\x12\x10\n\x08decimals\x18\x03 \x01(\r"B\n\x15AuthenticationRequest\x12)\n\x08verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier"+\n\x16AuthenticationResponse\x12\x11\n\tchallenge\x18\x01 \x01(\x0c"\x0f\n\rOneConnection"G\n\x0eManyConnection\x125\n\x14associated_verifiers\x18\x02 \x03(\x0b2\x17.common.DynamicVerifier"\x7f\n\x14AssociatedConnection\x121\n\x10primary_verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x124\n\x13associated_verifier\x18\x02 \x01(\x0b2\x17.common.DynamicVerifier"\xa5\x01\n\x11ConnectionRequest\x12$\n\x03one\x18\x01 \x01(\x0b2\x15.common.OneConnectionH\x00\x12&\n\x04many\x18\x02 \x01(\x0b2\x16.common.ManyConnectionH\x00\x122\n\tsignature\x18\x03 \x01(\x0b2\x1f.common.AuthenticationSignatureB\x0e\n\x0crequest_type"W\n\x17AuthenticationSignature\x12)\n\x08verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x12\x11\n\tsignature\x18\x02 \x01(\x0c"5\n\x1aAuthorizedSignatureRequest\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c"6\n\x1bAuthorizedSignatureResponse\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c"(\n\x05Media\x12\x11\n\tmime_type\x18\x01 \x01(\t\x12\x0c\n\x04data\x18\x02 \x01(\x0c"\xaa\x01\n\x04Role\x12\x10\n\x08verifier\x18\x01 \x01(\t\x12\x14\n\nauthorized\x18\x03 \x01(\x08H\x00\x12:\n\x11issue_permissions\x18\x04 \x01(\x0b2\x1d.common.Role.IssuePermissionsH\x00\x1a1\n\x10IssuePermissions\x12\x1d\n\x05inner\x18\x02 \x03(\x0b2\x0e.common.AmountB\x0b\n\trole_enum"\x14\n\x12GetVerifierRequest"P\n\x13GetVerifierResponse\x12&\n\x05bytes\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x12\x11\n\tmultibase\x18\x02 \x01(\t"\x17\n\x15GetServiceTypeRequest"C\n\x16GetServiceTypeResponse\x12)\n\x0cservice_type\x18\x01 \x01(\x0e2\x13.common.ServiceType"|\n\x19StartSimplePaymentRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x11\n\trecipient\x18\x02 \x01(\t\x12\x14\n\x0cpromissories\x18\x03 \x03(\t\x12\x16\n\x0euser_reference\x18\x04 \x01(\t"*\n\x1aStartSimplePaymentResponse\x12\x0c\n\x04uetr\x18\x01 \x01(\t"e\n\x17StartFundsChangeRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x12\n\npromissory\x18\x02 \x01(\t\x12\x16\n\x0euser_reference\x18\x03 \x01(\t"(\n\x18StartFundsChangeResponse\x12\x0c\n\x04uetr\x18\x01 \x01(\t"\x93\x01\n\x13TransactionSnapshot\x12)\n\x06status\x18\x01 \x01(\x0e2\x19.common.TransactionStatus\x12\x14\n\x0cpromissories\x18\x03 \x03(\t\x12*\n\x12amount_transferred\x18\x04 \x01(\x0b2\x0e.common.Amount\x12\x0f\n\x07message\x18\x05 \x01(\t"\x83\x03\n\x0bTransaction\x12&\n\x04type\x18\x01 \x01(\x0e2\x18.common.Transaction.Type\x12&\n\x04role\x18\x02 \x01(\x0e2\x18.common.Transaction.Role\x12\x0c\n\x04uetr\x18\x03 \x01(\t\x12\x1e\n\x06amount\x18\x04 \x01(\x0b2\x0e.common.Amount\x12\x0f\n\x07created\x18\x05 \x01(\t\x12\x0f\n\x07updated\x18\x06 \x01(\t\x12\x0c\n\x04memo\x18\x07 \x01(\t\x12\r\n\x05owner\x18\x08 \x01(\t\x12\x14\n\x0ccounterparty\x18\t \x01(\t\x12)\n\x06status\x18\n \x01(\x0e2\x19.common.TransactionStatus",\n\x04Role\x12\x10\n\x0cROLE_STARTER\x10\x00\x12\x12\n\x0eROLE_RESPONDER\x10\x01"H\n\x04Type\x12\x10\n\x0cTYPE_UNKNOWN\x10\x00\x12\x17\n\x13TYPE_SIMPLE_PAYMENT\x10\x01\x12\x15\n\x11TYPE_FUNDS_CHANGE\x10\x02*\xc5\x01\n\x0bServiceType\x12\x12\n\x0eUnknownService\x10\x00\x12\x0b\n\x07Gateway\x10\x01\x12\n\n\x06Notary\x10\x02\x12\r\n\tAuthority\x10\x03\x12\x0c\n\x08Treasury\x10\x04\x12\x0c\n\x08Emissary\x10\x05\x12\n\n\x06Shared\x10\x06\x12\x0c\n\x08Identity\x10\x07\x12\t\n\x05Vault\x10\x08\x12\x13\n\x0fCustodialWallet\x10\t\x12\n\n\x06Issuer\x10\n\x12\x0c\n\x08Overload\x10\x0b\x12\n\n\x06Faucet\x10\x0c*q\n\x0fSignatureSystem\x12 \n\x1cSIGNATURE_SYSTEM_UNSPECIFIED\x10\x00\x12\x1c\n\x18SIGNATURE_SYSTEM_ED25519\x10\x01\x12\x1e\n\x1aSIGNATURE_SYSTEM_SECP256K1\x10\x02*\x90\x01\n\x11TransactionStatus\x12\x16\n\x12STATE_INITIALIZING\x10\x00\x12\x15\n\x11STATE_AUTHORIZING\x10\x01\x12\x11\n\rSTATE_SENDING\x10\x02\x12\x13\n\x0fSTATE_RECEIVING\x10\x05\x12\x12\n\x0eSTATE_COMPLETE\x10\x03\x12\x10\n\x0cSTATE_FAILED\x10\x04B5Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1b\x06proto3')
_SERVICETYPE = DESCRIPTOR.enum_types_by_name['ServiceType']
ServiceType = enum_type_wrapper.EnumTypeWrapper(_SERVICETYPE)
_SIGNATURESYSTEM = DESCRIPTOR.enum_types_by_name['SignatureSystem']
SignatureSystem = enum_type_wrapper.EnumTypeWrapper(_SIGNATURESYSTEM)
_TRANSACTIONSTATUS = DESCRIPTOR.enum_types_by_name['TransactionStatus']
TransactionStatus = enum_type_wrapper.EnumTypeWrapper(_TRANSACTIONSTATUS)
UnknownService = 0
Gateway = 1
Notary = 2
Authority = 3
Treasury = 4
Emissary = 5
Shared = 6
Identity = 7
Vault = 8
CustodialWallet = 9
Issuer = 10
Overload = 11
Faucet = 12
SIGNATURE_SYSTEM_UNSPECIFIED = 0
SIGNATURE_SYSTEM_ED25519 = 1
SIGNATURE_SYSTEM_SECP256K1 = 2
STATE_INITIALIZING = 0
STATE_AUTHORIZING = 1
STATE_SENDING = 2
STATE_RECEIVING = 5
STATE_COMPLETE = 3
STATE_FAILED = 4
_DYNAMICVERIFIER = DESCRIPTOR.message_types_by_name['DynamicVerifier']
_AMOUNT = DESCRIPTOR.message_types_by_name['Amount']
_AUTHENTICATIONREQUEST = DESCRIPTOR.message_types_by_name['AuthenticationRequest']
_AUTHENTICATIONRESPONSE = DESCRIPTOR.message_types_by_name['AuthenticationResponse']
_ONECONNECTION = DESCRIPTOR.message_types_by_name['OneConnection']
_MANYCONNECTION = DESCRIPTOR.message_types_by_name['ManyConnection']
_ASSOCIATEDCONNECTION = DESCRIPTOR.message_types_by_name['AssociatedConnection']
_CONNECTIONREQUEST = DESCRIPTOR.message_types_by_name['ConnectionRequest']
_AUTHENTICATIONSIGNATURE = DESCRIPTOR.message_types_by_name['AuthenticationSignature']
_AUTHORIZEDSIGNATUREREQUEST = DESCRIPTOR.message_types_by_name['AuthorizedSignatureRequest']
_AUTHORIZEDSIGNATURERESPONSE = DESCRIPTOR.message_types_by_name['AuthorizedSignatureResponse']
_MEDIA = DESCRIPTOR.message_types_by_name['Media']
_ROLE = DESCRIPTOR.message_types_by_name['Role']
_ROLE_ISSUEPERMISSIONS = _ROLE.nested_types_by_name['IssuePermissions']
_GETVERIFIERREQUEST = DESCRIPTOR.message_types_by_name['GetVerifierRequest']
_GETVERIFIERRESPONSE = DESCRIPTOR.message_types_by_name['GetVerifierResponse']
_GETSERVICETYPEREQUEST = DESCRIPTOR.message_types_by_name['GetServiceTypeRequest']
_GETSERVICETYPERESPONSE = DESCRIPTOR.message_types_by_name['GetServiceTypeResponse']
_STARTSIMPLEPAYMENTREQUEST = DESCRIPTOR.message_types_by_name['StartSimplePaymentRequest']
_STARTSIMPLEPAYMENTRESPONSE = DESCRIPTOR.message_types_by_name['StartSimplePaymentResponse']
_STARTFUNDSCHANGEREQUEST = DESCRIPTOR.message_types_by_name['StartFundsChangeRequest']
_STARTFUNDSCHANGERESPONSE = DESCRIPTOR.message_types_by_name['StartFundsChangeResponse']
_TRANSACTIONSNAPSHOT = DESCRIPTOR.message_types_by_name['TransactionSnapshot']
_TRANSACTION = DESCRIPTOR.message_types_by_name['Transaction']
_TRANSACTION_ROLE = _TRANSACTION.enum_types_by_name['Role']
_TRANSACTION_TYPE = _TRANSACTION.enum_types_by_name['Type']
DynamicVerifier = _reflection.GeneratedProtocolMessageType('DynamicVerifier', (_message.Message,), {'DESCRIPTOR': _DYNAMICVERIFIER, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(DynamicVerifier)
Amount = _reflection.GeneratedProtocolMessageType('Amount', (_message.Message,), {'DESCRIPTOR': _AMOUNT, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(Amount)
AuthenticationRequest = _reflection.GeneratedProtocolMessageType('AuthenticationRequest', (_message.Message,), {'DESCRIPTOR': _AUTHENTICATIONREQUEST, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(AuthenticationRequest)
AuthenticationResponse = _reflection.GeneratedProtocolMessageType('AuthenticationResponse', (_message.Message,), {'DESCRIPTOR': _AUTHENTICATIONRESPONSE, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(AuthenticationResponse)
OneConnection = _reflection.GeneratedProtocolMessageType('OneConnection', (_message.Message,), {'DESCRIPTOR': _ONECONNECTION, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(OneConnection)
ManyConnection = _reflection.GeneratedProtocolMessageType('ManyConnection', (_message.Message,), {'DESCRIPTOR': _MANYCONNECTION, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(ManyConnection)
AssociatedConnection = _reflection.GeneratedProtocolMessageType('AssociatedConnection', (_message.Message,), {'DESCRIPTOR': _ASSOCIATEDCONNECTION, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(AssociatedConnection)
ConnectionRequest = _reflection.GeneratedProtocolMessageType('ConnectionRequest', (_message.Message,), {'DESCRIPTOR': _CONNECTIONREQUEST, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(ConnectionRequest)
AuthenticationSignature = _reflection.GeneratedProtocolMessageType('AuthenticationSignature', (_message.Message,), {'DESCRIPTOR': _AUTHENTICATIONSIGNATURE, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(AuthenticationSignature)
AuthorizedSignatureRequest = _reflection.GeneratedProtocolMessageType('AuthorizedSignatureRequest', (_message.Message,), {'DESCRIPTOR': _AUTHORIZEDSIGNATUREREQUEST, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(AuthorizedSignatureRequest)
AuthorizedSignatureResponse = _reflection.GeneratedProtocolMessageType('AuthorizedSignatureResponse', (_message.Message,), {'DESCRIPTOR': _AUTHORIZEDSIGNATURERESPONSE, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(AuthorizedSignatureResponse)
Media = _reflection.GeneratedProtocolMessageType('Media', (_message.Message,), {'DESCRIPTOR': _MEDIA, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(Media)
Role = _reflection.GeneratedProtocolMessageType('Role', (_message.Message,), {'IssuePermissions': _reflection.GeneratedProtocolMessageType('IssuePermissions', (_message.Message,), {'DESCRIPTOR': _ROLE_ISSUEPERMISSIONS, '__module__': 'common.v1.common_pb2'}), 'DESCRIPTOR': _ROLE, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(Role)
_sym_db.RegisterMessage(Role.IssuePermissions)
GetVerifierRequest = _reflection.GeneratedProtocolMessageType('GetVerifierRequest', (_message.Message,), {'DESCRIPTOR': _GETVERIFIERREQUEST, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(GetVerifierRequest)
GetVerifierResponse = _reflection.GeneratedProtocolMessageType('GetVerifierResponse', (_message.Message,), {'DESCRIPTOR': _GETVERIFIERRESPONSE, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(GetVerifierResponse)
GetServiceTypeRequest = _reflection.GeneratedProtocolMessageType('GetServiceTypeRequest', (_message.Message,), {'DESCRIPTOR': _GETSERVICETYPEREQUEST, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(GetServiceTypeRequest)
GetServiceTypeResponse = _reflection.GeneratedProtocolMessageType('GetServiceTypeResponse', (_message.Message,), {'DESCRIPTOR': _GETSERVICETYPERESPONSE, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(GetServiceTypeResponse)
StartSimplePaymentRequest = _reflection.GeneratedProtocolMessageType('StartSimplePaymentRequest', (_message.Message,), {'DESCRIPTOR': _STARTSIMPLEPAYMENTREQUEST, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(StartSimplePaymentRequest)
StartSimplePaymentResponse = _reflection.GeneratedProtocolMessageType('StartSimplePaymentResponse', (_message.Message,), {'DESCRIPTOR': _STARTSIMPLEPAYMENTRESPONSE, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(StartSimplePaymentResponse)
StartFundsChangeRequest = _reflection.GeneratedProtocolMessageType('StartFundsChangeRequest', (_message.Message,), {'DESCRIPTOR': _STARTFUNDSCHANGEREQUEST, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(StartFundsChangeRequest)
StartFundsChangeResponse = _reflection.GeneratedProtocolMessageType('StartFundsChangeResponse', (_message.Message,), {'DESCRIPTOR': _STARTFUNDSCHANGERESPONSE, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(StartFundsChangeResponse)
TransactionSnapshot = _reflection.GeneratedProtocolMessageType('TransactionSnapshot', (_message.Message,), {'DESCRIPTOR': _TRANSACTIONSNAPSHOT, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(TransactionSnapshot)
Transaction = _reflection.GeneratedProtocolMessageType('Transaction', (_message.Message,), {'DESCRIPTOR': _TRANSACTION, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(Transaction)
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1'
    _SERVICETYPE._serialized_start = 2158
    _SERVICETYPE._serialized_end = 2355
    _SIGNATURESYSTEM._serialized_start = 2357
    _SIGNATURESYSTEM._serialized_end = 2470
    _TRANSACTIONSTATUS._serialized_start = 2473
    _TRANSACTIONSTATUS._serialized_end = 2617
    _DYNAMICVERIFIER._serialized_start = 34
    _DYNAMICVERIFIER._serialized_end = 120
    _AMOUNT._serialized_start = 122
    _AMOUNT._serialized_end = 187
    _AUTHENTICATIONREQUEST._serialized_start = 189
    _AUTHENTICATIONREQUEST._serialized_end = 255
    _AUTHENTICATIONRESPONSE._serialized_start = 257
    _AUTHENTICATIONRESPONSE._serialized_end = 300
    _ONECONNECTION._serialized_start = 302
    _ONECONNECTION._serialized_end = 317
    _MANYCONNECTION._serialized_start = 319
    _MANYCONNECTION._serialized_end = 390
    _ASSOCIATEDCONNECTION._serialized_start = 392
    _ASSOCIATEDCONNECTION._serialized_end = 519
    _CONNECTIONREQUEST._serialized_start = 522
    _CONNECTIONREQUEST._serialized_end = 687
    _AUTHENTICATIONSIGNATURE._serialized_start = 689
    _AUTHENTICATIONSIGNATURE._serialized_end = 776
    _AUTHORIZEDSIGNATUREREQUEST._serialized_start = 778
    _AUTHORIZEDSIGNATUREREQUEST._serialized_end = 831
    _AUTHORIZEDSIGNATURERESPONSE._serialized_start = 833
    _AUTHORIZEDSIGNATURERESPONSE._serialized_end = 887
    _MEDIA._serialized_start = 889
    _MEDIA._serialized_end = 929
    _ROLE._serialized_start = 932
    _ROLE._serialized_end = 1102
    _ROLE_ISSUEPERMISSIONS._serialized_start = 1040
    _ROLE_ISSUEPERMISSIONS._serialized_end = 1089
    _GETVERIFIERREQUEST._serialized_start = 1104
    _GETVERIFIERREQUEST._serialized_end = 1124
    _GETVERIFIERRESPONSE._serialized_start = 1126
    _GETVERIFIERRESPONSE._serialized_end = 1206
    _GETSERVICETYPEREQUEST._serialized_start = 1208
    _GETSERVICETYPEREQUEST._serialized_end = 1231
    _GETSERVICETYPERESPONSE._serialized_start = 1233
    _GETSERVICETYPERESPONSE._serialized_end = 1300
    _STARTSIMPLEPAYMENTREQUEST._serialized_start = 1302
    _STARTSIMPLEPAYMENTREQUEST._serialized_end = 1426
    _STARTSIMPLEPAYMENTRESPONSE._serialized_start = 1428
    _STARTSIMPLEPAYMENTRESPONSE._serialized_end = 1470
    _STARTFUNDSCHANGEREQUEST._serialized_start = 1472
    _STARTFUNDSCHANGEREQUEST._serialized_end = 1573
    _STARTFUNDSCHANGERESPONSE._serialized_start = 1575
    _STARTFUNDSCHANGERESPONSE._serialized_end = 1615
    _TRANSACTIONSNAPSHOT._serialized_start = 1618
    _TRANSACTIONSNAPSHOT._serialized_end = 1765
    _TRANSACTION._serialized_start = 1768
    _TRANSACTION._serialized_end = 2155
    _TRANSACTION_ROLE._serialized_start = 2037
    _TRANSACTION_ROLE._serialized_end = 2081
    _TRANSACTION_TYPE._serialized_start = 2083
    _TRANSACTION_TYPE._serialized_end = 2155