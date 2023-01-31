"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...treasury.v1 import treasury_pb2 as treasury_dot_v1_dot_treasury__pb2

class TreasuryStub(object):
    """The Treasury application implements value added functions that may be offered by the financial institution. APIs defined here are limited to those that are exclusively exposed via the Treasury Service, e.g. APIs to manage currency supply of the treasury and APIs for Remitatnce. Where functions are offered as part of a Transaction service, the APIs can be found under the Transactoin Service.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Remit = channel.stream_stream('/treasury.Treasury/Remit', request_serializer=treasury_dot_v1_dot_treasury__pb2.RemittanceRequest.SerializeToString, response_deserializer=treasury_dot_v1_dot_treasury__pb2.RemittanceResponse.FromString)
        self.GetSupply = channel.unary_unary('/treasury.Treasury/GetSupply', request_serializer=treasury_dot_v1_dot_treasury__pb2.GetSupplyRequest.SerializeToString, response_deserializer=treasury_dot_v1_dot_treasury__pb2.GetSupplyResponse.FromString)

class TreasuryServicer(object):
    """The Treasury application implements value added functions that may be offered by the financial institution. APIs defined here are limited to those that are exclusively exposed via the Treasury Service, e.g. APIs to manage currency supply of the treasury and APIs for Remitatnce. Where functions are offered as part of a Transaction service, the APIs can be found under the Transactoin Service.
    """

    def Remit(self, request_iterator, context):
        """Start a new remittance, passing in a streamed request and getting back a streamed response.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetSupply(self, request, context):
        """Get a list of denomination counts of the promissories held in the treasury vault.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_TreasuryServicer_to_server(servicer, server):
    rpc_method_handlers = {'Remit': grpc.stream_stream_rpc_method_handler(servicer.Remit, request_deserializer=treasury_dot_v1_dot_treasury__pb2.RemittanceRequest.FromString, response_serializer=treasury_dot_v1_dot_treasury__pb2.RemittanceResponse.SerializeToString), 'GetSupply': grpc.unary_unary_rpc_method_handler(servicer.GetSupply, request_deserializer=treasury_dot_v1_dot_treasury__pb2.GetSupplyRequest.FromString, response_serializer=treasury_dot_v1_dot_treasury__pb2.GetSupplyResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('treasury.Treasury', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class Treasury(object):
    """The Treasury application implements value added functions that may be offered by the financial institution. APIs defined here are limited to those that are exclusively exposed via the Treasury Service, e.g. APIs to manage currency supply of the treasury and APIs for Remitatnce. Where functions are offered as part of a Transaction service, the APIs can be found under the Transactoin Service.
    """

    @staticmethod
    def Remit(request_iterator, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.stream_stream(request_iterator, target, '/treasury.Treasury/Remit', treasury_dot_v1_dot_treasury__pb2.RemittanceRequest.SerializeToString, treasury_dot_v1_dot_treasury__pb2.RemittanceResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetSupply(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/treasury.Treasury/GetSupply', treasury_dot_v1_dot_treasury__pb2.GetSupplyRequest.SerializeToString, treasury_dot_v1_dot_treasury__pb2.GetSupplyResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)