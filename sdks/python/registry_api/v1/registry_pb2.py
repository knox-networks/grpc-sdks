# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: registry_api/v1/registry.proto
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
    'registry_api/v1/registry.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2
from google.protobuf import struct_pb2 as google_dot_protobuf_dot_struct__pb2
from google.api import annotations_pb2 as google_dot_api_dot_annotations__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1eregistry_api/v1/registry.proto\x12\x0fregistry_api.v1\x1a\x1fgoogle/protobuf/timestamp.proto\x1a\x1cgoogle/protobuf/struct.proto\x1a\x1cgoogle/api/annotations.proto\".\n\rCreateRequest\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\x12\x10\n\x08\x64ocument\x18\x02 \x01(\t\"\x10\n\x0e\x43reateResponse\"Z\n\x0eResolveRequest\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\x12;\n\x10resolutionOption\x18\x02 \x01(\x0b\x32!.registry_api.v1.ResolutionOption\"h\n\x1cResolveRepresentationRequest\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\x12;\n\x10resolutionOption\x18\x02 \x01(\x0b\x32!.registry_api.v1.ResolutionOption\"K\n\x10ResolutionOption\x12\x37\n\x06\x61\x63\x63\x65pt\x18\x01 \x01(\x0e\x32\'.registry_api.v1.ResolutionOptionAccept\"o\n\x13\x44idDocumentMetadata\x12+\n\x07\x63reated\x18\x01 \x01(\x0b\x32\x1a.google.protobuf.Timestamp\x12+\n\x07updated\x18\x02 \x01(\x0b\x32\x1a.google.protobuf.Timestamp\"\xc6\x01\n\x0fResolveResponse\x12\x42\n\x15\x64idResolutionMetadata\x18\x01 \x01(\x0b\x32#.registry_api.v1.ResolutionMetadata\x12,\n\x0b\x64idDocument\x18\x02 \x01(\x0b\x32\x17.google.protobuf.Struct\x12\x41\n\x13\x64idDocumentMetadata\x18\x03 \x01(\x0b\x32$.registry_api.v1.DidDocumentMetadata\"\xc1\x01\n\x1dResolveRepresentationResponse\x12\x19\n\x11\x64idDocumentStream\x18\x01 \x01(\t\x12\x42\n\x15\x64idResolutionMetadata\x18\x02 \x01(\x0b\x32#.registry_api.v1.ResolutionMetadata\x12\x41\n\x13\x64idDocumentMetadata\x18\x03 \x01(\x0b\x32$.registry_api.v1.DidDocumentMetadata\"\xcb\x01\n\x12ResolutionMetadata\x12\x18\n\x0b\x63ontentType\x18\x01 \x01(\tH\x00\x88\x01\x01\x12\x15\n\x08\x64uration\x18\x02 \x01(\x03H\x01\x88\x01\x01\x12>\n\x06\x64idUrl\x18\x03 \x01(\x0b\x32).registry_api.v1.ResolutionMetadataDidUrlH\x02\x88\x01\x01\x12\x12\n\x05\x65rror\x18\x04 \x01(\tH\x03\x88\x01\x01\x42\x0e\n\x0c_contentTypeB\x0b\n\t_durationB\t\n\x07_didUrlB\x08\n\x06_error\"U\n\x18ResolutionMetadataDidUrl\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\x12\x12\n\nmethodName\x18\x02 \x01(\t\x12\x18\n\x10methodSpecificId\x18\x03 \x01(\t\".\n\rUpdateRequest\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\x12\x10\n\x08\x64ocument\x18\x02 \x01(\t\"\x10\n\x0eUpdateResponse\".\n\rRevokeRequest\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\x12\x10\n\x08\x64ocument\x18\x02 \x01(\t\"\x10\n\x0eRevokeResponse\"b\n\rPostalAddress\x12\x0f\n\x07\x63ountry\x18\x01 \x01(\t\x12\x0c\n\x04\x63ity\x18\x02 \x01(\t\x12\r\n\x05state\x18\x03 \x01(\t\x12\x13\n\x0bpostal_code\x18\x04 \x01(\t\x12\x0e\n\x06street\x18\x05 \x01(\t\"\xed\x01\n\x10\x43redentialIssuer\x12\x12\n\nissuer_did\x18\x01 \x01(\t\x12\x13\n\x0bissuer_name\x18\x02 \x01(\t\x12\r\n\x05\x65mail\x18\x03 \x01(\t\x12\x0f\n\x07website\x18\x04 \x01(\t\x12\x36\n\x0epostal_address\x18\x05 \x01(\x0b\x32\x1e.registry_api.v1.PostalAddress\x12-\n\x06status\x18\x06 \x01(\x0e\x32\x1d.registry_api.v1.IssuerStatus\x12)\n\x08metadata\x18\x07 \x01(\x0b\x32\x17.google.protobuf.Struct\"]\n\x1d\x43reateCredentialIssuerRequest\x12<\n\x11\x63redential_issuer\x18\x01 \x01(\x0b\x32!.registry_api.v1.CredentialIssuer\",\n\x1e\x43reateCredentialIssuerResponse\x12\n\n\x02ok\x18\x01 \x01(\x08\"0\n\x1aGetCredentialIssuerRequest\x12\x12\n\nissuer_did\x18\x01 \x01(\t\"[\n\x1bGetCredentialIssuerResponse\x12<\n\x11\x63redential_issuer\x18\x01 \x01(\x0b\x32!.registry_api.v1.CredentialIssuer\"]\n\x1dUpdateCredentialIssuerRequest\x12<\n\x11\x63redential_issuer\x18\x01 \x01(\x0b\x32!.registry_api.v1.CredentialIssuer\",\n\x1eUpdateCredentialIssuerResponse\x12\n\n\x02ok\x18\x01 \x01(\x08\"\xe5\x01\n\x0fStatusListEntry\x12\x15\n\rcredential_id\x18\x01 \x01(\t\x12\x1c\n\x14\x63redential_issuer_id\x18\x02 \x01(\t\x12\x36\n\x0estatus_purpose\x18\x03 \x01(\x0e\x32\x1e.registry_api.v1.StatusPurpose\x12\x17\n\x0f\x63redential_type\x18\x04 \x01(\t\x12\x19\n\x11status_list_index\x18\x05 \x01(\x03\x12!\n\x19status_list_credential_id\x18\x06 \x01(\t\x12\x0e\n\x06status\x18\x07 \x01(\x08\"\xb4\x01\n\x1c\x43reateStatusListEntryRequest\x12\x15\n\rcredential_id\x18\x01 \x01(\t\x12\x1c\n\x14\x63redential_issuer_id\x18\x02 \x01(\t\x12\x36\n\x0estatus_purpose\x18\x03 \x01(\x0e\x32\x1e.registry_api.v1.StatusPurpose\x12\x17\n\x0f\x63redential_type\x18\x04 \x01(\t\x12\x0e\n\x06status\x18\x05 \x01(\x08\"\\\n\x1d\x43reateStatusListEntryResponse\x12;\n\x11status_list_entry\x18\x01 \x01(\x0b\x32 .registry_api.v1.StatusListEntry\"C\n\x1eGetStatusListCredentialRequest\x12!\n\x19status_list_credential_id\x18\x01 \x01(\t\"A\n\x1fGetStatusListCredentialResponse\x12\x1e\n\x16status_list_credential\x18\x01 \x01(\t\"\xb4\x01\n\x1cUpdateStatusListEntryRequest\x12\x15\n\rcredential_id\x18\x01 \x01(\t\x12\x1c\n\x14\x63redential_issuer_id\x18\x02 \x01(\t\x12\x36\n\x0estatus_purpose\x18\x03 \x01(\x0e\x32\x1e.registry_api.v1.StatusPurpose\x12\x17\n\x0f\x63redential_type\x18\x04 \x01(\t\x12\x0e\n\x06status\x18\x05 \x01(\x08\"\x1f\n\x1dUpdateStatusListEntryResponse*h\n\x16ResolutionOptionAccept\x12(\n$RESOLUTION_OPTION_ACCEPT_UNSPECIFIED\x10\x00\x12$\n RESOLUTION_OPTION_ACCEPT_JSON_LD\x10\x01*d\n\x0cIssuerStatus\x12\x1d\n\x19ISSUER_STATUS_UNSPECIFIED\x10\x00\x12\x18\n\x14ISSUER_STATUS_ACTIVE\x10\x01\x12\x1b\n\x17ISSUER_STATUS_SUSPENDED\x10\x02*m\n\rStatusPurpose\x12\x1e\n\x1aSTATUS_PURPOSE_UNSPECIFIED\x10\x00\x12\x1d\n\x19STATUS_PURPOSE_REVOCATION\x10\x01\x12\x1d\n\x19STATUS_PURPOSE_SUSPENSION\x10\x02\x32\xb7\x04\n\x0fRegistryService\x12]\n\x06\x43reate\x12\x1e.registry_api.v1.CreateRequest\x1a\x1f.registry_api.v1.CreateResponse\"\x12\x82\xd3\xe4\x93\x02\x0c\"\x07/v1/did:\x01*\x12\x62\n\x07Resolve\x12\x1f.registry_api.v1.ResolveRequest\x1a .registry_api.v1.ResolveResponse\"\x14\x82\xd3\xe4\x93\x02\x0e\x12\x0c/v1/did/{id}\x12\x9b\x01\n\x15ResolveRepresentation\x12-.registry_api.v1.ResolveRepresentationRequest\x1a..registry_api.v1.ResolveRepresentationResponse\"#\x82\xd3\xe4\x93\x02\x1d\x12\x1b/v1/did/{id}/representation\x12\x62\n\x06Update\x12\x1e.registry_api.v1.UpdateRequest\x1a\x1f.registry_api.v1.UpdateResponse\"\x17\x82\xd3\xe4\x93\x02\x11\x1a\x0c/v1/did/{id}:\x01*\x12_\n\x06Revoke\x12\x1e.registry_api.v1.RevokeRequest\x1a\x1f.registry_api.v1.RevokeResponse\"\x14\x82\xd3\xe4\x93\x02\x0e*\x0c/v1/did/{id}2\xfc\x03\n\x1f\x43redentialIssuerRegistryService\x12\x9b\x01\n\x16\x43reateCredentialIssuer\x12..registry_api.v1.CreateCredentialIssuerRequest\x1a/.registry_api.v1.CreateCredentialIssuerResponse\" \x82\xd3\xe4\x93\x02\x1a\"\x15/v1/credential-issuer:\x01*\x12\x9c\x01\n\x13GetCredentialIssuer\x12+.registry_api.v1.GetCredentialIssuerRequest\x1a,.registry_api.v1.GetCredentialIssuerResponse\"*\x82\xd3\xe4\x93\x02$\x12\"/v1/credential-issuer/{issuer_did}\x12\x9b\x01\n\x16UpdateCredentialIssuer\x12..registry_api.v1.UpdateCredentialIssuerRequest\x1a/.registry_api.v1.UpdateCredentialIssuerResponse\" \x82\xd3\xe4\x93\x02\x1a\x1a\x15/v1/credential-issuer:\x01*2\x94\x04\n\x19StatusListRegistryService\x12\x92\x01\n\x15\x43reateStatusListEntry\x12-.registry_api.v1.CreateStatusListEntryRequest\x1a..registry_api.v1.CreateStatusListEntryResponse\"\x1a\x82\xd3\xe4\x93\x02\x14\"\x0f/v1/status-list:\x01*\x12\xbc\x01\n\x17GetStatusListCredential\x12/.registry_api.v1.GetStatusListCredentialRequest\x1a\x30.registry_api.v1.GetStatusListCredentialResponse\">\x82\xd3\xe4\x93\x02\x38\x12\x36/v1/status-list/credential/{status_list_credential_id}\x12\xa2\x01\n\x15UpdateStatusListEntry\x12-.registry_api.v1.UpdateStatusListEntryRequest\x1a..registry_api.v1.UpdateStatusListEntryResponse\"*\x82\xd3\xe4\x93\x02$\x1a\x1f/v1/status-list/{credential_id}:\x01*BFZDgithub.com/knox-networks/gomono/protos/registry-mgmt/registry_api/v1b\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'registry_api.v1.registry_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'ZDgithub.com/knox-networks/gomono/protos/registry-mgmt/registry_api/v1'
  _globals['_REGISTRYSERVICE'].methods_by_name['Create']._loaded_options = None
  _globals['_REGISTRYSERVICE'].methods_by_name['Create']._serialized_options = b'\202\323\344\223\002\014\"\007/v1/did:\001*'
  _globals['_REGISTRYSERVICE'].methods_by_name['Resolve']._loaded_options = None
  _globals['_REGISTRYSERVICE'].methods_by_name['Resolve']._serialized_options = b'\202\323\344\223\002\016\022\014/v1/did/{id}'
  _globals['_REGISTRYSERVICE'].methods_by_name['ResolveRepresentation']._loaded_options = None
  _globals['_REGISTRYSERVICE'].methods_by_name['ResolveRepresentation']._serialized_options = b'\202\323\344\223\002\035\022\033/v1/did/{id}/representation'
  _globals['_REGISTRYSERVICE'].methods_by_name['Update']._loaded_options = None
  _globals['_REGISTRYSERVICE'].methods_by_name['Update']._serialized_options = b'\202\323\344\223\002\021\032\014/v1/did/{id}:\001*'
  _globals['_REGISTRYSERVICE'].methods_by_name['Revoke']._loaded_options = None
  _globals['_REGISTRYSERVICE'].methods_by_name['Revoke']._serialized_options = b'\202\323\344\223\002\016*\014/v1/did/{id}'
  _globals['_CREDENTIALISSUERREGISTRYSERVICE'].methods_by_name['CreateCredentialIssuer']._loaded_options = None
  _globals['_CREDENTIALISSUERREGISTRYSERVICE'].methods_by_name['CreateCredentialIssuer']._serialized_options = b'\202\323\344\223\002\032\"\025/v1/credential-issuer:\001*'
  _globals['_CREDENTIALISSUERREGISTRYSERVICE'].methods_by_name['GetCredentialIssuer']._loaded_options = None
  _globals['_CREDENTIALISSUERREGISTRYSERVICE'].methods_by_name['GetCredentialIssuer']._serialized_options = b'\202\323\344\223\002$\022\"/v1/credential-issuer/{issuer_did}'
  _globals['_CREDENTIALISSUERREGISTRYSERVICE'].methods_by_name['UpdateCredentialIssuer']._loaded_options = None
  _globals['_CREDENTIALISSUERREGISTRYSERVICE'].methods_by_name['UpdateCredentialIssuer']._serialized_options = b'\202\323\344\223\002\032\032\025/v1/credential-issuer:\001*'
  _globals['_STATUSLISTREGISTRYSERVICE'].methods_by_name['CreateStatusListEntry']._loaded_options = None
  _globals['_STATUSLISTREGISTRYSERVICE'].methods_by_name['CreateStatusListEntry']._serialized_options = b'\202\323\344\223\002\024\"\017/v1/status-list:\001*'
  _globals['_STATUSLISTREGISTRYSERVICE'].methods_by_name['GetStatusListCredential']._loaded_options = None
  _globals['_STATUSLISTREGISTRYSERVICE'].methods_by_name['GetStatusListCredential']._serialized_options = b'\202\323\344\223\0028\0226/v1/status-list/credential/{status_list_credential_id}'
  _globals['_STATUSLISTREGISTRYSERVICE'].methods_by_name['UpdateStatusListEntry']._loaded_options = None
  _globals['_STATUSLISTREGISTRYSERVICE'].methods_by_name['UpdateStatusListEntry']._serialized_options = b'\202\323\344\223\002$\032\037/v1/status-list/{credential_id}:\001*'
  _globals['_RESOLUTIONOPTIONACCEPT']._serialized_start=3046
  _globals['_RESOLUTIONOPTIONACCEPT']._serialized_end=3150
  _globals['_ISSUERSTATUS']._serialized_start=3152
  _globals['_ISSUERSTATUS']._serialized_end=3252
  _globals['_STATUSPURPOSE']._serialized_start=3254
  _globals['_STATUSPURPOSE']._serialized_end=3363
  _globals['_CREATEREQUEST']._serialized_start=144
  _globals['_CREATEREQUEST']._serialized_end=190
  _globals['_CREATERESPONSE']._serialized_start=192
  _globals['_CREATERESPONSE']._serialized_end=208
  _globals['_RESOLVEREQUEST']._serialized_start=210
  _globals['_RESOLVEREQUEST']._serialized_end=300
  _globals['_RESOLVEREPRESENTATIONREQUEST']._serialized_start=302
  _globals['_RESOLVEREPRESENTATIONREQUEST']._serialized_end=406
  _globals['_RESOLUTIONOPTION']._serialized_start=408
  _globals['_RESOLUTIONOPTION']._serialized_end=483
  _globals['_DIDDOCUMENTMETADATA']._serialized_start=485
  _globals['_DIDDOCUMENTMETADATA']._serialized_end=596
  _globals['_RESOLVERESPONSE']._serialized_start=599
  _globals['_RESOLVERESPONSE']._serialized_end=797
  _globals['_RESOLVEREPRESENTATIONRESPONSE']._serialized_start=800
  _globals['_RESOLVEREPRESENTATIONRESPONSE']._serialized_end=993
  _globals['_RESOLUTIONMETADATA']._serialized_start=996
  _globals['_RESOLUTIONMETADATA']._serialized_end=1199
  _globals['_RESOLUTIONMETADATADIDURL']._serialized_start=1201
  _globals['_RESOLUTIONMETADATADIDURL']._serialized_end=1286
  _globals['_UPDATEREQUEST']._serialized_start=1288
  _globals['_UPDATEREQUEST']._serialized_end=1334
  _globals['_UPDATERESPONSE']._serialized_start=1336
  _globals['_UPDATERESPONSE']._serialized_end=1352
  _globals['_REVOKEREQUEST']._serialized_start=1354
  _globals['_REVOKEREQUEST']._serialized_end=1400
  _globals['_REVOKERESPONSE']._serialized_start=1402
  _globals['_REVOKERESPONSE']._serialized_end=1418
  _globals['_POSTALADDRESS']._serialized_start=1420
  _globals['_POSTALADDRESS']._serialized_end=1518
  _globals['_CREDENTIALISSUER']._serialized_start=1521
  _globals['_CREDENTIALISSUER']._serialized_end=1758
  _globals['_CREATECREDENTIALISSUERREQUEST']._serialized_start=1760
  _globals['_CREATECREDENTIALISSUERREQUEST']._serialized_end=1853
  _globals['_CREATECREDENTIALISSUERRESPONSE']._serialized_start=1855
  _globals['_CREATECREDENTIALISSUERRESPONSE']._serialized_end=1899
  _globals['_GETCREDENTIALISSUERREQUEST']._serialized_start=1901
  _globals['_GETCREDENTIALISSUERREQUEST']._serialized_end=1949
  _globals['_GETCREDENTIALISSUERRESPONSE']._serialized_start=1951
  _globals['_GETCREDENTIALISSUERRESPONSE']._serialized_end=2042
  _globals['_UPDATECREDENTIALISSUERREQUEST']._serialized_start=2044
  _globals['_UPDATECREDENTIALISSUERREQUEST']._serialized_end=2137
  _globals['_UPDATECREDENTIALISSUERRESPONSE']._serialized_start=2139
  _globals['_UPDATECREDENTIALISSUERRESPONSE']._serialized_end=2183
  _globals['_STATUSLISTENTRY']._serialized_start=2186
  _globals['_STATUSLISTENTRY']._serialized_end=2415
  _globals['_CREATESTATUSLISTENTRYREQUEST']._serialized_start=2418
  _globals['_CREATESTATUSLISTENTRYREQUEST']._serialized_end=2598
  _globals['_CREATESTATUSLISTENTRYRESPONSE']._serialized_start=2600
  _globals['_CREATESTATUSLISTENTRYRESPONSE']._serialized_end=2692
  _globals['_GETSTATUSLISTCREDENTIALREQUEST']._serialized_start=2694
  _globals['_GETSTATUSLISTCREDENTIALREQUEST']._serialized_end=2761
  _globals['_GETSTATUSLISTCREDENTIALRESPONSE']._serialized_start=2763
  _globals['_GETSTATUSLISTCREDENTIALRESPONSE']._serialized_end=2828
  _globals['_UPDATESTATUSLISTENTRYREQUEST']._serialized_start=2831
  _globals['_UPDATESTATUSLISTENTRYREQUEST']._serialized_end=3011
  _globals['_UPDATESTATUSLISTENTRYRESPONSE']._serialized_start=3013
  _globals['_UPDATESTATUSLISTENTRYRESPONSE']._serialized_end=3044
  _globals['_REGISTRYSERVICE']._serialized_start=3366
  _globals['_REGISTRYSERVICE']._serialized_end=3933
  _globals['_CREDENTIALISSUERREGISTRYSERVICE']._serialized_start=3936
  _globals['_CREDENTIALISSUERREGISTRYSERVICE']._serialized_end=4444
  _globals['_STATUSLISTREGISTRYSERVICE']._serialized_start=4447
  _globals['_STATUSLISTREGISTRYSERVICE']._serialized_end=4979
# @@protoc_insertion_point(module_scope)