"""Client and server classes corresponding to protobuf-defined services."""
import grpc

class TransactionManagerStub(object):
    """THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

    Transaction Manager Service.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """

class TransactionManagerServicer(object):
    """THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

    Transaction Manager Service.
    """

def add_TransactionManagerServicer_to_server(servicer, server):
    rpc_method_handlers = {}
    generic_handler = grpc.method_handlers_generic_handler('transaction_manager.TransactionManager', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class TransactionManager(object):
    """THIS PROTO FILE IS INTENDED FOR USE FOR KNOX AND KNOX CUSTOMERS.

    Transaction Manager Service.
    """