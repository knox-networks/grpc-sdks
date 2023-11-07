"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n"orchestrator/v1/orchestrator.proto\x12\x0corchestrator\x1a\x16common/v1/common.proto"\xfb\x01\n\x0fAgentConnection\x12)\n\x08verifier\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x124\n\x06status\x18\x02 \x01(\x0e2$.orchestrator.AgentConnection.Status"\x86\x01\n\x06Status\x12\x12\n\x0eSTATUS_UNKNOWN\x10\x00\x12\x14\n\x10STATUS_CONNECTED\x10\x01\x12\x17\n\x13STATUS_DISCONNECTED\x10\x02\x12\x12\n\x0eSTATUS_EXECUTE\x10\x03\x12\x13\n\x0fSTATUS_SHUTDOWN\x10\x04\x12\x10\n\x0cSTATUS_SETUP\x10\x05"\x10\n\x0eExecuteRequest"\x11\n\x0fExecuteResponse"\x11\n\x0fShutdownRequest"\x12\n\x10ShutdownResponse"\x0e\n\x0cSetupRequest"\x0f\n\rSetupResponse2\xbe\x02\n\x1bMonetaeOverloadOrchestrator\x12J\n\x0eBroadcastAgent\x12\x17.common.DynamicVerifier\x1a\x1d.orchestrator.AgentConnection0\x01\x12F\n\x07Execute\x12\x1c.orchestrator.ExecuteRequest\x1a\x1d.orchestrator.ExecuteResponse\x12I\n\x08Shutdown\x12\x1d.orchestrator.ShutdownRequest\x1a\x1e.orchestrator.ShutdownResponse\x12@\n\x05Setup\x12\x1a.orchestrator.SetupRequest\x1a\x1b.orchestrator.SetupResponseB;Z9github.com/knox-networks/grpc-sdks/sdk/go/orchestrator/v1b\x06proto3')
_AGENTCONNECTION = DESCRIPTOR.message_types_by_name['AgentConnection']
_EXECUTEREQUEST = DESCRIPTOR.message_types_by_name['ExecuteRequest']
_EXECUTERESPONSE = DESCRIPTOR.message_types_by_name['ExecuteResponse']
_SHUTDOWNREQUEST = DESCRIPTOR.message_types_by_name['ShutdownRequest']
_SHUTDOWNRESPONSE = DESCRIPTOR.message_types_by_name['ShutdownResponse']
_SETUPREQUEST = DESCRIPTOR.message_types_by_name['SetupRequest']
_SETUPRESPONSE = DESCRIPTOR.message_types_by_name['SetupResponse']
_AGENTCONNECTION_STATUS = _AGENTCONNECTION.enum_types_by_name['Status']
AgentConnection = _reflection.GeneratedProtocolMessageType('AgentConnection', (_message.Message,), {'DESCRIPTOR': _AGENTCONNECTION, '__module__': 'orchestrator.v1.orchestrator_pb2'})
_sym_db.RegisterMessage(AgentConnection)
ExecuteRequest = _reflection.GeneratedProtocolMessageType('ExecuteRequest', (_message.Message,), {'DESCRIPTOR': _EXECUTEREQUEST, '__module__': 'orchestrator.v1.orchestrator_pb2'})
_sym_db.RegisterMessage(ExecuteRequest)
ExecuteResponse = _reflection.GeneratedProtocolMessageType('ExecuteResponse', (_message.Message,), {'DESCRIPTOR': _EXECUTERESPONSE, '__module__': 'orchestrator.v1.orchestrator_pb2'})
_sym_db.RegisterMessage(ExecuteResponse)
ShutdownRequest = _reflection.GeneratedProtocolMessageType('ShutdownRequest', (_message.Message,), {'DESCRIPTOR': _SHUTDOWNREQUEST, '__module__': 'orchestrator.v1.orchestrator_pb2'})
_sym_db.RegisterMessage(ShutdownRequest)
ShutdownResponse = _reflection.GeneratedProtocolMessageType('ShutdownResponse', (_message.Message,), {'DESCRIPTOR': _SHUTDOWNRESPONSE, '__module__': 'orchestrator.v1.orchestrator_pb2'})
_sym_db.RegisterMessage(ShutdownResponse)
SetupRequest = _reflection.GeneratedProtocolMessageType('SetupRequest', (_message.Message,), {'DESCRIPTOR': _SETUPREQUEST, '__module__': 'orchestrator.v1.orchestrator_pb2'})
_sym_db.RegisterMessage(SetupRequest)
SetupResponse = _reflection.GeneratedProtocolMessageType('SetupResponse', (_message.Message,), {'DESCRIPTOR': _SETUPRESPONSE, '__module__': 'orchestrator.v1.orchestrator_pb2'})
_sym_db.RegisterMessage(SetupResponse)
_MONETAEOVERLOADORCHESTRATOR = DESCRIPTOR.services_by_name['MonetaeOverloadOrchestrator']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z9github.com/knox-networks/grpc-sdks/sdk/go/orchestrator/v1'
    _AGENTCONNECTION._serialized_start = 77
    _AGENTCONNECTION._serialized_end = 328
    _AGENTCONNECTION_STATUS._serialized_start = 194
    _AGENTCONNECTION_STATUS._serialized_end = 328
    _EXECUTEREQUEST._serialized_start = 330
    _EXECUTEREQUEST._serialized_end = 346
    _EXECUTERESPONSE._serialized_start = 348
    _EXECUTERESPONSE._serialized_end = 365
    _SHUTDOWNREQUEST._serialized_start = 367
    _SHUTDOWNREQUEST._serialized_end = 384
    _SHUTDOWNRESPONSE._serialized_start = 386
    _SHUTDOWNRESPONSE._serialized_end = 404
    _SETUPREQUEST._serialized_start = 406
    _SETUPREQUEST._serialized_end = 420
    _SETUPRESPONSE._serialized_start = 422
    _SETUPRESPONSE._serialized_end = 437
    _MONETAEOVERLOADORCHESTRATOR._serialized_start = 440
    _MONETAEOVERLOADORCHESTRATOR._serialized_end = 758