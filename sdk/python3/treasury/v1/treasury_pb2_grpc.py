"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...treasury.v1 import treasury_pb2 as treasury_dot_v1_dot_treasury__pb2

class MonetaeTreasuryStub(object):
    """The Treasury application implements value added functions that may be offered by the financial institution. APIs
    defined here are limited to those that are exclusively exposed via the Treasury service, e.g. APIs to manage currency
    supply of the treasury and APIs for remittance. Where functions are offered as part of a Transaction service, the
    APIs can be found under the Transaction Service.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.GetSupply = channel.unary_unary('/treasury.MonetaeTreasury/GetSupply', request_serializer=treasury_dot_v1_dot_treasury__pb2.GetSupplyRequest.SerializeToString, response_deserializer=treasury_dot_v1_dot_treasury__pb2.GetSupplyResponse.FromString)

class MonetaeTreasuryServicer(object):
    """The Treasury application implements value added functions that may be offered by the financial institution. APIs
    defined here are limited to those that are exclusively exposed via the Treasury service, e.g. APIs to manage currency
    supply of the treasury and APIs for remittance. Where functions are offered as part of a Transaction service, the
    APIs can be found under the Transaction Service.
    """

    def GetSupply(self, request, context):
        """Get a list of denomination counts of the digital banknotes held in the Treasury service's vault.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_MonetaeTreasuryServicer_to_server(servicer, server):
    rpc_method_handlers = {'GetSupply': grpc.unary_unary_rpc_method_handler(servicer.GetSupply, request_deserializer=treasury_dot_v1_dot_treasury__pb2.GetSupplyRequest.FromString, response_serializer=treasury_dot_v1_dot_treasury__pb2.GetSupplyResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('treasury.MonetaeTreasury', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class MonetaeTreasury(object):
    """The Treasury application implements value added functions that may be offered by the financial institution. APIs
    defined here are limited to those that are exclusively exposed via the Treasury service, e.g. APIs to manage currency
    supply of the treasury and APIs for remittance. Where functions are offered as part of a Transaction service, the
    APIs can be found under the Transaction Service.
    """

    @staticmethod
    def GetSupply(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/treasury.MonetaeTreasury/GetSupply', treasury_dot_v1_dot_treasury__pb2.GetSupplyRequest.SerializeToString, treasury_dot_v1_dot_treasury__pb2.GetSupplyResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)