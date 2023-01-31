"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2

class NotaryStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Authorize = channel.stream_stream('/notary.Notary/Authorize', request_serializer=common_dot_v1_dot_common__pb2.AuthorizedSignatureRequest.SerializeToString, response_deserializer=common_dot_v1_dot_common__pb2.AuthorizedSignatureResponse.FromString)

class NotaryServicer(object):
    """Missing associated documentation comment in .proto file."""

    def Authorize(self, request_iterator, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_NotaryServicer_to_server(servicer, server):
    rpc_method_handlers = {'Authorize': grpc.stream_stream_rpc_method_handler(servicer.Authorize, request_deserializer=common_dot_v1_dot_common__pb2.AuthorizedSignatureRequest.FromString, response_serializer=common_dot_v1_dot_common__pb2.AuthorizedSignatureResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('notary.Notary', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class Notary(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def Authorize(request_iterator, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.stream_stream(request_iterator, target, '/notary.Notary/Authorize', common_dot_v1_dot_common__pb2.AuthorizedSignatureRequest.SerializeToString, common_dot_v1_dot_common__pb2.AuthorizedSignatureResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)