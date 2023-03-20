"""Client and server classes corresponding to protobuf-defined services."""
import grpc

class NotaryStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """

class NotaryServicer(object):
    """Missing associated documentation comment in .proto file."""

def add_NotaryServicer_to_server(servicer, server):
    rpc_method_handlers = {}
    generic_handler = grpc.method_handlers_generic_handler('notary.Notary', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class Notary(object):
    """Missing associated documentation comment in .proto file."""