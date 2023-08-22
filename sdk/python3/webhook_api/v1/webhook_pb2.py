"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1cwebhook_api/v1/webhook.proto\x12\x0ewebhook_api.v1\x1a\x16common/v1/common.proto"\xbc\x01\n\x07Webhook\x12\n\n\x02id\x18\x01 \x01(\t\x12\x10\n\x08owner_id\x18\x02 \x01(\t\x12\x0c\n\x04name\x18\x03 \x01(\t\x12\x13\n\x0bwebhook_url\x18\x04 \x01(\t\x123\n\x10supported_events\x18\x05 \x03(\x0e2\x19.webhook_api.v1.EventType\x12;\n\x0fdelivery_method\x18\x06 \x01(\x0e2".webhook_api.v1.DeliveryMethodType"A\n\x13CreateWalletPayload\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x0b\n\x03did\x18\x02 \x01(\t\x12\x0f\n\x07created\x18\x03 \x01(\t".\n\x19PromissoryReceivedPayload\x12\x11\n\twallet_id\x18\x01 \x01(\t"\x1e\n\x0bPingPayload\x12\x0f\n\x07message\x18\x01 \x01(\t"\xf3\x02\n\x05Event\x12-\n\nevent_type\x18\x01 \x01(\x0e2\x19.webhook_api.v1.EventType\x12\x11\n\tsignature\x18\x02 \x01(\t\x12\x10\n\x08event_id\x18\x03 \x01(\t\x12\x15\n\rdelivery_date\x18\x04 \x01(\t\x12\x10\n\x08owner_id\x18\x05 \x01(\t\x12E\n\x16wallet_created_payload\x18\x06 \x01(\x0b2#.webhook_api.v1.CreateWalletPayloadH\x00\x12P\n\x1bpromissory_received_payload\x18\x07 \x01(\x0b2).webhook_api.v1.PromissoryReceivedPayloadH\x00\x123\n\x0cping_payload\x18\x08 \x01(\x0b2\x1b.webhook_api.v1.PingPayloadH\x00\x12\x14\n\x0ccreated_date\x18\t \x01(\tB\t\n\x07payload"\xbc\x01\n\x14CreateWebhookRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x123\n\x10supported_events\x18\x03 \x03(\x0e2\x19.webhook_api.v1.EventType\x12\x13\n\x0bwebhook_url\x18\x04 \x01(\t\x12;\n\x0fdelivery_method\x18\x05 \x01(\x0e2".webhook_api.v1.DeliveryMethodType"]\n\x15CreateWebhookResponse\x12\x1a\n\x12webhook_public_key\x18\x01 \x01(\t\x12(\n\x07webhook\x18\x02 \x01(\x0b2\x17.webhook_api.v1.Webhook"G\n\x13ListWebhooksRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04page\x18\x02 \x01(\x05\x12\x11\n\tpage_size\x18\x03 \x01(\x05"_\n\x14ListWebhooksResponse\x12)\n\x08webhooks\x18\x01 \x03(\x0b2\x17.webhook_api.v1.Webhook\x12\x1c\n\x14total_owned_webhooks\x18\x02 \x01(\r";\n\x14DeleteWebhookRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x12\n\nwebhook_id\x18\x02 \x01(\t"\x17\n\x15DeleteWebhookResponse"4\n\x0cEventRequest\x12$\n\x05event\x18\x01 \x01(\x0b2\x15.webhook_api.v1.Event"\x0f\n\rEventResponse"\x87\x01\n\x11ListEventsRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04page\x18\x02 \x01(\x05\x12\x11\n\tpage_size\x18\x03 \x01(\x05\x12\x1f\n\x07filters\x18\x04 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x05 \x01(\x0b2\x0e.common.SortBy"W\n\x12ListEventsResponse\x12%\n\x06events\x18\x01 \x03(\x0b2\x15.webhook_api.v1.Event\x12\x1a\n\x12total_owned_events\x18\x02 \x01(\r"J\n\x11RetryEventRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x10\n\x08event_id\x18\x02 \x01(\t\x12\x12\n\nwebhook_id\x18\x03 \x01(\t"\x14\n\x12RetryEventResponse"/\n\x0bPingRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0f\n\x07message\x18\x02 \x01(\t"\x1f\n\x0cPingResponse\x12\x0f\n\x07message\x18\x01 \x01(\t"\x90\x01\n\x1aListDeliveryHistoryRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04page\x18\x02 \x01(\x05\x12\x11\n\tpage_size\x18\x03 \x01(\x05\x12\x1f\n\x07filters\x18\x04 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x05 \x01(\x0b2\x0e.common.SortBy"\xba\x02\n\x1bListDeliveryHistoryResponse\x12U\n\x10delivery_history\x18\x01 \x03(\x0b2;.webhook_api.v1.ListDeliveryHistoryResponse.DeliveryHistory\x12$\n\x1ctotal_owned_delivery_history\x18\x02 \x01(\r\x1a\x9d\x01\n\x0fDeliveryHistory\x12\n\n\x02id\x18\x01 \x01(\t\x12\x12\n\nwebhook_id\x18\x02 \x01(\t\x12\x10\n\x08event_id\x18\x03 \x01(\t\x12\x10\n\x08owner_id\x18\x04 \x01(\t\x125\n\x0edelivery_stage\x18\x05 \x01(\x0e2\x1d.webhook_api.v1.DeliveryStage\x12\x0f\n\x07created\x18\x06 \x01(\t*\x88\x02\n\rDeliveryStage\x12\x1e\n\x1aDELIVERY_STAGE_UNSPECIFIED\x10\x00\x12\x1e\n\x1aDELIVERY_STAGE_FIRST_RETRY\x10\x01\x12\x1f\n\x1bDELIVERY_STAGE_SECOND_RETRY\x10\x02\x12\x1e\n\x1aDELIVERY_STAGE_THIRD_RETRY\x10\x03\x12\x1f\n\x1bDELIVERY_STAGE_FOURTH_RETRY\x10\x04\x12\x1e\n\x1aDELIVERY_STAGE_FIFTH_RETRY\x10\x05\x12\x19\n\x15DELIVERY_STAGE_FAILED\x10\x06\x12\x1a\n\x16DELIVERY_STAGE_SUCCESS\x10\x07*\x7f\n\tEventType\x12\x1a\n\x16EVENT_TYPE_UNSPECIFIED\x10\x00\x12\x1d\n\x19EVENT_TYPE_WALLET_CREATED\x10\x01\x12"\n\x1eEVENT_TYPE_PROMISSORY_RECEIVED\x10\x02\x12\x13\n\x0fEVENT_TYPE_PING\x10\x03*x\n\x12DeliveryMethodType\x12$\n DELIVERY_METHOD_TYPE_UNSPECIFIED\x10\x00\x12\x1d\n\x19DELIVERY_METHOD_TYPE_HTTP\x10\x01\x12\x1d\n\x19DELIVERY_METHOD_TYPE_GRPC\x10\x022\x8b\x05\n\x15WebhookManagerService\x12\\\n\rCreateWebhook\x12$.webhook_api.v1.CreateWebhookRequest\x1a%.webhook_api.v1.CreateWebhookResponse\x12Y\n\x0cListWebhooks\x12#.webhook_api.v1.ListWebhooksRequest\x1a$.webhook_api.v1.ListWebhooksResponse\x12\\\n\rDeleteWebhook\x12$.webhook_api.v1.DeleteWebhookRequest\x1a%.webhook_api.v1.DeleteWebhookResponse\x12S\n\nListEvents\x12!.webhook_api.v1.ListEventsRequest\x1a".webhook_api.v1.ListEventsResponse\x12S\n\nRetryEvent\x12!.webhook_api.v1.RetryEventRequest\x1a".webhook_api.v1.RetryEventResponse\x12A\n\x04Ping\x12\x1b.webhook_api.v1.PingRequest\x1a\x1c.webhook_api.v1.PingResponse\x12n\n\x13ListDeliveryHistory\x12*.webhook_api.v1.ListDeliveryHistoryRequest\x1a+.webhook_api.v1.ListDeliveryHistoryResponse2\\\n\x14EventConsumerService\x12D\n\x05Event\x12\x1c.webhook_api.v1.EventRequest\x1a\x1d.webhook_api.v1.EventResponseB:Z8github.com/knox-networks/grpc-sdks/sdk/go/webhook_api/v1b\x06proto3')
_DELIVERYSTAGE = DESCRIPTOR.enum_types_by_name['DeliveryStage']
DeliveryStage = enum_type_wrapper.EnumTypeWrapper(_DELIVERYSTAGE)
_EVENTTYPE = DESCRIPTOR.enum_types_by_name['EventType']
EventType = enum_type_wrapper.EnumTypeWrapper(_EVENTTYPE)
_DELIVERYMETHODTYPE = DESCRIPTOR.enum_types_by_name['DeliveryMethodType']
DeliveryMethodType = enum_type_wrapper.EnumTypeWrapper(_DELIVERYMETHODTYPE)
DELIVERY_STAGE_UNSPECIFIED = 0
DELIVERY_STAGE_FIRST_RETRY = 1
DELIVERY_STAGE_SECOND_RETRY = 2
DELIVERY_STAGE_THIRD_RETRY = 3
DELIVERY_STAGE_FOURTH_RETRY = 4
DELIVERY_STAGE_FIFTH_RETRY = 5
DELIVERY_STAGE_FAILED = 6
DELIVERY_STAGE_SUCCESS = 7
EVENT_TYPE_UNSPECIFIED = 0
EVENT_TYPE_WALLET_CREATED = 1
EVENT_TYPE_PROMISSORY_RECEIVED = 2
EVENT_TYPE_PING = 3
DELIVERY_METHOD_TYPE_UNSPECIFIED = 0
DELIVERY_METHOD_TYPE_HTTP = 1
DELIVERY_METHOD_TYPE_GRPC = 2
_WEBHOOK = DESCRIPTOR.message_types_by_name['Webhook']
_CREATEWALLETPAYLOAD = DESCRIPTOR.message_types_by_name['CreateWalletPayload']
_PROMISSORYRECEIVEDPAYLOAD = DESCRIPTOR.message_types_by_name['PromissoryReceivedPayload']
_PINGPAYLOAD = DESCRIPTOR.message_types_by_name['PingPayload']
_EVENT = DESCRIPTOR.message_types_by_name['Event']
_CREATEWEBHOOKREQUEST = DESCRIPTOR.message_types_by_name['CreateWebhookRequest']
_CREATEWEBHOOKRESPONSE = DESCRIPTOR.message_types_by_name['CreateWebhookResponse']
_LISTWEBHOOKSREQUEST = DESCRIPTOR.message_types_by_name['ListWebhooksRequest']
_LISTWEBHOOKSRESPONSE = DESCRIPTOR.message_types_by_name['ListWebhooksResponse']
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
_LISTDELIVERYHISTORYREQUEST = DESCRIPTOR.message_types_by_name['ListDeliveryHistoryRequest']
_LISTDELIVERYHISTORYRESPONSE = DESCRIPTOR.message_types_by_name['ListDeliveryHistoryResponse']
_LISTDELIVERYHISTORYRESPONSE_DELIVERYHISTORY = _LISTDELIVERYHISTORYRESPONSE.nested_types_by_name['DeliveryHistory']
Webhook = _reflection.GeneratedProtocolMessageType('Webhook', (_message.Message,), {'DESCRIPTOR': _WEBHOOK, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(Webhook)
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
ListWebhooksResponse = _reflection.GeneratedProtocolMessageType('ListWebhooksResponse', (_message.Message,), {'DESCRIPTOR': _LISTWEBHOOKSRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ListWebhooksResponse)
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
ListDeliveryHistoryRequest = _reflection.GeneratedProtocolMessageType('ListDeliveryHistoryRequest', (_message.Message,), {'DESCRIPTOR': _LISTDELIVERYHISTORYREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ListDeliveryHistoryRequest)
ListDeliveryHistoryResponse = _reflection.GeneratedProtocolMessageType('ListDeliveryHistoryResponse', (_message.Message,), {'DeliveryHistory': _reflection.GeneratedProtocolMessageType('DeliveryHistory', (_message.Message,), {'DESCRIPTOR': _LISTDELIVERYHISTORYRESPONSE_DELIVERYHISTORY, '__module__': 'webhook_api.v1.webhook_pb2'}), 'DESCRIPTOR': _LISTDELIVERYHISTORYRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ListDeliveryHistoryResponse)
_sym_db.RegisterMessage(ListDeliveryHistoryResponse.DeliveryHistory)
_WEBHOOKMANAGERSERVICE = DESCRIPTOR.services_by_name['WebhookManagerService']
_EVENTCONSUMERSERVICE = DESCRIPTOR.services_by_name['EventConsumerService']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z8github.com/knox-networks/grpc-sdks/sdk/go/webhook_api/v1'
    _DELIVERYSTAGE._serialized_start = 2269
    _DELIVERYSTAGE._serialized_end = 2533
    _EVENTTYPE._serialized_start = 2535
    _EVENTTYPE._serialized_end = 2662
    _DELIVERYMETHODTYPE._serialized_start = 2664
    _DELIVERYMETHODTYPE._serialized_end = 2784
    _WEBHOOK._serialized_start = 73
    _WEBHOOK._serialized_end = 261
    _CREATEWALLETPAYLOAD._serialized_start = 263
    _CREATEWALLETPAYLOAD._serialized_end = 328
    _PROMISSORYRECEIVEDPAYLOAD._serialized_start = 330
    _PROMISSORYRECEIVEDPAYLOAD._serialized_end = 376
    _PINGPAYLOAD._serialized_start = 378
    _PINGPAYLOAD._serialized_end = 408
    _EVENT._serialized_start = 411
    _EVENT._serialized_end = 782
    _CREATEWEBHOOKREQUEST._serialized_start = 785
    _CREATEWEBHOOKREQUEST._serialized_end = 973
    _CREATEWEBHOOKRESPONSE._serialized_start = 975
    _CREATEWEBHOOKRESPONSE._serialized_end = 1068
    _LISTWEBHOOKSREQUEST._serialized_start = 1070
    _LISTWEBHOOKSREQUEST._serialized_end = 1141
    _LISTWEBHOOKSRESPONSE._serialized_start = 1143
    _LISTWEBHOOKSRESPONSE._serialized_end = 1238
    _DELETEWEBHOOKREQUEST._serialized_start = 1240
    _DELETEWEBHOOKREQUEST._serialized_end = 1299
    _DELETEWEBHOOKRESPONSE._serialized_start = 1301
    _DELETEWEBHOOKRESPONSE._serialized_end = 1324
    _EVENTREQUEST._serialized_start = 1326
    _EVENTREQUEST._serialized_end = 1378
    _EVENTRESPONSE._serialized_start = 1380
    _EVENTRESPONSE._serialized_end = 1395
    _LISTEVENTSREQUEST._serialized_start = 1398
    _LISTEVENTSREQUEST._serialized_end = 1533
    _LISTEVENTSRESPONSE._serialized_start = 1535
    _LISTEVENTSRESPONSE._serialized_end = 1622
    _RETRYEVENTREQUEST._serialized_start = 1624
    _RETRYEVENTREQUEST._serialized_end = 1698
    _RETRYEVENTRESPONSE._serialized_start = 1700
    _RETRYEVENTRESPONSE._serialized_end = 1720
    _PINGREQUEST._serialized_start = 1722
    _PINGREQUEST._serialized_end = 1769
    _PINGRESPONSE._serialized_start = 1771
    _PINGRESPONSE._serialized_end = 1802
    _LISTDELIVERYHISTORYREQUEST._serialized_start = 1805
    _LISTDELIVERYHISTORYREQUEST._serialized_end = 1949
    _LISTDELIVERYHISTORYRESPONSE._serialized_start = 1952
    _LISTDELIVERYHISTORYRESPONSE._serialized_end = 2266
    _LISTDELIVERYHISTORYRESPONSE_DELIVERYHISTORY._serialized_start = 2109
    _LISTDELIVERYHISTORYRESPONSE_DELIVERYHISTORY._serialized_end = 2266
    _WEBHOOKMANAGERSERVICE._serialized_start = 2787
    _WEBHOOKMANAGERSERVICE._serialized_end = 3438
    _EVENTCONSUMERSERVICE._serialized_start = 3440
    _EVENTCONSUMERSERVICE._serialized_end = 3532