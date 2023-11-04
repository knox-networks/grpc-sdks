"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from google.protobuf import timestamp_pb2 as google_dot_protobuf_dot_timestamp__pb2
from google.protobuf import struct_pb2 as google_dot_protobuf_dot_struct__pb2
from ...google.api import annotations_pb2 as google_dot_api_dot_annotations__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1eregistry_api/v1/registry.proto\x12\x0fregistry_api.v1\x1a\x1fgoogle/protobuf/timestamp.proto\x1a\x1cgoogle/protobuf/struct.proto\x1a\x1cgoogle/api/annotations.proto".\n\rCreateRequest\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x10\n\x08document\x18\x02 \x01(\t"\x10\n\x0eCreateResponse"Z\n\x0eResolveRequest\x12\x0b\n\x03did\x18\x01 \x01(\t\x12;\n\x10resolutionOption\x18\x02 \x01(\x0b2!.registry_api.v1.ResolutionOption"h\n\x1cResolveRepresentationRequest\x12\x0b\n\x03did\x18\x01 \x01(\t\x12;\n\x10resolutionOption\x18\x02 \x01(\x0b2!.registry_api.v1.ResolutionOption"K\n\x10ResolutionOption\x127\n\x06accept\x18\x01 \x01(\x0e2\'.registry_api.v1.ResolutionOptionAccept"o\n\x13DidDocumentMetadata\x12+\n\x07created\x18\x01 \x01(\x0b2\x1a.google.protobuf.Timestamp\x12+\n\x07updated\x18\x02 \x01(\x0b2\x1a.google.protobuf.Timestamp"\xc6\x01\n\x0fResolveResponse\x12B\n\x15didResolutionMetadata\x18\x01 \x01(\x0b2#.registry_api.v1.ResolutionMetadata\x12,\n\x0bdidDocument\x18\x02 \x01(\x0b2\x17.google.protobuf.Struct\x12A\n\x13didDocumentMetadata\x18\x03 \x01(\x0b2$.registry_api.v1.DidDocumentMetadata"\xc1\x01\n\x1dResolveRepresentationResponse\x12\x19\n\x11didDocumentStream\x18\x01 \x01(\t\x12B\n\x15didResolutionMetadata\x18\x02 \x01(\x0b2#.registry_api.v1.ResolutionMetadata\x12A\n\x13didDocumentMetadata\x18\x03 \x01(\x0b2$.registry_api.v1.DidDocumentMetadata"\xcb\x01\n\x12ResolutionMetadata\x12\x18\n\x0bcontentType\x18\x01 \x01(\tH\x00\x88\x01\x01\x12\x15\n\x08duration\x18\x02 \x01(\x03H\x01\x88\x01\x01\x12>\n\x06didUrl\x18\x03 \x01(\x0b2).registry_api.v1.ResolutionMetadataDidUrlH\x02\x88\x01\x01\x12\x12\n\x05error\x18\x04 \x01(\tH\x03\x88\x01\x01B\x0e\n\x0c_contentTypeB\x0b\n\t_durationB\t\n\x07_didUrlB\x08\n\x06_error"U\n\x18ResolutionMetadataDidUrl\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x12\n\nmethodName\x18\x02 \x01(\t\x12\x18\n\x10methodSpecificId\x18\x03 \x01(\t".\n\rUpdateRequest\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x10\n\x08document\x18\x02 \x01(\t"\x10\n\x0eUpdateResponse".\n\rRevokeRequest\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x10\n\x08document\x18\x02 \x01(\t"\x10\n\x0eRevokeResponse"b\n\rPostalAddress\x12\x0f\n\x07country\x18\x01 \x01(\t\x12\x0c\n\x04city\x18\x02 \x01(\t\x12\r\n\x05state\x18\x03 \x01(\t\x12\x13\n\x0bpostal_code\x18\x04 \x01(\t\x12\x0e\n\x06street\x18\x05 \x01(\t"\xc2\x01\n\x10CredentialIssuer\x12\x12\n\nissuer_did\x18\x01 \x01(\t\x12\x13\n\x0bissuer_name\x18\x02 \x01(\t\x12\r\n\x05email\x18\x03 \x01(\t\x12\x0f\n\x07website\x18\x04 \x01(\t\x126\n\x0epostal_address\x18\x05 \x01(\x0b2\x1e.registry_api.v1.PostalAddress\x12-\n\x06status\x18\x06 \x01(\x0e2\x1d.registry_api.v1.IssuerStatus"]\n\x1dCreateCredentialIssuerRequest\x12<\n\x11credential_issuer\x18\x01 \x01(\x0b2!.registry_api.v1.CredentialIssuer",\n\x1eCreateCredentialIssuerResponse\x12\n\n\x02ok\x18\x01 \x01(\x08"0\n\x1aGetCredentialIssuerRequest\x12\x12\n\nissuer_did\x18\x01 \x01(\t"[\n\x1bGetCredentialIssuerResponse\x12<\n\x11credential_issuer\x18\x01 \x01(\x0b2!.registry_api.v1.CredentialIssuer"]\n\x1dUpdateCredentialIssuerRequest\x12<\n\x11credential_issuer\x18\x01 \x01(\x0b2!.registry_api.v1.CredentialIssuer",\n\x1eUpdateCredentialIssuerResponse\x12\n\n\x02ok\x18\x01 \x01(\x08"\xe5\x01\n\x0fStatusListEntry\x12\x15\n\rcredential_id\x18\x01 \x01(\t\x12\x1c\n\x14credential_issuer_id\x18\x02 \x01(\t\x126\n\x0estatus_purpose\x18\x03 \x01(\x0e2\x1e.registry_api.v1.StatusPurpose\x12\x17\n\x0fcredential_type\x18\x04 \x01(\t\x12\x19\n\x11status_list_index\x18\x05 \x01(\x03\x12!\n\x19status_list_credential_id\x18\x06 \x01(\t\x12\x0e\n\x06status\x18\x07 \x01(\x08"\xb4\x01\n\x1cCreateStatusListEntryRequest\x12\x15\n\rcredential_id\x18\x01 \x01(\t\x12\x1c\n\x14credential_issuer_id\x18\x02 \x01(\t\x126\n\x0estatus_purpose\x18\x03 \x01(\x0e2\x1e.registry_api.v1.StatusPurpose\x12\x17\n\x0fcredential_type\x18\x04 \x01(\t\x12\x0e\n\x06status\x18\x05 \x01(\x08"\\\n\x1dCreateStatusListEntryResponse\x12;\n\x11status_list_entry\x18\x01 \x01(\x0b2 .registry_api.v1.StatusListEntry"C\n\x1eGetStatusListCredentialRequest\x12!\n\x19status_list_credential_id\x18\x01 \x01(\t"A\n\x1fGetStatusListCredentialResponse\x12\x1e\n\x16status_list_credential\x18\x01 \x01(\t"\xb4\x01\n\x1cUpdateStatusListEntryRequest\x12\x15\n\rcredential_id\x18\x01 \x01(\t\x12\x1c\n\x14credential_issuer_id\x18\x02 \x01(\t\x126\n\x0estatus_purpose\x18\x03 \x01(\x0e2\x1e.registry_api.v1.StatusPurpose\x12\x17\n\x0fcredential_type\x18\x04 \x01(\t\x12\x0e\n\x06status\x18\x05 \x01(\x08"\x1f\n\x1dUpdateStatusListEntryResponse*h\n\x16ResolutionOptionAccept\x12(\n$RESOLUTION_OPTION_ACCEPT_UNSPECIFIED\x10\x00\x12$\n RESOLUTION_OPTION_ACCEPT_JSON_LD\x10\x01*d\n\x0cIssuerStatus\x12\x1d\n\x19ISSUER_STATUS_UNSPECIFIED\x10\x00\x12\x18\n\x14ISSUER_STATUS_ACTIVE\x10\x01\x12\x1b\n\x17ISSUER_STATUS_SUSPENDED\x10\x02*m\n\rStatusPurpose\x12\x1e\n\x1aSTATUS_PURPOSE_UNSPECIFIED\x10\x00\x12\x1d\n\x19STATUS_PURPOSE_REVOCATION\x10\x01\x12\x1d\n\x19STATUS_PURPOSE_SUSPENSION\x10\x022\xb7\x04\n\x0fRegistryService\x12]\n\x06Create\x12\x1e.registry_api.v1.CreateRequest\x1a\x1f.registry_api.v1.CreateResponse"\x12\x82\xd3\xe4\x93\x02\x0c"\x07/v1/did:\x01*\x12b\n\x07Resolve\x12\x1f.registry_api.v1.ResolveRequest\x1a .registry_api.v1.ResolveResponse"\x14\x82\xd3\xe4\x93\x02\x0e\x12\x0c/v1/did/{id}\x12\x9b\x01\n\x15ResolveRepresentation\x12-.registry_api.v1.ResolveRepresentationRequest\x1a..registry_api.v1.ResolveRepresentationResponse"#\x82\xd3\xe4\x93\x02\x1d\x12\x1b/v1/did/{id}/representation\x12b\n\x06Update\x12\x1e.registry_api.v1.UpdateRequest\x1a\x1f.registry_api.v1.UpdateResponse"\x17\x82\xd3\xe4\x93\x02\x11\x1a\x0c/v1/did/{id}:\x01*\x12_\n\x06Revoke\x12\x1e.registry_api.v1.RevokeRequest\x1a\x1f.registry_api.v1.RevokeResponse"\x14\x82\xd3\xe4\x93\x02\x0e*\x0c/v1/did/{id}2\xfc\x03\n\x1fCredentialIssuerRegistryService\x12\x9b\x01\n\x16CreateCredentialIssuer\x12..registry_api.v1.CreateCredentialIssuerRequest\x1a/.registry_api.v1.CreateCredentialIssuerResponse" \x82\xd3\xe4\x93\x02\x1a"\x15/v1/credential-issuer:\x01*\x12\x9c\x01\n\x13GetCredentialIssuer\x12+.registry_api.v1.GetCredentialIssuerRequest\x1a,.registry_api.v1.GetCredentialIssuerResponse"*\x82\xd3\xe4\x93\x02$\x12"/v1/credential-issuer/{issuer_did}\x12\x9b\x01\n\x16UpdateCredentialIssuer\x12..registry_api.v1.UpdateCredentialIssuerRequest\x1a/.registry_api.v1.UpdateCredentialIssuerResponse" \x82\xd3\xe4\x93\x02\x1a\x1a\x15/v1/credential-issuer:\x01*2\x94\x04\n\x19StatusListRegistryService\x12\x92\x01\n\x15CreateStatusListEntry\x12-.registry_api.v1.CreateStatusListEntryRequest\x1a..registry_api.v1.CreateStatusListEntryResponse"\x1a\x82\xd3\xe4\x93\x02\x14"\x0f/v1/status-list:\x01*\x12\xbc\x01\n\x17GetStatusListCredential\x12/.registry_api.v1.GetStatusListCredentialRequest\x1a0.registry_api.v1.GetStatusListCredentialResponse">\x82\xd3\xe4\x93\x028\x126/v1/status-list/credential/{status_list_credential_id}\x12\xa2\x01\n\x15UpdateStatusListEntry\x12-.registry_api.v1.UpdateStatusListEntryRequest\x1a..registry_api.v1.UpdateStatusListEntryResponse"*\x82\xd3\xe4\x93\x02$\x1a\x1f/v1/status-list/{credential_id}:\x01*B;Z9github.com/knox-networks/grpc-sdks/sdk/go/registry_api/v1b\x06proto3')
_RESOLUTIONOPTIONACCEPT = DESCRIPTOR.enum_types_by_name['ResolutionOptionAccept']
ResolutionOptionAccept = enum_type_wrapper.EnumTypeWrapper(_RESOLUTIONOPTIONACCEPT)
_ISSUERSTATUS = DESCRIPTOR.enum_types_by_name['IssuerStatus']
IssuerStatus = enum_type_wrapper.EnumTypeWrapper(_ISSUERSTATUS)
_STATUSPURPOSE = DESCRIPTOR.enum_types_by_name['StatusPurpose']
StatusPurpose = enum_type_wrapper.EnumTypeWrapper(_STATUSPURPOSE)
RESOLUTION_OPTION_ACCEPT_UNSPECIFIED = 0
RESOLUTION_OPTION_ACCEPT_JSON_LD = 1
ISSUER_STATUS_UNSPECIFIED = 0
ISSUER_STATUS_ACTIVE = 1
ISSUER_STATUS_SUSPENDED = 2
STATUS_PURPOSE_UNSPECIFIED = 0
STATUS_PURPOSE_REVOCATION = 1
STATUS_PURPOSE_SUSPENSION = 2
_CREATEREQUEST = DESCRIPTOR.message_types_by_name['CreateRequest']
_CREATERESPONSE = DESCRIPTOR.message_types_by_name['CreateResponse']
_RESOLVEREQUEST = DESCRIPTOR.message_types_by_name['ResolveRequest']
_RESOLVEREPRESENTATIONREQUEST = DESCRIPTOR.message_types_by_name['ResolveRepresentationRequest']
_RESOLUTIONOPTION = DESCRIPTOR.message_types_by_name['ResolutionOption']
_DIDDOCUMENTMETADATA = DESCRIPTOR.message_types_by_name['DidDocumentMetadata']
_RESOLVERESPONSE = DESCRIPTOR.message_types_by_name['ResolveResponse']
_RESOLVEREPRESENTATIONRESPONSE = DESCRIPTOR.message_types_by_name['ResolveRepresentationResponse']
_RESOLUTIONMETADATA = DESCRIPTOR.message_types_by_name['ResolutionMetadata']
_RESOLUTIONMETADATADIDURL = DESCRIPTOR.message_types_by_name['ResolutionMetadataDidUrl']
_UPDATEREQUEST = DESCRIPTOR.message_types_by_name['UpdateRequest']
_UPDATERESPONSE = DESCRIPTOR.message_types_by_name['UpdateResponse']
_REVOKEREQUEST = DESCRIPTOR.message_types_by_name['RevokeRequest']
_REVOKERESPONSE = DESCRIPTOR.message_types_by_name['RevokeResponse']
_POSTALADDRESS = DESCRIPTOR.message_types_by_name['PostalAddress']
_CREDENTIALISSUER = DESCRIPTOR.message_types_by_name['CredentialIssuer']
_CREATECREDENTIALISSUERREQUEST = DESCRIPTOR.message_types_by_name['CreateCredentialIssuerRequest']
_CREATECREDENTIALISSUERRESPONSE = DESCRIPTOR.message_types_by_name['CreateCredentialIssuerResponse']
_GETCREDENTIALISSUERREQUEST = DESCRIPTOR.message_types_by_name['GetCredentialIssuerRequest']
_GETCREDENTIALISSUERRESPONSE = DESCRIPTOR.message_types_by_name['GetCredentialIssuerResponse']
_UPDATECREDENTIALISSUERREQUEST = DESCRIPTOR.message_types_by_name['UpdateCredentialIssuerRequest']
_UPDATECREDENTIALISSUERRESPONSE = DESCRIPTOR.message_types_by_name['UpdateCredentialIssuerResponse']
_STATUSLISTENTRY = DESCRIPTOR.message_types_by_name['StatusListEntry']
_CREATESTATUSLISTENTRYREQUEST = DESCRIPTOR.message_types_by_name['CreateStatusListEntryRequest']
_CREATESTATUSLISTENTRYRESPONSE = DESCRIPTOR.message_types_by_name['CreateStatusListEntryResponse']
_GETSTATUSLISTCREDENTIALREQUEST = DESCRIPTOR.message_types_by_name['GetStatusListCredentialRequest']
_GETSTATUSLISTCREDENTIALRESPONSE = DESCRIPTOR.message_types_by_name['GetStatusListCredentialResponse']
_UPDATESTATUSLISTENTRYREQUEST = DESCRIPTOR.message_types_by_name['UpdateStatusListEntryRequest']
_UPDATESTATUSLISTENTRYRESPONSE = DESCRIPTOR.message_types_by_name['UpdateStatusListEntryResponse']
CreateRequest = _reflection.GeneratedProtocolMessageType('CreateRequest', (_message.Message,), {'DESCRIPTOR': _CREATEREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(CreateRequest)
CreateResponse = _reflection.GeneratedProtocolMessageType('CreateResponse', (_message.Message,), {'DESCRIPTOR': _CREATERESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(CreateResponse)
ResolveRequest = _reflection.GeneratedProtocolMessageType('ResolveRequest', (_message.Message,), {'DESCRIPTOR': _RESOLVEREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(ResolveRequest)
ResolveRepresentationRequest = _reflection.GeneratedProtocolMessageType('ResolveRepresentationRequest', (_message.Message,), {'DESCRIPTOR': _RESOLVEREPRESENTATIONREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(ResolveRepresentationRequest)
ResolutionOption = _reflection.GeneratedProtocolMessageType('ResolutionOption', (_message.Message,), {'DESCRIPTOR': _RESOLUTIONOPTION, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(ResolutionOption)
DidDocumentMetadata = _reflection.GeneratedProtocolMessageType('DidDocumentMetadata', (_message.Message,), {'DESCRIPTOR': _DIDDOCUMENTMETADATA, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(DidDocumentMetadata)
ResolveResponse = _reflection.GeneratedProtocolMessageType('ResolveResponse', (_message.Message,), {'DESCRIPTOR': _RESOLVERESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(ResolveResponse)
ResolveRepresentationResponse = _reflection.GeneratedProtocolMessageType('ResolveRepresentationResponse', (_message.Message,), {'DESCRIPTOR': _RESOLVEREPRESENTATIONRESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(ResolveRepresentationResponse)
ResolutionMetadata = _reflection.GeneratedProtocolMessageType('ResolutionMetadata', (_message.Message,), {'DESCRIPTOR': _RESOLUTIONMETADATA, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(ResolutionMetadata)
ResolutionMetadataDidUrl = _reflection.GeneratedProtocolMessageType('ResolutionMetadataDidUrl', (_message.Message,), {'DESCRIPTOR': _RESOLUTIONMETADATADIDURL, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(ResolutionMetadataDidUrl)
UpdateRequest = _reflection.GeneratedProtocolMessageType('UpdateRequest', (_message.Message,), {'DESCRIPTOR': _UPDATEREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(UpdateRequest)
UpdateResponse = _reflection.GeneratedProtocolMessageType('UpdateResponse', (_message.Message,), {'DESCRIPTOR': _UPDATERESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(UpdateResponse)
RevokeRequest = _reflection.GeneratedProtocolMessageType('RevokeRequest', (_message.Message,), {'DESCRIPTOR': _REVOKEREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(RevokeRequest)
RevokeResponse = _reflection.GeneratedProtocolMessageType('RevokeResponse', (_message.Message,), {'DESCRIPTOR': _REVOKERESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(RevokeResponse)
PostalAddress = _reflection.GeneratedProtocolMessageType('PostalAddress', (_message.Message,), {'DESCRIPTOR': _POSTALADDRESS, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(PostalAddress)
CredentialIssuer = _reflection.GeneratedProtocolMessageType('CredentialIssuer', (_message.Message,), {'DESCRIPTOR': _CREDENTIALISSUER, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(CredentialIssuer)
CreateCredentialIssuerRequest = _reflection.GeneratedProtocolMessageType('CreateCredentialIssuerRequest', (_message.Message,), {'DESCRIPTOR': _CREATECREDENTIALISSUERREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(CreateCredentialIssuerRequest)
CreateCredentialIssuerResponse = _reflection.GeneratedProtocolMessageType('CreateCredentialIssuerResponse', (_message.Message,), {'DESCRIPTOR': _CREATECREDENTIALISSUERRESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(CreateCredentialIssuerResponse)
GetCredentialIssuerRequest = _reflection.GeneratedProtocolMessageType('GetCredentialIssuerRequest', (_message.Message,), {'DESCRIPTOR': _GETCREDENTIALISSUERREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(GetCredentialIssuerRequest)
GetCredentialIssuerResponse = _reflection.GeneratedProtocolMessageType('GetCredentialIssuerResponse', (_message.Message,), {'DESCRIPTOR': _GETCREDENTIALISSUERRESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(GetCredentialIssuerResponse)
UpdateCredentialIssuerRequest = _reflection.GeneratedProtocolMessageType('UpdateCredentialIssuerRequest', (_message.Message,), {'DESCRIPTOR': _UPDATECREDENTIALISSUERREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(UpdateCredentialIssuerRequest)
UpdateCredentialIssuerResponse = _reflection.GeneratedProtocolMessageType('UpdateCredentialIssuerResponse', (_message.Message,), {'DESCRIPTOR': _UPDATECREDENTIALISSUERRESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(UpdateCredentialIssuerResponse)
StatusListEntry = _reflection.GeneratedProtocolMessageType('StatusListEntry', (_message.Message,), {'DESCRIPTOR': _STATUSLISTENTRY, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(StatusListEntry)
CreateStatusListEntryRequest = _reflection.GeneratedProtocolMessageType('CreateStatusListEntryRequest', (_message.Message,), {'DESCRIPTOR': _CREATESTATUSLISTENTRYREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(CreateStatusListEntryRequest)
CreateStatusListEntryResponse = _reflection.GeneratedProtocolMessageType('CreateStatusListEntryResponse', (_message.Message,), {'DESCRIPTOR': _CREATESTATUSLISTENTRYRESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(CreateStatusListEntryResponse)
GetStatusListCredentialRequest = _reflection.GeneratedProtocolMessageType('GetStatusListCredentialRequest', (_message.Message,), {'DESCRIPTOR': _GETSTATUSLISTCREDENTIALREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(GetStatusListCredentialRequest)
GetStatusListCredentialResponse = _reflection.GeneratedProtocolMessageType('GetStatusListCredentialResponse', (_message.Message,), {'DESCRIPTOR': _GETSTATUSLISTCREDENTIALRESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(GetStatusListCredentialResponse)
UpdateStatusListEntryRequest = _reflection.GeneratedProtocolMessageType('UpdateStatusListEntryRequest', (_message.Message,), {'DESCRIPTOR': _UPDATESTATUSLISTENTRYREQUEST, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(UpdateStatusListEntryRequest)
UpdateStatusListEntryResponse = _reflection.GeneratedProtocolMessageType('UpdateStatusListEntryResponse', (_message.Message,), {'DESCRIPTOR': _UPDATESTATUSLISTENTRYRESPONSE, '__module__': 'registry_api.v1.registry_pb2'})
_sym_db.RegisterMessage(UpdateStatusListEntryResponse)
_REGISTRYSERVICE = DESCRIPTOR.services_by_name['RegistryService']
_CREDENTIALISSUERREGISTRYSERVICE = DESCRIPTOR.services_by_name['CredentialIssuerRegistryService']
_STATUSLISTREGISTRYSERVICE = DESCRIPTOR.services_by_name['StatusListRegistryService']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z9github.com/knox-networks/grpc-sdks/sdk/go/registry_api/v1'
    _REGISTRYSERVICE.methods_by_name['Create']._options = None
    _REGISTRYSERVICE.methods_by_name['Create']._serialized_options = b'\x82\xd3\xe4\x93\x02\x0c"\x07/v1/did:\x01*'
    _REGISTRYSERVICE.methods_by_name['Resolve']._options = None
    _REGISTRYSERVICE.methods_by_name['Resolve']._serialized_options = b'\x82\xd3\xe4\x93\x02\x0e\x12\x0c/v1/did/{id}'
    _REGISTRYSERVICE.methods_by_name['ResolveRepresentation']._options = None
    _REGISTRYSERVICE.methods_by_name['ResolveRepresentation']._serialized_options = b'\x82\xd3\xe4\x93\x02\x1d\x12\x1b/v1/did/{id}/representation'
    _REGISTRYSERVICE.methods_by_name['Update']._options = None
    _REGISTRYSERVICE.methods_by_name['Update']._serialized_options = b'\x82\xd3\xe4\x93\x02\x11\x1a\x0c/v1/did/{id}:\x01*'
    _REGISTRYSERVICE.methods_by_name['Revoke']._options = None
    _REGISTRYSERVICE.methods_by_name['Revoke']._serialized_options = b'\x82\xd3\xe4\x93\x02\x0e*\x0c/v1/did/{id}'
    _CREDENTIALISSUERREGISTRYSERVICE.methods_by_name['CreateCredentialIssuer']._options = None
    _CREDENTIALISSUERREGISTRYSERVICE.methods_by_name['CreateCredentialIssuer']._serialized_options = b'\x82\xd3\xe4\x93\x02\x1a"\x15/v1/credential-issuer:\x01*'
    _CREDENTIALISSUERREGISTRYSERVICE.methods_by_name['GetCredentialIssuer']._options = None
    _CREDENTIALISSUERREGISTRYSERVICE.methods_by_name['GetCredentialIssuer']._serialized_options = b'\x82\xd3\xe4\x93\x02$\x12"/v1/credential-issuer/{issuer_did}'
    _CREDENTIALISSUERREGISTRYSERVICE.methods_by_name['UpdateCredentialIssuer']._options = None
    _CREDENTIALISSUERREGISTRYSERVICE.methods_by_name['UpdateCredentialIssuer']._serialized_options = b'\x82\xd3\xe4\x93\x02\x1a\x1a\x15/v1/credential-issuer:\x01*'
    _STATUSLISTREGISTRYSERVICE.methods_by_name['CreateStatusListEntry']._options = None
    _STATUSLISTREGISTRYSERVICE.methods_by_name['CreateStatusListEntry']._serialized_options = b'\x82\xd3\xe4\x93\x02\x14"\x0f/v1/status-list:\x01*'
    _STATUSLISTREGISTRYSERVICE.methods_by_name['GetStatusListCredential']._options = None
    _STATUSLISTREGISTRYSERVICE.methods_by_name['GetStatusListCredential']._serialized_options = b'\x82\xd3\xe4\x93\x028\x126/v1/status-list/credential/{status_list_credential_id}'
    _STATUSLISTREGISTRYSERVICE.methods_by_name['UpdateStatusListEntry']._options = None
    _STATUSLISTREGISTRYSERVICE.methods_by_name['UpdateStatusListEntry']._serialized_options = b'\x82\xd3\xe4\x93\x02$\x1a\x1f/v1/status-list/{credential_id}:\x01*'
    _RESOLUTIONOPTIONACCEPT._serialized_start = 3003
    _RESOLUTIONOPTIONACCEPT._serialized_end = 3107
    _ISSUERSTATUS._serialized_start = 3109
    _ISSUERSTATUS._serialized_end = 3209
    _STATUSPURPOSE._serialized_start = 3211
    _STATUSPURPOSE._serialized_end = 3320
    _CREATEREQUEST._serialized_start = 144
    _CREATEREQUEST._serialized_end = 190
    _CREATERESPONSE._serialized_start = 192
    _CREATERESPONSE._serialized_end = 208
    _RESOLVEREQUEST._serialized_start = 210
    _RESOLVEREQUEST._serialized_end = 300
    _RESOLVEREPRESENTATIONREQUEST._serialized_start = 302
    _RESOLVEREPRESENTATIONREQUEST._serialized_end = 406
    _RESOLUTIONOPTION._serialized_start = 408
    _RESOLUTIONOPTION._serialized_end = 483
    _DIDDOCUMENTMETADATA._serialized_start = 485
    _DIDDOCUMENTMETADATA._serialized_end = 596
    _RESOLVERESPONSE._serialized_start = 599
    _RESOLVERESPONSE._serialized_end = 797
    _RESOLVEREPRESENTATIONRESPONSE._serialized_start = 800
    _RESOLVEREPRESENTATIONRESPONSE._serialized_end = 993
    _RESOLUTIONMETADATA._serialized_start = 996
    _RESOLUTIONMETADATA._serialized_end = 1199
    _RESOLUTIONMETADATADIDURL._serialized_start = 1201
    _RESOLUTIONMETADATADIDURL._serialized_end = 1286
    _UPDATEREQUEST._serialized_start = 1288
    _UPDATEREQUEST._serialized_end = 1334
    _UPDATERESPONSE._serialized_start = 1336
    _UPDATERESPONSE._serialized_end = 1352
    _REVOKEREQUEST._serialized_start = 1354
    _REVOKEREQUEST._serialized_end = 1400
    _REVOKERESPONSE._serialized_start = 1402
    _REVOKERESPONSE._serialized_end = 1418
    _POSTALADDRESS._serialized_start = 1420
    _POSTALADDRESS._serialized_end = 1518
    _CREDENTIALISSUER._serialized_start = 1521
    _CREDENTIALISSUER._serialized_end = 1715
    _CREATECREDENTIALISSUERREQUEST._serialized_start = 1717
    _CREATECREDENTIALISSUERREQUEST._serialized_end = 1810
    _CREATECREDENTIALISSUERRESPONSE._serialized_start = 1812
    _CREATECREDENTIALISSUERRESPONSE._serialized_end = 1856
    _GETCREDENTIALISSUERREQUEST._serialized_start = 1858
    _GETCREDENTIALISSUERREQUEST._serialized_end = 1906
    _GETCREDENTIALISSUERRESPONSE._serialized_start = 1908
    _GETCREDENTIALISSUERRESPONSE._serialized_end = 1999
    _UPDATECREDENTIALISSUERREQUEST._serialized_start = 2001
    _UPDATECREDENTIALISSUERREQUEST._serialized_end = 2094
    _UPDATECREDENTIALISSUERRESPONSE._serialized_start = 2096
    _UPDATECREDENTIALISSUERRESPONSE._serialized_end = 2140
    _STATUSLISTENTRY._serialized_start = 2143
    _STATUSLISTENTRY._serialized_end = 2372
    _CREATESTATUSLISTENTRYREQUEST._serialized_start = 2375
    _CREATESTATUSLISTENTRYREQUEST._serialized_end = 2555
    _CREATESTATUSLISTENTRYRESPONSE._serialized_start = 2557
    _CREATESTATUSLISTENTRYRESPONSE._serialized_end = 2649
    _GETSTATUSLISTCREDENTIALREQUEST._serialized_start = 2651
    _GETSTATUSLISTCREDENTIALREQUEST._serialized_end = 2718
    _GETSTATUSLISTCREDENTIALRESPONSE._serialized_start = 2720
    _GETSTATUSLISTCREDENTIALRESPONSE._serialized_end = 2785
    _UPDATESTATUSLISTENTRYREQUEST._serialized_start = 2788
    _UPDATESTATUSLISTENTRYREQUEST._serialized_end = 2968
    _UPDATESTATUSLISTENTRYRESPONSE._serialized_start = 2970
    _UPDATESTATUSLISTENTRYRESPONSE._serialized_end = 3001
    _REGISTRYSERVICE._serialized_start = 3323
    _REGISTRYSERVICE._serialized_end = 3890
    _CREDENTIALISSUERREGISTRYSERVICE._serialized_start = 3893
    _CREDENTIALISSUERREGISTRYSERVICE._serialized_end = 4401
    _STATUSLISTREGISTRYSERVICE._serialized_start = 4404
    _STATUSLISTREGISTRYSERVICE._serialized_end = 4936