"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1fcommon_private/v1/service.proto\x12\x06common\x1a\x16common/v1/common.proto2\xc4\x01\n\x12TransactionService\x12Y\n\x10NewSimplePayment\x12!.common.StartSimplePaymentRequest\x1a".common.StartSimplePaymentResponse\x12S\n\x0eNewFundsChange\x12\x1f.common.StartFundsChangeRequest\x1a .common.StartFundsChangeResponse2\xa6\x01\n\x0bServiceInfo\x12F\n\x0bGetVerifier\x12\x1a.common.GetVerifierRequest\x1a\x1b.common.GetVerifierResponse\x12O\n\x0eGetServiceType\x12\x1d.common.GetServiceTypeRequest\x1a\x1e.common.GetServiceTypeResponseB=Z;github.com/knox-networks/grpc-sdks/sdk/go/common_private/v1b\x06proto3')
_TRANSACTIONSERVICE = DESCRIPTOR.services_by_name['TransactionService']
_SERVICEINFO = DESCRIPTOR.services_by_name['ServiceInfo']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z;github.com/knox-networks/grpc-sdks/sdk/go/common_private/v1'
    _TRANSACTIONSERVICE._serialized_start = 68
    _TRANSACTIONSERVICE._serialized_end = 264
    _SERVICEINFO._serialized_start = 267
    _SERVICEINFO._serialized_end = 433