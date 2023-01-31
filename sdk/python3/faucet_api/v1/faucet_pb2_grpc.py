"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...faucet_api.v1 import faucet_pb2 as faucet__api_dot_v1_dot_faucet__pb2

class FaucetStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Faucet = channel.unary_unary('/faucet_api.v1.Faucet/Faucet', request_serializer=faucet__api_dot_v1_dot_faucet__pb2.FaucetRequest.SerializeToString, response_deserializer=faucet__api_dot_v1_dot_faucet__pb2.FaucetResponse.FromString)

class FaucetServicer(object):
    """Missing associated documentation comment in .proto file."""

    def Faucet(self, request, context):
        """Request promissory files to be issued to a given recipient, for a
        total amount with decimal precision and currency code.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_FaucetServicer_to_server(servicer, server):
    rpc_method_handlers = {'Faucet': grpc.unary_unary_rpc_method_handler(servicer.Faucet, request_deserializer=faucet__api_dot_v1_dot_faucet__pb2.FaucetRequest.FromString, response_serializer=faucet__api_dot_v1_dot_faucet__pb2.FaucetResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('faucet_api.v1.Faucet', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class Faucet(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def Faucet(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/faucet_api.v1.Faucet/Faucet', faucet__api_dot_v1_dot_faucet__pb2.FaucetRequest.SerializeToString, faucet__api_dot_v1_dot_faucet__pb2.FaucetResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)