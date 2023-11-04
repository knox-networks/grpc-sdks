"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
from ...orchestrator.v1 import orchestrator_pb2 as orchestrator_dot_v1_dot_orchestrator__pb2

class MonetaeOverloadOrchestratorStub(object):
    """Orchestrator Service for Managing Load Tests.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.BroadcastAgent = channel.unary_stream('/orchestrator.MonetaeOverloadOrchestrator/BroadcastAgent', request_serializer=common_dot_v1_dot_common__pb2.DynamicVerifier.SerializeToString, response_deserializer=orchestrator_dot_v1_dot_orchestrator__pb2.AgentConnection.FromString)
        self.Execute = channel.unary_unary('/orchestrator.MonetaeOverloadOrchestrator/Execute', request_serializer=orchestrator_dot_v1_dot_orchestrator__pb2.ExecuteRequest.SerializeToString, response_deserializer=orchestrator_dot_v1_dot_orchestrator__pb2.ExecuteResponse.FromString)
        self.Shutdown = channel.unary_unary('/orchestrator.MonetaeOverloadOrchestrator/Shutdown', request_serializer=orchestrator_dot_v1_dot_orchestrator__pb2.ShutdownRequest.SerializeToString, response_deserializer=orchestrator_dot_v1_dot_orchestrator__pb2.ShutdownResponse.FromString)
        self.Setup = channel.unary_unary('/orchestrator.MonetaeOverloadOrchestrator/Setup', request_serializer=orchestrator_dot_v1_dot_orchestrator__pb2.SetupRequest.SerializeToString, response_deserializer=orchestrator_dot_v1_dot_orchestrator__pb2.SetupResponse.FromString)

class MonetaeOverloadOrchestratorServicer(object):
    """Orchestrator Service for Managing Load Tests.
    """

    def BroadcastAgent(self, request, context):
        """Broadcasts overload agent connections to all agents
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Execute(self, request, context):
        """Execute the Overload Agent.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Shutdown(self, request, context):
        """Shutdown all Overload Agents.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Setup(self, request, context):
        """Commands Overload Agents to run test setup.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_MonetaeOverloadOrchestratorServicer_to_server(servicer, server):
    rpc_method_handlers = {'BroadcastAgent': grpc.unary_stream_rpc_method_handler(servicer.BroadcastAgent, request_deserializer=common_dot_v1_dot_common__pb2.DynamicVerifier.FromString, response_serializer=orchestrator_dot_v1_dot_orchestrator__pb2.AgentConnection.SerializeToString), 'Execute': grpc.unary_unary_rpc_method_handler(servicer.Execute, request_deserializer=orchestrator_dot_v1_dot_orchestrator__pb2.ExecuteRequest.FromString, response_serializer=orchestrator_dot_v1_dot_orchestrator__pb2.ExecuteResponse.SerializeToString), 'Shutdown': grpc.unary_unary_rpc_method_handler(servicer.Shutdown, request_deserializer=orchestrator_dot_v1_dot_orchestrator__pb2.ShutdownRequest.FromString, response_serializer=orchestrator_dot_v1_dot_orchestrator__pb2.ShutdownResponse.SerializeToString), 'Setup': grpc.unary_unary_rpc_method_handler(servicer.Setup, request_deserializer=orchestrator_dot_v1_dot_orchestrator__pb2.SetupRequest.FromString, response_serializer=orchestrator_dot_v1_dot_orchestrator__pb2.SetupResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('orchestrator.MonetaeOverloadOrchestrator', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class MonetaeOverloadOrchestrator(object):
    """Orchestrator Service for Managing Load Tests.
    """

    @staticmethod
    def BroadcastAgent(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_stream(request, target, '/orchestrator.MonetaeOverloadOrchestrator/BroadcastAgent', common_dot_v1_dot_common__pb2.DynamicVerifier.SerializeToString, orchestrator_dot_v1_dot_orchestrator__pb2.AgentConnection.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Execute(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/orchestrator.MonetaeOverloadOrchestrator/Execute', orchestrator_dot_v1_dot_orchestrator__pb2.ExecuteRequest.SerializeToString, orchestrator_dot_v1_dot_orchestrator__pb2.ExecuteResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Shutdown(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/orchestrator.MonetaeOverloadOrchestrator/Shutdown', orchestrator_dot_v1_dot_orchestrator__pb2.ShutdownRequest.SerializeToString, orchestrator_dot_v1_dot_orchestrator__pb2.ShutdownResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Setup(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/orchestrator.MonetaeOverloadOrchestrator/Setup', orchestrator_dot_v1_dot_orchestrator__pb2.SetupRequest.SerializeToString, orchestrator_dot_v1_dot_orchestrator__pb2.SetupResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)