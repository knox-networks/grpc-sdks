"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n"orchestrator/v1/orchestrator.proto\x12\x0corchestrator\x1a\x16common/v1/common.proto"\xbe\x01\n\x0fAgentConnection\x12)\n\x08verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x12>\n\x06status\x18\x02 \x01(\x0e2..orchestrator.AgentConnection.ConnectionStatus"@\n\x10ConnectionStatus\x12\r\n\tCONNECTED\x10\x00\x12\x10\n\x0cDISCONNECTED\x10\x01\x12\x0b\n\x07EXECUTE\x10\x02"\x10\n\x0eExecuteRequest"\x11\n\x0fExecuteResponse2\xb1\x01\n\x1bMonetaeOverloadOrchestrator\x12J\n\x0eBroadcastAgent\x12\x17.common.DynamicVerifier\x1a\x1d.orchestrator.AgentConnection0\x01\x12F\n\x07Execute\x12\x1c.orchestrator.ExecuteRequest\x1a\x1d.orchestrator.ExecuteResponseB;Z9github.com/knox-networks/grpc-sdks/sdk/go/orchestrator/v1b\x06proto3')
_AGENTCONNECTION = DESCRIPTOR.message_types_by_name['AgentConnection']
_EXECUTEREQUEST = DESCRIPTOR.message_types_by_name['ExecuteRequest']
_EXECUTERESPONSE = DESCRIPTOR.message_types_by_name['ExecuteResponse']
_AGENTCONNECTION_CONNECTIONSTATUS = _AGENTCONNECTION.enum_types_by_name['ConnectionStatus']
AgentConnection = _reflection.GeneratedProtocolMessageType('AgentConnection', (_message.Message,), {'DESCRIPTOR': _AGENTCONNECTION, '__module__': 'orchestrator.v1.orchestrator_pb2'})
_sym_db.RegisterMessage(AgentConnection)
ExecuteRequest = _reflection.GeneratedProtocolMessageType('ExecuteRequest', (_message.Message,), {'DESCRIPTOR': _EXECUTEREQUEST, '__module__': 'orchestrator.v1.orchestrator_pb2'})
_sym_db.RegisterMessage(ExecuteRequest)
ExecuteResponse = _reflection.GeneratedProtocolMessageType('ExecuteResponse', (_message.Message,), {'DESCRIPTOR': _EXECUTERESPONSE, '__module__': 'orchestrator.v1.orchestrator_pb2'})
_sym_db.RegisterMessage(ExecuteResponse)
_MONETAEOVERLOADORCHESTRATOR = DESCRIPTOR.services_by_name['MonetaeOverloadOrchestrator']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z9github.com/knox-networks/grpc-sdks/sdk/go/orchestrator/v1'
    _AGENTCONNECTION._serialized_start = 77
    _AGENTCONNECTION._serialized_end = 267
    _AGENTCONNECTION_CONNECTIONSTATUS._serialized_start = 203
    _AGENTCONNECTION_CONNECTIONSTATUS._serialized_end = 267
    _EXECUTEREQUEST._serialized_start = 269
    _EXECUTEREQUEST._serialized_end = 285
    _EXECUTERESPONSE._serialized_start = 287
    _EXECUTERESPONSE._serialized_end = 304
    _MONETAEOVERLOADORCHESTRATOR._serialized_start = 307
    _MONETAEOVERLOADORCHESTRATOR._serialized_end = 484