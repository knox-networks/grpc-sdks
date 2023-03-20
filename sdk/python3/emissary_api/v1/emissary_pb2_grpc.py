"""Client and server classes corresponding to protobuf-defined services."""
import grpc

class EmissaryStub(object):
    """Emissary service is empty but still needed for health probes.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """

class EmissaryServicer(object):
    """Emissary service is empty but still needed for health probes.
    """

def add_EmissaryServicer_to_server(servicer, server):
    rpc_method_handlers = {}
    generic_handler = grpc.method_handlers_generic_handler('emissary.Emissary', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class Emissary(object):
    """Emissary service is empty but still needed for health probes.
    """