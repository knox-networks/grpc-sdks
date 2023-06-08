"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...issuer_api.v1 import issuer_pb2 as issuer__api_dot_v1_dot_issuer__pb2

class IssuerServiceStub(object):
    """Financial intermediary with a reserve account with the Authority. A role typically played by commercial banks and
    other depository institutions, it can issue, transfer, redeem digital banknotes and request an increase in issuance
    limits.  It holds digital banknotes in its own vault.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Authenticate = channel.unary_unary('/issuer_api.v1.IssuerService/Authenticate', request_serializer=issuer__api_dot_v1_dot_issuer__pb2.AuthenticateRequest.SerializeToString, response_deserializer=issuer__api_dot_v1_dot_issuer__pb2.AuthenticateResponse.FromString)
        self.Issue = channel.unary_unary('/issuer_api.v1.IssuerService/Issue', request_serializer=issuer__api_dot_v1_dot_issuer__pb2.IssueRequest.SerializeToString, response_deserializer=issuer__api_dot_v1_dot_issuer__pb2.IssueResponse.FromString)
        self.GetRoles = channel.unary_unary('/issuer_api.v1.IssuerService/GetRoles', request_serializer=issuer__api_dot_v1_dot_issuer__pb2.GetRolesRequest.SerializeToString, response_deserializer=issuer__api_dot_v1_dot_issuer__pb2.GetRolesResponse.FromString)
        self.SetRole = channel.unary_unary('/issuer_api.v1.IssuerService/SetRole', request_serializer=issuer__api_dot_v1_dot_issuer__pb2.SetRoleRequest.SerializeToString, response_deserializer=issuer__api_dot_v1_dot_issuer__pb2.SetRoleResponse.FromString)

class IssuerServiceServicer(object):
    """Financial intermediary with a reserve account with the Authority. A role typically played by commercial banks and
    other depository institutions, it can issue, transfer, redeem digital banknotes and request an increase in issuance
    limits.  It holds digital banknotes in its own vault.
    """

    def Authenticate(self, request, context):
        """Checks that the Verifier is authenticated, then stores and returns the challenge nonce.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Issue(self, request, context):
        """Issues digital banknotes against the corresponding issuance limit set by an Authority.  The Issuer must collect the
        Authenticate challenge signature, the Amount with a total amount, decimal place precision, and a currency code,
        along with the verifier of the Authority. Issued promissories are sent to the given recipient.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetRoles(self, request, context):
        """Gets the roles configured for the Issuer- ex Currency limit for issuance.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def SetRole(self, request, context):
        """Sets the roles configured for the Issuer- Ex List of the currency limits.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_IssuerServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'Authenticate': grpc.unary_unary_rpc_method_handler(servicer.Authenticate, request_deserializer=issuer__api_dot_v1_dot_issuer__pb2.AuthenticateRequest.FromString, response_serializer=issuer__api_dot_v1_dot_issuer__pb2.AuthenticateResponse.SerializeToString), 'Issue': grpc.unary_unary_rpc_method_handler(servicer.Issue, request_deserializer=issuer__api_dot_v1_dot_issuer__pb2.IssueRequest.FromString, response_serializer=issuer__api_dot_v1_dot_issuer__pb2.IssueResponse.SerializeToString), 'GetRoles': grpc.unary_unary_rpc_method_handler(servicer.GetRoles, request_deserializer=issuer__api_dot_v1_dot_issuer__pb2.GetRolesRequest.FromString, response_serializer=issuer__api_dot_v1_dot_issuer__pb2.GetRolesResponse.SerializeToString), 'SetRole': grpc.unary_unary_rpc_method_handler(servicer.SetRole, request_deserializer=issuer__api_dot_v1_dot_issuer__pb2.SetRoleRequest.FromString, response_serializer=issuer__api_dot_v1_dot_issuer__pb2.SetRoleResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('issuer_api.v1.IssuerService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class IssuerService(object):
    """Financial intermediary with a reserve account with the Authority. A role typically played by commercial banks and
    other depository institutions, it can issue, transfer, redeem digital banknotes and request an increase in issuance
    limits.  It holds digital banknotes in its own vault.
    """

    @staticmethod
    def Authenticate(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/issuer_api.v1.IssuerService/Authenticate', issuer__api_dot_v1_dot_issuer__pb2.AuthenticateRequest.SerializeToString, issuer__api_dot_v1_dot_issuer__pb2.AuthenticateResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Issue(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/issuer_api.v1.IssuerService/Issue', issuer__api_dot_v1_dot_issuer__pb2.IssueRequest.SerializeToString, issuer__api_dot_v1_dot_issuer__pb2.IssueResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetRoles(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/issuer_api.v1.IssuerService/GetRoles', issuer__api_dot_v1_dot_issuer__pb2.GetRolesRequest.SerializeToString, issuer__api_dot_v1_dot_issuer__pb2.GetRolesResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def SetRole(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/issuer_api.v1.IssuerService/SetRole', issuer__api_dot_v1_dot_issuer__pb2.SetRoleRequest.SerializeToString, issuer__api_dot_v1_dot_issuer__pb2.SetRoleResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)