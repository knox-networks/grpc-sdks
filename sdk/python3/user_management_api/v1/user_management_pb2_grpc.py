"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...user_management_api.v1 import user_management_pb2 as user__management__api_dot_v1_dot_user__management__pb2

class UserManagementStub(object):
    """This service integrates with existing identity systems (Ex. Bank accounts) to bridge traditional account identifiers to DID/public key identifiers used in the Knox ecosystem. Leverages existing KYC/AML data to enable financial transaction compliance.  Supports OIDC, SAML and other traditional AuthN/Z standards. 
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.AuthnWithPassword = channel.unary_unary('/auth_api.v1.UserManagement/AuthnWithPassword', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWithPasswordRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWithPasswordResponse.FromString)
        self.RefreshAccessToken = channel.unary_unary('/auth_api.v1.UserManagement/RefreshAccessToken', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.RefreshAccessTokenRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.RefreshAccessTokenResponse.FromString)
        self.AuthnWithProvider = channel.unary_unary('/auth_api.v1.UserManagement/AuthnWithProvider', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWithProviderRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWithProviderResponse.FromString)
        self.HandleOIDCCallback = channel.unary_unary('/auth_api.v1.UserManagement/HandleOIDCCallback', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.HandleOIDCCallbackRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.HandleOIDCCallbackResponse.FromString)
        self.HandleSAMLCallback = channel.unary_unary('/auth_api.v1.UserManagement/HandleSAMLCallback', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.HandleSAMLCallbackRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.HandleSAMLCallbackResponse.FromString)
        self.SAMLSPMetadata = channel.unary_unary('/auth_api.v1.UserManagement/SAMLSPMetadata', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.SAMLSPMetadataRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.SAMLSPMetadataResponse.FromString)
        self.CreateAuthnBrowserWithWalletChallenge = channel.unary_stream('/auth_api.v1.UserManagement/CreateAuthnBrowserWithWalletChallenge', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnBrowserWithWalletChallengeRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnBrowserWithWalletChallengeResponse.FromString)
        self.AuthnBrowserWithWallet = channel.unary_unary('/auth_api.v1.UserManagement/AuthnBrowserWithWallet', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnBrowserWithWalletRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnBrowserWithWalletResponse.FromString)
        self.CreateAuthnWalletChallenge = channel.unary_unary('/auth_api.v1.UserManagement/CreateAuthnWalletChallenge', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnWalletChallengeRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnWalletChallengeResponse.FromString)
        self.AuthnWallet = channel.unary_unary('/auth_api.v1.UserManagement/AuthnWallet', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWalletRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWalletResponse.FromString)
        self.CreateRegisterWalletChallenge = channel.unary_stream('/auth_api.v1.UserManagement/CreateRegisterWalletChallenge', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.CreateRegisterWalletChallengeRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.CreateRegisterWalletChallengeResponse.FromString)
        self.RegisterWallet = channel.unary_unary('/auth_api.v1.UserManagement/RegisterWallet', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.RegisterWalletRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.RegisterWalletResponse.FromString)
        self.AssociateWallet = channel.unary_unary('/auth_api.v1.UserManagement/AssociateWallet', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.AssociateWalletRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.AssociateWalletResponse.FromString)
        self.GetAppSettings = channel.unary_unary('/auth_api.v1.UserManagement/GetAppSettings', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.GetAppSettingsRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.GetAppSettingsResponse.FromString)
        self.RegisterUser = channel.unary_unary('/auth_api.v1.UserManagement/RegisterUser', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.RegisterUserRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.RegisterUserResponse.FromString)
        self.ConfirmUser = channel.unary_unary('/auth_api.v1.UserManagement/ConfirmUser', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.ConfirmUserRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.ConfirmUserResponse.FromString)
        self.FindByEmail = channel.unary_unary('/auth_api.v1.UserManagement/FindByEmail', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.FindByEmailRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.FindByEmailResponse.FromString)
        self.FindByID = channel.unary_unary('/auth_api.v1.UserManagement/FindByID', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.FindByIDRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.FindByIDResponse.FromString)
        self.GetMe = channel.unary_unary('/auth_api.v1.UserManagement/GetMe', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.GetMeRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.GetMeResponse.FromString)
        self.GetUserByDID = channel.unary_unary('/auth_api.v1.UserManagement/GetUserByDID', request_serializer=user__management__api_dot_v1_dot_user__management__pb2.GetUserByDIDRequest.SerializeToString, response_deserializer=user__management__api_dot_v1_dot_user__management__pb2.GetUserByDIDResponse.FromString)

class UserManagementServicer(object):
    """This service integrates with existing identity systems (Ex. Bank accounts) to bridge traditional account identifiers to DID/public key identifiers used in the Knox ecosystem. Leverages existing KYC/AML data to enable financial transaction compliance.  Supports OIDC, SAML and other traditional AuthN/Z standards. 
    """

    def AuthnWithPassword(self, request, context):
        """Login with username and password for admin and test users.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def RefreshAccessToken(self, request, context):
        """Login with refresh token and get a new access token.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def AuthnWithProvider(self, request, context):
        """Start login process against a selected identity provider. Supports OIDC and SAML.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def HandleOIDCCallback(self, request, context):
        """Complete login process with the information provided by the OIDC identity provider in previous step.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def HandleSAMLCallback(self, request, context):
        """Complete login process with the information provided by the SAML identity provider in previous step.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def SAMLSPMetadata(self, request, context):
        """Retrieve SAML service provider metadata.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateAuthnBrowserWithWalletChallenge(self, request, context):
        """Used for logging into the browser using a device wallet. Called from the browser, bearer token is received from the stream response once authentication is successful via the wallet. 
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def AuthnBrowserWithWallet(self, request, context):
        """Called from the device wallet to authenticate the browser and bearer token is sent to the browser session.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateAuthnWalletChallenge(self, request, context):
        """Start login process on the device wallet. 
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def AuthnWallet(self, request, context):
        """Finish login with cryptographic challenge and bearer token is returned to the caller.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateRegisterWalletChallenge(self, request, context):
        """Called by the browser in an authenticated session to start a device wallet registration process. Acknowledgement is received from the stream response once registration is successful via the wallet. 
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def RegisterWallet(self, request, context):
        """Called by the device wallet to register against the given authenticated account. 
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def AssociateWallet(self, request, context):
        """Associate Wallet with the authenticated account in the bearer token.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetAppSettings(self, request, context):
        """Get application specific settings in a given environment.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def RegisterUser(self, request, context):
        """Admin API to register a user.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ConfirmUser(self, request, context):
        """Admin API to finish the user registration process.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def FindByEmail(self, request, context):
        """Admin API to retrieve User object via email.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def FindByID(self, request, context):
        """Admin API to retrieve User object via user ID.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetMe(self, request, context):
        """Get current authenticated User by bearer token.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetUserByDID(self, request, context):
        """M2M helper API to retrieve User object given a Decentralized Identifier (DID). API Key required for this call.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_UserManagementServicer_to_server(servicer, server):
    rpc_method_handlers = {'AuthnWithPassword': grpc.unary_unary_rpc_method_handler(servicer.AuthnWithPassword, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWithPasswordRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWithPasswordResponse.SerializeToString), 'RefreshAccessToken': grpc.unary_unary_rpc_method_handler(servicer.RefreshAccessToken, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.RefreshAccessTokenRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.RefreshAccessTokenResponse.SerializeToString), 'AuthnWithProvider': grpc.unary_unary_rpc_method_handler(servicer.AuthnWithProvider, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWithProviderRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWithProviderResponse.SerializeToString), 'HandleOIDCCallback': grpc.unary_unary_rpc_method_handler(servicer.HandleOIDCCallback, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.HandleOIDCCallbackRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.HandleOIDCCallbackResponse.SerializeToString), 'HandleSAMLCallback': grpc.unary_unary_rpc_method_handler(servicer.HandleSAMLCallback, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.HandleSAMLCallbackRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.HandleSAMLCallbackResponse.SerializeToString), 'SAMLSPMetadata': grpc.unary_unary_rpc_method_handler(servicer.SAMLSPMetadata, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.SAMLSPMetadataRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.SAMLSPMetadataResponse.SerializeToString), 'CreateAuthnBrowserWithWalletChallenge': grpc.unary_stream_rpc_method_handler(servicer.CreateAuthnBrowserWithWalletChallenge, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnBrowserWithWalletChallengeRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnBrowserWithWalletChallengeResponse.SerializeToString), 'AuthnBrowserWithWallet': grpc.unary_unary_rpc_method_handler(servicer.AuthnBrowserWithWallet, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnBrowserWithWalletRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnBrowserWithWalletResponse.SerializeToString), 'CreateAuthnWalletChallenge': grpc.unary_unary_rpc_method_handler(servicer.CreateAuthnWalletChallenge, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnWalletChallengeRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnWalletChallengeResponse.SerializeToString), 'AuthnWallet': grpc.unary_unary_rpc_method_handler(servicer.AuthnWallet, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWalletRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.AuthnWalletResponse.SerializeToString), 'CreateRegisterWalletChallenge': grpc.unary_stream_rpc_method_handler(servicer.CreateRegisterWalletChallenge, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.CreateRegisterWalletChallengeRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.CreateRegisterWalletChallengeResponse.SerializeToString), 'RegisterWallet': grpc.unary_unary_rpc_method_handler(servicer.RegisterWallet, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.RegisterWalletRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.RegisterWalletResponse.SerializeToString), 'AssociateWallet': grpc.unary_unary_rpc_method_handler(servicer.AssociateWallet, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.AssociateWalletRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.AssociateWalletResponse.SerializeToString), 'GetAppSettings': grpc.unary_unary_rpc_method_handler(servicer.GetAppSettings, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.GetAppSettingsRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.GetAppSettingsResponse.SerializeToString), 'RegisterUser': grpc.unary_unary_rpc_method_handler(servicer.RegisterUser, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.RegisterUserRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.RegisterUserResponse.SerializeToString), 'ConfirmUser': grpc.unary_unary_rpc_method_handler(servicer.ConfirmUser, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.ConfirmUserRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.ConfirmUserResponse.SerializeToString), 'FindByEmail': grpc.unary_unary_rpc_method_handler(servicer.FindByEmail, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.FindByEmailRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.FindByEmailResponse.SerializeToString), 'FindByID': grpc.unary_unary_rpc_method_handler(servicer.FindByID, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.FindByIDRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.FindByIDResponse.SerializeToString), 'GetMe': grpc.unary_unary_rpc_method_handler(servicer.GetMe, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.GetMeRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.GetMeResponse.SerializeToString), 'GetUserByDID': grpc.unary_unary_rpc_method_handler(servicer.GetUserByDID, request_deserializer=user__management__api_dot_v1_dot_user__management__pb2.GetUserByDIDRequest.FromString, response_serializer=user__management__api_dot_v1_dot_user__management__pb2.GetUserByDIDResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('auth_api.v1.UserManagement', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class UserManagement(object):
    """This service integrates with existing identity systems (Ex. Bank accounts) to bridge traditional account identifiers to DID/public key identifiers used in the Knox ecosystem. Leverages existing KYC/AML data to enable financial transaction compliance.  Supports OIDC, SAML and other traditional AuthN/Z standards. 
    """

    @staticmethod
    def AuthnWithPassword(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/AuthnWithPassword', user__management__api_dot_v1_dot_user__management__pb2.AuthnWithPasswordRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.AuthnWithPasswordResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def RefreshAccessToken(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/RefreshAccessToken', user__management__api_dot_v1_dot_user__management__pb2.RefreshAccessTokenRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.RefreshAccessTokenResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def AuthnWithProvider(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/AuthnWithProvider', user__management__api_dot_v1_dot_user__management__pb2.AuthnWithProviderRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.AuthnWithProviderResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def HandleOIDCCallback(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/HandleOIDCCallback', user__management__api_dot_v1_dot_user__management__pb2.HandleOIDCCallbackRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.HandleOIDCCallbackResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def HandleSAMLCallback(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/HandleSAMLCallback', user__management__api_dot_v1_dot_user__management__pb2.HandleSAMLCallbackRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.HandleSAMLCallbackResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def SAMLSPMetadata(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/SAMLSPMetadata', user__management__api_dot_v1_dot_user__management__pb2.SAMLSPMetadataRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.SAMLSPMetadataResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def CreateAuthnBrowserWithWalletChallenge(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_stream(request, target, '/auth_api.v1.UserManagement/CreateAuthnBrowserWithWalletChallenge', user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnBrowserWithWalletChallengeRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnBrowserWithWalletChallengeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def AuthnBrowserWithWallet(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/AuthnBrowserWithWallet', user__management__api_dot_v1_dot_user__management__pb2.AuthnBrowserWithWalletRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.AuthnBrowserWithWalletResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def CreateAuthnWalletChallenge(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/CreateAuthnWalletChallenge', user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnWalletChallengeRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.CreateAuthnWalletChallengeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def AuthnWallet(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/AuthnWallet', user__management__api_dot_v1_dot_user__management__pb2.AuthnWalletRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.AuthnWalletResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def CreateRegisterWalletChallenge(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_stream(request, target, '/auth_api.v1.UserManagement/CreateRegisterWalletChallenge', user__management__api_dot_v1_dot_user__management__pb2.CreateRegisterWalletChallengeRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.CreateRegisterWalletChallengeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def RegisterWallet(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/RegisterWallet', user__management__api_dot_v1_dot_user__management__pb2.RegisterWalletRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.RegisterWalletResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def AssociateWallet(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/AssociateWallet', user__management__api_dot_v1_dot_user__management__pb2.AssociateWalletRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.AssociateWalletResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetAppSettings(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/GetAppSettings', user__management__api_dot_v1_dot_user__management__pb2.GetAppSettingsRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.GetAppSettingsResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def RegisterUser(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/RegisterUser', user__management__api_dot_v1_dot_user__management__pb2.RegisterUserRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.RegisterUserResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ConfirmUser(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/ConfirmUser', user__management__api_dot_v1_dot_user__management__pb2.ConfirmUserRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.ConfirmUserResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def FindByEmail(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/FindByEmail', user__management__api_dot_v1_dot_user__management__pb2.FindByEmailRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.FindByEmailResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def FindByID(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/FindByID', user__management__api_dot_v1_dot_user__management__pb2.FindByIDRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.FindByIDResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetMe(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/GetMe', user__management__api_dot_v1_dot_user__management__pb2.GetMeRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.GetMeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetUserByDID(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/auth_api.v1.UserManagement/GetUserByDID', user__management__api_dot_v1_dot_user__management__pb2.GetUserByDIDRequest.SerializeToString, user__management__api_dot_v1_dot_user__management__pb2.GetUserByDIDResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)