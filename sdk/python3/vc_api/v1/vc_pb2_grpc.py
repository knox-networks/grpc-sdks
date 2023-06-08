"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...vc_api.v1 import vc_pb2 as vc__api_dot_v1_dot_vc__pb2

class CredentialAdapterServiceStub(object):
    """Enables existing systems to support W3C Verifiable Credential (VC) interactions between holders, VC issuers and verifiers as per https://www.w3.org/TR/vc-data-model/.  It is either used by credential providers (VC Issuer) or verifiers as required.  It provides predefined interfaces to expose (Example Bank customer info APIs) but allows for custom integrations as required.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.CreateIssuanceChallenge = channel.unary_unary('/vc_api.v1.CredentialAdapterService/CreateIssuanceChallenge', request_serializer=vc__api_dot_v1_dot_vc__pb2.CreateIssuanceChallengeRequest.SerializeToString, response_deserializer=vc__api_dot_v1_dot_vc__pb2.CreateIssuanceChallengeResponse.FromString)
        self.IssueVerifiableCredential = channel.unary_unary('/vc_api.v1.CredentialAdapterService/IssueVerifiableCredential', request_serializer=vc__api_dot_v1_dot_vc__pb2.IssueVerifiableCredentialRequest.SerializeToString, response_deserializer=vc__api_dot_v1_dot_vc__pb2.IssueVerifiableCredentialResponse.FromString)
        self.CreatePresentationChallenge = channel.unary_unary('/vc_api.v1.CredentialAdapterService/CreatePresentationChallenge', request_serializer=vc__api_dot_v1_dot_vc__pb2.CreatePresentationChallengeRequest.SerializeToString, response_deserializer=vc__api_dot_v1_dot_vc__pb2.CreatePresentationChallengeResponse.FromString)
        self.PresentVerifiableCredential = channel.unary_unary('/vc_api.v1.CredentialAdapterService/PresentVerifiableCredential', request_serializer=vc__api_dot_v1_dot_vc__pb2.PresentVerifiableCredentialRequest.SerializeToString, response_deserializer=vc__api_dot_v1_dot_vc__pb2.PresentVerifiableCredentialResponse.FromString)
        self.WaitForCompletion = channel.unary_stream('/vc_api.v1.CredentialAdapterService/WaitForCompletion', request_serializer=vc__api_dot_v1_dot_vc__pb2.WaitForCompletionRequest.SerializeToString, response_deserializer=vc__api_dot_v1_dot_vc__pb2.WaitForCompletionResponse.FromString)

class CredentialAdapterServiceServicer(object):
    """Enables existing systems to support W3C Verifiable Credential (VC) interactions between holders, VC issuers and verifiers as per https://www.w3.org/TR/vc-data-model/.  It is either used by credential providers (VC Issuer) or verifiers as required.  It provides predefined interfaces to expose (Example Bank customer info APIs) but allows for custom integrations as required.
    """

    def CreateIssuanceChallenge(self, request, context):
        """Called by the holder to the VC issuer to initiate the challenge needed for credential issuance.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def IssueVerifiableCredential(self, request, context):
        """Called by the holder to the VC issuer to answer the challenge and receive the credential.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreatePresentationChallenge(self, request, context):
        """Called by the holder to the verifier to initiate the challenge needed to verify the presentation with the verifier.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def PresentVerifiableCredential(self, request, context):
        """Called by the holder to the verifier to verify the presentation with the verifier.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def WaitForCompletion(self, request, context):
        """Called by the browser to initialize a stream with the server to listen for events such as challenge completion.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_CredentialAdapterServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'CreateIssuanceChallenge': grpc.unary_unary_rpc_method_handler(servicer.CreateIssuanceChallenge, request_deserializer=vc__api_dot_v1_dot_vc__pb2.CreateIssuanceChallengeRequest.FromString, response_serializer=vc__api_dot_v1_dot_vc__pb2.CreateIssuanceChallengeResponse.SerializeToString), 'IssueVerifiableCredential': grpc.unary_unary_rpc_method_handler(servicer.IssueVerifiableCredential, request_deserializer=vc__api_dot_v1_dot_vc__pb2.IssueVerifiableCredentialRequest.FromString, response_serializer=vc__api_dot_v1_dot_vc__pb2.IssueVerifiableCredentialResponse.SerializeToString), 'CreatePresentationChallenge': grpc.unary_unary_rpc_method_handler(servicer.CreatePresentationChallenge, request_deserializer=vc__api_dot_v1_dot_vc__pb2.CreatePresentationChallengeRequest.FromString, response_serializer=vc__api_dot_v1_dot_vc__pb2.CreatePresentationChallengeResponse.SerializeToString), 'PresentVerifiableCredential': grpc.unary_unary_rpc_method_handler(servicer.PresentVerifiableCredential, request_deserializer=vc__api_dot_v1_dot_vc__pb2.PresentVerifiableCredentialRequest.FromString, response_serializer=vc__api_dot_v1_dot_vc__pb2.PresentVerifiableCredentialResponse.SerializeToString), 'WaitForCompletion': grpc.unary_stream_rpc_method_handler(servicer.WaitForCompletion, request_deserializer=vc__api_dot_v1_dot_vc__pb2.WaitForCompletionRequest.FromString, response_serializer=vc__api_dot_v1_dot_vc__pb2.WaitForCompletionResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('vc_api.v1.CredentialAdapterService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class CredentialAdapterService(object):
    """Enables existing systems to support W3C Verifiable Credential (VC) interactions between holders, VC issuers and verifiers as per https://www.w3.org/TR/vc-data-model/.  It is either used by credential providers (VC Issuer) or verifiers as required.  It provides predefined interfaces to expose (Example Bank customer info APIs) but allows for custom integrations as required.
    """

    @staticmethod
    def CreateIssuanceChallenge(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/vc_api.v1.CredentialAdapterService/CreateIssuanceChallenge', vc__api_dot_v1_dot_vc__pb2.CreateIssuanceChallengeRequest.SerializeToString, vc__api_dot_v1_dot_vc__pb2.CreateIssuanceChallengeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def IssueVerifiableCredential(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/vc_api.v1.CredentialAdapterService/IssueVerifiableCredential', vc__api_dot_v1_dot_vc__pb2.IssueVerifiableCredentialRequest.SerializeToString, vc__api_dot_v1_dot_vc__pb2.IssueVerifiableCredentialResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def CreatePresentationChallenge(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/vc_api.v1.CredentialAdapterService/CreatePresentationChallenge', vc__api_dot_v1_dot_vc__pb2.CreatePresentationChallengeRequest.SerializeToString, vc__api_dot_v1_dot_vc__pb2.CreatePresentationChallengeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def PresentVerifiableCredential(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/vc_api.v1.CredentialAdapterService/PresentVerifiableCredential', vc__api_dot_v1_dot_vc__pb2.PresentVerifiableCredentialRequest.SerializeToString, vc__api_dot_v1_dot_vc__pb2.PresentVerifiableCredentialResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def WaitForCompletion(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_stream(request, target, '/vc_api.v1.CredentialAdapterService/WaitForCompletion', vc__api_dot_v1_dot_vc__pb2.WaitForCompletionRequest.SerializeToString, vc__api_dot_v1_dot_vc__pb2.WaitForCompletionResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)