"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...common.v1 import packet_pb2 as common_dot_v1_dot_packet__pb2

class EmissaryStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Connection = channel.stream_stream('/emissary.Emissary/Connection', request_serializer=common_dot_v1_dot_packet__pb2.Packet.SerializeToString, response_deserializer=common_dot_v1_dot_packet__pb2.Packet.FromString)

class EmissaryServicer(object):
    """Missing associated documentation comment in .proto file."""

    def Connection(self, request_iterator, context):
        """Establish an emissary connection to the gateway.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_EmissaryServicer_to_server(servicer, server):
    rpc_method_handlers = {'Connection': grpc.stream_stream_rpc_method_handler(servicer.Connection, request_deserializer=common_dot_v1_dot_packet__pb2.Packet.FromString, response_serializer=common_dot_v1_dot_packet__pb2.Packet.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('emissary.Emissary', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class Emissary(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def Connection(request_iterator, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.stream_stream(request_iterator, target, '/emissary.Emissary/Connection', common_dot_v1_dot_packet__pb2.Packet.SerializeToString, common_dot_v1_dot_packet__pb2.Packet.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)