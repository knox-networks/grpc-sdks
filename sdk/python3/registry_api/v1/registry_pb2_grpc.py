"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...registry_api.v1 import registry_pb2 as registry__api_dot_v1_dot_registry__pb2

class RegistryServiceStub(object):
    """RegistryService is a publicly accessible repo of DID Documents for cryptographic verification of DIDs. Contains no sensitive data and default implementation of Knox DID method is operated by Knox in AWS. https:// Www.w3.org/TR/did-core/
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Create = channel.unary_unary('/registry_api.v1.RegistryService/Create', request_serializer=registry__api_dot_v1_dot_registry__pb2.CreateRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.CreateResponse.FromString)
        self.Resolve = channel.unary_unary('/registry_api.v1.RegistryService/Resolve', request_serializer=registry__api_dot_v1_dot_registry__pb2.ResolveRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.ResolveResponse.FromString)
        self.ResolveRepresentation = channel.unary_unary('/registry_api.v1.RegistryService/ResolveRepresentation', request_serializer=registry__api_dot_v1_dot_registry__pb2.ResolveRepresentationRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.ResolveRepresentationResponse.FromString)
        self.Update = channel.unary_unary('/registry_api.v1.RegistryService/Update', request_serializer=registry__api_dot_v1_dot_registry__pb2.UpdateRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.UpdateResponse.FromString)
        self.Revoke = channel.unary_unary('/registry_api.v1.RegistryService/Revoke', request_serializer=registry__api_dot_v1_dot_registry__pb2.RevokeRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.RevokeResponse.FromString)

class RegistryServiceServicer(object):
    """RegistryService is a publicly accessible repo of DID Documents for cryptographic verification of DIDs. Contains no sensitive data and default implementation of Knox DID method is operated by Knox in AWS. https:// Www.w3.org/TR/did-core/
    """

    def Create(self, request, context):
        """Creates a new DID document entry for a given DID in the Knox Registry Management Service. 
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Resolve(self, request, context):
        """Returns a DID document entry for a given DID in the Knox Registry Management Service.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ResolveRepresentation(self, request, context):
        """Returns a byte stream of a DID document instead of a map in the Knox Registry Management Service.
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

