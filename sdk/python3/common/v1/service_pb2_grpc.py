"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
from ...common.v1 import service_pb2 as common_dot_v1_dot_service__pb2

class ServiceInfoStub(object):
    """THIS PROTO FILE IS INTENDED FOR INTERNAL KNOX USE ONLY.

    ServiceInfo is a common service implemented by several applications so that a service endpoint can be queried to
    determine the type of service and its public key (i.e. verifier).
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.GetVerifier = channel.unary_unary('/common.ServiceInfo/GetVerifier', request_serializer=common_dot_v1_dot_common__pb2.GetVerifierRequest.SerializeToString, response_deserializer=common_dot_v1_dot_common__pb2.GetVerifierResponse.FromString)
        self.GetServiceType = channel.unary_unary('/common.ServiceInfo/GetServiceType', request_serializer=common_dot_v1_dot_common__pb2.GetServiceTypeRequest.SerializeToString, response_deserializer=common_dot_v1_dot_common__pb2.GetServiceTypeResponse.FromString)
        self.GetInstance = channel.unary_unary('/common.ServiceInfo/GetInstance', request_serializer=common_dot_v1_dot_common__pb2.GetInstanceRequest.SerializeToString, response_deserializer=common_dot_v1_dot_common__pb2.GetInstanceResponse.FromString)

class ServiceInfoServicer(object):
    """THIS PROTO FILE IS INTENDED FOR INTERNAL KNOX USE ONLY.

    ServiceInfo is a common service implemented by several applications so that a service endpoint can be queried to
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

    def GetInstance(self, request, context):
        """GetInstance returns an instance UUID that is distinct per startup
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_ServiceInfoServicer_to_server(servicer, server):
    rpc_method_handlers = {'GetVerifier': grpc.unary_unary_rpc_method_handler(servicer.GetVerifier, request_deserializer=common_dot_v1_dot_common__pb2.GetVerifierRequest.FromString, response_serializer=common_dot_v1_dot_common__pb2.GetVerifierResponse.SerializeToString), 'GetServiceType': grpc.unary_unary_rpc_method_handler(servicer.GetServiceType, request_deserializer=common_dot_v1_dot_common__pb2.GetServiceTypeRequest.FromString, response_serializer=common_dot_v1_dot_common__pb2.GetServiceTypeResponse.SerializeToString), 'GetInstance': grpc.unary_unary_rpc_method_handler(servicer.GetInstance, request_deserializer=common_dot_v1_dot_common__pb2.GetInstanceRequest.FromString, response_serializer=common_dot_v1_dot_common__pb2.GetInstanceResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('common.ServiceInfo', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class ServiceInfo(object):
    """THIS PROTO FILE IS INTENDED FOR INTERNAL KNOX USE ONLY.

    ServiceInfo is a common service implemented by several applications so that a service endpoint can be queried to
    determine the type of service and its public key (i.e. verifier).
    """

    @staticmethod
    def GetVerifier(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/common.ServiceInfo/GetVerifier', common_dot_v1_dot_common__pb2.GetVerifierRequest.SerializeToString, common_dot_v1_dot_common__pb2.GetVerifierResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetServiceType(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/common.ServiceInfo/GetServiceType', common_dot_v1_dot_common__pb2.GetServiceTypeRequest.SerializeToString, common_dot_v1_dot_common__pb2.GetServiceTypeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetInstance(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/common.ServiceInfo/GetInstance', common_dot_v1_dot_common__pb2.GetInstanceRequest.SerializeToString, common_dot_v1_dot_common__pb2.GetInstanceResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

class StatusServiceStub(object):
    """Check the Status of a Service.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.IsProvisioned = channel.unary_unary('/common.StatusService/IsProvisioned', request_serializer=common_dot_v1_dot_service__pb2.IsProvisionedRequest.SerializeToString, response_deserializer=common_dot_v1_dot_service__pb2.IsProvisionedResponse.FromString)

class StatusServiceServicer(object):
    """Check the Status of a Service.
    """

    def IsProvisioned(self, request, context):
        """Checks Whether Service is Provisioned.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_StatusServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'IsProvisioned': grpc.unary_unary_rpc_method_handler(servicer.IsProvisioned, request_deserializer=common_dot_v1_dot_service__pb2.IsProvisionedRequest.FromString, response_serializer=common_dot_v1_dot_service__pb2.IsProvisionedResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('common.StatusService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class StatusService(object):
    """Check the Status of a Service.
    """

    @staticmethod
    def IsProvisioned(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/common.StatusService/IsProvisioned', common_dot_v1_dot_service__pb2.IsProvisionedRequest.SerializeToString, common_dot_v1_dot_service__pb2.IsProvisionedResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)