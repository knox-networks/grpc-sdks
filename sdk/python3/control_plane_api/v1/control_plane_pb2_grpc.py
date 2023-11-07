"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...control_plane_api.v1 import control_plane_pb2 as control__plane__api_dot_v1_dot_control__plane__pb2

class ControlPlaneAPIServiceStub(object):
    """Control Plane Service Manages the Deployment of other Services.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.CreateService = channel.unary_unary('/control_plane_api.v1.ControlPlaneAPIService/CreateService', request_serializer=control__plane__api_dot_v1_dot_control__plane__pb2.CreateServiceRequest.SerializeToString, response_deserializer=control__plane__api_dot_v1_dot_control__plane__pb2.CreateServiceResponse.FromString)
        self.GetService = channel.unary_unary('/control_plane_api.v1.ControlPlaneAPIService/GetService', request_serializer=control__plane__api_dot_v1_dot_control__plane__pb2.GetServiceRequest.SerializeToString, response_deserializer=control__plane__api_dot_v1_dot_control__plane__pb2.GetServiceResponse.FromString)
        self.GetServices = channel.unary_unary('/control_plane_api.v1.ControlPlaneAPIService/GetServices', request_serializer=control__plane__api_dot_v1_dot_control__plane__pb2.GetServicesRequest.SerializeToString, response_deserializer=control__plane__api_dot_v1_dot_control__plane__pb2.GetServicesResponse.FromString)
        self.DeleteService = channel.unary_unary('/control_plane_api.v1.ControlPlaneAPIService/DeleteService', request_serializer=control__plane__api_dot_v1_dot_control__plane__pb2.DeleteServiceRequest.SerializeToString, response_deserializer=control__plane__api_dot_v1_dot_control__plane__pb2.DeleteServiceResponse.FromString)
        self.UpdateService = channel.unary_unary('/control_plane_api.v1.ControlPlaneAPIService/UpdateService', request_serializer=control__plane__api_dot_v1_dot_control__plane__pb2.UpdateServiceRequest.SerializeToString, response_deserializer=control__plane__api_dot_v1_dot_control__plane__pb2.UpdateServiceResponse.FromString)

class ControlPlaneAPIServiceServicer(object):
    """Control Plane Service Manages the Deployment of other Services.
    """

    def CreateService(self, request, context):
        """Create Service.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetService(self, request, context):
        """Get Service.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetServices(self, request, context):
        """Get List of Services.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def DeleteService(self, request, context):
        """Delete Service.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def UpdateService(self, request, context):
        """Allow the service to be upgrades, e.g. increase/decrease replicas
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_ControlPlaneAPIServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'CreateService': grpc.unary_unary_rpc_method_handler(servicer.CreateService, request_deserializer=control__plane__api_dot_v1_dot_control__plane__pb2.CreateServiceRequest.FromString, response_serializer=control__plane__api_dot_v1_dot_control__plane__pb2.CreateServiceResponse.SerializeToString), 'GetService': grpc.unary_unary_rpc_method_handler(servicer.GetService, request_deserializer=control__plane__api_dot_v1_dot_control__plane__pb2.GetServiceRequest.FromString, response_serializer=control__plane__api_dot_v1_dot_control__plane__pb2.GetServiceResponse.SerializeToString), 'GetServices': grpc.unary_unary_rpc_method_handler(servicer.GetServices, request_deserializer=control__plane__api_dot_v1_dot_control__plane__pb2.GetServicesRequest.FromString, response_serializer=control__plane__api_dot_v1_dot_control__plane__pb2.GetServicesResponse.SerializeToString), 'DeleteService': grpc.unary_unary_rpc_method_handler(servicer.DeleteService, request_deserializer=control__plane__api_dot_v1_dot_control__plane__pb2.DeleteServiceRequest.FromString, response_serializer=control__plane__api_dot_v1_dot_control__plane__pb2.DeleteServiceResponse.SerializeToString), 'UpdateService': grpc.unary_unary_rpc_method_handler(servicer.UpdateService, request_deserializer=control__plane__api_dot_v1_dot_control__plane__pb2.UpdateServiceRequest.FromString, response_serializer=control__plane__api_dot_v1_dot_control__plane__pb2.UpdateServiceResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('control_plane_api.v1.ControlPlaneAPIService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class ControlPlaneAPIService(object):
    """Control Plane Service Manages the Deployment of other Services.
    """

    @staticmethod
    def CreateService(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/control_plane_api.v1.ControlPlaneAPIService/CreateService', control__plane__api_dot_v1_dot_control__plane__pb2.CreateServiceRequest.SerializeToString, control__plane__api_dot_v1_dot_control__plane__pb2.CreateServiceResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetService(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/control_plane_api.v1.ControlPlaneAPIService/GetService', control__plane__api_dot_v1_dot_control__plane__pb2.GetServiceRequest.SerializeToString, control__plane__api_dot_v1_dot_control__plane__pb2.GetServiceResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetServices(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/control_plane_api.v1.ControlPlaneAPIService/GetServices', control__plane__api_dot_v1_dot_control__plane__pb2.GetServicesRequest.SerializeToString, control__plane__api_dot_v1_dot_control__plane__pb2.GetServicesResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def DeleteService(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/control_plane_api.v1.ControlPlaneAPIService/DeleteService', control__plane__api_dot_v1_dot_control__plane__pb2.DeleteServiceRequest.SerializeToString, control__plane__api_dot_v1_dot_control__plane__pb2.DeleteServiceResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def UpdateService(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/control_plane_api.v1.ControlPlaneAPIService/UpdateService', control__plane__api_dot_v1_dot_control__plane__pb2.UpdateServiceRequest.SerializeToString, control__plane__api_dot_v1_dot_control__plane__pb2.UpdateServiceResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)