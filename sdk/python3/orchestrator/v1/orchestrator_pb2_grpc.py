"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
from ...orchestrator.v1 import orchestrator_pb2 as orchestrator_dot_v1_dot_orchestrator__pb2

class OverloadOrchestratorStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.BroadcastAgent = channel.unary_stream('/orchestrator.OverloadOrchestrator/BroadcastAgent', request_serializer=common_dot_v1_dot_common__pb2.DynamicVerifier.SerializeToString, response_deserializer=orchestrator_dot_v1_dot_orchestrator__pb2.AgentConnection.FromString)

class OverloadOrchestratorServicer(object):
    """Missing associated documentation comment in .proto file."""

    def BroadcastAgent(self, request, context):
        """Broadcasts overload agent connections to all agents
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_OverloadOrchestratorServicer_to_server(servicer, server):
    rpc_method_handlers = {'BroadcastAgent': grpc.unary_stream_rpc_method_handler(servicer.BroadcastAgent, request_deserializer=common_dot_v1_dot_common__pb2.DynamicVerifier.FromString, response_serializer=orchestrator_dot_v1_dot_orchestrator__pb2.AgentConnection.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('orchestrator.OverloadOrchestrator', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class OverloadOrchestrator(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def BroadcastAgent(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_stream(request, target, '/orchestrator.OverloadOrchestrator/BroadcastAgent', common_dot_v1_dot_common__pb2.DynamicVerifier.SerializeToString, orchestrator_dot_v1_dot_orchestrator__pb2.AgentConnection.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)