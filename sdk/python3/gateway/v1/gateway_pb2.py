"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
from ...common.v1 import packet_pb2 as common_dot_v1_dot_packet__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x18gateway/v1/gateway.proto\x12\x07gateway\x1a\x16common/v1/common.proto\x1a\x16common/v1/packet.proto"d\n\x08LogEntry\x12\x10\n\x08verifier\x18\x01 \x01(\t\x12\x0c\n\x04note\x18\x02 \x01(\t\x12\x11\n\ttimestamp\x18\x03 \x01(\t\x12\x0e\n\x06origin\x18\x04 \x01(\t\x12\x15\n\rconnection_id\x18\x05 \x01(\t"\x14\n\x12ConnectionsRequest"m\n\x13ConnectionsResponse\x12\x0e\n\x06active\x18\x01 \x03(\t\x12"\n\x07created\x18\x02 \x03(\x0b2\x11.gateway.LogEntry\x12"\n\x07removed\x18\x03 \x03(\x0b2\x11.gateway.LogEntry2\xd8\x01\n\x0eMonetaeGateway\x12M\n\x0cAuthenticate\x12\x1d.common.AuthenticationRequest\x1a\x1e.common.AuthenticationResponse\x12*\n\x04Data\x12\x0e.common.Packet\x1a\x0e.common.Packet(\x010\x01\x12K\n\x0eGetConnections\x12\x1b.gateway.ConnectionsRequest\x1a\x1c.gateway.ConnectionsResponseB6Z4github.com/knox-networks/grpc-sdks/sdk/go/gateway/v1b\x06proto3')
_LOGENTRY = DESCRIPTOR.message_types_by_name['LogEntry']
_CONNECTIONSREQUEST = DESCRIPTOR.message_types_by_name['ConnectionsRequest']
_CONNECTIONSRESPONSE = DESCRIPTOR.message_types_by_name['ConnectionsResponse']
LogEntry = _reflection.GeneratedProtocolMessageType('LogEntry', (_message.Message,), {'DESCRIPTOR': _LOGENTRY, '__module__': 'gateway.v1.gateway_pb2'})
_sym_db.RegisterMessage(LogEntry)
ConnectionsRequest = _reflection.GeneratedProtocolMessageType('ConnectionsRequest', (_message.Message,), {'DESCRIPTOR': _CONNECTIONSREQUEST, '__module__': 'gateway.v1.gateway_pb2'})
_sym_db.RegisterMessage(ConnectionsRequest)
ConnectionsResponse = _reflection.GeneratedProtocolMessageType('ConnectionsResponse', (_message.Message,), {'DESCRIPTOR': _CONNECTIONSRESPONSE, '__module__': 'gateway.v1.gateway_pb2'})
_sym_db.RegisterMessage(ConnectionsResponse)
_MONETAEGATEWAY = DESCRIPTOR.services_by_name['MonetaeGateway']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z4github.com/knox-networks/grpc-sdks/sdk/go/gateway/v1'
    _LOGENTRY._serialized_start = 85
    _LOGENTRY._serialized_end = 185
    _CONNECTIONSREQUEST._serialized_start = 187
    _CONNECTIONSREQUEST._serialized_end = 207
    _CONNECTIONSRESPONSE._serialized_start = 209
    _CONNECTIONSRESPONSE._serialized_end = 318
    _MONETAEGATEWAY._serialized_start = 321
    _MONETAEGATEWAY._serialized_end = 537