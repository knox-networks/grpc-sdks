"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...faucet_api.v1 import faucet_pb2 as faucet__api_dot_v1_dot_faucet__pb2

class MonetaeFaucetServiceStub(object):
    """Faucet Service for Sandbox.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Faucet = channel.unary_unary('/faucet_api.v1.MonetaeFaucetService/Faucet', request_serializer=faucet__api_dot_v1_dot_faucet__pb2.FaucetRequest.SerializeToString, response_deserializer=faucet__api_dot_v1_dot_faucet__pb2.FaucetResponse.FromString)

class MonetaeFaucetServiceServicer(object):
    """Faucet Service for Sandbox.
    """

    def Faucet(self, request, context):
        """Request promissory files to be issued to a given recipient, for a
        total amount with decimal precision and currency code.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_MonetaeFaucetServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'Faucet': grpc.unary_unary_rpc_method_handler(servicer.Faucet, request_deserializer=faucet__api_dot_v1_dot_faucet__pb2.FaucetRequest.FromString, response_serializer=faucet__api_dot_v1_dot_faucet__pb2.FaucetResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('faucet_api.v1.MonetaeFaucetService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class MonetaeFaucetService(object):
    """Faucet Service for Sandbox.
    """

    @staticmethod
    def Faucet(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/faucet_api.v1.MonetaeFaucetService/Faucet', faucet__api_dot_v1_dot_faucet__pb2.FaucetRequest.SerializeToString, faucet__api_dot_v1_dot_faucet__pb2.FaucetResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)