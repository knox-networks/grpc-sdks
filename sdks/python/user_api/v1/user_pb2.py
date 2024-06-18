# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: user_api/v1/user.proto
# Protobuf Python Version: 5.27.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    27,
    1,
    '',
    'user_api/v1/user.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import struct_pb2 as google_dot_protobuf_dot_struct__pb2
from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2
from common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
from google.api import annotations_pb2 as google_dot_api_dot_annotations__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x16user_api/v1/user.proto\x12\x0buser_api.v1\x1a\x1cgoogle/protobuf/struct.proto\x1a\x1fgoogle/protobuf/timestamp.proto\x1a\x16\x63ommon/v1/common.proto\x1a\x1cgoogle/api/annotations.proto\"\xc2\x02\n\x04User\x12\x0c\n\x04uuid\x18\x01 \x01(\t\x12\x12\n\nfirst_name\x18\x02 \x01(\t\x12\x11\n\tlast_name\x18\x03 \x01(\t\x12\r\n\x05\x65mail\x18\x06 \x01(\t\x12\x1f\n\x04role\x18\x07 \x01(\x0e\x32\x11.user_api.v1.Role\x12\x0e\n\x06\x61vatar\x18\x08 \x01(\t\x12\r\n\x05phone\x18\t \x01(\t\x12\x0c\n\x04\x64ids\x18\n \x03(\t\x12\x38\n\x0c\x65xternal_ids\x18\x0b \x03(\x0b\x32\".user_api.v1.User.ExternalIdsEntry\x12)\n\x08metadata\x18\x0c \x01(\x0b\x32\x17.google.protobuf.Struct\x12\x0f\n\x07\x61\x63\x63ount\x18\r \x01(\t\x1a\x32\n\x10\x45xternalIdsEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\t:\x02\x38\x01\"\xae\x01\n\x13RegisterUserRequest\x12\r\n\x05\x65mail\x18\x01 \x01(\t\x12\x12\n\nfirst_name\x18\x02 \x01(\t\x12\x11\n\tlast_name\x18\x03 \x01(\t\x12\x10\n\x08password\x18\x05 \x01(\t\x12\x1f\n\x04role\x18\x06 \x01(\x0e\x32\x11.user_api.v1.Role\x12\x0e\n\x06\x61vatar\x18\x07 \x01(\t\x12\r\n\x05phone\x18\x08 \x01(\t\x12\x0f\n\x07\x61\x63\x63ount\x18\t \x01(\t\"Y\n\tAuthToken\x12\r\n\x05token\x18\x01 \x01(\t\x12\x12\n\ntoken_type\x18\x02 \x01(\t\x12\x12\n\nexpires_in\x18\x03 \x01(\x05\x12\x15\n\rrefresh_token\x18\x04 \x01(\t\"7\n\x14RegisterUserResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\";\n\x12\x43onfirmUserRequest\x12\x16\n\x0e\x65mail_or_phone\x18\x01 \x01(\t\x12\r\n\x05token\x18\x02 \x01(\t\"\x15\n\x13\x43onfirmUserResponse\"\xa1\x01\n\x18\x41uthnWithProviderRequest\x12\'\n\x08provider\x18\x01 \x01(\x0e\x32\x15.user_api.v1.Provider\x12\x15\n\rinstance_name\x18\x02 \x01(\t\x12\x16\n\x0erequest_origin\x18\x03 \x01(\t\x12-\n\x0c\x63lient_state\x18\x04 \x01(\x0b\x32\x17.google.protobuf.Struct\"1\n\x19\x41uthnWithProviderResponse\x12\x14\n\x0cprovider_url\x18\x01 \x01(\t\"8\n\x19HandleOIDCCallbackRequest\x12\x0c\n\x04\x63ode\x18\x01 \x01(\t\x12\r\n\x05state\x18\x02 \x01(\t\"\xc7\x01\n\x1aHandleOIDCCallbackResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b\x32\x16.user_api.v1.AuthToken\x12\x15\n\rinstance_name\x18\x03 \x01(\t\x12\x16\n\x0erequest_origin\x18\x04 \x01(\t\x12-\n\x0c\x63lient_state\x18\x05 \x01(\x0b\x32\x17.google.protobuf.Struct\"#\n\x12\x46indByEmailRequest\x12\r\n\x05\x65mail\x18\x01 \x01(\t\"6\n\x13\x46indByEmailResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\"\x1f\n\x0f\x46indByIDRequest\x12\x0c\n\x04uuid\x18\x01 \x01(\t\"3\n\x10\x46indByIDResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\";\n\x18\x41uthnWithPasswordRequest\x12\r\n\x05\x65mail\x18\x01 \x01(\t\x12\x10\n\x08password\x18\x02 \x01(\t\"h\n\x19\x41uthnWithPasswordResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b\x32\x16.user_api.v1.AuthToken\"2\n\x19RefreshAccessTokenRequest\x12\x15\n\rrefresh_token\x18\x01 \x01(\t\"i\n\x1aRefreshAccessTokenResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b\x32\x16.user_api.v1.AuthToken\"\x0e\n\x0cGetMeRequest\"0\n\rGetMeResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\"`\n\x11\x41uthTokenResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b\x32\x16.user_api.v1.AuthToken\".\n,CreateAuthnBrowserWithWalletChallengeRequest\"\x83\x01\n-CreateAuthnBrowserWithWalletChallengeResponse\x12\x0f\n\x05nonce\x18\x01 \x01(\tH\x00\x12\x34\n\nauth_token\x18\x02 \x01(\x0b\x32\x1e.user_api.v1.AuthTokenResponseH\x00\x42\x0b\n\tdid_start\"N\n\x1d\x41uthnBrowserWithWalletRequest\x12\x11\n\tsignature\x18\x01 \x01(\x0c\x12\r\n\x05nonce\x18\x02 \x01(\t\x12\x0b\n\x03\x64id\x18\x03 \x01(\t\" \n\x1e\x41uthnBrowserWithWalletResponse\"&\n$CreateRegisterWalletChallengeRequest\"\\\n%CreateRegisterWalletChallengeResponse\x12\x0f\n\x05nonce\x18\x01 \x01(\tH\x00\x12\x0c\n\x02ok\x18\x02 \x01(\tH\x00\x42\x14\n\x12registration_start\"\x94\x01\n\x15RegisterWalletRequest\x12\x11\n\tsignature\x18\x01 \x01(\x0c\x12\r\n\x05nonce\x18\x02 \x01(\t\x12\x0b\n\x03\x64id\x18\x03 \x01(\t\x12\x10\n\x08\x64id_type\x18\x04 \x01(\t\x12\x0f\n\x07purpose\x18\x05 \x01(\t\x12)\n\x08metadata\x18\x0c \x01(\x0b\x32\x17.google.protobuf.Struct\"e\n\x16RegisterWalletResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b\x32\x16.user_api.v1.AuthToken\"0\n!CreateAuthnWalletChallengeRequest\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\"3\n\"CreateAuthnWalletChallengeResponse\x12\r\n\x05nonce\x18\x01 \x01(\t\"C\n\x12\x41uthnWalletRequest\x12\x11\n\tsignature\x18\x01 \x01(\x0c\x12\r\n\x05nonce\x18\x02 \x01(\t\x12\x0b\n\x03\x64id\x18\x03 \x01(\t\"b\n\x13\x41uthnWalletResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b\x32\x16.user_api.v1.AuthToken\"G\n\x19HandleSAMLCallbackRequest\x12\x15\n\rsaml_response\x18\x01 \x01(\t\x12\x13\n\x0brelay_state\x18\x02 \x01(\t\"\x98\x01\n\x1aHandleSAMLCallbackResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\x12*\n\nauth_token\x18\x02 \x01(\x0b\x32\x16.user_api.v1.AuthToken\x12\x15\n\rinstance_name\x18\x03 \x01(\t\x12\x16\n\x0erequest_origin\x18\x04 \x01(\t\"\x17\n\x15SAMLSPMetadataRequest\"-\n\x16SAMLSPMetadataResponse\x12\x13\n\x0bsp_metadata\x18\x01 \x01(\x0c\"\x17\n\x15GetAppSettingsRequest\"B\n\x0b\x41ppSettings\x12\x33\n\x0f\x61uthn_providers\x18\x01 \x03(\x0b\x32\x1a.user_api.v1.AuthnProvider\"I\n\rAuthnProvider\x12\'\n\x08provider\x18\x01 \x01(\x0e\x32\x15.user_api.v1.Provider\x12\x0f\n\x07\x65nabled\x18\x02 \x01(\x08\"H\n\x16GetAppSettingsResponse\x12.\n\x0c\x61pp_settings\x18\x01 \x01(\x0b\x32\x18.user_api.v1.AppSettings\"\"\n\x13GetUserByDIDRequest\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\"7\n\x14GetUserByDIDResponse\x12\x1f\n\x04user\x18\x01 \x01(\x0b\x32\x11.user_api.v1.User\"s\n\x16\x41ssociateWalletRequest\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\x12\x10\n\x08\x64id_type\x18\x02 \x01(\t\x12\x0f\n\x07purpose\x18\x03 \x01(\t\x12)\n\x08metadata\x18\x0c \x01(\x0b\x32\x17.google.protobuf.Struct\"\x19\n\x17\x41ssociateWalletResponse\"&\n\x17\x44issociateWalletRequest\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\"\x1a\n\x18\x44issociateWalletResponse\"t\n\x10ListUsersRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x1f\n\x07sort_by\x18\x03 \x01(\x0b\x32\x0e.common.SortBy\x12\x1e\n\x06\x66ilter\x18\x04 \x01(\x0b\x32\x0e.common.Filter\"S\n\x11ListUsersResponse\x12 \n\x05users\x18\x01 \x03(\x0b\x32\x11.user_api.v1.User\x12\x1c\n\x14total_matching_users\x18\x02 \x01(\x05\"\xa3\x01\n\x0c\x41piKeyEntity\x12\x11\n\tclient_id\x18\x01 \x01(\t\x12\x13\n\x0b\x63lient_name\x18\x02 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x03 \x01(\t\x12)\n\x06status\x18\x04 \x01(\x0e\x32\x19.user_api.v1.ApiKeyStatus\x12/\n\x0b\x65xpiry_date\x18\x05 \x01(\x0b\x32\x1a.google.protobuf.Timestamp\"*\n\x13\x43reateApiKeyRequest\x12\x13\n\x0b\x63lient_name\x18\x01 \x01(\t\"I\n\x14\x43reateApiKeyResponse\x12\x31\n\x0e\x61pi_key_entity\x18\x01 \x01(\x0b\x32\x19.user_api.v1.ApiKeyEntity\"r\n\x19UpdateApiKeyStatusRequest\x12\x11\n\tclient_id\x18\x01 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x02 \x01(\t\x12\x31\n\x0e\x61pi_key_status\x18\x03 \x01(\x0e\x32\x19.user_api.v1.ApiKeyStatus\"\x1c\n\x1aUpdateApiKeyStatusResponse\"\x13\n\x11GetApiKeysRequest\"I\n\x12GetApiKeysResponse\x12\x33\n\x10\x61pi_key_entities\x18\x01 \x03(\x0b\x32\x19.user_api.v1.ApiKeyEntity\"\x14\n\x12\x43heckApiKeyRequest\"\'\n\x13\x43heckApiKeyResponse\x12\x10\n\x08is_valid\x18\x03 \x01(\x08*\x8e\x01\n\x08Provider\x12\x18\n\x14PROVIDER_UNSPECIFIED\x10\x00\x12\x13\n\x0fPROVIDER_GITHUB\x10\x01\x12\x15\n\x11PROVIDER_FACEBOOK\x10\x02\x12\x13\n\x0fPROVIDER_GOOGLE\x10\x03\x12\x11\n\rPROVIDER_SAML\x10\x04\x12\x14\n\x10PROVIDER_COGNITO\x10\x05*;\n\x04Role\x12\x14\n\x10ROLE_UNSPECIFIED\x10\x00\x12\r\n\tROLE_USER\x10\x01\x12\x0e\n\nROLE_ADMIN\x10\x02*f\n\x0c\x41piKeyStatus\x12\x1e\n\x1a\x41PI_KEY_STATUS_UNSPECIFIED\x10\x00\x12\x19\n\x15\x41PI_KEY_STATUS_ACTIVE\x10\x01\x12\x1b\n\x17\x41PI_KEY_STATUS_INACTIVE\x10\x02\x32\xea\x16\n\x0eUserApiService\x12\x8a\x01\n\x11\x41uthnWithPassword\x12%.user_api.v1.AuthnWithPasswordRequest\x1a&.user_api.v1.AuthnWithPasswordResponse\"&\x82\xd3\xe4\x93\x02 \"\x1b/v1/user-mgmt/auth/password:\x01*\x12\x8a\x01\n\x12RefreshAccessToken\x12&.user_api.v1.RefreshAccessTokenRequest\x1a\'.user_api.v1.RefreshAccessTokenResponse\"#\x82\xd3\xe4\x93\x02\x1d\"\x18/v1/user-mgmt/auth/token:\x01*\x12\x8a\x01\n\x11\x41uthnWithProvider\x12%.user_api.v1.AuthnWithProviderRequest\x1a&.user_api.v1.AuthnWithProviderResponse\"&\x82\xd3\xe4\x93\x02 \"\x1b/v1/user-mgmt/auth/provider:\x01*\x12\x92\x01\n\x12HandleOIDCCallback\x12&.user_api.v1.HandleOIDCCallbackRequest\x1a\'.user_api.v1.HandleOIDCCallbackResponse\"+\x82\xd3\xe4\x93\x02%\" /v1/user-mgmt/auth/oidc/callback:\x01*\x12\x92\x01\n\x12HandleSAMLCallback\x12&.user_api.v1.HandleSAMLCallbackRequest\x1a\'.user_api.v1.HandleSAMLCallbackResponse\"+\x82\xd3\xe4\x93\x02%\" /v1/user-mgmt/auth/saml/callback:\x01*\x12\x83\x01\n\x0eSAMLSPMetadata\x12\".user_api.v1.SAMLSPMetadataRequest\x1a#.user_api.v1.SAMLSPMetadataResponse\"(\x82\xd3\xe4\x93\x02\"\x12 /v1/user-mgmt/auth/saml/metadata\x12\xa0\x01\n%CreateAuthnBrowserWithWalletChallenge\x12\x39.user_api.v1.CreateAuthnBrowserWithWalletChallengeRequest\x1a:.user_api.v1.CreateAuthnBrowserWithWalletChallengeResponse0\x01\x12q\n\x16\x41uthnBrowserWithWallet\x12*.user_api.v1.AuthnBrowserWithWalletRequest\x1a+.user_api.v1.AuthnBrowserWithWalletResponse\x12\xad\x01\n\x1a\x43reateAuthnWalletChallenge\x12..user_api.v1.CreateAuthnWalletChallengeRequest\x1a/.user_api.v1.CreateAuthnWalletChallengeResponse\".\x82\xd3\xe4\x93\x02(\"#/v1/user-mgmt/auth/wallet/challenge:\x01*\x12v\n\x0b\x41uthnWallet\x12\x1f.user_api.v1.AuthnWalletRequest\x1a .user_api.v1.AuthnWalletResponse\"$\x82\xd3\xe4\x93\x02\x1e\"\x19/v1/user-mgmt/auth/wallet:\x01*\x12\x88\x01\n\x1d\x43reateRegisterWalletChallenge\x12\x31.user_api.v1.CreateRegisterWalletChallengeRequest\x1a\x32.user_api.v1.CreateRegisterWalletChallengeResponse0\x01\x12Y\n\x0eRegisterWallet\x12\".user_api.v1.RegisterWalletRequest\x1a#.user_api.v1.RegisterWalletResponse\x12\x8f\x01\n\x0f\x41ssociateWallet\x12#.user_api.v1.AssociateWalletRequest\x1a$.user_api.v1.AssociateWalletResponse\"1\x82\xd3\xe4\x93\x02+\"&/v1/user-mgmt/wallet/{did}/association:\x01*\x12\x92\x01\n\x10\x44issociateWallet\x12$.user_api.v1.DissociateWalletRequest\x1a%.user_api.v1.DissociateWalletResponse\"1\x82\xd3\xe4\x93\x02+*&/v1/user-mgmt/wallet/{did}/association:\x01*\x12}\n\x0eGetAppSettings\x12\".user_api.v1.GetAppSettingsRequest\x1a#.user_api.v1.GetAppSettingsResponse\"\"\x82\xd3\xe4\x93\x02\x1c\x12\x1a/v1/user-mgmt/settings/app\x12y\n\x0cRegisterUser\x12 .user_api.v1.RegisterUserRequest\x1a!.user_api.v1.RegisterUserResponse\"$\x82\xd3\xe4\x93\x02\x1e\"\x19/v1/user-mgmt/admin/users:\x01*\x12\x8f\x01\n\x0b\x43onfirmUser\x12\x1f.user_api.v1.ConfirmUserRequest\x1a .user_api.v1.ConfirmUserResponse\"=\x82\xd3\xe4\x93\x02\x37\"2/v1/user-mgmt/admin/users/{email_or_phone}/confirm:\x01*\x12x\n\x0b\x46indByEmail\x12\x1f.user_api.v1.FindByEmailRequest\x1a .user_api.v1.FindByEmailResponse\"&\x82\xd3\xe4\x93\x02 \x12\x1e/v1/user-mgmt/admin/users/{id}\x12o\n\x08\x46indByID\x12\x1c.user_api.v1.FindByIDRequest\x1a\x1d.user_api.v1.FindByIDResponse\"&\x82\xd3\xe4\x93\x02 \x12\x1e/v1/user-mgmt/admin/users/{id}\x12^\n\x05GetMe\x12\x19.user_api.v1.GetMeRequest\x1a\x1a.user_api.v1.GetMeResponse\"\x1e\x82\xd3\xe4\x93\x02\x18\x12\x16/v1/user-mgmt/users/me\x12v\n\x0cGetUserByDID\x12 .user_api.v1.GetUserByDIDRequest\x1a!.user_api.v1.GetUserByDIDResponse\"!\x82\xd3\xe4\x93\x02\x1b\x12\x19/v1/user-mgmt/users/{did}\x12g\n\tListUsers\x12\x1d.user_api.v1.ListUsersRequest\x1a\x1e.user_api.v1.ListUsersResponse\"\x1b\x82\xd3\xe4\x93\x02\x15\x12\x13/v1/user-mgmt/users2\xf3\x02\n\x14\x41piKeyManagerService\x12S\n\x0c\x43reateApiKey\x12 .user_api.v1.CreateApiKeyRequest\x1a!.user_api.v1.CreateApiKeyResponse\x12\x65\n\x12UpdateApiKeyStatus\x12&.user_api.v1.UpdateApiKeyStatusRequest\x1a\'.user_api.v1.UpdateApiKeyStatusResponse\x12M\n\nGetApiKeys\x12\x1e.user_api.v1.GetApiKeysRequest\x1a\x1f.user_api.v1.GetApiKeysResponse\x12P\n\x0b\x43heckApiKey\x12\x1f.user_api.v1.CheckApiKeyRequest\x1a .user_api.v1.CheckApiKeyResponseB:Z8go.buf.build/grpc/go/knox-networks/user-mgmt/user_api/v1b\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'user_api.v1.user_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'Z8go.buf.build/grpc/go/knox-networks/user-mgmt/user_api/v1'
  _globals['_USER_EXTERNALIDSENTRY']._loaded_options = None
  _globals['_USER_EXTERNALIDSENTRY']._serialized_options = b'8\001'
  _globals['_USERAPISERVICE'].methods_by_name['AuthnWithPassword']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['AuthnWithPassword']._serialized_options = b'\202\323\344\223\002 \"\033/v1/user-mgmt/auth/password:\001*'
  _globals['_USERAPISERVICE'].methods_by_name['RefreshAccessToken']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['RefreshAccessToken']._serialized_options = b'\202\323\344\223\002\035\"\030/v1/user-mgmt/auth/token:\001*'
  _globals['_USERAPISERVICE'].methods_by_name['AuthnWithProvider']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['AuthnWithProvider']._serialized_options = b'\202\323\344\223\002 \"\033/v1/user-mgmt/auth/provider:\001*'
  _globals['_USERAPISERVICE'].methods_by_name['HandleOIDCCallback']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['HandleOIDCCallback']._serialized_options = b'\202\323\344\223\002%\" /v1/user-mgmt/auth/oidc/callback:\001*'
  _globals['_USERAPISERVICE'].methods_by_name['HandleSAMLCallback']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['HandleSAMLCallback']._serialized_options = b'\202\323\344\223\002%\" /v1/user-mgmt/auth/saml/callback:\001*'
  _globals['_USERAPISERVICE'].methods_by_name['SAMLSPMetadata']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['SAMLSPMetadata']._serialized_options = b'\202\323\344\223\002\"\022 /v1/user-mgmt/auth/saml/metadata'
  _globals['_USERAPISERVICE'].methods_by_name['CreateAuthnWalletChallenge']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['CreateAuthnWalletChallenge']._serialized_options = b'\202\323\344\223\002(\"#/v1/user-mgmt/auth/wallet/challenge:\001*'
  _globals['_USERAPISERVICE'].methods_by_name['AuthnWallet']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['AuthnWallet']._serialized_options = b'\202\323\344\223\002\036\"\031/v1/user-mgmt/auth/wallet:\001*'
  _globals['_USERAPISERVICE'].methods_by_name['AssociateWallet']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['AssociateWallet']._serialized_options = b'\202\323\344\223\002+\"&/v1/user-mgmt/wallet/{did}/association:\001*'
  _globals['_USERAPISERVICE'].methods_by_name['DissociateWallet']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['DissociateWallet']._serialized_options = b'\202\323\344\223\002+*&/v1/user-mgmt/wallet/{did}/association:\001*'
  _globals['_USERAPISERVICE'].methods_by_name['GetAppSettings']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['GetAppSettings']._serialized_options = b'\202\323\344\223\002\034\022\032/v1/user-mgmt/settings/app'
  _globals['_USERAPISERVICE'].methods_by_name['RegisterUser']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['RegisterUser']._serialized_options = b'\202\323\344\223\002\036\"\031/v1/user-mgmt/admin/users:\001*'
  _globals['_USERAPISERVICE'].methods_by_name['ConfirmUser']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['ConfirmUser']._serialized_options = b'\202\323\344\223\0027\"2/v1/user-mgmt/admin/users/{email_or_phone}/confirm:\001*'
  _globals['_USERAPISERVICE'].methods_by_name['FindByEmail']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['FindByEmail']._serialized_options = b'\202\323\344\223\002 \022\036/v1/user-mgmt/admin/users/{id}'
  _globals['_USERAPISERVICE'].methods_by_name['FindByID']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['FindByID']._serialized_options = b'\202\323\344\223\002 \022\036/v1/user-mgmt/admin/users/{id}'
  _globals['_USERAPISERVICE'].methods_by_name['GetMe']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['GetMe']._serialized_options = b'\202\323\344\223\002\030\022\026/v1/user-mgmt/users/me'
  _globals['_USERAPISERVICE'].methods_by_name['GetUserByDID']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['GetUserByDID']._serialized_options = b'\202\323\344\223\002\033\022\031/v1/user-mgmt/users/{did}'
  _globals['_USERAPISERVICE'].methods_by_name['ListUsers']._loaded_options = None
  _globals['_USERAPISERVICE'].methods_by_name['ListUsers']._serialized_options = b'\202\323\344\223\002\025\022\023/v1/user-mgmt/users'
  _globals['_PROVIDER']._serialized_start=4631
  _globals['_PROVIDER']._serialized_end=4773
  _globals['_ROLE']._serialized_start=4775
  _globals['_ROLE']._serialized_end=4834
  _globals['_APIKEYSTATUS']._serialized_start=4836
  _globals['_APIKEYSTATUS']._serialized_end=4938
  _globals['_USER']._serialized_start=157
  _globals['_USER']._serialized_end=479
  _globals['_USER_EXTERNALIDSENTRY']._serialized_start=429
  _globals['_USER_EXTERNALIDSENTRY']._serialized_end=479
  _globals['_REGISTERUSERREQUEST']._serialized_start=482
  _globals['_REGISTERUSERREQUEST']._serialized_end=656
  _globals['_AUTHTOKEN']._serialized_start=658
  _globals['_AUTHTOKEN']._serialized_end=747
  _globals['_REGISTERUSERRESPONSE']._serialized_start=749
  _globals['_REGISTERUSERRESPONSE']._serialized_end=804
  _globals['_CONFIRMUSERREQUEST']._serialized_start=806
  _globals['_CONFIRMUSERREQUEST']._serialized_end=865
  _globals['_CONFIRMUSERRESPONSE']._serialized_start=867
  _globals['_CONFIRMUSERRESPONSE']._serialized_end=888
  _globals['_AUTHNWITHPROVIDERREQUEST']._serialized_start=891
  _globals['_AUTHNWITHPROVIDERREQUEST']._serialized_end=1052
  _globals['_AUTHNWITHPROVIDERRESPONSE']._serialized_start=1054
  _globals['_AUTHNWITHPROVIDERRESPONSE']._serialized_end=1103
  _globals['_HANDLEOIDCCALLBACKREQUEST']._serialized_start=1105
  _globals['_HANDLEOIDCCALLBACKREQUEST']._serialized_end=1161
  _globals['_HANDLEOIDCCALLBACKRESPONSE']._serialized_start=1164
  _globals['_HANDLEOIDCCALLBACKRESPONSE']._serialized_end=1363
  _globals['_FINDBYEMAILREQUEST']._serialized_start=1365
  _globals['_FINDBYEMAILREQUEST']._serialized_end=1400
  _globals['_FINDBYEMAILRESPONSE']._serialized_start=1402
  _globals['_FINDBYEMAILRESPONSE']._serialized_end=1456
  _globals['_FINDBYIDREQUEST']._serialized_start=1458
  _globals['_FINDBYIDREQUEST']._serialized_end=1489
  _globals['_FINDBYIDRESPONSE']._serialized_start=1491
  _globals['_FINDBYIDRESPONSE']._serialized_end=1542
  _globals['_AUTHNWITHPASSWORDREQUEST']._serialized_start=1544
  _globals['_AUTHNWITHPASSWORDREQUEST']._serialized_end=1603
  _globals['_AUTHNWITHPASSWORDRESPONSE']._serialized_start=1605
  _globals['_AUTHNWITHPASSWORDRESPONSE']._serialized_end=1709
  _globals['_REFRESHACCESSTOKENREQUEST']._serialized_start=1711
  _globals['_REFRESHACCESSTOKENREQUEST']._serialized_end=1761
  _globals['_REFRESHACCESSTOKENRESPONSE']._serialized_start=1763
  _globals['_REFRESHACCESSTOKENRESPONSE']._serialized_end=1868
  _globals['_GETMEREQUEST']._serialized_start=1870
  _globals['_GETMEREQUEST']._serialized_end=1884
  _globals['_GETMERESPONSE']._serialized_start=1886
  _globals['_GETMERESPONSE']._serialized_end=1934
  _globals['_AUTHTOKENRESPONSE']._serialized_start=1936
  _globals['_AUTHTOKENRESPONSE']._serialized_end=2032
  _globals['_CREATEAUTHNBROWSERWITHWALLETCHALLENGEREQUEST']._serialized_start=2034
  _globals['_CREATEAUTHNBROWSERWITHWALLETCHALLENGEREQUEST']._serialized_end=2080
  _globals['_CREATEAUTHNBROWSERWITHWALLETCHALLENGERESPONSE']._serialized_start=2083
  _globals['_CREATEAUTHNBROWSERWITHWALLETCHALLENGERESPONSE']._serialized_end=2214
  _globals['_AUTHNBROWSERWITHWALLETREQUEST']._serialized_start=2216
  _globals['_AUTHNBROWSERWITHWALLETREQUEST']._serialized_end=2294
  _globals['_AUTHNBROWSERWITHWALLETRESPONSE']._serialized_start=2296
  _globals['_AUTHNBROWSERWITHWALLETRESPONSE']._serialized_end=2328
  _globals['_CREATEREGISTERWALLETCHALLENGEREQUEST']._serialized_start=2330
  _globals['_CREATEREGISTERWALLETCHALLENGEREQUEST']._serialized_end=2368
  _globals['_CREATEREGISTERWALLETCHALLENGERESPONSE']._serialized_start=2370
  _globals['_CREATEREGISTERWALLETCHALLENGERESPONSE']._serialized_end=2462
  _globals['_REGISTERWALLETREQUEST']._serialized_start=2465
  _globals['_REGISTERWALLETREQUEST']._serialized_end=2613
  _globals['_REGISTERWALLETRESPONSE']._serialized_start=2615
  _globals['_REGISTERWALLETRESPONSE']._serialized_end=2716
  _globals['_CREATEAUTHNWALLETCHALLENGEREQUEST']._serialized_start=2718
  _globals['_CREATEAUTHNWALLETCHALLENGEREQUEST']._serialized_end=2766
  _globals['_CREATEAUTHNWALLETCHALLENGERESPONSE']._serialized_start=2768
  _globals['_CREATEAUTHNWALLETCHALLENGERESPONSE']._serialized_end=2819
  _globals['_AUTHNWALLETREQUEST']._serialized_start=2821
  _globals['_AUTHNWALLETREQUEST']._serialized_end=2888
  _globals['_AUTHNWALLETRESPONSE']._serialized_start=2890
  _globals['_AUTHNWALLETRESPONSE']._serialized_end=2988
  _globals['_HANDLESAMLCALLBACKREQUEST']._serialized_start=2990
  _globals['_HANDLESAMLCALLBACKREQUEST']._serialized_end=3061
  _globals['_HANDLESAMLCALLBACKRESPONSE']._serialized_start=3064
  _globals['_HANDLESAMLCALLBACKRESPONSE']._serialized_end=3216
  _globals['_SAMLSPMETADATAREQUEST']._serialized_start=3218
  _globals['_SAMLSPMETADATAREQUEST']._serialized_end=3241
  _globals['_SAMLSPMETADATARESPONSE']._serialized_start=3243
  _globals['_SAMLSPMETADATARESPONSE']._serialized_end=3288
  _globals['_GETAPPSETTINGSREQUEST']._serialized_start=3290
  _globals['_GETAPPSETTINGSREQUEST']._serialized_end=3313
  _globals['_APPSETTINGS']._serialized_start=3315
  _globals['_APPSETTINGS']._serialized_end=3381
  _globals['_AUTHNPROVIDER']._serialized_start=3383
  _globals['_AUTHNPROVIDER']._serialized_end=3456
  _globals['_GETAPPSETTINGSRESPONSE']._serialized_start=3458
  _globals['_GETAPPSETTINGSRESPONSE']._serialized_end=3530
  _globals['_GETUSERBYDIDREQUEST']._serialized_start=3532
  _globals['_GETUSERBYDIDREQUEST']._serialized_end=3566
  _globals['_GETUSERBYDIDRESPONSE']._serialized_start=3568
  _globals['_GETUSERBYDIDRESPONSE']._serialized_end=3623
  _globals['_ASSOCIATEWALLETREQUEST']._serialized_start=3625
  _globals['_ASSOCIATEWALLETREQUEST']._serialized_end=3740
  _globals['_ASSOCIATEWALLETRESPONSE']._serialized_start=3742
  _globals['_ASSOCIATEWALLETRESPONSE']._serialized_end=3767
  _globals['_DISSOCIATEWALLETREQUEST']._serialized_start=3769
  _globals['_DISSOCIATEWALLETREQUEST']._serialized_end=3807
  _globals['_DISSOCIATEWALLETRESPONSE']._serialized_start=3809
  _globals['_DISSOCIATEWALLETRESPONSE']._serialized_end=3835
  _globals['_LISTUSERSREQUEST']._serialized_start=3837
  _globals['_LISTUSERSREQUEST']._serialized_end=3953
  _globals['_LISTUSERSRESPONSE']._serialized_start=3955
  _globals['_LISTUSERSRESPONSE']._serialized_end=4038
  _globals['_APIKEYENTITY']._serialized_start=4041
  _globals['_APIKEYENTITY']._serialized_end=4204
  _globals['_CREATEAPIKEYREQUEST']._serialized_start=4206
  _globals['_CREATEAPIKEYREQUEST']._serialized_end=4248
  _globals['_CREATEAPIKEYRESPONSE']._serialized_start=4250
  _globals['_CREATEAPIKEYRESPONSE']._serialized_end=4323
  _globals['_UPDATEAPIKEYSTATUSREQUEST']._serialized_start=4325
  _globals['_UPDATEAPIKEYSTATUSREQUEST']._serialized_end=4439
  _globals['_UPDATEAPIKEYSTATUSRESPONSE']._serialized_start=4441
  _globals['_UPDATEAPIKEYSTATUSRESPONSE']._serialized_end=4469
  _globals['_GETAPIKEYSREQUEST']._serialized_start=4471
  _globals['_GETAPIKEYSREQUEST']._serialized_end=4490
  _globals['_GETAPIKEYSRESPONSE']._serialized_start=4492
  _globals['_GETAPIKEYSRESPONSE']._serialized_end=4565
  _globals['_CHECKAPIKEYREQUEST']._serialized_start=4567
  _globals['_CHECKAPIKEYREQUEST']._serialized_end=4587
  _globals['_CHECKAPIKEYRESPONSE']._serialized_start=4589
  _globals['_CHECKAPIKEYRESPONSE']._serialized_end=4628
  _globals['_USERAPISERVICE']._serialized_start=4941
  _globals['_USERAPISERVICE']._serialized_end=7863
  _globals['_APIKEYMANAGERSERVICE']._serialized_start=7866
  _globals['_APIKEYMANAGERSERVICE']._serialized_end=8237
# @@protoc_insertion_point(module_scope)
