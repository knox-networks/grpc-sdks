"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x17common/v1/service.proto\x12\x06common\x1a\x16common/v1/common.proto"\x16\n\x14IsProvisionedRequest"\xaf\x01\n\x15IsProvisionedResponse\x124\n\x06status\x18\x01 \x01(\x0e2$.common.IsProvisionedResponse.Status\x12\x0e\n\x06reason\x18\x02 \x01(\t"P\n\x06Status\x12\x12\n\x0eSTATUS_UNKNOWN\x10\x00\x12\x16\n\x12STATUS_PROVISIONED\x10\x01\x12\x1a\n\x16STATUS_NOT_PROVISIONED\x10\x022\xee\x01\n\x0bServiceInfo\x12F\n\x0bGetVerifier\x12\x1a.common.GetVerifierRequest\x1a\x1b.common.GetVerifierResponse\x12O\n\x0eGetServiceType\x12\x1d.common.GetServiceTypeRequest\x1a\x1e.common.GetServiceTypeResponse\x12F\n\x0bGetInstance\x12\x1a.common.GetInstanceRequest\x1a\x1b.common.GetInstanceResponse2]\n\rStatusService\x12L\n\rIsProvisioned\x12\x1c.common.IsProvisionedRequest\x1a\x1d.common.IsProvisionedResponseB5Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1b\x06proto3')
_ISPROVISIONEDREQUEST = DESCRIPTOR.message_types_by_name['IsProvisionedRequest']
_ISPROVISIONEDRESPONSE = DESCRIPTOR.message_types_by_name['IsProvisionedResponse']
_ISPROVISIONEDRESPONSE_STATUS = _ISPROVISIONEDRESPONSE.enum_types_by_name['Status']
IsProvisionedRequest = _reflection.GeneratedProtocolMessageType('IsProvisionedRequest', (_message.Message,), {'DESCRIPTOR': _ISPROVISIONEDREQUEST, '__module__': 'common.v1.service_pb2'})
_sym_db.RegisterMessage(IsProvisionedRequest)
IsProvisionedResponse = _reflection.GeneratedProtocolMessageType('IsProvisionedResponse', (_message.Message,), {'DESCRIPTOR': _ISPROVISIONEDRESPONSE, '__module__': 'common.v1.service_pb2'})
_sym_db.RegisterMessage(IsProvisionedResponse)
_SERVICEINFO = DESCRIPTOR.services_by_name['ServiceInfo']
_STATUSSERVICE = DESCRIPTOR.services_by_name['StatusService']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1'
    _ISPROVISIONEDREQUEST._serialized_start = 59
    _ISPROVISIONEDREQUEST._serialized_end = 81
    _ISPROVISIONEDRESPONSE._serialized_start = 84
    _ISPROVISIONEDRESPONSE._serialized_end = 259
    _ISPROVISIONEDRESPONSE_STATUS._serialized_start = 179
    _ISPROVISIONEDRESPONSE_STATUS._serialized_end = 259
    _SERVICEINFO._serialized_start = 262
    _SERVICEINFO._serialized_end = 500
    _STATUSSERVICE._serialized_start = 502
    _STATUSSERVICE._serialized_end = 595