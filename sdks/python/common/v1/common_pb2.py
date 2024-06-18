# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: common/v1/common.proto
# Protobuf Python Version: 5.27.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    27,
    1,
    '',
    'common/v1/common.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x16\x63ommon/v1/common.proto\x12\x06\x63ommon\"V\n\x0f\x44ynamicVerifier\x12\x31\n\x10signature_system\x18\x01 \x01(\x0e\x32\x17.common.SignatureSystem\x12\x10\n\x08verifier\x18\x02 \x01(\x0c\"A\n\x06\x41mount\x12\x15\n\rcurrency_code\x18\x01 \x01(\t\x12\x0e\n\x06\x61mount\x18\x02 \x01(\x03\x12\x10\n\x08\x64\x65\x63imals\x18\x03 \x01(\r\"l\n\x0c\x44istribution\x12.\n\x05value\x18\x01 \x03(\x0b\x32\x1f.common.Distribution.ValueEntry\x1a,\n\nValueEntry\x12\x0b\n\x03key\x18\x01 \x01(\x03\x12\r\n\x05value\x18\x02 \x01(\x05:\x02\x38\x01\"Y\n\x15\x41uthenticationRequest\x12)\n\x08verifier\x18\x01 \x01(\x0b\x32\x17.common.DynamicVerifier\x12\x15\n\rconnection_id\x18\x02 \x01(\t\"+\n\x16\x41uthenticationResponse\x12\x11\n\tchallenge\x18\x01 \x01(\x0c\"\x0f\n\rOneConnection\"E\n\x16ManyVerifierConnection\x12+\n\nassociated\x18\x02 \x03(\x0b\x32\x17.common.DynamicVerifier\"(\n\x12ManyUetrConnection\x12\x12\n\nassociated\x18\x03 \x03(\t\"\x87\x01\n\x1c\x41ssociatedVerifierConnection\x12\x31\n\x10primary_verifier\x18\x01 \x01(\x0b\x32\x17.common.DynamicVerifier\x12\x34\n\x13\x61ssociated_verifier\x18\x02 \x01(\x0b\x32\x17.common.DynamicVerifier\"\xfe\x01\n\x11\x43onnectionRequest\x12$\n\x03one\x18\x01 \x01(\x0b\x32\x15.common.OneConnectionH\x00\x12\x37\n\rmany_verifier\x18\x02 \x01(\x0b\x32\x1e.common.ManyVerifierConnectionH\x00\x12/\n\tmany_uetr\x18\x03 \x01(\x0b\x32\x1a.common.ManyUetrConnectionH\x00\x12\x32\n\tsignature\x18\x04 \x01(\x0b\x32\x1f.common.AuthenticationSignature\x12\x15\n\rconnection_id\x18\x05 \x01(\tB\x0e\n\x0crequest_type\"W\n\x17\x41uthenticationSignature\x12)\n\x08verifier\x18\x01 \x01(\x0b\x32\x17.common.DynamicVerifier\x12\x11\n\tsignature\x18\x02 \x01(\x0c\"6\n\x1b\x41uthorizedSignatureResponse\x12\x17\n\x0fpromissory_file\x18\x01 \x01(\x0c\"(\n\x05Media\x12\x11\n\tmime_type\x18\x01 \x01(\t\x12\x0c\n\x04\x64\x61ta\x18\x02 \x01(\x0c\"\xaa\x01\n\x04Role\x12\x10\n\x08verifier\x18\x01 \x01(\t\x12\x14\n\nauthorized\x18\x03 \x01(\x08H\x00\x12:\n\x11issue_permissions\x18\x04 \x01(\x0b\x32\x1d.common.Role.IssuePermissionsH\x00\x1a\x31\n\x10IssuePermissions\x12\x1d\n\x05inner\x18\x02 \x03(\x0b\x32\x0e.common.AmountB\x0b\n\trole_enum\"\x14\n\x12GetVerifierRequest\"P\n\x13GetVerifierResponse\x12&\n\x05\x62ytes\x18\x01 \x01(\x0b\x32\x17.common.DynamicVerifier\x12\x11\n\tmultibase\x18\x02 \x01(\t\"\x17\n\x15GetServiceTypeRequest\"C\n\x16GetServiceTypeResponse\x12)\n\x0cservice_type\x18\x01 \x01(\x0e\x32\x13.common.ServiceType\"\x14\n\x12GetInstanceRequest\"*\n\x13GetInstanceResponse\x12\x13\n\x0binstance_id\x18\x01 \x01(\t\"e\n\x17StartFundsChangeRequest\x12\x1e\n\x06\x61mount\x18\x01 \x01(\x0b\x32\x0e.common.Amount\x12\x12\n\npromissory\x18\x02 \x01(\t\x12\x16\n\x0euser_reference\x18\x03 \x01(\t\"(\n\x18StartFundsChangeResponse\x12\x0c\n\x04uetr\x18\x01 \x01(\t\"\xcb\x03\n\nFilterItem\x12\r\n\x05\x66ield\x18\x01 \x01(\t\x12\x33\n\x08operator\x18\x02 \x01(\x0e\x32!.common.FilterItem.FilterOperator\x12\x13\n\tstr_value\x18\x03 \x01(\tH\x00\x12\x13\n\tint_value\x18\x04 \x01(\x03H\x00\x12\x14\n\nbool_value\x18\x05 \x01(\x08H\x00\x12 \n\x16rfc3339_datetime_value\x18\x06 \x01(\tH\x00\x12\x14\n\nuuid_value\x18\x07 \x01(\tH\x00\"\xf7\x01\n\x0e\x46ilterOperator\x12\x1f\n\x1b\x46ILTER_OPERATOR_UNSPECIFIED\x10\x00\x12\x16\n\x12\x46ILTER_OPERATOR_EQ\x10\x01\x12\x16\n\x12\x46ILTER_OPERATOR_NE\x10\x02\x12\x16\n\x12\x46ILTER_OPERATOR_GT\x10\x03\x12\x17\n\x13\x46ILTER_OPERATOR_GTE\x10\x04\x12\x16\n\x12\x46ILTER_OPERATOR_LT\x10\x05\x12\x17\n\x13\x46ILTER_OPERATOR_LTE\x10\x06\x12\x16\n\x12\x46ILTER_OPERATOR_IN\x10\x07\x12\x1a\n\x16\x46ILTER_OPERATOR_NOT_IN\x10\x08\x42\x07\n\x05value\"X\n\x06\x46ilter\x12#\n\x07\x66ilters\x18\x01 \x03(\x0b\x32\x12.common.FilterItem\x12)\n\x08operator\x18\x02 \x01(\x0e\x32\x17.common.LogicalOperator\"C\n\x08SortItem\x12\r\n\x05\x66ield\x18\x01 \x01(\t\x12(\n\tdirection\x18\x02 \x01(\x0e\x32\x15.common.SortDirection\")\n\x06SortBy\x12\x1f\n\x05items\x18\x01 \x03(\x0b\x32\x10.common.SortItem\"/\n\x0e\x41ssetAuthority\x12\x10\n\x08verifier\x18\x01 \x01(\t\x12\x0b\n\x03url\x18\x02 \x01(\t\"=\n\x11\x41ssetDenomination\x12\x15\n\rdenominations\x18\x01 \x03(\x03\x12\x11\n\tprecision\x18\x02 \x01(\r*\xeb\x01\n\x0bServiceType\x12\x12\n\x0eUnknownService\x10\x00\x12\x0b\n\x07Gateway\x10\x01\x12\n\n\x06Notary\x10\x02\x12\r\n\tAuthority\x10\x03\x12\x0c\n\x08Treasury\x10\x04\x12\x0c\n\x08\x45missary\x10\x05\x12\n\n\x06Shared\x10\x06\x12\x0c\n\x08Identity\x10\x07\x12\t\n\x05Vault\x10\x08\x12\x13\n\x0f\x43ustodialWallet\x10\t\x12\n\n\x06Issuer\x10\n\x12\x0c\n\x08Overload\x10\x0b\x12\n\n\x06\x46\x61ucet\x10\x0c\x12\x16\n\x12TransactionManager\x10\r\x12\x0c\n\x08Recovery\x10\x0e*q\n\x0fSignatureSystem\x12 \n\x1cSIGNATURE_SYSTEM_UNSPECIFIED\x10\x00\x12\x1c\n\x18SIGNATURE_SYSTEM_ED25519\x10\x01\x12\x1e\n\x1aSIGNATURE_SYSTEM_SECP256K1\x10\x02*f\n\x0fLogicalOperator\x12 \n\x1cLOGICAL_OPERATOR_UNSPECIFIED\x10\x00\x12\x18\n\x14LOGICAL_OPERATOR_AND\x10\x01\x12\x17\n\x13LOGICAL_OPERATOR_OR\x10\x02*l\n\rSortDirection\x12\x1e\n\x1aSORT_DIRECTION_UNSPECIFIED\x10\x00\x12\x1c\n\x18SORT_DIRECTION_ASCENDING\x10\x01\x12\x1d\n\x19SORT_DIRECTION_DESCENDING\x10\x02\x42\x04Z\x02./b\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'common.v1.common_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'Z\002./'
  _globals['_DISTRIBUTION_VALUEENTRY']._loaded_options = None
  _globals['_DISTRIBUTION_VALUEENTRY']._serialized_options = b'8\001'
  _globals['_SERVICETYPE']._serialized_start=2506
  _globals['_SERVICETYPE']._serialized_end=2741
  _globals['_SIGNATURESYSTEM']._serialized_start=2743
  _globals['_SIGNATURESYSTEM']._serialized_end=2856
  _globals['_LOGICALOPERATOR']._serialized_start=2858
  _globals['_LOGICALOPERATOR']._serialized_end=2960
  _globals['_SORTDIRECTION']._serialized_start=2962
  _globals['_SORTDIRECTION']._serialized_end=3070
  _globals['_DYNAMICVERIFIER']._serialized_start=34
  _globals['_DYNAMICVERIFIER']._serialized_end=120
  _globals['_AMOUNT']._serialized_start=122
  _globals['_AMOUNT']._serialized_end=187
  _globals['_DISTRIBUTION']._serialized_start=189
  _globals['_DISTRIBUTION']._serialized_end=297
  _globals['_DISTRIBUTION_VALUEENTRY']._serialized_start=253
  _globals['_DISTRIBUTION_VALUEENTRY']._serialized_end=297
  _globals['_AUTHENTICATIONREQUEST']._serialized_start=299
  _globals['_AUTHENTICATIONREQUEST']._serialized_end=388
  _globals['_AUTHENTICATIONRESPONSE']._serialized_start=390
  _globals['_AUTHENTICATIONRESPONSE']._serialized_end=433
  _globals['_ONECONNECTION']._serialized_start=435
  _globals['_ONECONNECTION']._serialized_end=450
  _globals['_MANYVERIFIERCONNECTION']._serialized_start=452
  _globals['_MANYVERIFIERCONNECTION']._serialized_end=521
  _globals['_MANYUETRCONNECTION']._serialized_start=523
  _globals['_MANYUETRCONNECTION']._serialized_end=563
  _globals['_ASSOCIATEDVERIFIERCONNECTION']._serialized_start=566
  _globals['_ASSOCIATEDVERIFIERCONNECTION']._serialized_end=701
  _globals['_CONNECTIONREQUEST']._serialized_start=704
  _globals['_CONNECTIONREQUEST']._serialized_end=958
  _globals['_AUTHENTICATIONSIGNATURE']._serialized_start=960
  _globals['_AUTHENTICATIONSIGNATURE']._serialized_end=1047
  _globals['_AUTHORIZEDSIGNATURERESPONSE']._serialized_start=1049
  _globals['_AUTHORIZEDSIGNATURERESPONSE']._serialized_end=1103
  _globals['_MEDIA']._serialized_start=1105
  _globals['_MEDIA']._serialized_end=1145
  _globals['_ROLE']._serialized_start=1148
  _globals['_ROLE']._serialized_end=1318
  _globals['_ROLE_ISSUEPERMISSIONS']._serialized_start=1256
  _globals['_ROLE_ISSUEPERMISSIONS']._serialized_end=1305
  _globals['_GETVERIFIERREQUEST']._serialized_start=1320
  _globals['_GETVERIFIERREQUEST']._serialized_end=1340
  _globals['_GETVERIFIERRESPONSE']._serialized_start=1342
  _globals['_GETVERIFIERRESPONSE']._serialized_end=1422
  _globals['_GETSERVICETYPEREQUEST']._serialized_start=1424
  _globals['_GETSERVICETYPEREQUEST']._serialized_end=1447
  _globals['_GETSERVICETYPERESPONSE']._serialized_start=1449
  _globals['_GETSERVICETYPERESPONSE']._serialized_end=1516
  _globals['_GETINSTANCEREQUEST']._serialized_start=1518
  _globals['_GETINSTANCEREQUEST']._serialized_end=1538
  _globals['_GETINSTANCERESPONSE']._serialized_start=1540
  _globals['_GETINSTANCERESPONSE']._serialized_end=1582
  _globals['_STARTFUNDSCHANGEREQUEST']._serialized_start=1584
  _globals['_STARTFUNDSCHANGEREQUEST']._serialized_end=1685
  _globals['_STARTFUNDSCHANGERESPONSE']._serialized_start=1687
  _globals['_STARTFUNDSCHANGERESPONSE']._serialized_end=1727
  _globals['_FILTERITEM']._serialized_start=1730
  _globals['_FILTERITEM']._serialized_end=2189
  _globals['_FILTERITEM_FILTEROPERATOR']._serialized_start=1933
  _globals['_FILTERITEM_FILTEROPERATOR']._serialized_end=2180
  _globals['_FILTER']._serialized_start=2191
  _globals['_FILTER']._serialized_end=2279
  _globals['_SORTITEM']._serialized_start=2281
  _globals['_SORTITEM']._serialized_end=2348
  _globals['_SORTBY']._serialized_start=2350
  _globals['_SORTBY']._serialized_end=2391
  _globals['_ASSETAUTHORITY']._serialized_start=2393
  _globals['_ASSETAUTHORITY']._serialized_end=2440
  _globals['_ASSETDENOMINATION']._serialized_start=2442
  _globals['_ASSETDENOMINATION']._serialized_end=2503
# @@protoc_insertion_point(module_scope)