def add_RegistryServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'Create': grpc.unary_unary_rpc_method_handler(servicer.Create, request_deserializer=registry__api_dot_v1_dot_registry__pb2.CreateRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.CreateResponse.SerializeToString), 'Resolve': grpc.unary_unary_rpc_method_handler(servicer.Resolve, request_deserializer=registry__api_dot_v1_dot_registry__pb2.ResolveRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.ResolveResponse.SerializeToString), 'ResolveRepresentation': grpc.unary_unary_rpc_method_handler(servicer.ResolveRepresentation, request_deserializer=registry__api_dot_v1_dot_registry__pb2.ResolveRepresentationRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.ResolveRepresentationResponse.SerializeToString), 'Update': grpc.unary_unary_rpc_method_handler(servicer.Update, request_deserializer=registry__api_dot_v1_dot_registry__pb2.UpdateRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.UpdateResponse.SerializeToString), 'Revoke': grpc.unary_unary_rpc_method_handler(servicer.Revoke, request_deserializer=registry__api_dot_v1_dot_registry__pb2.RevokeRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.RevokeResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('registry_api.v1.RegistryService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class RegistryService(object):
    """RegistryService is a publicly accessible repo of DID Documents for cryptographic verification of DIDs. Contains no sensitive data and default implementation of Knox DID method is operated by Knox in AWS. https:// Www.w3.org/TR/did-core/
    """

    @staticmethod
    def Create(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.RegistryService/Create', registry__api_dot_v1_dot_registry__pb2.CreateRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.CreateResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Resolve(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.RegistryService/Resolve', registry__api_dot_v1_dot_registry__pb2.ResolveRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.ResolveResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ResolveRepresentation(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.RegistryService/ResolveRepresentation', registry__api_dot_v1_dot_registry__pb2.ResolveRepresentationRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.ResolveRepresentationResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Update(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.RegistryService/Update', registry__api_dot_v1_dot_registry__pb2.UpdateRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.UpdateResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Revoke(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.RegistryService/Revoke', registry__api_dot_v1_dot_registry__pb2.RevokeRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.RevokeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

class CredentialIssuerRegistryServiceStub(object):
    """Credential Issuer Registry Service Manages the Lifecycle of a Credential Issuer.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.CreateCredentialIssuer = channel.unary_unary('/registry_api.v1.CredentialIssuerRegistryService/CreateCredentialIssuer', request_serializer=registry__api_dot_v1_dot_registry__pb2.CreateCredentialIssuerRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.CreateCredentialIssuerResponse.FromString)
        self.GetCredentialIssuer = channel.unary_unary('/registry_api.v1.CredentialIssuerRegistryService/GetCredentialIssuer', request_serializer=registry__api_dot_v1_dot_registry__pb2.GetCredentialIssuerRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.GetCredentialIssuerResponse.FromString)
        self.UpdateCredentialIssuer = channel.unary_unary('/registry_api.v1.CredentialIssuerRegistryService/UpdateCredentialIssuer', request_serializer=registry__api_dot_v1_dot_registry__pb2.UpdateCredentialIssuerRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.UpdateCredentialIssuerResponse.FromString)

class CredentialIssuerRegistryServiceServicer(object):
    """Credential Issuer Registry Service Manages the Lifecycle of a Credential Issuer.
    """

    def CreateCredentialIssuer(self, request, context):
        """Creates a verifiable credential Issuer
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetCredentialIssuer(self, request, context):
        """Gets a verifiable credential Issuer
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def UpdateCredentialIssuer(self, request, context):
        """Updates a verifiable credential Issuer
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_CredentialIssuerRegistryServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'CreateCredentialIssuer': grpc.unary_unary_rpc_method_handler(servicer.CreateCredentialIssuer, request_deserializer=registry__api_dot_v1_dot_registry__pb2.CreateCredentialIssuerRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.CreateCredentialIssuerResponse.SerializeToString), 'GetCredentialIssuer': grpc.unary_unary_rpc_method_handler(servicer.GetCredentialIssuer, request_deserializer=registry__api_dot_v1_dot_registry__pb2.GetCredentialIssuerRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.GetCredentialIssuerResponse.SerializeToString), 'UpdateCredentialIssuer': grpc.unary_unary_rpc_method_handler(servicer.UpdateCredentialIssuer, request_deserializer=registry__api_dot_v1_dot_registry__pb2.UpdateCredentialIssuerRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.UpdateCredentialIssuerResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('registry_api.v1.CredentialIssuerRegistryService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class CredentialIssuerRegistryService(object):
    """Credential Issuer Registry Service Manages the Lifecycle of a Credential Issuer.
    """

    @staticmethod
    def CreateCredentialIssuer(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.CredentialIssuerRegistryService/CreateCredentialIssuer', registry__api_dot_v1_dot_registry__pb2.CreateCredentialIssuerRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.CreateCredentialIssuerResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetCredentialIssuer(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.CredentialIssuerRegistryService/GetCredentialIssuer', registry__api_dot_v1_dot_registry__pb2.GetCredentialIssuerRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.GetCredentialIssuerResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def UpdateCredentialIssuer(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.CredentialIssuerRegistryService/UpdateCredentialIssuer', registry__api_dot_v1_dot_registry__pb2.UpdateCredentialIssuerRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.UpdateCredentialIssuerResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

class StatusListRegistryServiceStub(object):
    """Status List Registry Service manages the Lifecycle of the VC Registry.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.CreateStatusListEntry = channel.unary_unary('/registry_api.v1.StatusListRegistryService/CreateStatusListEntry', request_serializer=registry__api_dot_v1_dot_registry__pb2.CreateStatusListEntryRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.CreateStatusListEntryResponse.FromString)
        self.GetStatusListCredential = channel.unary_unary('/registry_api.v1.StatusListRegistryService/GetStatusListCredential', request_serializer=registry__api_dot_v1_dot_registry__pb2.GetStatusListCredentialRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.GetStatusListCredentialResponse.FromString)
        self.UpdateStatusListEntry = channel.unary_unary('/registry_api.v1.StatusListRegistryService/UpdateStatusListEntry', request_serializer=registry__api_dot_v1_dot_registry__pb2.UpdateStatusListEntryRequest.SerializeToString, response_deserializer=registry__api_dot_v1_dot_registry__pb2.UpdateStatusListEntryResponse.FromString)

class StatusListRegistryServiceServicer(object):
    """Status List Registry Service manages the Lifecycle of the VC Registry.
    """

    def CreateStatusListEntry(self, request, context):
        """Create status registry entry
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetStatusListCredential(self, request, context):
        """Get VC status Credential
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def UpdateStatusListEntry(self, request, context):
        """Update status registry entry
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_StatusListRegistryServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'CreateStatusListEntry': grpc.unary_unary_rpc_method_handler(servicer.CreateStatusListEntry, request_deserializer=registry__api_dot_v1_dot_registry__pb2.CreateStatusListEntryRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.CreateStatusListEntryResponse.SerializeToString), 'GetStatusListCredential': grpc.unary_unary_rpc_method_handler(servicer.GetStatusListCredential, request_deserializer=registry__api_dot_v1_dot_registry__pb2.GetStatusListCredentialRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.GetStatusListCredentialResponse.SerializeToString), 'UpdateStatusListEntry': grpc.unary_unary_rpc_method_handler(servicer.UpdateStatusListEntry, request_deserializer=registry__api_dot_v1_dot_registry__pb2.UpdateStatusListEntryRequest.FromString, response_serializer=registry__api_dot_v1_dot_registry__pb2.UpdateStatusListEntryResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('registry_api.v1.StatusListRegistryService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class StatusListRegistryService(object):
    """Status List Registry Service manages the Lifecycle of the VC Registry.
    """

    @staticmethod
    def CreateStatusListEntry(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.StatusListRegistryService/CreateStatusListEntry', registry__api_dot_v1_dot_registry__pb2.CreateStatusListEntryRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.CreateStatusListEntryResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetStatusListCredential(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.StatusListRegistryService/GetStatusListCredential', registry__api_dot_v1_dot_registry__pb2.GetStatusListCredentialRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.GetStatusListCredentialResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def UpdateStatusListEntry(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/registry_api.v1.StatusListRegistryService/UpdateStatusListEntry', registry__api_dot_v1_dot_registry__pb2.UpdateStatusListEntryRequest.SerializeToString, registry__api_dot_v1_dot_registry__pb2.UpdateStatusListEntryResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)