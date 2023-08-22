"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from google.protobuf import struct_pb2 as google_dot_protobuf_dot_struct__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n(control_plane_api/v1/control_plane.proto\x12\x14control_plane_api.v1\x1a\x1cgoogle/protobuf/struct.proto"\xad\x01\n\x14CreateServiceRequest\x127\n\x0cservice_type\x18\x01 \x01(\x0e2!.control_plane_api.v1.ServiceType\x12\x16\n\x0eservice_suffix\x18\x02 \x01(\t\x12D\n\x0fservice_configs\x18\x03 \x01(\x0b2+.control_plane_api.v1.ServiceConfigurations"u\n\x15CreateServiceResponse\x122\n\x06status\x18\x01 \x01(\x0e2".control_plane_api.v1.DeployStatus\x12\x14\n\x0cservice_name\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t"9\n\x0eServiceDetails\x12\'\n\x06values\x18\x01 \x01(\x0b2\x17.google.protobuf.Struct"d\n\x11GetServiceRequest\x127\n\x0cservice_type\x18\x01 \x01(\x0e2!.control_plane_api.v1.ServiceType\x12\x16\n\x0eservice_suffix\x18\x02 \x01(\t"^\n\rInstanceState\x12=\n\x0finstance_status\x18\x01 \x01(\x0e2$.control_plane_api.v1.InstanceStatus\x12\x0e\n\x06reason\x18\x02 \x01(\t"\xbb\x01\n\x12GetServiceResponse\x12\x14\n\x0cservice_name\x18\x01 \x01(\t\x12<\n\x0finstance_states\x18\x02 \x03(\x0b2#.control_plane_api.v1.InstanceState\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12=\n\x0fservice_details\x18\x04 \x01(\x0b2$.control_plane_api.v1.ServiceDetails"M\n\x12GetServicesRequest\x127\n\x0cservice_type\x18\x01 \x01(\x0e2!.control_plane_api.v1.ServiceType"Q\n\x13GetServicesResponse\x12:\n\x08services\x18\x01 \x03(\x0b2(.control_plane_api.v1.GetServiceResponse"g\n\x14DeleteServiceRequest\x127\n\x0cservice_type\x18\x01 \x01(\x0e2!.control_plane_api.v1.ServiceType\x12\x16\n\x0eservice_suffix\x18\x02 \x01(\t"a\n\x15DeleteServiceResponse\x12\x14\n\x0cservice_name\x18\x01 \x01(\t\x122\n\x06status\x18\x02 \x01(\x0e2".control_plane_api.v1.DeployStatus"@\n\x15ServiceConfigurations\x12\'\n\x06values\x18\x01 \x01(\x0b2\x17.google.protobuf.Struct"\xad\x01\n\x14UpdateServiceRequest\x127\n\x0cservice_type\x18\x01 \x01(\x0e2!.control_plane_api.v1.ServiceType\x12\x16\n\x0eservice_suffix\x18\x02 \x01(\t\x12D\n\x0fservice_configs\x18\x03 \x01(\x0b2+.control_plane_api.v1.ServiceConfigurations"a\n\x15UpdateServiceResponse\x12\x14\n\x0cservice_name\x18\x01 \x01(\t\x122\n\x06status\x18\x02 \x01(\x0e2".control_plane_api.v1.DeployStatus*\xeb\x03\n\x0bServiceType\x12\x1c\n\x18SERVICE_TYPE_UNSPECIFIED\x10\x00\x12\x1a\n\x16SERVICE_TYPE_AUTHORITY\x10\x01\x12\x17\n\x13SERVICE_TYPE_ISSUER\x10\x02\x12\x17\n\x13SERVICE_TYPE_FAUCET\x10\x03\x12\x19\n\x15SERVICE_TYPE_OVERLOAD\x10\x04\x12\x17\n\x13SERVICE_TYPE_NOTARY\x10\x05\x12\x19\n\x15SERVICE_TYPE_TREASURY\x10\x06\x12\x1a\n\x16SERVICE_TYPE_SANCTIONS\x10\x07\x12\x16\n\x12SERVICE_TYPE_VAULT\x10\x08\x12\x19\n\x15SERVICE_TYPE_EMISSARY\x10\t\x12\x18\n\x14SERVICE_TYPE_GATEWAY\x10\n\x12!\n\x1dSERVICE_TYPE_CUSTODIAL_WALLET\x10\x0b\x12\x1a\n\x16SERVICE_TYPE_USER_MGMT\x10\x0c\x12#\n\x1fSERVICE_TYPE_CREDENTIAL_ADAPTER\x10\r\x12\x1e\n\x1aSERVICE_TYPE_REGISTRY_MGMT\x10\x0e\x12\x1b\n\x17SERVICE_TYPE_SANDBOX_UI\x10\x0f\x12\x17\n\x13SERVICE_TYPE_GRPCUI\x10\x10*\xdb\x01\n\x0cDeployStatus\x12\x1d\n\x19DEPLOY_STATUS_UNSPECIFIED\x10\x00\x12\x1a\n\x16DEPLOY_STATUS_DEPLOYED\x10\x01\x12\x1d\n\x19DEPLOY_STATUS_UNINSTALLED\x10\x02\x12\x1c\n\x18DEPLOY_STATUS_SUPERSEDED\x10\x03\x12\x18\n\x14DEPLOY_STATUS_FAILED\x10\x04\x12\x1e\n\x1aDEPLOY_STATUS_UNINSTALLING\x10\x05\x12\x19\n\x15DEPLOY_STATUS_PENDING\x10\x06*\x8b\x01\n\x0eInstanceStatus\x12\x1f\n\x1bINSTANCE_STATUS_UNSPECIFIED\x10\x00\x12\x1b\n\x17INSTANCE_STATUS_RUNNING\x10\x01\x12\x1b\n\x17INSTANCE_STATUS_WAITING\x10\x02\x12\x1e\n\x1aINSTANCE_STATUS_TERMINATED\x10\x032\x9b\x04\n\x16ControlPlaneAPIService\x12h\n\rCreateService\x12*.control_plane_api.v1.CreateServiceRequest\x1a+.control_plane_api.v1.CreateServiceResponse\x12_\n\nGetService\x12\'.control_plane_api.v1.GetServiceRequest\x1a(.control_plane_api.v1.GetServiceResponse\x12b\n\x0bGetServices\x12(.control_plane_api.v1.GetServicesRequest\x1a).control_plane_api.v1.GetServicesResponse\x12h\n\rDeleteService\x12*.control_plane_api.v1.DeleteServiceRequest\x1a+.control_plane_api.v1.DeleteServiceResponse\x12h\n\rUpdateService\x12*.control_plane_api.v1.UpdateServiceRequest\x1a+.control_plane_api.v1.UpdateServiceResponseB@Z>github.com/knox-networks/grpc-sdks/sdk/go/control_plane_api/v1b\x06proto3')
_SERVICETYPE = DESCRIPTOR.enum_types_by_name['ServiceType']
ServiceType = enum_type_wrapper.EnumTypeWrapper(_SERVICETYPE)
_DEPLOYSTATUS = DESCRIPTOR.enum_types_by_name['DeployStatus']
DeployStatus = enum_type_wrapper.EnumTypeWrapper(_DEPLOYSTATUS)
_INSTANCESTATUS = DESCRIPTOR.enum_types_by_name['InstanceStatus']
InstanceStatus = enum_type_wrapper.EnumTypeWrapper(_INSTANCESTATUS)
SERVICE_TYPE_UNSPECIFIED = 0
SERVICE_TYPE_AUTHORITY = 1
SERVICE_TYPE_ISSUER = 2
SERVICE_TYPE_FAUCET = 3
SERVICE_TYPE_OVERLOAD = 4
SERVICE_TYPE_NOTARY = 5
SERVICE_TYPE_TREASURY = 6
SERVICE_TYPE_SANCTIONS = 7
SERVICE_TYPE_VAULT = 8
SERVICE_TYPE_EMISSARY = 9
SERVICE_TYPE_GATEWAY = 10
SERVICE_TYPE_CUSTODIAL_WALLET = 11
SERVICE_TYPE_USER_MGMT = 12
SERVICE_TYPE_CREDENTIAL_ADAPTER = 13
SERVICE_TYPE_REGISTRY_MGMT = 14
SERVICE_TYPE_SANDBOX_UI = 15
SERVICE_TYPE_GRPCUI = 16
DEPLOY_STATUS_UNSPECIFIED = 0
DEPLOY_STATUS_DEPLOYED = 1
DEPLOY_STATUS_UNINSTALLED = 2
DEPLOY_STATUS_SUPERSEDED = 3
DEPLOY_STATUS_FAILED = 4
DEPLOY_STATUS_UNINSTALLING = 5
DEPLOY_STATUS_PENDING = 6
INSTANCE_STATUS_UNSPECIFIED = 0
INSTANCE_STATUS_RUNNING = 1
INSTANCE_STATUS_WAITING = 2
INSTANCE_STATUS_TERMINATED = 3
_CREATESERVICEREQUEST = DESCRIPTOR.message_types_by_name['CreateServiceRequest']
_CREATESERVICERESPONSE = DESCRIPTOR.message_types_by_name['CreateServiceResponse']
_SERVICEDETAILS = DESCRIPTOR.message_types_by_name['ServiceDetails']
_GETSERVICEREQUEST = DESCRIPTOR.message_types_by_name['GetServiceRequest']
_INSTANCESTATE = DESCRIPTOR.message_types_by_name['InstanceState']
_GETSERVICERESPONSE = DESCRIPTOR.message_types_by_name['GetServiceResponse']
_GETSERVICESREQUEST = DESCRIPTOR.message_types_by_name['GetServicesRequest']
_GETSERVICESRESPONSE = DESCRIPTOR.message_types_by_name['GetServicesResponse']
_DELETESERVICEREQUEST = DESCRIPTOR.message_types_by_name['DeleteServiceRequest']
_DELETESERVICERESPONSE = DESCRIPTOR.message_types_by_name['DeleteServiceResponse']
_SERVICECONFIGURATIONS = DESCRIPTOR.message_types_by_name['ServiceConfigurations']
_UPDATESERVICEREQUEST = DESCRIPTOR.message_types_by_name['UpdateServiceRequest']
_UPDATESERVICERESPONSE = DESCRIPTOR.message_types_by_name['UpdateServiceResponse']
CreateServiceRequest = _reflection.GeneratedProtocolMessageType('CreateServiceRequest', (_message.Message,), {'DESCRIPTOR': _CREATESERVICEREQUEST, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(CreateServiceRequest)
CreateServiceResponse = _reflection.GeneratedProtocolMessageType('CreateServiceResponse', (_message.Message,), {'DESCRIPTOR': _CREATESERVICERESPONSE, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(CreateServiceResponse)
ServiceDetails = _reflection.GeneratedProtocolMessageType('ServiceDetails', (_message.Message,), {'DESCRIPTOR': _SERVICEDETAILS, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(ServiceDetails)
GetServiceRequest = _reflection.GeneratedProtocolMessageType('GetServiceRequest', (_message.Message,), {'DESCRIPTOR': _GETSERVICEREQUEST, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(GetServiceRequest)
InstanceState = _reflection.GeneratedProtocolMessageType('InstanceState', (_message.Message,), {'DESCRIPTOR': _INSTANCESTATE, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(InstanceState)
GetServiceResponse = _reflection.GeneratedProtocolMessageType('GetServiceResponse', (_message.Message,), {'DESCRIPTOR': _GETSERVICERESPONSE, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(GetServiceResponse)
GetServicesRequest = _reflection.GeneratedProtocolMessageType('GetServicesRequest', (_message.Message,), {'DESCRIPTOR': _GETSERVICESREQUEST, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(GetServicesRequest)
GetServicesResponse = _reflection.GeneratedProtocolMessageType('GetServicesResponse', (_message.Message,), {'DESCRIPTOR': _GETSERVICESRESPONSE, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(GetServicesResponse)
DeleteServiceRequest = _reflection.GeneratedProtocolMessageType('DeleteServiceRequest', (_message.Message,), {'DESCRIPTOR': _DELETESERVICEREQUEST, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(DeleteServiceRequest)
DeleteServiceResponse = _reflection.GeneratedProtocolMessageType('DeleteServiceResponse', (_message.Message,), {'DESCRIPTOR': _DELETESERVICERESPONSE, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(DeleteServiceResponse)
ServiceConfigurations = _reflection.GeneratedProtocolMessageType('ServiceConfigurations', (_message.Message,), {'DESCRIPTOR': _SERVICECONFIGURATIONS, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(ServiceConfigurations)
UpdateServiceRequest = _reflection.GeneratedProtocolMessageType('UpdateServiceRequest', (_message.Message,), {'DESCRIPTOR': _UPDATESERVICEREQUEST, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(UpdateServiceRequest)
UpdateServiceResponse = _reflection.GeneratedProtocolMessageType('UpdateServiceResponse', (_message.Message,), {'DESCRIPTOR': _UPDATESERVICERESPONSE, '__module__': 'control_plane_api.v1.control_plane_pb2'})
_sym_db.RegisterMessage(UpdateServiceResponse)
_CONTROLPLANEAPISERVICE = DESCRIPTOR.services_by_name['ControlPlaneAPIService']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z>github.com/knox-networks/grpc-sdks/sdk/go/control_plane_api/v1'
    _SERVICETYPE._serialized_start = 1546
    _SERVICETYPE._serialized_end = 2037
    _DEPLOYSTATUS._serialized_start = 2040
    _DEPLOYSTATUS._serialized_end = 2259
    _INSTANCESTATUS._serialized_start = 2262
    _INSTANCESTATUS._serialized_end = 2401
    _CREATESERVICEREQUEST._serialized_start = 97
    _CREATESERVICEREQUEST._serialized_end = 270
    _CREATESERVICERESPONSE._serialized_start = 272
    _CREATESERVICERESPONSE._serialized_end = 389
    _SERVICEDETAILS._serialized_start = 391
    _SERVICEDETAILS._serialized_end = 448
    _GETSERVICEREQUEST._serialized_start = 450
    _GETSERVICEREQUEST._serialized_end = 550
    _INSTANCESTATE._serialized_start = 552
    _INSTANCESTATE._serialized_end = 646
    _GETSERVICERESPONSE._serialized_start = 649
    _GETSERVICERESPONSE._serialized_end = 836
    _GETSERVICESREQUEST._serialized_start = 838
    _GETSERVICESREQUEST._serialized_end = 915
    _GETSERVICESRESPONSE._serialized_start = 917
    _GETSERVICESRESPONSE._serialized_end = 998
    _DELETESERVICEREQUEST._serialized_start = 1000
    _DELETESERVICEREQUEST._serialized_end = 1103
    _DELETESERVICERESPONSE._serialized_start = 1105
    _DELETESERVICERESPONSE._serialized_end = 1202
    _SERVICECONFIGURATIONS._serialized_start = 1204
    _SERVICECONFIGURATIONS._serialized_end = 1268
    _UPDATESERVICEREQUEST._serialized_start = 1271
    _UPDATESERVICEREQUEST._serialized_end = 1444
    _UPDATESERVICERESPONSE._serialized_start = 1446
    _UPDATESERVICERESPONSE._serialized_end = 1543
    _CONTROLPLANEAPISERVICE._serialized_start = 2404
    _CONTROLPLANEAPISERVICE._serialized_end = 2943