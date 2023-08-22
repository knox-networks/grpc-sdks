"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x16common/v1/common.proto\x12\x06common"V\n\x0fDynamicVerifier\x121\n\x10signature_system\x18\x01 \x01(\x0e2\x17.common.SignatureSystem\x12\x10\n\x08verifier\x18\x02 \x01(\x0c"A\n\x06Amount\x12\x15\n\rcurrency_code\x18\x01 \x01(\t\x12\x0e\n\x06amount\x18\x02 \x01(\x03\x12\x10\n\x08decimals\x18\x03 \x01(\r"l\n\x0cDistribution\x12.\n\x05value\x18\x01 \x03(\x0b2\x1f.common.Distribution.ValueEntry\x1a,\n\nValueEntry\x12\x0b\n\x03key\x18\x01 \x01(\x03\x12\r\n\x05value\x18\x02 \x01(\x05:\x028\x01"Y\n\x15AuthenticationRequest\x12)\n\x08verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x12\x15\n\rconnection_id\x18\x02 \x01(\t"+\n\x16AuthenticationResponse\x12\x11\n\tchallenge\x18\x01 \x01(\x0c"\x0f\n\rOneConnection"G\n\x0eManyConnection\x125\n\x14associated_verifiers\x18\x02 \x03(\x0b2\x17.common.DynamicVerifier"\x7f\n\x14AssociatedConnection\x121\n\x10primary_verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x124\n\x13associated_verifier\x18\x02 \x01(\x0b2\x17.common.DynamicVerifier"\xbc\x01\n\x11ConnectionRequest\x12$\n\x03one\x18\x01 \x01(\x0b2\x15.common.OneConnectionH\x00\x12&\n\x04many\x18\x02 \x01(\x0b2\x16.common.ManyConnectionH\x00\x122\n\tsignature\x18\x03 \x01(\x0b2\x1f.common.AuthenticationSignature\x12\x15\n\rconnection_id\x18\x04 \x01(\tB\x0e\n\x0crequest_type"W\n\x17AuthenticationSignature\x12)\n\x08verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x12\x11\n\tsignature\x18\x02 \x01(\x0c"6\n\x1bAuthorizedSignatureResponse\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c"(\n\x05Media\x12\x11\n\tmime_type\x18\x01 \x01(\t\x12\x0c\n\x04data\x18\x02 \x01(\x0c"\xaa\x01\n\x04Role\x12\x10\n\x08verifier\x18\x01 \x01(\t\x12\x14\n\nauthorized\x18\x03 \x01(\x08H\x00\x12:\n\x11issue_permissions\x18\x04 \x01(\x0b2\x1d.common.Role.IssuePermissionsH\x00\x1a1\n\x10IssuePermissions\x12\x1d\n\x05inner\x18\x02 \x03(\x0b2\x0e.common.AmountB\x0b\n\trole_enum"\x14\n\x12GetVerifierRequest"P\n\x13GetVerifierResponse\x12&\n\x05bytes\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x12\x11\n\tmultibase\x18\x02 \x01(\t"\x17\n\x15GetServiceTypeRequest"C\n\x16GetServiceTypeResponse\x12)\n\x0cservice_type\x18\x01 \x01(\x0e2\x13.common.ServiceType"\x14\n\x12GetInstanceRequest"*\n\x13GetInstanceResponse\x12\x13\n\x0binstance_id\x18\x01 \x01(\t"|\n\x19StartSimplePaymentRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x11\n\trecipient\x18\x02 \x01(\t\x12\x14\n\x0cpromissories\x18\x03 \x03(\t\x12\x16\n\x0euser_reference\x18\x04 \x01(\t"*\n\x1aStartSimplePaymentResponse\x12\x0c\n\x04uetr\x18\x01 \x01(\t"e\n\x17StartFundsChangeRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x12\n\npromissory\x18\x02 \x01(\t\x12\x16\n\x0euser_reference\x18\x03 \x01(\t"(\n\x18StartFundsChangeResponse\x12\x0c\n\x04uetr\x18\x01 \x01(\t"\x93\x01\n\x13TransactionSnapshot\x12)\n\x06status\x18\x01 \x01(\x0e2\x19.common.TransactionStatus\x12\x14\n\x0cpromissories\x18\x03 \x03(\t\x12*\n\x12amount_transferred\x18\x04 \x01(\x0b2\x0e.common.Amount\x12\x0f\n\x07message\x18\x05 \x01(\t"\x83\x03\n\x0bTransaction\x12&\n\x04type\x18\x01 \x01(\x0e2\x18.common.Transaction.Type\x12&\n\x04role\x18\x02 \x01(\x0e2\x18.common.Transaction.Role\x12\x0c\n\x04uetr\x18\x03 \x01(\t\x12\x1e\n\x06amount\x18\x04 \x01(\x0b2\x0e.common.Amount\x12\x0f\n\x07created\x18\x05 \x01(\t\x12\x0f\n\x07updated\x18\x06 \x01(\t\x12\x0c\n\x04memo\x18\x07 \x01(\t\x12\r\n\x05owner\x18\x08 \x01(\t\x12\x14\n\x0ccounterparty\x18\t \x01(\t\x12)\n\x06status\x18\n \x01(\x0e2\x19.common.TransactionStatus",\n\x04Role\x12\x10\n\x0cROLE_STARTER\x10\x00\x12\x12\n\x0eROLE_RESPONDER\x10\x01"H\n\x04Type\x12\x10\n\x0cTYPE_UNKNOWN\x10\x00\x12\x17\n\x13TYPE_SIMPLE_PAYMENT\x10\x01\x12\x15\n\x11TYPE_FUNDS_CHANGE\x10\x02"\xcb\x03\n\nFilterItem\x12\r\n\x05field\x18\x01 \x01(\t\x123\n\x08operator\x18\x02 \x01(\x0e2!.common.FilterItem.FilterOperator\x12\x13\n\tstr_value\x18\x03 \x01(\tH\x00\x12\x13\n\tint_value\x18\x04 \x01(\x03H\x00\x12\x14\n\nbool_value\x18\x05 \x01(\x08H\x00\x12 \n\x16rfc3339_datetime_value\x18\x06 \x01(\tH\x00\x12\x14\n\nuuid_value\x18\x07 \x01(\tH\x00"\xf7\x01\n\x0eFilterOperator\x12\x1f\n\x1bFILTER_OPERATOR_UNSPECIFIED\x10\x00\x12\x16\n\x12FILTER_OPERATOR_EQ\x10\x01\x12\x16\n\x12FILTER_OPERATOR_NE\x10\x02\x12\x16\n\x12FILTER_OPERATOR_GT\x10\x03\x12\x17\n\x13FILTER_OPERATOR_GTE\x10\x04\x12\x16\n\x12FILTER_OPERATOR_LT\x10\x05\x12\x17\n\x13FILTER_OPERATOR_LTE\x10\x06\x12\x16\n\x12FILTER_OPERATOR_IN\x10\x07\x12\x1a\n\x16FILTER_OPERATOR_NOT_IN\x10\x08B\x07\n\x05value"X\n\x06Filter\x12#\n\x07filters\x18\x01 \x03(\x0b2\x12.common.FilterItem\x12)\n\x08operator\x18\x02 \x01(\x0e2\x17.common.LogicalOperator"C\n\x08SortItem\x12\r\n\x05field\x18\x01 \x01(\t\x12(\n\tdirection\x18\x02 \x01(\x0e2\x15.common.SortDirection")\n\x06SortBy\x12\x1f\n\x05items\x18\x01 \x03(\x0b2\x10.common.SortItem"/\n\x0eAssetAuthority\x12\x10\n\x08verifier\x18\x01 \x01(\t\x12\x0b\n\x03url\x18\x02 \x01(\t"=\n\x11AssetDenomination\x12\x15\n\rdenominations\x18\x01 \x03(\x03\x12\x11\n\tprecision\x18\x02 \x01(\r*\xdd\x01\n\x0bServiceType\x12\x12\n\x0eUnknownService\x10\x00\x12\x0b\n\x07Gateway\x10\x01\x12\n\n\x06Notary\x10\x02\x12\r\n\tAuthority\x10\x03\x12\x0c\n\x08Treasury\x10\x04\x12\x0c\n\x08Emissary\x10\x05\x12\n\n\x06Shared\x10\x06\x12\x0c\n\x08Identity\x10\x07\x12\t\n\x05Vault\x10\x08\x12\x13\n\x0fCustodialWallet\x10\t\x12\n\n\x06Issuer\x10\n\x12\x0c\n\x08Overload\x10\x0b\x12\n\n\x06Faucet\x10\x0c\x12\x16\n\x12TransactionManager\x10\r*q\n\x0fSignatureSystem\x12 \n\x1cSIGNATURE_SYSTEM_UNSPECIFIED\x10\x00\x12\x1c\n\x18SIGNATURE_SYSTEM_ED25519\x10\x01\x12\x1e\n\x1aSIGNATURE_SYSTEM_SECP256K1\x10\x02*\x90\x01\n\x11TransactionStatus\x12\x16\n\x12STATE_INITIALIZING\x10\x00\x12\x15\n\x11STATE_AUTHORIZING\x10\x01\x12\x11\n\rSTATE_SENDING\x10\x02\x12\x13\n\x0fSTATE_RECEIVING\x10\x05\x12\x12\n\x0eSTATE_COMPLETE\x10\x03\x12\x10\n\x0cSTATE_FAILED\x10\x04*f\n\x0fLogicalOperator\x12 \n\x1cLOGICAL_OPERATOR_UNSPECIFIED\x10\x00\x12\x18\n\x14LOGICAL_OPERATOR_AND\x10\x01\x12\x17\n\x13LOGICAL_OPERATOR_OR\x10\x02*l\n\rSortDirection\x12\x1e\n\x1aSORT_DIRECTION_UNSPECIFIED\x10\x00\x12\x1c\n\x18SORT_DIRECTION_ASCENDING\x10\x01\x12\x1d\n\x19SORT_DIRECTION_DESCENDING\x10\x02B5Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1b\x06proto3')
_SERVICETYPE = DESCRIPTOR.enum_types_by_name['ServiceType']
ServiceType = enum_type_wrapper.EnumTypeWrapper(_SERVICETYPE)
_SIGNATURESYSTEM = DESCRIPTOR.enum_types_by_name['SignatureSystem']
SignatureSystem = enum_type_wrapper.EnumTypeWrapper(_SIGNATURESYSTEM)
_TRANSACTIONSTATUS = DESCRIPTOR.enum_types_by_name['TransactionStatus']
TransactionStatus = enum_type_wrapper.EnumTypeWrapper(_TRANSACTIONSTATUS)
_LOGICALOPERATOR = DESCRIPTOR.enum_types_by_name['LogicalOperator']
LogicalOperator = enum_type_wrapper.EnumTypeWrapper(_LOGICALOPERATOR)
_SORTDIRECTION = DESCRIPTOR.enum_types_by_name['SortDirection']
SortDirection = enum_type_wrapper.EnumTypeWrapper(_SORTDIRECTION)
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
TransactionManager = 13
SIGNATURE_SYSTEM_UNSPECIFIED = 0
SIGNATURE_SYSTEM_ED25519 = 1
SIGNATURE_SYSTEM_SECP256K1 = 2
STATE_INITIALIZING = 0
STATE_AUTHORIZING = 1
STATE_SENDING = 2
STATE_RECEIVING = 5
STATE_COMPLETE = 3
STATE_FAILED = 4
LOGICAL_OPERATOR_UNSPECIFIED = 0
LOGICAL_OPERATOR_AND = 1
LOGICAL_OPERATOR_OR = 2
SORT_DIRECTION_UNSPECIFIED = 0
SORT_DIRECTION_ASCENDING = 1
SORT_DIRECTION_DESCENDING = 2
_DYNAMICVERIFIER = DESCRIPTOR.message_types_by_name['DynamicVerifier']
_AMOUNT = DESCRIPTOR.message_types_by_name['Amount']
_DISTRIBUTION = DESCRIPTOR.message_types_by_name['Distribution']
_DISTRIBUTION_VALUEENTRY = _DISTRIBUTION.nested_types_by_name['ValueEntry']
_AUTHENTICATIONREQUEST = DESCRIPTOR.message_types_by_name['AuthenticationRequest']
_AUTHENTICATIONRESPONSE = DESCRIPTOR.message_types_by_name['AuthenticationResponse']
_ONECONNECTION = DESCRIPTOR.message_types_by_name['OneConnection']
_MANYCONNECTION = DESCRIPTOR.message_types_by_name['ManyConnection']
_ASSOCIATEDCONNECTION = DESCRIPTOR.message_types_by_name['AssociatedConnection']
_CONNECTIONREQUEST = DESCRIPTOR.message_types_by_name['ConnectionRequest']
_AUTHENTICATIONSIGNATURE = DESCRIPTOR.message_types_by_name['AuthenticationSignature']
_AUTHORIZEDSIGNATURERESPONSE = DESCRIPTOR.message_types_by_name['AuthorizedSignatureResponse']
_MEDIA = DESCRIPTOR.message_types_by_name['Media']
_ROLE = DESCRIPTOR.message_types_by_name['Role']
_ROLE_ISSUEPERMISSIONS = _ROLE.nested_types_by_name['IssuePermissions']
_GETVERIFIERREQUEST = DESCRIPTOR.message_types_by_name['GetVerifierRequest']
_GETVERIFIERRESPONSE = DESCRIPTOR.message_types_by_name['GetVerifierResponse']
_GETSERVICETYPEREQUEST = DESCRIPTOR.message_types_by_name['GetServiceTypeRequest']
_GETSERVICETYPERESPONSE = DESCRIPTOR.message_types_by_name['GetServiceTypeResponse']
_GETINSTANCEREQUEST = DESCRIPTOR.message_types_by_name['GetInstanceRequest']
_GETINSTANCERESPONSE = DESCRIPTOR.message_types_by_name['GetInstanceResponse']
_STARTSIMPLEPAYMENTREQUEST = DESCRIPTOR.message_types_by_name['StartSimplePaymentRequest']
_STARTSIMPLEPAYMENTRESPONSE = DESCRIPTOR.message_types_by_name['StartSimplePaymentResponse']
_STARTFUNDSCHANGEREQUEST = DESCRIPTOR.message_types_by_name['StartFundsChangeRequest']
_STARTFUNDSCHANGERESPONSE = DESCRIPTOR.message_types_by_name['StartFundsChangeResponse']
_TRANSACTIONSNAPSHOT = DESCRIPTOR.message_types_by_name['TransactionSnapshot']
_TRANSACTION = DESCRIPTOR.message_types_by_name['Transaction']
_FILTERITEM = DESCRIPTOR.message_types_by_name['FilterItem']
_FILTER = DESCRIPTOR.message_types_by_name['Filter']
_SORTITEM = DESCRIPTOR.message_types_by_name['SortItem']
_SORTBY = DESCRIPTOR.message_types_by_name['SortBy']
_ASSETAUTHORITY = DESCRIPTOR.message_types_by_name['AssetAuthority']
_ASSETDENOMINATION = DESCRIPTOR.message_types_by_name['AssetDenomination']
_TRANSACTION_ROLE = _TRANSACTION.enum_types_by_name['Role']
_TRANSACTION_TYPE = _TRANSACTION.enum_types_by_name['Type']
_FILTERITEM_FILTEROPERATOR = _FILTERITEM.enum_types_by_name['FilterOperator']
DynamicVerifier = _reflection.GeneratedProtocolMessageType('DynamicVerifier', (_message.Message,), {'DESCRIPTOR': _DYNAMICVERIFIER, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(DynamicVerifier)
Amount = _reflection.GeneratedProtocolMessageType('Amount', (_message.Message,), {'DESCRIPTOR': _AMOUNT, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(Amount)
Distribution = _reflection.GeneratedProtocolMessageType('Distribution', (_message.Message,), {'ValueEntry': _reflection.GeneratedProtocolMessageType('ValueEntry', (_message.Message,), {'DESCRIPTOR': _DISTRIBUTION_VALUEENTRY, '__module__': 'common.v1.common_pb2'}), 'DESCRIPTOR': _DISTRIBUTION, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(Distribution)
_sym_db.RegisterMessage(Distribution.ValueEntry)
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
GetInstanceRequest = _reflection.GeneratedProtocolMessageType('GetInstanceRequest', (_message.Message,), {'DESCRIPTOR': _GETINSTANCEREQUEST, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(GetInstanceRequest)
GetInstanceResponse = _reflection.GeneratedProtocolMessageType('GetInstanceResponse', (_message.Message,), {'DESCRIPTOR': _GETINSTANCERESPONSE, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(GetInstanceResponse)
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
FilterItem = _reflection.GeneratedProtocolMessageType('FilterItem', (_message.Message,), {'DESCRIPTOR': _FILTERITEM, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(FilterItem)
Filter = _reflection.GeneratedProtocolMessageType('Filter', (_message.Message,), {'DESCRIPTOR': _FILTER, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(Filter)
SortItem = _reflection.GeneratedProtocolMessageType('SortItem', (_message.Message,), {'DESCRIPTOR': _SORTITEM, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(SortItem)
SortBy = _reflection.GeneratedProtocolMessageType('SortBy', (_message.Message,), {'DESCRIPTOR': _SORTBY, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(SortBy)
AssetAuthority = _reflection.GeneratedProtocolMessageType('AssetAuthority', (_message.Message,), {'DESCRIPTOR': _ASSETAUTHORITY, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(AssetAuthority)
AssetDenomination = _reflection.GeneratedProtocolMessageType('AssetDenomination', (_message.Message,), {'DESCRIPTOR': _ASSETDENOMINATION, '__module__': 'common.v1.common_pb2'})
_sym_db.RegisterMessage(AssetDenomination)
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1'
    _DISTRIBUTION_VALUEENTRY._options = None
    _DISTRIBUTION_VALUEENTRY._serialized_options = b'8\x01'
    _SERVICETYPE._serialized_start = 3101
    _SERVICETYPE._serialized_end = 3322
    _SIGNATURESYSTEM._serialized_start = 3324
    _SIGNATURESYSTEM._serialized_end = 3437
    _TRANSACTIONSTATUS._serialized_start = 3440
    _TRANSACTIONSTATUS._serialized_end = 3584
    _LOGICALOPERATOR._serialized_start = 3586
    _LOGICALOPERATOR._serialized_end = 3688
    _SORTDIRECTION._serialized_start = 3690
    _SORTDIRECTION._serialized_end = 3798
    _DYNAMICVERIFIER._serialized_start = 34
    _DYNAMICVERIFIER._serialized_end = 120
    _AMOUNT._serialized_start = 122
    _AMOUNT._serialized_end = 187
    _DISTRIBUTION._serialized_start = 189
    _DISTRIBUTION._serialized_end = 297
    _DISTRIBUTION_VALUEENTRY._serialized_start = 253
    _DISTRIBUTION_VALUEENTRY._serialized_end = 297
    _AUTHENTICATIONREQUEST._serialized_start = 299
    _AUTHENTICATIONREQUEST._serialized_end = 388
    _AUTHENTICATIONRESPONSE._serialized_start = 390
    _AUTHENTICATIONRESPONSE._serialized_end = 433
    _ONECONNECTION._serialized_start = 435
    _ONECONNECTION._serialized_end = 450
    _MANYCONNECTION._serialized_start = 452
    _MANYCONNECTION._serialized_end = 523
    _ASSOCIATEDCONNECTION._serialized_start = 525
    _ASSOCIATEDCONNECTION._serialized_end = 652
    _CONNECTIONREQUEST._serialized_start = 655
    _CONNECTIONREQUEST._serialized_end = 843
    _AUTHENTICATIONSIGNATURE._serialized_start = 845
    _AUTHENTICATIONSIGNATURE._serialized_end = 932
    _AUTHORIZEDSIGNATURERESPONSE._serialized_start = 934
    _AUTHORIZEDSIGNATURERESPONSE._serialized_end = 988
    _MEDIA._serialized_start = 990
    _MEDIA._serialized_end = 1030
    _ROLE._serialized_start = 1033
    _ROLE._serialized_end = 1203
    _ROLE_ISSUEPERMISSIONS._serialized_start = 1141
    _ROLE_ISSUEPERMISSIONS._serialized_end = 1190
    _GETVERIFIERREQUEST._serialized_start = 1205
    _GETVERIFIERREQUEST._serialized_end = 1225
    _GETVERIFIERRESPONSE._serialized_start = 1227
    _GETVERIFIERRESPONSE._serialized_end = 1307
    _GETSERVICETYPEREQUEST._serialized_start = 1309
    _GETSERVICETYPEREQUEST._serialized_end = 1332
    _GETSERVICETYPERESPONSE._serialized_start = 1334
    _GETSERVICETYPERESPONSE._serialized_end = 1401
    _GETINSTANCEREQUEST._serialized_start = 1403
    _GETINSTANCEREQUEST._serialized_end = 1423
    _GETINSTANCERESPONSE._serialized_start = 1425
    _GETINSTANCERESPONSE._serialized_end = 1467
    _STARTSIMPLEPAYMENTREQUEST._serialized_start = 1469
    _STARTSIMPLEPAYMENTREQUEST._serialized_end = 1593
    _STARTSIMPLEPAYMENTRESPONSE._serialized_start = 1595
    _STARTSIMPLEPAYMENTRESPONSE._serialized_end = 1637
    _STARTFUNDSCHANGEREQUEST._serialized_start = 1639
    _STARTFUNDSCHANGEREQUEST._serialized_end = 1740
    _STARTFUNDSCHANGERESPONSE._serialized_start = 1742
    _STARTFUNDSCHANGERESPONSE._serialized_end = 1782
    _TRANSACTIONSNAPSHOT._serialized_start = 1785
    _TRANSACTIONSNAPSHOT._serialized_end = 1932
    _TRANSACTION._serialized_start = 1935
    _TRANSACTION._serialized_end = 2322
    _TRANSACTION_ROLE._serialized_start = 2204
    _TRANSACTION_ROLE._serialized_end = 2248
    _TRANSACTION_TYPE._serialized_start = 2250
    _TRANSACTION_TYPE._serialized_end = 2322
    _FILTERITEM._serialized_start = 2325
    _FILTERITEM._serialized_end = 2784
    _FILTERITEM_FILTEROPERATOR._serialized_start = 2528
    _FILTERITEM_FILTEROPERATOR._serialized_end = 2775
    _FILTER._serialized_start = 2786
    _FILTER._serialized_end = 2874
    _SORTITEM._serialized_start = 2876
    _SORTITEM._serialized_end = 2943
    _SORTBY._serialized_start = 2945
    _SORTBY._serialized_end = 2986
    _ASSETAUTHORITY._serialized_start = 2988
    _ASSETAUTHORITY._serialized_end = 3035
    _ASSETDENOMINATION._serialized_start = 3037
    _ASSETDENOMINATION._serialized_end = 3098