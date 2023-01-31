"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n"orchestrator/v1/orchestrator.proto\x12\x0corchestrator\x1a\x16common/v1/common.proto"\xb1\x01\n\x0fAgentConnection\x12)\n\x08verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x12>\n\x06status\x18\x02 \x01(\x0e2..orchestrator.AgentConnection.ConnectionStatus"3\n\x10ConnectionStatus\x12\r\n\tCONNECTED\x10\x00\x12\x10\n\x0cDISCONNECTED\x10\x012b\n\x14OverloadOrchestrator\x12J\n\x0eBroadcastAgent\x12\x17.common.DynamicVerifier\x1a\x1d.orchestrator.AgentConnection0\x01B;Z9github.com/knox-networks/grpc-sdks/sdk/go/orchestrator/v1b\x06proto3')
_AGENTCONNECTION = DESCRIPTOR.message_types_by_name['AgentConnection']
_AGENTCONNECTION_CONNECTIONSTATUS = _AGENTCONNECTION.enum_types_by_name['ConnectionStatus']
AgentConnection = _reflection.GeneratedProtocolMessageType('AgentConnection', (_message.Message,), {'DESCRIPTOR': _AGENTCONNECTION, '__module__': 'orchestrator.v1.orchestrator_pb2'})
_sym_db.RegisterMessage(AgentConnection)
_OVERLOADORCHESTRATOR = DESCRIPTOR.services_by_name['OverloadOrchestrator']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z9github.com/knox-networks/grpc-sdks/sdk/go/orchestrator/v1'
    _AGENTCONNECTION._serialized_start = 77
    _AGENTCONNECTION._serialized_end = 254
    _AGENTCONNECTION_CONNECTIONSTATUS._serialized_start = 203
    _AGENTCONNECTION_CONNECTIONSTATUS._serialized_end = 254
    _OVERLOADORCHESTRATOR._serialized_start = 256
    _OVERLOADORCHESTRATOR._serialized_end = 354