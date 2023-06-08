"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1cwebhook_api/v1/webhook.proto\x12\x0ewebhook_api.v1"A\n\x13CreateWalletPayload\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x0b\n\x03did\x18\x02 \x01(\t\x12\x0f\n\x07created\x18\x03 \x01(\t".\n\x19PromissoryReceivedPayload\x12\x11\n\twallet_id\x18\x01 \x01(\t"\x1e\n\x0bPingPayload\x12\x0f\n\x07message\x18\x01 \x01(\t"\xdd\x02\n\x05Event\x12-\n\nevent_type\x18\x01 \x01(\x0e2\x19.webhook_api.v1.EventType\x12\x11\n\tsignature\x18\x02 \x01(\t\x12\x10\n\x08event_id\x18\x03 \x01(\t\x12\x15\n\rdelivery_date\x18\x04 \x01(\t\x12\x10\n\x08owner_id\x18\x05 \x01(\t\x12E\n\x16wallet_created_payload\x18\x06 \x01(\x0b2#.webhook_api.v1.CreateWalletPayloadH\x00\x12P\n\x1bpromissory_received_payload\x18\x07 \x01(\x0b2).webhook_api.v1.PromissoryReceivedPayloadH\x00\x123\n\x0cping_payload\x18\x08 \x01(\x0b2\x1b.webhook_api.v1.PingPayloadH\x00B\t\n\x07payload"\xbc\x01\n\x14CreateWebhookRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x123\n\x10supported_events\x18\x03 \x03(\x0e2\x19.webhook_api.v1.EventType\x12\x13\n\x0bwebhook_url\x18\x04 \x01(\t\x12;\n\x0fdelivery_method\x18\x05 \x01(\x0e2".webhook_api.v1.DeliveryMethodType"3\n\x15CreateWebhookResponse\x12\x1a\n\x12webhook_public_key\x18\x01 \x01(\t"G\n\x13ListWebhooksRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04page\x18\x02 \x01(\x05\x12\x11\n\tpage_size\x18\x03 \x01(\x05"\xb3\x02\n\x14ListWebhooksResponse\x12>\n\x08webhooks\x18\x01 \x03(\x0b2,.webhook_api.v1.ListWebhooksResponse.Webhook\x12\x1c\n\x14total_owned_webhooks\x18\x02 \x01(\r\x1a\xbc\x01\n\x07Webhook\x12\n\n\x02id\x18\x01 \x01(\t\x12\x10\n\x08owner_id\x18\x02 \x01(\t\x12\x0c\n\x04name\x18\x03 \x01(\t\x12\x13\n\x0bwebhook_url\x18\x04 \x01(\t\x123\n\x10supported_events\x18\x05 \x03(\x0e2\x19.webhook_api.v1.EventType\x12;\n\x0fdelivery_method\x18\x06 \x01(\x0e2".webhook_api.v1.DeliveryMethodType";\n\x14DeleteWebhookRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x12\n\nwebhook_id\x18\x02 \x01(\t"\x17\n\x15DeleteWebhookResponse"4\n\x0cEventRequest\x12$\n\x05event\x18\x01 \x01(\x0b2\x15.webhook_api.v1.Event"\x0f\n\rEventResponse"E\n\x11ListEventsRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04page\x18\x02 \x01(\x05\x12\x11\n\tpage_size\x18\x03 \x01(\x05"W\n\x12ListEventsResponse\x12%\n\x06events\x18\x01 \x03(\x0b2\x15.webhook_api.v1.Event\x12\x1a\n\x12total_owned_events\x18\x02 \x01(\r"J\n\x11RetryEventRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x10\n\x08event_id\x18\x02 \x01(\t\x12\x12\n\nwebhook_id\x18\x03 \x01(\t"\x14\n\x12RetryEventResponse"/\n\x0bPingRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0f\n\x07message\x18\x02 \x01(\t"\x1f\n\x0cPingResponse\x12\x0f\n\x07message\x18\x01 \x01(\t*\x7f\n\tEventType\x12\x1a\n\x16EVENT_TYPE_UNSPECIFIED\x10\x00\x12\x1d\n\x19EVENT_TYPE_WALLET_CREATED\x10\x01\x12"\n\x1eEVENT_TYPE_PROMISSORY_RECEIVED\x10\x02\x12\x13\n\x0fEVENT_TYPE_PING\x10\x03*x\n\x12DeliveryMethodType\x12$\n DELIVERY_METHOD_TYPE_UNSPECIFIED\x10\x00\x12\x1d\n\x19DELIVERY_METHOD_TYPE_HTTP\x10\x01\x12\x1d\n\x19DELIVERY_METHOD_TYPE_GRPC\x10\x022\x9b\x04\n\x15WebhookManagerService\x12\\\n\rCreateWebhook\x12$.webhook_api.v1.CreateWebhookRequest\x1a%.webhook_api.v1.CreateWebhookResponse\x12Y\n\x0cListWebhooks\x12#.webhook_api.v1.ListWebhooksRequest\x1a$.webhook_api.v1.ListWebhooksResponse\x12\\\n\rDeleteWebhook\x12$.webhook_api.v1.DeleteWebhookRequest\x1a%.webhook_api.v1.DeleteWebhookResponse\x12S\n\nListEvents\x12!.webhook_api.v1.ListEventsRequest\x1a".webhook_api.v1.ListEventsResponse\x12S\n\nRetryEvent\x12!.webhook_api.v1.RetryEventRequest\x1a".webhook_api.v1.RetryEventResponse\x12A\n\x04Ping\x12\x1b.webhook_api.v1.PingRequest\x1a\x1c.webhook_api.v1.PingResponse2\\\n\x14EventConsumerService\x12D\n\x05Event\x12\x1c.webhook_api.v1.EventRequest\x1a\x1d.webhook_api.v1.EventResponseB:Z8github.com/knox-networks/grpc-sdks/sdk/go/webhook_api/v1b\x06proto3')
_EVENTTYPE = DESCRIPTOR.enum_types_by_name['EventType']
EventType = enum_type_wrapper.EnumTypeWrapper(_EVENTTYPE)
_DELIVERYMETHODTYPE = DESCRIPTOR.enum_types_by_name['DeliveryMethodType']
DeliveryMethodType = enum_type_wrapper.EnumTypeWrapper(_DELIVERYMETHODTYPE)
EVENT_TYPE_UNSPECIFIED = 0
EVENT_TYPE_WALLET_CREATED = 1
EVENT_TYPE_PROMISSORY_RECEIVED = 2
EVENT_TYPE_PING = 3
DELIVERY_METHOD_TYPE_UNSPECIFIED = 0
DELIVERY_METHOD_TYPE_HTTP = 1
DELIVERY_METHOD_TYPE_GRPC = 2
_CREATEWALLETPAYLOAD = DESCRIPTOR.message_types_by_name['CreateWalletPayload']
_PROMISSORYRECEIVEDPAYLOAD = DESCRIPTOR.message_types_by_name['PromissoryReceivedPayload']
_PINGPAYLOAD = DESCRIPTOR.message_types_by_name['PingPayload']
_EVENT = DESCRIPTOR.message_types_by_name['Event']
_CREATEWEBHOOKREQUEST = DESCRIPTOR.message_types_by_name['CreateWebhookRequest']
_CREATEWEBHOOKRESPONSE = DESCRIPTOR.message_types_by_name['CreateWebhookResponse']
_LISTWEBHOOKSREQUEST = DESCRIPTOR.message_types_by_name['ListWebhooksRequest']
_LISTWEBHOOKSRESPONSE = DESCRIPTOR.message_types_by_name['ListWebhooksResponse']
_LISTWEBHOOKSRESPONSE_WEBHOOK = _LISTWEBHOOKSRESPONSE.nested_types_by_name['Webhook']
_DELETEWEBHOOKREQUEST = DESCRIPTOR.message_types_by_name['DeleteWebhookRequest']
_DELETEWEBHOOKRESPONSE = DESCRIPTOR.message_types_by_name['DeleteWebhookResponse']
_EVENTREQUEST = DESCRIPTOR.message_types_by_name['EventRequest']
_EVENTRESPONSE = DESCRIPTOR.message_types_by_name['EventResponse']
_LISTEVENTSREQUEST = DESCRIPTOR.message_types_by_name['ListEventsRequest']
_LISTEVENTSRESPONSE = DESCRIPTOR.message_types_by_name['ListEventsResponse']
_RETRYEVENTREQUEST = DESCRIPTOR.message_types_by_name['RetryEventRequest']
_RETRYEVENTRESPONSE = DESCRIPTOR.message_types_by_name['RetryEventResponse']
_PINGREQUEST = DESCRIPTOR.message_types_by_name['PingRequest']
_PINGRESPONSE = DESCRIPTOR.message_types_by_name['PingResponse']
CreateWalletPayload = _reflection.GeneratedProtocolMessageType('CreateWalletPayload', (_message.Message,), {'DESCRIPTOR': _CREATEWALLETPAYLOAD, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(CreateWalletPayload)
PromissoryReceivedPayload = _reflection.GeneratedProtocolMessageType('PromissoryReceivedPayload', (_message.Message,), {'DESCRIPTOR': _PROMISSORYRECEIVEDPAYLOAD, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(PromissoryReceivedPayload)
PingPayload = _reflection.GeneratedProtocolMessageType('PingPayload', (_message.Message,), {'DESCRIPTOR': _PINGPAYLOAD, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(PingPayload)
Event = _reflection.GeneratedProtocolMessageType('Event', (_message.Message,), {'DESCRIPTOR': _EVENT, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(Event)
CreateWebhookRequest = _reflection.GeneratedProtocolMessageType('CreateWebhookRequest', (_message.Message,), {'DESCRIPTOR': _CREATEWEBHOOKREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(CreateWebhookRequest)
CreateWebhookResponse = _reflection.GeneratedProtocolMessageType('CreateWebhookResponse', (_message.Message,), {'DESCRIPTOR': _CREATEWEBHOOKRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(CreateWebhookResponse)
ListWebhooksRequest = _reflection.GeneratedProtocolMessageType('ListWebhooksRequest', (_message.Message,), {'DESCRIPTOR': _LISTWEBHOOKSREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ListWebhooksRequest)
ListWebhooksResponse = _reflection.GeneratedProtocolMessageType('ListWebhooksResponse', (_message.Message,), {'Webhook': _reflection.GeneratedProtocolMessageType('Webhook', (_message.Message,), {'DESCRIPTOR': _LISTWEBHOOKSRESPONSE_WEBHOOK, '__module__': 'webhook_api.v1.webhook_pb2'}), 'DESCRIPTOR': _LISTWEBHOOKSRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ListWebhooksResponse)
_sym_db.RegisterMessage(ListWebhooksResponse.Webhook)
DeleteWebhookRequest = _reflection.GeneratedProtocolMessageType('DeleteWebhookRequest', (_message.Message,), {'DESCRIPTOR': _DELETEWEBHOOKREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(DeleteWebhookRequest)
DeleteWebhookResponse = _reflection.GeneratedProtocolMessageType('DeleteWebhookResponse', (_message.Message,), {'DESCRIPTOR': _DELETEWEBHOOKRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(DeleteWebhookResponse)
EventRequest = _reflection.GeneratedProtocolMessageType('EventRequest', (_message.Message,), {'DESCRIPTOR': _EVENTREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(EventRequest)
EventResponse = _reflection.GeneratedProtocolMessageType('EventResponse', (_message.Message,), {'DESCRIPTOR': _EVENTRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(EventResponse)
ListEventsRequest = _reflection.GeneratedProtocolMessageType('ListEventsRequest', (_message.Message,), {'DESCRIPTOR': _LISTEVENTSREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ListEventsRequest)
ListEventsResponse = _reflection.GeneratedProtocolMessageType('ListEventsResponse', (_message.Message,), {'DESCRIPTOR': _LISTEVENTSRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ListEventsResponse)
RetryEventRequest = _reflection.GeneratedProtocolMessageType('RetryEventRequest', (_message.Message,), {'DESCRIPTOR': _RETRYEVENTREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(RetryEventRequest)
RetryEventResponse = _reflection.GeneratedProtocolMessageType('RetryEventResponse', (_message.Message,), {'DESCRIPTOR': _RETRYEVENTRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(RetryEventResponse)
PingRequest = _reflection.GeneratedProtocolMessageType('PingRequest', (_message.Message,), {'DESCRIPTOR': _PINGREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(PingRequest)
PingResponse = _reflection.GeneratedProtocolMessageType('PingResponse', (_message.Message,), {'DESCRIPTOR': _PINGRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(PingResponse)
_WEBHOOKMANAGERSERVICE = DESCRIPTOR.services_by_name['WebhookManagerService']
_EVENTCONSUMERSERVICE = DESCRIPTOR.services_by_name['EventConsumerService']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z8github.com/knox-networks/grpc-sdks/sdk/go/webhook_api/v1'
    _EVENTTYPE._serialized_start = 1671
    _EVENTTYPE._serialized_end = 1798
    _DELIVERYMETHODTYPE._serialized_start = 1800
    _DELIVERYMETHODTYPE._serialized_end = 1920
    _CREATEWALLETPAYLOAD._serialized_start = 48
    _CREATEWALLETPAYLOAD._serialized_end = 113
    _PROMISSORYRECEIVEDPAYLOAD._serialized_start = 115
    _PROMISSORYRECEIVEDPAYLOAD._serialized_end = 161
    _PINGPAYLOAD._serialized_start = 163
    _PINGPAYLOAD._serialized_end = 193
    _EVENT._serialized_start = 196
    _EVENT._serialized_end = 545
    _CREATEWEBHOOKREQUEST._serialized_start = 548
    _CREATEWEBHOOKREQUEST._serialized_end = 736
    _CREATEWEBHOOKRESPONSE._serialized_start = 738
    _CREATEWEBHOOKRESPONSE._serialized_end = 789
    _LISTWEBHOOKSREQUEST._serialized_start = 791
    _LISTWEBHOOKSREQUEST._serialized_end = 862
    _LISTWEBHOOKSRESPONSE._serialized_start = 865
    _LISTWEBHOOKSRESPONSE._serialized_end = 1172
    _LISTWEBHOOKSRESPONSE_WEBHOOK._serialized_start = 984
    _LISTWEBHOOKSRESPONSE_WEBHOOK._serialized_end = 1172
    _DELETEWEBHOOKREQUEST._serialized_start = 1174
    _DELETEWEBHOOKREQUEST._serialized_end = 1233
    _DELETEWEBHOOKRESPONSE._serialized_start = 1235
    _DELETEWEBHOOKRESPONSE._serialized_end = 1258
    _EVENTREQUEST._serialized_start = 1260
    _EVENTREQUEST._serialized_end = 1312
    _EVENTRESPONSE._serialized_start = 1314
    _EVENTRESPONSE._serialized_end = 1329
    _LISTEVENTSREQUEST._serialized_start = 1331
    _LISTEVENTSREQUEST._serialized_end = 1400
    _LISTEVENTSRESPONSE._serialized_start = 1402
    _LISTEVENTSRESPONSE._serialized_end = 1489
    _RETRYEVENTREQUEST._serialized_start = 1491
    _RETRYEVENTREQUEST._serialized_end = 1565
    _RETRYEVENTRESPONSE._serialized_start = 1567
    _RETRYEVENTRESPONSE._serialized_end = 1587
    _PINGREQUEST._serialized_start = 1589
    _PINGREQUEST._serialized_end = 1636
    _PINGRESPONSE._serialized_start = 1638
    _PINGRESPONSE._serialized_end = 1669
    _WEBHOOKMANAGERSERVICE._serialized_start = 1923
    _WEBHOOKMANAGERSERVICE._serialized_end = 2462
    _EVENTCONSUMERSERVICE._serialized_start = 2464
    _EVENTCONSUMERSERVICE._serialized_end = 2556