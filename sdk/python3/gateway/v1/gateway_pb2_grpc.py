"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
from ...common.v1 import packet_pb2 as common_dot_v1_dot_packet__pb2
from ...gateway.v1 import gateway_pb2 as gateway_dot_v1_dot_gateway__pb2

class MonetaeGatewayStub(object):
    """The Gateway service allows fast switching of streaming messages between senders and recipients identified by their
    public key, over authenticated connections. It exposes the APIs needed to connect, authenticate and send data.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Authenticate = channel.unary_unary('/gateway.MonetaeGateway/Authenticate', request_serializer=common_dot_v1_dot_common__pb2.AuthenticationRequest.SerializeToString, response_deserializer=common_dot_v1_dot_common__pb2.AuthenticationResponse.FromString)
        self.Data = channel.stream_stream('/gateway.MonetaeGateway/Data', request_serializer=common_dot_v1_dot_packet__pb2.Packet.SerializeToString, response_deserializer=common_dot_v1_dot_packet__pb2.Packet.FromString)
        self.GetConnections = channel.unary_unary('/gateway.MonetaeGateway/GetConnections', request_serializer=gateway_dot_v1_dot_gateway__pb2.ConnectionsRequest.SerializeToString, response_deserializer=gateway_dot_v1_dot_gateway__pb2.ConnectionsResponse.FromString)

class MonetaeGatewayServicer(object):
    """The Gateway service allows fast switching of streaming messages between senders and recipients identified by their
    public key, over authenticated connections. It exposes the APIs needed to connect, authenticate and send data.
    """

    def Authenticate(self, request, context):
        """Start a new authenticated session and get a challenge nonce.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Data(self, request_iterator, context):
        """Authenticates the connection and opens a bidirectional stream.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetConnections(self, request, context):
        """Get a list of connections to the Gateway, including the active connections and connections that have been removed.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_MonetaeGatewayServicer_to_server(servicer, server):
    rpc_method_handlers = {'Authenticate': grpc.unary_unary_rpc_method_handler(servicer.Authenticate, request_deserializer=common_dot_v1_dot_common__pb2.AuthenticationRequest.FromString, response_serializer=common_dot_v1_dot_common__pb2.AuthenticationResponse.SerializeToString), 'Data': grpc.stream_stream_rpc_method_handler(servicer.Data, request_deserializer=common_dot_v1_dot_packet__pb2.Packet.FromString, response_serializer=common_dot_v1_dot_packet__pb2.Packet.SerializeToString), 'GetConnections': grpc.unary_unary_rpc_method_handler(servicer.GetConnections, request_deserializer=gateway_dot_v1_dot_gateway__pb2.ConnectionsRequest.FromString, response_serializer=gateway_dot_v1_dot_gateway__pb2.ConnectionsResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('gateway.MonetaeGateway', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class MonetaeGateway(object):
    """The Gateway service allows fast switching of streaming messages between senders and recipients identified by their
    public key, over authenticated connections. It exposes the APIs needed to connect, authenticate and send data.
    """

    @staticmethod
    def Authenticate(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/gateway.MonetaeGateway/Authenticate', common_dot_v1_dot_common__pb2.AuthenticationRequest.SerializeToString, common_dot_v1_dot_common__pb2.AuthenticationResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Data(request_iterator, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.stream_stream(request_iterator, target, '/gateway.MonetaeGateway/Data', common_dot_v1_dot_packet__pb2.Packet.SerializeToString, common_dot_v1_dot_packet__pb2.Packet.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetConnections(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/gateway.MonetaeGateway/GetConnections', gateway_dot_v1_dot_gateway__pb2.ConnectionsRequest.SerializeToString, gateway_dot_v1_dot_gateway__pb2.ConnectionsResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)