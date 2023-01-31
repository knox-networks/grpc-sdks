"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2

class TransactionServiceStub(object):
    """The Transaction Service is a common service implemented by applications that play a role in payment transactions. For
    example, the Wallet application and the Treasury application both implement the Transaction service.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.NewSimplePayment = channel.unary_unary('/common.TransactionService/NewSimplePayment', request_serializer=common_dot_v1_dot_common__pb2.StartSimplePaymentRequest.SerializeToString, response_deserializer=common_dot_v1_dot_common__pb2.StartSimplePaymentResponse.FromString)
        self.NewFundsChange = channel.unary_unary('/common.TransactionService/NewFundsChange', request_serializer=common_dot_v1_dot_common__pb2.StartFundsChangeRequest.SerializeToString, response_deserializer=common_dot_v1_dot_common__pb2.StartFundsChangeResponse.FromString)

class TransactionServiceServicer(object):
    """The Transaction Service is a common service implemented by applications that play a role in payment transactions. For
    example, the Wallet application and the Treasury application both implement the Transaction service.
    """

    def NewSimplePayment(self, request, context):
        """initiates a Simple Payment transaction.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def NewFundsChange(self, request, context):
        """initiates a Funds Change transaction.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_TransactionServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'NewSimplePayment': grpc.unary_unary_rpc_method_handler(servicer.NewSimplePayment, request_deserializer=common_dot_v1_dot_common__pb2.StartSimplePaymentRequest.FromString, response_serializer=common_dot_v1_dot_common__pb2.StartSimplePaymentResponse.SerializeToString), 'NewFundsChange': grpc.unary_unary_rpc_method_handler(servicer.NewFundsChange, request_deserializer=common_dot_v1_dot_common__pb2.StartFundsChangeRequest.FromString, response_serializer=common_dot_v1_dot_common__pb2.StartFundsChangeResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('common.TransactionService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class TransactionService(object):
    """The Transaction Service is a common service implemented by applications that play a role in payment transactions. For
    example, the Wallet application and the Treasury application both implement the Transaction service.
    """

    @staticmethod
    def NewSimplePayment(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/common.TransactionService/NewSimplePayment', common_dot_v1_dot_common__pb2.StartSimplePaymentRequest.SerializeToString, common_dot_v1_dot_common__pb2.StartSimplePaymentResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def NewFundsChange(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/common.TransactionService/NewFundsChange', common_dot_v1_dot_common__pb2.StartFundsChangeRequest.SerializeToString, common_dot_v1_dot_common__pb2.StartFundsChangeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

class ServiceInfoStub(object):
    """ServiceInfo is a common service implemented by several applications so that a service endpoint can be queried to
    determine the type of service and its public key (i.e. verifier).
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.GetVerifier = channel.unary_unary('/common.ServiceInfo/GetVerifier', request_serializer=common_dot_v1_dot_common__pb2.GetVerifierRequest.SerializeToString, response_deserializer=common_dot_v1_dot_common__pb2.GetVerifierResponse.FromString)
        self.GetServiceType = channel.unary_unary('/common.ServiceInfo/GetServiceType', request_serializer=common_dot_v1_dot_common__pb2.GetServiceTypeRequest.SerializeToString, response_deserializer=common_dot_v1_dot_common__pb2.GetServiceTypeResponse.FromString)

class ServiceInfoServicer(object):
    """ServiceInfo is a common service implemented by several applications so that a service endpoint can be queried to
    determine the type of service and its public key (i.e. verifier).
    """

    def GetVerifier(self, request, context):
        """GetVerifier returns the public key associated with a service endpoint.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetServiceType(self, request, context):
        """GetServiceType returns the type of service provided by the endpoint.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_ServiceInfoServicer_to_server(servicer, server):
    rpc_method_handlers = {'GetVerifier': grpc.unary_unary_rpc_method_handler(servicer.GetVerifier, request_deserializer=common_dot_v1_dot_common__pb2.GetVerifierRequest.FromString, response_serializer=common_dot_v1_dot_common__pb2.GetVerifierResponse.SerializeToString), 'GetServiceType': grpc.unary_unary_rpc_method_handler(servicer.GetServiceType, request_deserializer=common_dot_v1_dot_common__pb2.GetServiceTypeRequest.FromString, response_serializer=common_dot_v1_dot_common__pb2.GetServiceTypeResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('common.ServiceInfo', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class ServiceInfo(object):
    """ServiceInfo is a common service implemented by several applications so that a service endpoint can be queried to
    determine the type of service and its public key (i.e. verifier).
    """

    @staticmethod
    def GetVerifier(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/common.ServiceInfo/GetVerifier', common_dot_v1_dot_common__pb2.GetVerifierRequest.SerializeToString, common_dot_v1_dot_common__pb2.GetVerifierResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetServiceType(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/common.ServiceInfo/GetServiceType', common_dot_v1_dot_common__pb2.GetServiceTypeRequest.SerializeToString, common_dot_v1_dot_common__pb2.GetServiceTypeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)