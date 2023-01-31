"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1anotary_api/v1/notary.proto\x12\x06notary\x1a\x16common/v1/common.proto2b\n\x06Notary\x12X\n\tAuthorize\x12".common.AuthorizedSignatureRequest\x1a#.common.AuthorizedSignatureResponse(\x010\x01B9Z7github.com/knox-networks/grpc-sdks/sdk/go/notary_api/v1b\x06proto3')
_NOTARY = DESCRIPTOR.services_by_name['Notary']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z7github.com/knox-networks/grpc-sdks/sdk/go/notary_api/v1'
    _NOTARY._serialized_start = 62
    _NOTARY._serialized_end = 160