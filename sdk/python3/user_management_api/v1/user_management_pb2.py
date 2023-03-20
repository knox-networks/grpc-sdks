"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from google.protobuf import struct_pb2 as google_dot_protobuf_dot_struct__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n,user_management_api/v1/user_management.proto\x12\x0buser_api.v1\x1a\x1cgoogle/protobuf/struct.proto"\xb1\x02\n\x04User\x12\x0c\n\x04uuid\x18\x01 \x01(\t\x12\x12\n\nfirst_name\x18\x02 \x01(\t\x12\x11\n\tlast_name\x18\x03 \x01(\t\x12\r\n\x05email\x18\x06 \x01(\t\x12\x1f\n\x04role\x18\x07 \x01(\x0e2\x11.user_api.v1.Role\x12\x0e\n\x06avatar\x18\x08 \x01(\t\x12\r\n\x05phone\x18\t \x01(\t\x12\x0c\n\x04dids\x18\n \x03(\t\x128\n\x0cexternal_ids\x18\x0b \x03(\x0b2".user_api.v1.User.ExternalIdsEntry\x12)\n\x08metadata\x18\x0c \x01(\x0b2\x17.google.protobuf.Struct\x1a2\n\x10ExternalIdsEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\t:\x028\x01"\x9d\x01\n\x13RegisterUserRequest\x12\r\n\x05email\x18\x01 \x01(\t\x12\x12\n\nfirst_name\x18\x02 \x01(\t\x12\x11\n\tlast_name\x18\x03 \x01(\t\x12\x10\n\x08password\x18\x05 \x01(\t\x12\x1f\n\x04role\x18\x06 \x01(\x0e2\x11.user_api.v1.Role\x12\x0e\n\x06avatar\x18\x07 \x01(\t\x12\r\n\x05phone\x18\x08 \x01(\t"Y\n\tAuthToken\x12\r\n\x05token\x18\x01 \x01(\t\x12\x12\n\ntoken_type\x18\x02 \x01(\t\x12\x12\n\nexpires_in\x18\x03 \x01(\x05\x12\x15\n\rrefresh_token\x18\x04 \x01(\t"7\n\x14RegisterUserResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User";\n\x12ConfirmUserRequest\x12\x16\n\x0eemail_or_phone\x18\x01 \x01(\t\x12\r\n\x05token\x18\x02 \x01(\t"\x15\n\x13ConfirmUserResponse"\xa1\x01\n\x18AuthnWithProviderRequest\x12\'\n\x08provider\x18\x01 \x01(\x0e2\x15.user_api.v1.Provider\x12\x15\n\rinstance_name\x18\x02 \x01(\t\x12\x16\n\x0erequest_origin\x18\x03 \x01(\t\x12-\n\x0cclient_state\x18\x04 \x01(\x0b2\x17.google.protobuf.Struct"1\n\x19AuthnWithProviderResponse\x12\x14\n\x0cprovider_url\x18\x01 \x01(\t"8\n\x19HandleOIDCCallbackRequest\x12\x0c\n\x04code\x18\x01 \x01(\t\x12\r\n\x05state\x18\x02 \x01(\t"\xc7\x01\n\x1aHandleOIDCCallbackResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b2\x16.user_api.v1.AuthToken\x12\x15\n\rinstance_name\x18\x03 \x01(\t\x12\x16\n\x0erequest_origin\x18\x04 \x01(\t\x12-\n\x0cclient_state\x18\x05 \x01(\x0b2\x17.google.protobuf.Struct"#\n\x12FindByEmailRequest\x12\r\n\x05email\x18\x01 \x01(\t"6\n\x13FindByEmailResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User"\x1f\n\x0fFindByIDRequest\x12\x0c\n\x04uuid\x18\x01 \x01(\t"3\n\x10FindByIDResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User";\n\x18AuthnWithPasswordRequest\x12\r\n\x05email\x18\x01 \x01(\t\x12\x10\n\x08password\x18\x02 \x01(\t"h\n\x19AuthnWithPasswordResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b2\x16.user_api.v1.AuthToken"2\n\x19RefreshAccessTokenRequest\x12\x15\n\rrefresh_token\x18\x01 \x01(\t"i\n\x1aRefreshAccessTokenResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b2\x16.user_api.v1.AuthToken"\x0e\n\x0cGetMeRequest"0\n\rGetMeResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User"`\n\x11AuthTokenResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b2\x16.user_api.v1.AuthToken".\n,CreateAuthnBrowserWithWalletChallengeRequest"\x83\x01\n-CreateAuthnBrowserWithWalletChallengeResponse\x12\x0f\n\x05nonce\x18\x01 \x01(\tH\x00\x124\n\nauth_token\x18\x02 \x01(\x0b2\x1e.user_api.v1.AuthTokenResponseH\x00B\x0b\n\tdid_start"N\n\x1dAuthnBrowserWithWalletRequest\x12\x11\n\tsignature\x18\x01 \x01(\x0c\x12\r\n\x05nonce\x18\x02 \x01(\t\x12\x0b\n\x03did\x18\x03 \x01(\t" \n\x1eAuthnBrowserWithWalletResponse"&\n$CreateRegisterWalletChallengeRequest"\\\n%CreateRegisterWalletChallengeResponse\x12\x0f\n\x05nonce\x18\x01 \x01(\tH\x00\x12\x0c\n\x02ok\x18\x02 \x01(\tH\x00B\x14\n\x12registration_start"F\n\x15RegisterWalletRequest\x12\x11\n\tsignature\x18\x01 \x01(\x0c\x12\r\n\x05nonce\x18\x02 \x01(\t\x12\x0b\n\x03did\x18\x03 \x01(\t"e\n\x16RegisterWalletResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b2\x16.user_api.v1.AuthToken"0\n!CreateAuthnWalletChallengeRequest\x12\x0b\n\x03did\x18\x01 \x01(\t"3\n"CreateAuthnWalletChallengeResponse\x12\r\n\x05nonce\x18\x01 \x01(\t"C\n\x12AuthnWalletRequest\x12\x11\n\tsignature\x18\x01 \x01(\x0c\x12\r\n\x05nonce\x18\x02 \x01(\t\x12\x0b\n\x03did\x18\x03 \x01(\t"b\n\x13AuthnWalletResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b2\x16.user_api.v1.AuthToken"G\n\x19HandleSAMLCallbackRequest\x12\x15\n\rsaml_response\x18\x01 \x01(\t\x12\x13\n\x0brelay_state\x18\x02 \x01(\t"\x98\x01\n\x1aHandleSAMLCallbackResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b2\x16.user_api.v1.AuthToken\x12\x15\n\rinstance_name\x18\x03 \x01(\t\x12\x16\n\x0erequest_origin\x18\x04 \x01(\t"\x17\n\x15SAMLSPMetadataRequest"-\n\x16SAMLSPMetadataResponse\x12\x13\n\x0bsp_metadata\x18\x01 \x01(\x0c"\x17\n\x15GetAppSettingsRequest"B\n\x0bAppSettings\x123\n\x0fauthn_providers\x18\x01 \x03(\x0b2\x1a.user_api.v1.AuthnProvider"I\n\rAuthnProvider\x12\'\n\x08provider\x18\x01 \x01(\x0e2\x15.user_api.v1.Provider\x12\x0f\n\x07enabled\x18\x02 \x01(\x08"H\n\x16GetAppSettingsResponse\x12.\n\x0capp_settings\x18\x01 \x01(\x0b2\x18.user_api.v1.AppSettings""\n\x13GetUserByDIDRequest\x12\x0b\n\x03did\x18\x01 \x01(\t"7\n\x14GetUserByDIDResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b2\x11.user_api.v1.User"%\n\x16AssociateWalletRequest\x12\x0b\n\x03did\x18\x01 \x01(\t"\x19\n\x17AssociateWalletResponse*\x8e\x01\n\x08Provider\x12\x18\n\x14PROVIDER_UNSPECIFIED\x10\x00\x12\x13\n\x0fPROVIDER_GITHUB\x10\x01\x12\x15\n\x11PROVIDER_FACEBOOK\x10\x02\x12\x13\n\x0fPROVIDER_GOOGLE\x10\x03\x12\x11\n\rPROVIDER_SAML\x10\x04\x12\x14\n\x10PROVIDER_COGNITO\x10\x05*;\n\x04Role\x12\x14\n\x10ROLE_UNSPECIFIED\x10\x00\x12\r\n\tROLE_USER\x10\x01\x12\x0e\n\nROLE_ADMIN\x10\x022\xc5\x0f\n\x0eUserManagement\x12b\n\x11AuthnWithPassword\x12%.user_api.v1.AuthnWithPasswordRequest\x1a&.user_api.v1.AuthnWithPasswordResponse\x12e\n\x12RefreshAccessToken\x12&.user_api.v1.RefreshAccessTokenRequest\x1a\'.user_api.v1.RefreshAccessTokenResponse\x12b\n\x11AuthnWithProvider\x12%.user_api.v1.AuthnWithProviderRequest\x1a&.user_api.v1.AuthnWithProviderResponse\x12e\n\x12HandleOIDCCallback\x12&.user_api.v1.HandleOIDCCallbackRequest\x1a\'.user_api.v1.HandleOIDCCallbackResponse\x12e\n\x12HandleSAMLCallback\x12&.user_api.v1.HandleSAMLCallbackRequest\x1a\'.user_api.v1.HandleSAMLCallbackResponse\x12Y\n\x0eSAMLSPMetadata\x12".user_api.v1.SAMLSPMetadataRequest\x1a#.user_api.v1.SAMLSPMetadataResponse\x12\xa0\x01\n%CreateAuthnBrowserWithWalletChallenge\x129.user_api.v1.CreateAuthnBrowserWithWalletChallengeRequest\x1a:.user_api.v1.CreateAuthnBrowserWithWalletChallengeResponse0\x01\x12q\n\x16AuthnBrowserWithWallet\x12*.user_api.v1.AuthnBrowserWithWalletRequest\x1a+.user_api.v1.AuthnBrowserWithWalletResponse\x12}\n\x1aCreateAuthnWalletChallenge\x12..user_api.v1.CreateAuthnWalletChallengeRequest\x1a/.user_api.v1.CreateAuthnWalletChallengeResponse\x12P\n\x0bAuthnWallet\x12\x1f.user_api.v1.AuthnWalletRequest\x1a .user_api.v1.AuthnWalletResponse\x12\x88\x01\n\x1dCreateRegisterWalletChallenge\x121.user_api.v1.CreateRegisterWalletChallengeRequest\x1a2.user_api.v1.CreateRegisterWalletChallengeResponse0\x01\x12Y\n\x0eRegisterWallet\x12".user_api.v1.RegisterWalletRequest\x1a#.user_api.v1.RegisterWalletResponse\x12\\\n\x0fAssociateWallet\x12#.user_api.v1.AssociateWalletRequest\x1a$.user_api.v1.AssociateWalletResponse\x12Y\n\x0eGetAppSettings\x12".user_api.v1.GetAppSettingsRequest\x1a#.user_api.v1.GetAppSettingsResponse\x12S\n\x0cRegisterUser\x12 .user_api.v1.RegisterUserRequest\x1a!.user_api.v1.RegisterUserResponse\x12P\n\x0bConfirmUser\x12\x1f.user_api.v1.ConfirmUserRequest\x1a .user_api.v1.ConfirmUserResponse\x12P\n\x0bFindByEmail\x12\x1f.user_api.v1.FindByEmailRequest\x1a .user_api.v1.FindByEmailResponse\x12G\n\x08FindByID\x12\x1c.user_api.v1.FindByIDRequest\x1a\x1d.user_api.v1.FindByIDResponse\x12>\n\x05GetMe\x12\x19.user_api.v1.GetMeRequest\x1a\x1a.user_api.v1.GetMeResponse\x12S\n\x0cGetUserByDID\x12 .user_api.v1.GetUserByDIDRequest\x1a!.user_api.v1.GetUserByDIDResponseBBZ@github.com/knox-networks/grpc-sdks/sdk/go/user_management_api/v1b\x06proto3')
_PROVIDER = DESCRIPTOR.enum_types_by_name['Provider']
Provider = enum_type_wrapper.EnumTypeWrapper(_PROVIDER)
_ROLE = DESCRIPTOR.enum_types_by_name['Role']
Role = enum_type_wrapper.EnumTypeWrapper(_ROLE)
PROVIDER_UNSPECIFIED = 0
PROVIDER_GITHUB = 1
PROVIDER_FACEBOOK = 2
PROVIDER_GOOGLE = 3
PROVIDER_SAML = 4
PROVIDER_COGNITO = 5
ROLE_UNSPECIFIED = 0
ROLE_USER = 1
ROLE_ADMIN = 2
_USER = DESCRIPTOR.message_types_by_name['User']
_USER_EXTERNALIDSENTRY = _USER.nested_types_by_name['ExternalIdsEntry']
_REGISTERUSERREQUEST = DESCRIPTOR.message_types_by_name['RegisterUserRequest']
_AUTHTOKEN = DESCRIPTOR.message_types_by_name['AuthToken']
_REGISTERUSERRESPONSE = DESCRIPTOR.message_types_by_name['RegisterUserResponse']
_CONFIRMUSERREQUEST = DESCRIPTOR.message_types_by_name['ConfirmUserRequest']
_CONFIRMUSERRESPONSE = DESCRIPTOR.message_types_by_name['ConfirmUserResponse']
_AUTHNWITHPROVIDERREQUEST = DESCRIPTOR.message_types_by_name['AuthnWithProviderRequest']
_AUTHNWITHPROVIDERRESPONSE = DESCRIPTOR.message_types_by_name['AuthnWithProviderResponse']
_HANDLEOIDCCALLBACKREQUEST = DESCRIPTOR.message_types_by_name['HandleOIDCCallbackRequest']
_HANDLEOIDCCALLBACKRESPONSE = DESCRIPTOR.message_types_by_name['HandleOIDCCallbackResponse']
_FINDBYEMAILREQUEST = DESCRIPTOR.message_types_by_name['FindByEmailRequest']
_FINDBYEMAILRESPONSE = DESCRIPTOR.message_types_by_name['FindByEmailResponse']
_FINDBYIDREQUEST = DESCRIPTOR.message_types_by_name['FindByIDRequest']
_FINDBYIDRESPONSE = DESCRIPTOR.message_types_by_name['FindByIDResponse']
_AUTHNWITHPASSWORDREQUEST = DESCRIPTOR.message_types_by_name['AuthnWithPasswordRequest']
_AUTHNWITHPASSWORDRESPONSE = DESCRIPTOR.message_types_by_name['AuthnWithPasswordResponse']
_REFRESHACCESSTOKENREQUEST = DESCRIPTOR.message_types_by_name['RefreshAccessTokenRequest']
_REFRESHACCESSTOKENRESPONSE = DESCRIPTOR.message_types_by_name['RefreshAccessTokenResponse']
_GETMEREQUEST = DESCRIPTOR.message_types_by_name['GetMeRequest']
_GETMERESPONSE = DESCRIPTOR.message_types_by_name['GetMeResponse']
_AUTHTOKENRESPONSE = DESCRIPTOR.message_types_by_name['AuthTokenResponse']
_CREATEAUTHNBROWSERWITHWALLETCHALLENGEREQUEST = DESCRIPTOR.message_types_by_name['CreateAuthnBrowserWithWalletChallengeRequest']
_CREATEAUTHNBROWSERWITHWALLETCHALLENGERESPONSE = DESCRIPTOR.message_types_by_name['CreateAuthnBrowserWithWalletChallengeResponse']
_AUTHNBROWSERWITHWALLETREQUEST = DESCRIPTOR.message_types_by_name['AuthnBrowserWithWalletRequest']
_AUTHNBROWSERWITHWALLETRESPONSE = DESCRIPTOR.message_types_by_name['AuthnBrowserWithWalletResponse']
_CREATEREGISTERWALLETCHALLENGEREQUEST = DESCRIPTOR.message_types_by_name['CreateRegisterWalletChallengeRequest']
_CREATEREGISTERWALLETCHALLENGERESPONSE = DESCRIPTOR.message_types_by_name['CreateRegisterWalletChallengeResponse']
_REGISTERWALLETREQUEST = DESCRIPTOR.message_types_by_name['RegisterWalletRequest']
_REGISTERWALLETRESPONSE = DESCRIPTOR.message_types_by_name['RegisterWalletResponse']
_CREATEAUTHNWALLETCHALLENGEREQUEST = DESCRIPTOR.message_types_by_name['CreateAuthnWalletChallengeRequest']
_CREATEAUTHNWALLETCHALLENGERESPONSE = DESCRIPTOR.message_types_by_name['CreateAuthnWalletChallengeResponse']
_AUTHNWALLETREQUEST = DESCRIPTOR.message_types_by_name['AuthnWalletRequest']
_AUTHNWALLETRESPONSE = DESCRIPTOR.message_types_by_name['AuthnWalletResponse']
_HANDLESAMLCALLBACKREQUEST = DESCRIPTOR.message_types_by_name['HandleSAMLCallbackRequest']
_HANDLESAMLCALLBACKRESPONSE = DESCRIPTOR.message_types_by_name['HandleSAMLCallbackResponse']
_SAMLSPMETADATAREQUEST = DESCRIPTOR.message_types_by_name['SAMLSPMetadataRequest']
_SAMLSPMETADATARESPONSE = DESCRIPTOR.message_types_by_name['SAMLSPMetadataResponse']
_GETAPPSETTINGSREQUEST = DESCRIPTOR.message_types_by_name['GetAppSettingsRequest']
_APPSETTINGS = DESCRIPTOR.message_types_by_name['AppSettings']
_AUTHNPROVIDER = DESCRIPTOR.message_types_by_name['AuthnProvider']
_GETAPPSETTINGSRESPONSE = DESCRIPTOR.message_types_by_name['GetAppSettingsResponse']
_GETUSERBYDIDREQUEST = DESCRIPTOR.message_types_by_name['GetUserByDIDRequest']
_GETUSERBYDIDRESPONSE = DESCRIPTOR.message_types_by_name['GetUserByDIDResponse']
_ASSOCIATEWALLETREQUEST = DESCRIPTOR.message_types_by_name['AssociateWalletRequest']
_ASSOCIATEWALLETRESPONSE = DESCRIPTOR.message_types_by_name['AssociateWalletResponse']
User = _reflection.GeneratedProtocolMessageType('User', (_message.Message,), {'ExternalIdsEntry': _reflection.GeneratedProtocolMessageType('ExternalIdsEntry', (_message.Message,), {'DESCRIPTOR': _USER_EXTERNALIDSENTRY, '__module__': 'user_management_api.v1.user_management_pb2'}), 'DESCRIPTOR': _USER, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(User)
_sym_db.RegisterMessage(User.ExternalIdsEntry)
RegisterUserRequest = _reflection.GeneratedProtocolMessageType('RegisterUserRequest', (_message.Message,), {'DESCRIPTOR': _REGISTERUSERREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(RegisterUserRequest)
AuthToken = _reflection.GeneratedProtocolMessageType('AuthToken', (_message.Message,), {'DESCRIPTOR': _AUTHTOKEN, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AuthToken)
RegisterUserResponse = _reflection.GeneratedProtocolMessageType('RegisterUserResponse', (_message.Message,), {'DESCRIPTOR': _REGISTERUSERRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(RegisterUserResponse)
ConfirmUserRequest = _reflection.GeneratedProtocolMessageType('ConfirmUserRequest', (_message.Message,), {'DESCRIPTOR': _CONFIRMUSERREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(ConfirmUserRequest)
ConfirmUserResponse = _reflection.GeneratedProtocolMessageType('ConfirmUserResponse', (_message.Message,), {'DESCRIPTOR': _CONFIRMUSERRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(ConfirmUserResponse)
AuthnWithProviderRequest = _reflection.GeneratedProtocolMessageType('AuthnWithProviderRequest', (_message.Message,), {'DESCRIPTOR': _AUTHNWITHPROVIDERREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AuthnWithProviderRequest)
AuthnWithProviderResponse = _reflection.GeneratedProtocolMessageType('AuthnWithProviderResponse', (_message.Message,), {'DESCRIPTOR': _AUTHNWITHPROVIDERRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AuthnWithProviderResponse)
HandleOIDCCallbackRequest = _reflection.GeneratedProtocolMessageType('HandleOIDCCallbackRequest', (_message.Message,), {'DESCRIPTOR': _HANDLEOIDCCALLBACKREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(HandleOIDCCallbackRequest)
HandleOIDCCallbackResponse = _reflection.GeneratedProtocolMessageType('HandleOIDCCallbackResponse', (_message.Message,), {'DESCRIPTOR': _HANDLEOIDCCALLBACKRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(HandleOIDCCallbackResponse)
FindByEmailRequest = _reflection.GeneratedProtocolMessageType('FindByEmailRequest', (_message.Message,), {'DESCRIPTOR': _FINDBYEMAILREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(FindByEmailRequest)
FindByEmailResponse = _reflection.GeneratedProtocolMessageType('FindByEmailResponse', (_message.Message,), {'DESCRIPTOR': _FINDBYEMAILRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(FindByEmailResponse)
FindByIDRequest = _reflection.GeneratedProtocolMessageType('FindByIDRequest', (_message.Message,), {'DESCRIPTOR': _FINDBYIDREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(FindByIDRequest)
FindByIDResponse = _reflection.GeneratedProtocolMessageType('FindByIDResponse', (_message.Message,), {'DESCRIPTOR': _FINDBYIDRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(FindByIDResponse)
AuthnWithPasswordRequest = _reflection.GeneratedProtocolMessageType('AuthnWithPasswordRequest', (_message.Message,), {'DESCRIPTOR': _AUTHNWITHPASSWORDREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AuthnWithPasswordRequest)
AuthnWithPasswordResponse = _reflection.GeneratedProtocolMessageType('AuthnWithPasswordResponse', (_message.Message,), {'DESCRIPTOR': _AUTHNWITHPASSWORDRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AuthnWithPasswordResponse)
RefreshAccessTokenRequest = _reflection.GeneratedProtocolMessageType('RefreshAccessTokenRequest', (_message.Message,), {'DESCRIPTOR': _REFRESHACCESSTOKENREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(RefreshAccessTokenRequest)
RefreshAccessTokenResponse = _reflection.GeneratedProtocolMessageType('RefreshAccessTokenResponse', (_message.Message,), {'DESCRIPTOR': _REFRESHACCESSTOKENRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(RefreshAccessTokenResponse)
GetMeRequest = _reflection.GeneratedProtocolMessageType('GetMeRequest', (_message.Message,), {'DESCRIPTOR': _GETMEREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(GetMeRequest)
GetMeResponse = _reflection.GeneratedProtocolMessageType('GetMeResponse', (_message.Message,), {'DESCRIPTOR': _GETMERESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(GetMeResponse)
AuthTokenResponse = _reflection.GeneratedProtocolMessageType('AuthTokenResponse', (_message.Message,), {'DESCRIPTOR': _AUTHTOKENRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AuthTokenResponse)
CreateAuthnBrowserWithWalletChallengeRequest = _reflection.GeneratedProtocolMessageType('CreateAuthnBrowserWithWalletChallengeRequest', (_message.Message,), {'DESCRIPTOR': _CREATEAUTHNBROWSERWITHWALLETCHALLENGEREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(CreateAuthnBrowserWithWalletChallengeRequest)
CreateAuthnBrowserWithWalletChallengeResponse = _reflection.GeneratedProtocolMessageType('CreateAuthnBrowserWithWalletChallengeResponse', (_message.Message,), {'DESCRIPTOR': _CREATEAUTHNBROWSERWITHWALLETCHALLENGERESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(CreateAuthnBrowserWithWalletChallengeResponse)
AuthnBrowserWithWalletRequest = _reflection.GeneratedProtocolMessageType('AuthnBrowserWithWalletRequest', (_message.Message,), {'DESCRIPTOR': _AUTHNBROWSERWITHWALLETREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AuthnBrowserWithWalletRequest)
AuthnBrowserWithWalletResponse = _reflection.GeneratedProtocolMessageType('AuthnBrowserWithWalletResponse', (_message.Message,), {'DESCRIPTOR': _AUTHNBROWSERWITHWALLETRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AuthnBrowserWithWalletResponse)
CreateRegisterWalletChallengeRequest = _reflection.GeneratedProtocolMessageType('CreateRegisterWalletChallengeRequest', (_message.Message,), {'DESCRIPTOR': _CREATEREGISTERWALLETCHALLENGEREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(CreateRegisterWalletChallengeRequest)
CreateRegisterWalletChallengeResponse = _reflection.GeneratedProtocolMessageType('CreateRegisterWalletChallengeResponse', (_message.Message,), {'DESCRIPTOR': _CREATEREGISTERWALLETCHALLENGERESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(CreateRegisterWalletChallengeResponse)
RegisterWalletRequest = _reflection.GeneratedProtocolMessageType('RegisterWalletRequest', (_message.Message,), {'DESCRIPTOR': _REGISTERWALLETREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(RegisterWalletRequest)
RegisterWalletResponse = _reflection.GeneratedProtocolMessageType('RegisterWalletResponse', (_message.Message,), {'DESCRIPTOR': _REGISTERWALLETRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(RegisterWalletResponse)
CreateAuthnWalletChallengeRequest = _reflection.GeneratedProtocolMessageType('CreateAuthnWalletChallengeRequest', (_message.Message,), {'DESCRIPTOR': _CREATEAUTHNWALLETCHALLENGEREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(CreateAuthnWalletChallengeRequest)
CreateAuthnWalletChallengeResponse = _reflection.GeneratedProtocolMessageType('CreateAuthnWalletChallengeResponse', (_message.Message,), {'DESCRIPTOR': _CREATEAUTHNWALLETCHALLENGERESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(CreateAuthnWalletChallengeResponse)
AuthnWalletRequest = _reflection.GeneratedProtocolMessageType('AuthnWalletRequest', (_message.Message,), {'DESCRIPTOR': _AUTHNWALLETREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AuthnWalletRequest)
AuthnWalletResponse = _reflection.GeneratedProtocolMessageType('AuthnWalletResponse', (_message.Message,), {'DESCRIPTOR': _AUTHNWALLETRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AuthnWalletResponse)
HandleSAMLCallbackRequest = _reflection.GeneratedProtocolMessageType('HandleSAMLCallbackRequest', (_message.Message,), {'DESCRIPTOR': _HANDLESAMLCALLBACKREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(HandleSAMLCallbackRequest)
HandleSAMLCallbackResponse = _reflection.GeneratedProtocolMessageType('HandleSAMLCallbackResponse', (_message.Message,), {'DESCRIPTOR': _HANDLESAMLCALLBACKRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(HandleSAMLCallbackResponse)
SAMLSPMetadataRequest = _reflection.GeneratedProtocolMessageType('SAMLSPMetadataRequest', (_message.Message,), {'DESCRIPTOR': _SAMLSPMETADATAREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(SAMLSPMetadataRequest)
SAMLSPMetadataResponse = _reflection.GeneratedProtocolMessageType('SAMLSPMetadataResponse', (_message.Message,), {'DESCRIPTOR': _SAMLSPMETADATARESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(SAMLSPMetadataResponse)
GetAppSettingsRequest = _reflection.GeneratedProtocolMessageType('GetAppSettingsRequest', (_message.Message,), {'DESCRIPTOR': _GETAPPSETTINGSREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(GetAppSettingsRequest)
AppSettings = _reflection.GeneratedProtocolMessageType('AppSettings', (_message.Message,), {'DESCRIPTOR': _APPSETTINGS, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AppSettings)
AuthnProvider = _reflection.GeneratedProtocolMessageType('AuthnProvider', (_message.Message,), {'DESCRIPTOR': _AUTHNPROVIDER, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AuthnProvider)
GetAppSettingsResponse = _reflection.GeneratedProtocolMessageType('GetAppSettingsResponse', (_message.Message,), {'DESCRIPTOR': _GETAPPSETTINGSRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(GetAppSettingsResponse)
GetUserByDIDRequest = _reflection.GeneratedProtocolMessageType('GetUserByDIDRequest', (_message.Message,), {'DESCRIPTOR': _GETUSERBYDIDREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(GetUserByDIDRequest)
GetUserByDIDResponse = _reflection.GeneratedProtocolMessageType('GetUserByDIDResponse', (_message.Message,), {'DESCRIPTOR': _GETUSERBYDIDRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(GetUserByDIDResponse)
AssociateWalletRequest = _reflection.GeneratedProtocolMessageType('AssociateWalletRequest', (_message.Message,), {'DESCRIPTOR': _ASSOCIATEWALLETREQUEST, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AssociateWalletRequest)
AssociateWalletResponse = _reflection.GeneratedProtocolMessageType('AssociateWalletResponse', (_message.Message,), {'DESCRIPTOR': _ASSOCIATEWALLETRESPONSE, '__module__': 'user_management_api.v1.user_management_pb2'})
_sym_db.RegisterMessage(AssociateWalletResponse)
_USERMANAGEMENT = DESCRIPTOR.services_by_name['UserManagement']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z@github.com/knox-networks/grpc-sdks/sdk/go/user_management_api/v1'
    _USER_EXTERNALIDSENTRY._options = None
    _USER_EXTERNALIDSENTRY._serialized_options = b'8\x01'
    _PROVIDER._serialized_start = 3514
    _PROVIDER._serialized_end = 3656
    _ROLE._serialized_start = 3658
    _ROLE._serialized_end = 3717
    _USER._serialized_start = 92
    _USER._serialized_end = 397
    _USER_EXTERNALIDSENTRY._serialized_start = 347
    _USER_EXTERNALIDSENTRY._serialized_end = 397
    _REGISTERUSERREQUEST._serialized_start = 400
    _REGISTERUSERREQUEST._serialized_end = 557
    _AUTHTOKEN._serialized_start = 559
    _AUTHTOKEN._serialized_end = 648
    _REGISTERUSERRESPONSE._serialized_start = 650
    _REGISTERUSERRESPONSE._serialized_end = 705
    _CONFIRMUSERREQUEST._serialized_start = 707
    _CONFIRMUSERREQUEST._serialized_end = 766
    _CONFIRMUSERRESPONSE._serialized_start = 768
    _CONFIRMUSERRESPONSE._serialized_end = 789
    _AUTHNWITHPROVIDERREQUEST._serialized_start = 792
    _AUTHNWITHPROVIDERREQUEST._serialized_end = 953
    _AUTHNWITHPROVIDERRESPONSE._serialized_start = 955
    _AUTHNWITHPROVIDERRESPONSE._serialized_end = 1004
    _HANDLEOIDCCALLBACKREQUEST._serialized_start = 1006
    _HANDLEOIDCCALLBACKREQUEST._serialized_end = 1062
    _HANDLEOIDCCALLBACKRESPONSE._serialized_start = 1065
    _HANDLEOIDCCALLBACKRESPONSE._serialized_end = 1264
    _FINDBYEMAILREQUEST._serialized_start = 1266
    _FINDBYEMAILREQUEST._serialized_end = 1301
    _FINDBYEMAILRESPONSE._serialized_start = 1303
    _FINDBYEMAILRESPONSE._serialized_end = 1357
    _FINDBYIDREQUEST._serialized_start = 1359
    _FINDBYIDREQUEST._serialized_end = 1390
    _FINDBYIDRESPONSE._serialized_start = 1392
    _FINDBYIDRESPONSE._serialized_end = 1443
    _AUTHNWITHPASSWORDREQUEST._serialized_start = 1445
    _AUTHNWITHPASSWORDREQUEST._serialized_end = 1504
    _AUTHNWITHPASSWORDRESPONSE._serialized_start = 1506
    _AUTHNWITHPASSWORDRESPONSE._serialized_end = 1610
    _REFRESHACCESSTOKENREQUEST._serialized_start = 1612
    _REFRESHACCESSTOKENREQUEST._serialized_end = 1662
    _REFRESHACCESSTOKENRESPONSE._serialized_start = 1664
    _REFRESHACCESSTOKENRESPONSE._serialized_end = 1769
    _GETMEREQUEST._serialized_start = 1771
    _GETMEREQUEST._serialized_end = 1785
    _GETMERESPONSE._serialized_start = 1787
    _GETMERESPONSE._serialized_end = 1835
    _AUTHTOKENRESPONSE._serialized_start = 1837
    _AUTHTOKENRESPONSE._serialized_end = 1933
    _CREATEAUTHNBROWSERWITHWALLETCHALLENGEREQUEST._serialized_start = 1935
    _CREATEAUTHNBROWSERWITHWALLETCHALLENGEREQUEST._serialized_end = 1981
    _CREATEAUTHNBROWSERWITHWALLETCHALLENGERESPONSE._serialized_start = 1984
    _CREATEAUTHNBROWSERWITHWALLETCHALLENGERESPONSE._serialized_end = 2115
    _AUTHNBROWSERWITHWALLETREQUEST._serialized_start = 2117
    _AUTHNBROWSERWITHWALLETREQUEST._serialized_end = 2195
    _AUTHNBROWSERWITHWALLETRESPONSE._serialized_start = 2197
    _AUTHNBROWSERWITHWALLETRESPONSE._serialized_end = 2229
    _CREATEREGISTERWALLETCHALLENGEREQUEST._serialized_start = 2231
    _CREATEREGISTERWALLETCHALLENGEREQUEST._serialized_end = 2269
    _CREATEREGISTERWALLETCHALLENGERESPONSE._serialized_start = 2271
    _CREATEREGISTERWALLETCHALLENGERESPONSE._serialized_end = 2363
    _REGISTERWALLETREQUEST._serialized_start = 2365
    _REGISTERWALLETREQUEST._serialized_end = 2435
    _REGISTERWALLETRESPONSE._serialized_start = 2437
    _REGISTERWALLETRESPONSE._serialized_end = 2538
    _CREATEAUTHNWALLETCHALLENGEREQUEST._serialized_start = 2540
    _CREATEAUTHNWALLETCHALLENGEREQUEST._serialized_end = 2588
    _CREATEAUTHNWALLETCHALLENGERESPONSE._serialized_start = 2590
    _CREATEAUTHNWALLETCHALLENGERESPONSE._serialized_end = 2641
    _AUTHNWALLETREQUEST._serialized_start = 2643
    _AUTHNWALLETREQUEST._serialized_end = 2710
    _AUTHNWALLETRESPONSE._serialized_start = 2712
    _AUTHNWALLETRESPONSE._serialized_end = 2810
    _HANDLESAMLCALLBACKREQUEST._serialized_start = 2812
    _HANDLESAMLCALLBACKREQUEST._serialized_end = 2883
    _HANDLESAMLCALLBACKRESPONSE._serialized_start = 2886
    _HANDLESAMLCALLBACKRESPONSE._serialized_end = 3038
    _SAMLSPMETADATAREQUEST._serialized_start = 3040
    _SAMLSPMETADATAREQUEST._serialized_end = 3063
    _SAMLSPMETADATARESPONSE._serialized_start = 3065
    _SAMLSPMETADATARESPONSE._serialized_end = 3110
    _GETAPPSETTINGSREQUEST._serialized_start = 3112
    _GETAPPSETTINGSREQUEST._serialized_end = 3135
    _APPSETTINGS._serialized_start = 3137
    _APPSETTINGS._serialized_end = 3203
    _AUTHNPROVIDER._serialized_start = 3205
    _AUTHNPROVIDER._serialized_end = 3278
    _GETAPPSETTINGSRESPONSE._serialized_start = 3280
    _GETAPPSETTINGSRESPONSE._serialized_end = 3352
    _GETUSERBYDIDREQUEST._serialized_start = 3354
    _GETUSERBYDIDREQUEST._serialized_end = 3388
    _GETUSERBYDIDRESPONSE._serialized_start = 3390
    _GETUSERBYDIDRESPONSE._serialized_end = 3445
    _ASSOCIATEWALLETREQUEST._serialized_start = 3447
    _ASSOCIATEWALLETREQUEST._serialized_end = 3484
    _ASSOCIATEWALLETRESPONSE._serialized_start = 3486
    _ASSOCIATEWALLETRESPONSE._serialized_end = 3511
    _USERMANAGEMENT._serialized_start = 3720
    _USERMANAGEMENT._serialized_end = 5709