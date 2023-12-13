"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...notary_api.v1 import notary_pb2 as notary__api_dot_v1_dot_notary__pb2

class MonetaeNotaryStub(object):
    """Notary Service for Executing Transaction Operations.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.SetAssetAuthorities = channel.unary_unary('/notary.MonetaeNotary/SetAssetAuthorities', request_serializer=notary__api_dot_v1_dot_notary__pb2.SetAssetAuthoritiesRequest.SerializeToString, response_deserializer=notary__api_dot_v1_dot_notary__pb2.SetAssetAuthoritiesResponse.FromString)

class MonetaeNotaryServicer(object):
    """Notary Service for Executing Transaction Operations.
    """

    def SetAssetAuthorities(self, request, context):
        """Set the Authorities for Assets.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_MonetaeNotaryServicer_to_server(servicer, server):
    rpc_method_handlers = {'SetAssetAuthorities': grpc.unary_unary_rpc_method_handler(servicer.SetAssetAuthorities, request_deserializer=notary__api_dot_v1_dot_notary__pb2.SetAssetAuthoritiesRequest.FromString, response_serializer=notary__api_dot_v1_dot_notary__pb2.SetAssetAuthoritiesResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('notary.MonetaeNotary', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class MonetaeNotary(object):
    """Notary Service for Executing Transaction Operations.
    """

    @staticmethod
    def SetAssetAuthorities(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/notary.MonetaeNotary/SetAssetAuthorities', notary__api_dot_v1_dot_notary__pb2.SetAssetAuthoritiesRequest.SerializeToString, notary__api_dot_v1_dot_notary__pb2.SetAssetAuthoritiesResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)