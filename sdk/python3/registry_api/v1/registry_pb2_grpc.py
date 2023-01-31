"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...registry_api.v1 import registry_pb2 as registry__api_dot_v1_dot_registry__pb2

class RegistryStub(object):
    """Publicly accessible repo of DID Documents for cryptographic verification of DIDs. Contains no sensitive data and default implementation of Knox DID method is operated by Knox in AWS. https://www.w3.org/TR/did-core/
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Create = channel.unary_unary('/registry_api.v1.Registry/Create', request_serializer=registry__api_dot_v1_dot_registry__pb2.CreateRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.CreateResponse.FromString)
        self.Read = channel.unary_unary('/registry_api.v1.Registry/Read', request_serializer=registry__api_dot_v1_dot_registry__pb2.ReadRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.ReadResponse.FromString)
        self.Update = channel.unary_unary('/registry_api.v1.Registry/Update', request_serializer=registry__api_dot_v1_dot_registry__pb2.UpdateRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.UpdateResponse.FromString)
        self.Revoke = channel.unary_unary('/registry_api.v1.Registry/Revoke', request_serializer=registry__api_dot_v1_dot_registry__pb2.RevokeRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.RevokeResponse.FromString)

class RegistryServicer(object):
    """Publicly accessible repo of DID Documents for cryptographic verification of DIDs. Contains no sensitive data and default implementation of Knox DID method is operated by Knox in AWS. https://www.w3.org/TR/did-core/
    """

    def Create(self, request, context):
        """Creates a new DID document entry for a given DID in the Knox Registry Management Service. 
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Read(self, request, context):
        """Retrieves a DID document entry for a given DID in the Knox Registry Management Service.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Update(self, request, context):
        """Updates a DID document entry for a given DID in the Knox Registry Management Service.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Revoke(self, request, context):
        """Revokes a DID document entry for a given DID in the Knox Registry Management Service.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_RegistryServicer_to_server(servicer, server):
    rpc_method_handlers = {'Create': grpc.unary_unary_rpc_method_handler(servicer.Create, request_deserializer=registry__api_dot_v1_dot_registry__pb2.CreateRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.CreateResponse.SerializeToString), 'Read': grpc.unary_unary_rpc_method_handler(servicer.Read, request_deserializer=registry__api_dot_v1_dot_registry__pb2.ReadRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.ReadResponse.SerializeToString), 'Update': grpc.unary_unary_rpc_method_handler(servicer.Update, request_deserializer=registry__api_dot_v1_dot_registry__pb2.UpdateRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.UpdateResponse.SerializeToString), 'Revoke': grpc.unary_unary_rpc_method_handler(servicer.Revoke, request_deserializer=registry__api_dot_v1_dot_registry__pb2.RevokeRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.RevokeResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('registry_api.v1.Registry', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class Registry(object):
    """Publicly accessible repo of DID Documents for cryptographic verification of DIDs. Contains no sensitive data and default implementation of Knox DID method is operated by Knox in AWS. https://www.w3.org/TR/did-core/
    """

    @staticmethod
    def Create(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.Registry/Create', registry__api_dot_v1_dot_registry__pb2.CreateRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.CreateResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Read(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.Registry/Read', registry__api_dot_v1_dot_registry__pb2.ReadRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.ReadResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Update(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.Registry/Update', registry__api_dot_v1_dot_registry__pb2.UpdateRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.UpdateResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Revoke(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.Registry/Revoke', registry__api_dot_v1_dot_registry__pb2.RevokeRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.RevokeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)