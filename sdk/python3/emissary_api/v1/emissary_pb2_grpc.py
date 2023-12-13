"""Client and server classes corresponding to protobuf-defined services."""
import grpc

class MonetaeEmissaryStub(object):
    """THIS PROTO FILE IS INTENDED FOR INTERNAL KNOX USE ONLY.

    MonetaeEmissary service is empty but still needed for health probes.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """

class MonetaeEmissaryServicer(object):
    """THIS PROTO FILE IS INTENDED FOR INTERNAL KNOX USE ONLY.

    MonetaeEmissary service is empty but still needed for health probes.
    """

def add_MonetaeEmissaryServicer_to_server(servicer, server):
    rpc_method_handlers = {}
    generic_handler = grpc.method_handlers_generic_handler('emissary.MonetaeEmissary', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class MonetaeEmissary(object):
    """THIS PROTO FILE IS INTENDED FOR INTERNAL KNOX USE ONLY.

    MonetaeEmissary service is empty but still needed for health probes.
    """