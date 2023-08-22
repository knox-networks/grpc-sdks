"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...authority_api.v1 import authority_pb2 as authority__api_dot_v1_dot_authority__pb2

class AuthorityServiceStub(object):
    """Entity that is the approving monetary authority- Ex Central bank or commercial bank. It can create Issuers with
    limits, approve additional issuance limits, authorize issuing of digital banknotes, and replace digital banknotes in
    circulation.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.SetIssuerLimit = channel.unary_unary('/authority_api.v1.AuthorityService/SetIssuerLimit', request_serializer=authority__api_dot_v1_dot_authority__pb2.SetIssuerLimitRequest.SerializeToString, response_deserializer=authority__api_dot_v1_dot_authority__pb2.SetIssuerLimitResponse.FromString)
        self.GetIssuerLimits = channel.unary_unary('/authority_api.v1.AuthorityService/GetIssuerLimits', request_serializer=authority__api_dot_v1_dot_authority__pb2.GetIssuerLimitsRequest.SerializeToString, response_deserializer=authority__api_dot_v1_dot_authority__pb2.GetIssuerLimitsResponse.FromString)
        self.Authorize = channel.stream_stream('/authority_api.v1.AuthorityService/Authorize', request_serializer=authority__api_dot_v1_dot_authority__pb2.AuthorizeRequest.SerializeToString, response_deserializer=authority__api_dot_v1_dot_authority__pb2.AuthorizeResponse.FromString)
        self.Redeem = channel.unary_unary('/authority_api.v1.AuthorityService/Redeem', request_serializer=authority__api_dot_v1_dot_authority__pb2.RedeemRequest.SerializeToString, response_deserializer=authority__api_dot_v1_dot_authority__pb2.RedeemResponse.FromString)
        self.GetNetworkInfo = channel.unary_unary('/authority_api.v1.AuthorityService/GetNetworkInfo', request_serializer=authority__api_dot_v1_dot_authority__pb2.GetNetworkInfoRequest.SerializeToString, response_deserializer=authority__api_dot_v1_dot_authority__pb2.GetNetworkInfoResponse.FromString)
        self.GetNotary = channel.unary_unary('/authority_api.v1.AuthorityService/GetNotary', request_serializer=authority__api_dot_v1_dot_authority__pb2.GetNotaryRequest.SerializeToString, response_deserializer=authority__api_dot_v1_dot_authority__pb2.GetNotaryResponse.FromString)

class AuthorityServiceServicer(object):
    """Entity that is the approving monetary authority- Ex Central bank or commercial bank. It can create Issuers with
    limits, approve additional issuance limits, authorize issuing of digital banknotes, and replace digital banknotes in
    circulation.
    """

    def SetIssuerLimit(self, request, context):
        """Sets the issuance limit for an Issuer to a specified value and returns the new limit.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetIssuerLimits(self, request, context):
        """Gets the the issuance limits for all the currencies for which the Issuer is allowed to issue digital banknotes.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Authorize(self, request_iterator, context):
        """Accepts digital banknotes in a stream, authorizes them and returns authorized digital banknotes in a stream.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Redeem(self, request, context):
        """Redeems a digital banknote in exchange for an increase in issuance limit for the same currency as the redeemed
        digital banknote.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetNetworkInfo(self, request, context):
        """Gets the verifier of the Emissary for this Authority.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetNotary(self, request, context):
        """Gets the signature of the Notary for this Authority/Notary.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_AuthorityServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'SetIssuerLimit': grpc.unary_unary_rpc_method_handler(servicer.SetIssuerLimit, request_deserializer=authority__api_dot_v1_dot_authority__pb2.SetIssuerLimitRequest.FromString, response_serializer=authority__api_dot_v1_dot_authority__pb2.SetIssuerLimitResponse.SerializeToString), 'GetIssuerLimits': grpc.unary_unary_rpc_method_handler(servicer.GetIssuerLimits, request_deserializer=authority__api_dot_v1_dot_authority__pb2.GetIssuerLimitsRequest.FromString, response_serializer=authority__api_dot_v1_dot_authority__pb2.GetIssuerLimitsResponse.SerializeToString), 'Authorize': grpc.stream_stream_rpc_method_handler(servicer.Authorize, request_deserializer=authority__api_dot_v1_dot_authority__pb2.AuthorizeRequest.FromString, response_serializer=authority__api_dot_v1_dot_authority__pb2.AuthorizeResponse.SerializeToString), 'Redeem': grpc.unary_unary_rpc_method_handler(servicer.Redeem, request_deserializer=authority__api_dot_v1_dot_authority__pb2.RedeemRequest.FromString, response_serializer=authority__api_dot_v1_dot_authority__pb2.RedeemResponse.SerializeToString), 'GetNetworkInfo': grpc.unary_unary_rpc_method_handler(servicer.GetNetworkInfo, request_deserializer=authority__api_dot_v1_dot_authority__pb2.GetNetworkInfoRequest.FromString, response_serializer=authority__api_dot_v1_dot_authority__pb2.GetNetworkInfoResponse.SerializeToString), 'GetNotary': grpc.unary_unary_rpc_method_handler(servicer.GetNotary, request_deserializer=authority__api_dot_v1_dot_authority__pb2.GetNotaryRequest.FromString, response_serializer=authority__api_dot_v1_dot_authority__pb2.GetNotaryResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('authority_api.v1.AuthorityService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class AuthorityService(object):
    """Entity that is the approving monetary authority- Ex Central bank or commercial bank. It can create Issuers with
    limits, approve additional issuance limits, authorize issuing of digital banknotes, and replace digital banknotes in
    circulation.
    """

    @staticmethod
    def SetIssuerLimit(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/authority_api.v1.AuthorityService/SetIssuerLimit', authority__api_dot_v1_dot_authority__pb2.SetIssuerLimitRequest.SerializeToString, authority__api_dot_v1_dot_authority__pb2.SetIssuerLimitResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetIssuerLimits(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/authority_api.v1.AuthorityService/GetIssuerLimits', authority__api_dot_v1_dot_authority__pb2.GetIssuerLimitsRequest.SerializeToString, authority__api_dot_v1_dot_authority__pb2.GetIssuerLimitsResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Authorize(request_iterator, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.stream_stream(request_iterator, target, '/authority_api.v1.AuthorityService/Authorize', authority__api_dot_v1_dot_authority__pb2.AuthorizeRequest.SerializeToString, authority__api_dot_v1_dot_authority__pb2.AuthorizeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Redeem(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/authority_api.v1.AuthorityService/Redeem', authority__api_dot_v1_dot_authority__pb2.RedeemRequest.SerializeToString, authority__api_dot_v1_dot_authority__pb2.RedeemResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetNetworkInfo(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/authority_api.v1.AuthorityService/GetNetworkInfo', authority__api_dot_v1_dot_authority__pb2.GetNetworkInfoRequest.SerializeToString, authority__api_dot_v1_dot_authority__pb2.GetNetworkInfoResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetNotary(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/authority_api.v1.AuthorityService/GetNotary', authority__api_dot_v1_dot_authority__pb2.GetNotaryRequest.SerializeToString, authority__api_dot_v1_dot_authority__pb2.GetNotaryResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)