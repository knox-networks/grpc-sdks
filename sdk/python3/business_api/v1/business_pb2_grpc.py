"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...business_api.v1 import business_pb2 as business__api_dot_v1_dot_business__pb2

class BusinessServiceStub(object):
    """Business Service.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.List = channel.unary_unary('/business_api.v1.BusinessService/List', request_serializer=business__api_dot_v1_dot_business__pb2.ListRequest.SerializeToString, response_deserializer=business__api_dot_v1_dot_business__pb2.ListResponse.FromString)
        self.Clear = channel.unary_unary('/business_api.v1.BusinessService/Clear', request_serializer=business__api_dot_v1_dot_business__pb2.ClearRequest.SerializeToString, response_deserializer=business__api_dot_v1_dot_business__pb2.ClearResponse.FromString)
        self.SendPayload = channel.unary_unary('/business_api.v1.BusinessService/SendPayload', request_serializer=business__api_dot_v1_dot_business__pb2.SendPayloadRequest.SerializeToString, response_deserializer=business__api_dot_v1_dot_business__pb2.SendPayloadResponse.FromString)
        self.SendNewTransaction = channel.unary_unary('/business_api.v1.BusinessService/SendNewTransaction', request_serializer=business__api_dot_v1_dot_business__pb2.SendNewTransactionRequest.SerializeToString, response_deserializer=business__api_dot_v1_dot_business__pb2.SendNewTransactionResponse.FromString)
        self.SendProposalAcked = channel.unary_unary('/business_api.v1.BusinessService/SendProposalAcked', request_serializer=business__api_dot_v1_dot_business__pb2.SendProposalAckedRequest.SerializeToString, response_deserializer=business__api_dot_v1_dot_business__pb2.SendProposalAckedResponse.FromString)
        self.SendProposalNacked = channel.unary_unary('/business_api.v1.BusinessService/SendProposalNacked', request_serializer=business__api_dot_v1_dot_business__pb2.SendProposalNackedRequest.SerializeToString, response_deserializer=business__api_dot_v1_dot_business__pb2.SendProposalNackedResponse.FromString)
        self.SendSecretRevealed = channel.unary_unary('/business_api.v1.BusinessService/SendSecretRevealed', request_serializer=business__api_dot_v1_dot_business__pb2.SendSecretRevealedRequest.SerializeToString, response_deserializer=business__api_dot_v1_dot_business__pb2.SendSecretRevealedResponse.FromString)

class BusinessServiceServicer(object):
    """Business Service.
    """

    def List(self, request, context):
        """List.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Clear(self, request, context):
        """Clear Request.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def SendPayload(self, request, context):
        """Send Payload.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def SendNewTransaction(self, request, context):
        """Send New Transaction.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def SendProposalAcked(self, request, context):
        """Send Proposal Acked.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def SendProposalNacked(self, request, context):
        """Send Proposal Nacked.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def SendSecretRevealed(self, request, context):
        """Send Secret Revealed.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_BusinessServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'List': grpc.unary_unary_rpc_method_handler(servicer.List, request_deserializer=business__api_dot_v1_dot_business__pb2.ListRequest.FromString, response_serializer=business__api_dot_v1_dot_business__pb2.ListResponse.SerializeToString), 'Clear': grpc.unary_unary_rpc_method_handler(servicer.Clear, request_deserializer=business__api_dot_v1_dot_business__pb2.ClearRequest.FromString, response_serializer=business__api_dot_v1_dot_business__pb2.ClearResponse.SerializeToString), 'SendPayload': grpc.unary_unary_rpc_method_handler(servicer.SendPayload, request_deserializer=business__api_dot_v1_dot_business__pb2.SendPayloadRequest.FromString, response_serializer=business__api_dot_v1_dot_business__pb2.SendPayloadResponse.SerializeToString), 'SendNewTransaction': grpc.unary_unary_rpc_method_handler(servicer.SendNewTransaction, request_deserializer=business__api_dot_v1_dot_business__pb2.SendNewTransactionRequest.FromString, response_serializer=business__api_dot_v1_dot_business__pb2.SendNewTransactionResponse.SerializeToString), 'SendProposalAcked': grpc.unary_unary_rpc_method_handler(servicer.SendProposalAcked, request_deserializer=business__api_dot_v1_dot_business__pb2.SendProposalAckedRequest.FromString, response_serializer=business__api_dot_v1_dot_business__pb2.SendProposalAckedResponse.SerializeToString), 'SendProposalNacked': grpc.unary_unary_rpc_method_handler(servicer.SendProposalNacked, request_deserializer=business__api_dot_v1_dot_business__pb2.SendProposalNackedRequest.FromString, response_serializer=business__api_dot_v1_dot_business__pb2.SendProposalNackedResponse.SerializeToString), 'SendSecretRevealed': grpc.unary_unary_rpc_method_handler(servicer.SendSecretRevealed, request_deserializer=business__api_dot_v1_dot_business__pb2.SendSecretRevealedRequest.FromString, response_serializer=business__api_dot_v1_dot_business__pb2.SendSecretRevealedResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('business_api.v1.BusinessService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class BusinessService(object):
    """Business Service.
    """

    @staticmethod
    def List(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/business_api.v1.BusinessService/List', business__api_dot_v1_dot_business__pb2.ListRequest.SerializeToString, business__api_dot_v1_dot_business__pb2.ListResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Clear(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/business_api.v1.BusinessService/Clear', business__api_dot_v1_dot_business__pb2.ClearRequest.SerializeToString, business__api_dot_v1_dot_business__pb2.ClearResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def SendPayload(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/business_api.v1.BusinessService/SendPayload', business__api_dot_v1_dot_business__pb2.SendPayloadRequest.SerializeToString, business__api_dot_v1_dot_business__pb2.SendPayloadResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def SendNewTransaction(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/business_api.v1.BusinessService/SendNewTransaction', business__api_dot_v1_dot_business__pb2.SendNewTransactionRequest.SerializeToString, business__api_dot_v1_dot_business__pb2.SendNewTransactionResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def SendProposalAcked(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/business_api.v1.BusinessService/SendProposalAcked', business__api_dot_v1_dot_business__pb2.SendProposalAckedRequest.SerializeToString, business__api_dot_v1_dot_business__pb2.SendProposalAckedResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def SendProposalNacked(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/business_api.v1.BusinessService/SendProposalNacked', business__api_dot_v1_dot_business__pb2.SendProposalNackedRequest.SerializeToString, business__api_dot_v1_dot_business__pb2.SendProposalNackedResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def SendSecretRevealed(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/business_api.v1.BusinessService/SendSecretRevealed', business__api_dot_v1_dot_business__pb2.SendSecretRevealedRequest.SerializeToString, business__api_dot_v1_dot_business__pb2.SendSecretRevealedResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)