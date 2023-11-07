"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
from ...common.v1 import packet_pb2 as common_dot_v1_dot_packet__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1cwebhook_api/v1/webhook.proto\x12\x0ewebhook_api.v1\x1a\x16common/v1/common.proto\x1a\x16common/v1/packet.proto"\xbc\x01\n\x07Webhook\x12\n\n\x02id\x18\x01 \x01(\t\x12\x10\n\x08owner_id\x18\x02 \x01(\t\x12\x0c\n\x04name\x18\x03 \x01(\t\x12\x13\n\x0bwebhook_url\x18\x04 \x01(\t\x123\n\x10supported_events\x18\x05 \x03(\x0e2\x19.webhook_api.v1.EventType\x12;\n\x0fdelivery_method\x18\x06 \x01(\x0e2".webhook_api.v1.DeliveryMethodType"A\n\x13CreateWalletPayload\x12\x0c\n\x04name\x18\x01 \x01(\t\x12\x0b\n\x03did\x18\x02 \x01(\t\x12\x0f\n\x07created\x18\x03 \x01(\t".\n\x19PromissoryReceivedPayload\x12\x11\n\twallet_id\x18\x01 \x01(\t"\x1e\n\x0bPingPayload\x12\x0f\n\x07message\x18\x01 \x01(\t"\x85\x01\n ContractProposalCompletedPayload\x12\x13\n\x0bcontract_id\x18\x01 \x01(\t\x12\x10\n\x08owner_id\x18\x02 \x01(\t\x12\x11\n\twallet_id\x18\x03 \x01(\t\x12\'\n\x0bcommitments\x18\x04 \x03(\x0b2\x12.common.Commitment"\x84\x01\n\x1fContractPaymentCompletedPayload\x12\x13\n\x0bcontract_id\x18\x01 \x01(\t\x12\x10\n\x08owner_id\x18\x02 \x01(\t\x12\x11\n\twallet_id\x18\x03 \x01(\t\x12\'\n\x0bcommitments\x18\x04 \x03(\x0b2\x12.common.Commitment"\xb3\x04\n\x05Event\x12-\n\nevent_type\x18\x01 \x01(\x0e2\x19.webhook_api.v1.EventType\x12\x11\n\tsignature\x18\x02 \x01(\t\x12\x10\n\x08event_id\x18\x03 \x01(\t\x12\x15\n\rdelivery_date\x18\x04 \x01(\t\x12\x10\n\x08owner_id\x18\x05 \x01(\t\x12E\n\x16wallet_created_payload\x18\x06 \x01(\x0b2#.webhook_api.v1.CreateWalletPayloadH\x00\x12P\n\x1bpromissory_received_payload\x18\x07 \x01(\x0b2).webhook_api.v1.PromissoryReceivedPayloadH\x00\x123\n\x0cping_payload\x18\x08 \x01(\x0b2\x1b.webhook_api.v1.PingPayloadH\x00\x12_\n#contract_proposal_completed_payload\x18\n \x01(\x0b20.webhook_api.v1.ContractProposalCompletedPayloadH\x00\x12]\n"contract_payment_completed_payload\x18\x0b \x01(\x0b2/.webhook_api.v1.ContractPaymentCompletedPayloadH\x00\x12\x14\n\x0ccreated_date\x18\t \x01(\tB\t\n\x07payload"\xbc\x01\n\x14CreateWebhookRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x123\n\x10supported_events\x18\x03 \x03(\x0e2\x19.webhook_api.v1.EventType\x12\x13\n\x0bwebhook_url\x18\x04 \x01(\t\x12;\n\x0fdelivery_method\x18\x05 \x01(\x0e2".webhook_api.v1.DeliveryMethodType"]\n\x15CreateWebhookResponse\x12\x1a\n\x12webhook_public_key\x18\x01 \x01(\t\x12(\n\x07webhook\x18\x02 \x01(\x0b2\x17.webhook_api.v1.Webhook"G\n\x13ListWebhooksRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04page\x18\x02 \x01(\x05\x12\x11\n\tpage_size\x18\x03 \x01(\x05"_\n\x14ListWebhooksResponse\x12)\n\x08webhooks\x18\x01 \x03(\x0b2\x17.webhook_api.v1.Webhook\x12\x1c\n\x14total_owned_webhooks\x18\x02 \x01(\r";\n\x14DeleteWebhookRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x12\n\nwebhook_id\x18\x02 \x01(\t"\x17\n\x15DeleteWebhookResponse"4\n\x0cEventRequest\x12$\n\x05event\x18\x01 \x01(\x0b2\x15.webhook_api.v1.Event"\x0f\n\rEventResponse"\x87\x01\n\x11ListEventsRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04page\x18\x02 \x01(\x05\x12\x11\n\tpage_size\x18\x03 \x01(\x05\x12\x1f\n\x07filters\x18\x04 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x05 \x01(\x0b2\x0e.common.SortBy"W\n\x12ListEventsResponse\x12%\n\x06events\x18\x01 \x03(\x0b2\x15.webhook_api.v1.Event\x12\x1a\n\x12total_owned_events\x18\x02 \x01(\r"J\n\x11RetryEventRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x10\n\x08event_id\x18\x02 \x01(\t\x12\x12\n\nwebhook_id\x18\x03 \x01(\t"\x14\n\x12RetryEventResponse"/\n\x0bPingRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0f\n\x07message\x18\x02 \x01(\t"\x1f\n\x0cPingResponse\x12\x0f\n\x07message\x18\x01 \x01(\t"\x90\x01\n\x1aListDeliveryHistoryRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04page\x18\x02 \x01(\x05\x12\x11\n\tpage_size\x18\x03 \x01(\x05\x12\x1f\n\x07filters\x18\x04 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x05 \x01(\x0b2\x0e.common.SortBy"\xba\x02\n\x1bListDeliveryHistoryResponse\x12U\n\x10delivery_history\x18\x01 \x03(\x0b2;.webhook_api.v1.ListDeliveryHistoryResponse.DeliveryHistory\x12$\n\x1ctotal_owned_delivery_history\x18\x02 \x01(\r\x1a\x9d\x01\n\x0fDeliveryHistory\x12\n\n\x02id\x18\x01 \x01(\t\x12\x12\n\nwebhook_id\x18\x02 \x01(\t\x12\x10\n\x08event_id\x18\x03 \x01(\t\x12\x10\n\x08owner_id\x18\x04 \x01(\t\x125\n\x0edelivery_stage\x18\x05 \x01(\x0e2\x1d.webhook_api.v1.DeliveryStage\x12\x0f\n\x07created\x18\x06 \x01(\t"\xe2\x01\n\x11ScheduledReaction\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x12;\n\rreaction_type\x18\x03 \x01(\x0e2$.webhook_api.v1.ScheduleReactionType\x12\x10\n\x08cron_tab\x18\x04 \x01(\t\x12P\n\x17create_contract_payload\x18\x05 \x01(\x0b2-.webhook_api.v1.CreateContractReactionPayloadH\x00B\x12\n\x10reaction_payload"\xad\x01\n\x1dCreateContractReactionPayload\x12\x18\n\x10sender_wallet_id\x18\x01 \x01(\t\x12\'\n\x0bcommitments\x18\x02 \x03(\x0b2\x12.common.Commitment\x12%\n\nconditions\x18\x03 \x03(\x0b2\x11.common.Condition\x12\x14\n\x0ctimeout_secs\x18\x04 \x01(\r\x12\x0c\n\x04memo\x18\x05 \x01(\t"\xf4\x01\n\x1eCreateScheduledReactionRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x12;\n\rreaction_type\x18\x03 \x01(\x0e2$.webhook_api.v1.ScheduleReactionType\x12\x10\n\x08cron_tab\x18\x04 \x01(\t\x12P\n\x17create_contract_payload\x18\x05 \x01(\x0b2-.webhook_api.v1.CreateContractReactionPayloadH\x00B\x12\n\x10reaction_payload"`\n\x1fCreateScheduledReactionResponse\x12=\n\x12scheduled_reaction\x18\x01 \x01(\x0b2!.webhook_api.v1.ScheduledReaction"\x93\x01\n\x1dListScheduledReactionsRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04page\x18\x02 \x01(\x05\x12\x11\n\tpage_size\x18\x03 \x01(\x05\x12\x1f\n\x07filters\x18\x04 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x05 \x01(\x0b2\x0e.common.SortBy"\x89\x01\n\x1eListScheduledReactionsResponse\x12>\n\x13scheduled_reactions\x18\x01 \x03(\x0b2!.webhook_api.v1.ScheduledReaction\x12\'\n\x1ftotal_owned_scheduled_reactions\x18\x02 \x01(\r"P\n\x1eDeleteScheduledReactionRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x1d\n\x15scheduled_reaction_id\x18\x02 \x01(\t"!\n\x1fDeleteScheduledReactionResponse"\x94\x01\n\rPrevalidation\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x12^\n"considering_contract_prevalidation\x18\x03 \x01(\x0b20.webhook_api.v1.ConsideringContractPrevalidationH\x00B\t\n\x07trigger"\xcb\x05\n ConsideringContractPrevalidation\x12W\n\tcondition\x18\x01 \x03(\x0b2D.webhook_api.v1.ConsideringContractPrevalidation.ValidationCondition\x12S\n\x07outcome\x18\x02 \x01(\x0e2B.webhook_api.v1.ConsideringContractPrevalidation.ValidationOutcome\x12-\n\x0clogical_base\x18\x03 \x01(\x0e2\x17.common.LogicalOperator\x1a\xcf\x01\n\x13ValidationCondition\x12Q\n\x06target\x18\x01 \x01(\x0e2A.webhook_api.v1.ConsideringContractPrevalidation.ValidationTarget\x124\n\x08operator\x18\x02 \x01(\x0e2".webhook_api.v1.ValidationOperator\x12&\n\x0camount_value\x18\x03 \x01(\x0b2\x0e.common.AmountH\x00B\x07\n\x05value"^\n\x10ValidationTarget\x12!\n\x1dVALIDATION_TARGET_UNSPECIFIED\x10\x00\x12\'\n#VALIDATION_TARGET_RECIPIENT_BALANCE\x10\x01"\x97\x01\n\x11ValidationOutcome\x12,\n(CONSIDERING_CONTRACT_OUTCOME_UNSPECIFIED\x10\x00\x12)\n%CONSIDERING_CONTRACT_OUTCOME_ACCEPTED\x10\x01\x12)\n%CONSIDERING_CONTRACT_OUTCOME_REJECTED\x10\x02"\xa6\x01\n\x1aCreatePrevalidationRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04name\x18\x02 \x01(\t\x12^\n"considering_contract_prevalidation\x18\x03 \x01(\x0b20.webhook_api.v1.ConsideringContractPrevalidationH\x00B\t\n\x07trigger"S\n\x1bCreatePrevalidationResponse\x124\n\rprevalidation\x18\x01 \x01(\x0b2\x1d.webhook_api.v1.Prevalidation"\x8f\x01\n\x19ListPrevalidationsRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x0c\n\x04page\x18\x02 \x01(\x05\x12\x11\n\tpage_size\x18\x03 \x01(\x05\x12\x1f\n\x07filters\x18\x04 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x05 \x01(\x0b2\x0e.common.SortBy"w\n\x1aListPrevalidationsResponse\x125\n\x0eprevalidations\x18\x01 \x03(\x0b2\x1d.webhook_api.v1.Prevalidation\x12"\n\x1atotal_owned_prevalidations\x18\x02 \x01(\r"G\n\x1aDeletePrevalidationRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x18\n\x10prevalidation_id\x18\x02 \x01(\t"\x1d\n\x1bDeletePrevalidationResponse*\x88\x02\n\rDeliveryStage\x12\x1e\n\x1aDELIVERY_STAGE_UNSPECIFIED\x10\x00\x12\x1e\n\x1aDELIVERY_STAGE_FIRST_RETRY\x10\x01\x12\x1f\n\x1bDELIVERY_STAGE_SECOND_RETRY\x10\x02\x12\x1e\n\x1aDELIVERY_STAGE_THIRD_RETRY\x10\x03\x12\x1f\n\x1bDELIVERY_STAGE_FOURTH_RETRY\x10\x04\x12\x1e\n\x1aDELIVERY_STAGE_FIFTH_RETRY\x10\x05\x12\x19\n\x15DELIVERY_STAGE_FAILED\x10\x06\x12\x1a\n\x16DELIVERY_STAGE_SUCCESS\x10\x07*\xd6\x01\n\tEventType\x12\x1a\n\x16EVENT_TYPE_UNSPECIFIED\x10\x00\x12\x1d\n\x19EVENT_TYPE_WALLET_CREATED\x10\x01\x12"\n\x1eEVENT_TYPE_PROMISSORY_RECEIVED\x10\x02\x12\x13\n\x0fEVENT_TYPE_PING\x10\x03\x12*\n&EVENT_TYPE_CONTRACT_PROPOSAL_COMPLETED\x10\x04\x12)\n%EVENT_TYPE_CONTRACT_PAYMENT_COMPLETED\x10\x05*x\n\x12DeliveryMethodType\x12$\n DELIVERY_METHOD_TYPE_UNSPECIFIED\x10\x00\x12\x1d\n\x19DELIVERY_METHOD_TYPE_HTTP\x10\x01\x12\x1d\n\x19DELIVERY_METHOD_TYPE_GRPC\x10\x02*j\n\x14ScheduleReactionType\x12&\n"SCHEDULE_REACTION_TYPE_UNSPECIFIED\x10\x00\x12*\n&SCHEDULE_REACTION_TYPE_CREATE_CONTRACT\x10\x01*\xfc\x01\n\x12ValidationOperator\x12#\n\x1fVALIDATION_OPERATOR_UNSPECIFIED\x10\x00\x12\x1d\n\x19VALIDATION_OPERATOR_EQUAL\x10\x01\x12$\n VALIDATION_OPERATOR_GREATER_THAN\x10\x02\x12-\n)VALIDATION_OPERATOR_GREATER_THAN_OR_EQUAL\x10\x03\x12!\n\x1dVALIDATION_OPERATOR_LESS_THAN\x10\x04\x12*\n&VALIDATION_OPERATOR_LESS_THAN_OR_EQUAL\x10\x052\xc9\n\n\x15WebhookManagerService\x12\\\n\rCreateWebhook\x12$.webhook_api.v1.CreateWebhookRequest\x1a%.webhook_api.v1.CreateWebhookResponse\x12Y\n\x0cListWebhooks\x12#.webhook_api.v1.ListWebhooksRequest\x1a$.webhook_api.v1.ListWebhooksResponse\x12\\\n\rDeleteWebhook\x12$.webhook_api.v1.DeleteWebhookRequest\x1a%.webhook_api.v1.DeleteWebhookResponse\x12S\n\nListEvents\x12!.webhook_api.v1.ListEventsRequest\x1a".webhook_api.v1.ListEventsResponse\x12S\n\nRetryEvent\x12!.webhook_api.v1.RetryEventRequest\x1a".webhook_api.v1.RetryEventResponse\x12A\n\x04Ping\x12\x1b.webhook_api.v1.PingRequest\x1a\x1c.webhook_api.v1.PingResponse\x12n\n\x13ListDeliveryHistory\x12*.webhook_api.v1.ListDeliveryHistoryRequest\x1a+.webhook_api.v1.ListDeliveryHistoryResponse\x12z\n\x17CreateScheduledReaction\x12..webhook_api.v1.CreateScheduledReactionRequest\x1a/.webhook_api.v1.CreateScheduledReactionResponse\x12w\n\x16ListScheduledReactions\x12-.webhook_api.v1.ListScheduledReactionsRequest\x1a..webhook_api.v1.ListScheduledReactionsResponse\x12z\n\x17DeleteScheduledReaction\x12..webhook_api.v1.DeleteScheduledReactionRequest\x1a/.webhook_api.v1.DeleteScheduledReactionResponse\x12n\n\x13CreatePrevalidation\x12*.webhook_api.v1.CreatePrevalidationRequest\x1a+.webhook_api.v1.CreatePrevalidationResponse\x12k\n\x12ListPrevalidations\x12).webhook_api.v1.ListPrevalidationsRequest\x1a*.webhook_api.v1.ListPrevalidationsResponse\x12n\n\x13DeletePrevalidation\x12*.webhook_api.v1.DeletePrevalidationRequest\x1a+.webhook_api.v1.DeletePrevalidationResponse2\\\n\x14EventConsumerService\x12D\n\x05Event\x12\x1c.webhook_api.v1.EventRequest\x1a\x1d.webhook_api.v1.EventResponseB:Z8github.com/knox-networks/grpc-sdks/sdk/go/webhook_api/v1b\x06proto3')
_DELIVERYSTAGE = DESCRIPTOR.enum_types_by_name['DeliveryStage']
DeliveryStage = enum_type_wrapper.EnumTypeWrapper(_DELIVERYSTAGE)
_EVENTTYPE = DESCRIPTOR.enum_types_by_name['EventType']
EventType = enum_type_wrapper.EnumTypeWrapper(_EVENTTYPE)
_DELIVERYMETHODTYPE = DESCRIPTOR.enum_types_by_name['DeliveryMethodType']
DeliveryMethodType = enum_type_wrapper.EnumTypeWrapper(_DELIVERYMETHODTYPE)
_SCHEDULEREACTIONTYPE = DESCRIPTOR.enum_types_by_name['ScheduleReactionType']
ScheduleReactionType = enum_type_wrapper.EnumTypeWrapper(_SCHEDULEREACTIONTYPE)
_VALIDATIONOPERATOR = DESCRIPTOR.enum_types_by_name['ValidationOperator']
ValidationOperator = enum_type_wrapper.EnumTypeWrapper(_VALIDATIONOPERATOR)
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
EVENT_TYPE_CONTRACT_PROPOSAL_COMPLETED = 4
EVENT_TYPE_CONTRACT_PAYMENT_COMPLETED = 5
DELIVERY_METHOD_TYPE_UNSPECIFIED = 0
DELIVERY_METHOD_TYPE_HTTP = 1
DELIVERY_METHOD_TYPE_GRPC = 2
SCHEDULE_REACTION_TYPE_UNSPECIFIED = 0
SCHEDULE_REACTION_TYPE_CREATE_CONTRACT = 1
VALIDATION_OPERATOR_UNSPECIFIED = 0
VALIDATION_OPERATOR_EQUAL = 1
VALIDATION_OPERATOR_GREATER_THAN = 2
VALIDATION_OPERATOR_GREATER_THAN_OR_EQUAL = 3
VALIDATION_OPERATOR_LESS_THAN = 4
VALIDATION_OPERATOR_LESS_THAN_OR_EQUAL = 5
_WEBHOOK = DESCRIPTOR.message_types_by_name['Webhook']
_CREATEWALLETPAYLOAD = DESCRIPTOR.message_types_by_name['CreateWalletPayload']
_PROMISSORYRECEIVEDPAYLOAD = DESCRIPTOR.message_types_by_name['PromissoryReceivedPayload']
_PINGPAYLOAD = DESCRIPTOR.message_types_by_name['PingPayload']
_CONTRACTPROPOSALCOMPLETEDPAYLOAD = DESCRIPTOR.message_types_by_name['ContractProposalCompletedPayload']
_CONTRACTPAYMENTCOMPLETEDPAYLOAD = DESCRIPTOR.message_types_by_name['ContractPaymentCompletedPayload']
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
_SCHEDULEDREACTION = DESCRIPTOR.message_types_by_name['ScheduledReaction']
_CREATECONTRACTREACTIONPAYLOAD = DESCRIPTOR.message_types_by_name['CreateContractReactionPayload']
_CREATESCHEDULEDREACTIONREQUEST = DESCRIPTOR.message_types_by_name['CreateScheduledReactionRequest']
_CREATESCHEDULEDREACTIONRESPONSE = DESCRIPTOR.message_types_by_name['CreateScheduledReactionResponse']
_LISTSCHEDULEDREACTIONSREQUEST = DESCRIPTOR.message_types_by_name['ListScheduledReactionsRequest']
_LISTSCHEDULEDREACTIONSRESPONSE = DESCRIPTOR.message_types_by_name['ListScheduledReactionsResponse']
_DELETESCHEDULEDREACTIONREQUEST = DESCRIPTOR.message_types_by_name['DeleteScheduledReactionRequest']
_DELETESCHEDULEDREACTIONRESPONSE = DESCRIPTOR.message_types_by_name['DeleteScheduledReactionResponse']
_PREVALIDATION = DESCRIPTOR.message_types_by_name['Prevalidation']
_CONSIDERINGCONTRACTPREVALIDATION = DESCRIPTOR.message_types_by_name['ConsideringContractPrevalidation']
_CONSIDERINGCONTRACTPREVALIDATION_VALIDATIONCONDITION = _CONSIDERINGCONTRACTPREVALIDATION.nested_types_by_name['ValidationCondition']
_CREATEPREVALIDATIONREQUEST = DESCRIPTOR.message_types_by_name['CreatePrevalidationRequest']
_CREATEPREVALIDATIONRESPONSE = DESCRIPTOR.message_types_by_name['CreatePrevalidationResponse']
_LISTPREVALIDATIONSREQUEST = DESCRIPTOR.message_types_by_name['ListPrevalidationsRequest']
_LISTPREVALIDATIONSRESPONSE = DESCRIPTOR.message_types_by_name['ListPrevalidationsResponse']
_DELETEPREVALIDATIONREQUEST = DESCRIPTOR.message_types_by_name['DeletePrevalidationRequest']
_DELETEPREVALIDATIONRESPONSE = DESCRIPTOR.message_types_by_name['DeletePrevalidationResponse']
_CONSIDERINGCONTRACTPREVALIDATION_VALIDATIONTARGET = _CONSIDERINGCONTRACTPREVALIDATION.enum_types_by_name['ValidationTarget']
_CONSIDERINGCONTRACTPREVALIDATION_VALIDATIONOUTCOME = _CONSIDERINGCONTRACTPREVALIDATION.enum_types_by_name['ValidationOutcome']
Webhook = _reflection.GeneratedProtocolMessageType('Webhook', (_message.Message,), {'DESCRIPTOR': _WEBHOOK, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(Webhook)
CreateWalletPayload = _reflection.GeneratedProtocolMessageType('CreateWalletPayload', (_message.Message,), {'DESCRIPTOR': _CREATEWALLETPAYLOAD, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(CreateWalletPayload)
PromissoryReceivedPayload = _reflection.GeneratedProtocolMessageType('PromissoryReceivedPayload', (_message.Message,), {'DESCRIPTOR': _PROMISSORYRECEIVEDPAYLOAD, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(PromissoryReceivedPayload)
PingPayload = _reflection.GeneratedProtocolMessageType('PingPayload', (_message.Message,), {'DESCRIPTOR': _PINGPAYLOAD, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(PingPayload)
ContractProposalCompletedPayload = _reflection.GeneratedProtocolMessageType('ContractProposalCompletedPayload', (_message.Message,), {'DESCRIPTOR': _CONTRACTPROPOSALCOMPLETEDPAYLOAD, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ContractProposalCompletedPayload)
ContractPaymentCompletedPayload = _reflection.GeneratedProtocolMessageType('ContractPaymentCompletedPayload', (_message.Message,), {'DESCRIPTOR': _CONTRACTPAYMENTCOMPLETEDPAYLOAD, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ContractPaymentCompletedPayload)
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
ScheduledReaction = _reflection.GeneratedProtocolMessageType('ScheduledReaction', (_message.Message,), {'DESCRIPTOR': _SCHEDULEDREACTION, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ScheduledReaction)
CreateContractReactionPayload = _reflection.GeneratedProtocolMessageType('CreateContractReactionPayload', (_message.Message,), {'DESCRIPTOR': _CREATECONTRACTREACTIONPAYLOAD, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(CreateContractReactionPayload)
CreateScheduledReactionRequest = _reflection.GeneratedProtocolMessageType('CreateScheduledReactionRequest', (_message.Message,), {'DESCRIPTOR': _CREATESCHEDULEDREACTIONREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(CreateScheduledReactionRequest)
CreateScheduledReactionResponse = _reflection.GeneratedProtocolMessageType('CreateScheduledReactionResponse', (_message.Message,), {'DESCRIPTOR': _CREATESCHEDULEDREACTIONRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(CreateScheduledReactionResponse)
ListScheduledReactionsRequest = _reflection.GeneratedProtocolMessageType('ListScheduledReactionsRequest', (_message.Message,), {'DESCRIPTOR': _LISTSCHEDULEDREACTIONSREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ListScheduledReactionsRequest)
ListScheduledReactionsResponse = _reflection.GeneratedProtocolMessageType('ListScheduledReactionsResponse', (_message.Message,), {'DESCRIPTOR': _LISTSCHEDULEDREACTIONSRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ListScheduledReactionsResponse)
DeleteScheduledReactionRequest = _reflection.GeneratedProtocolMessageType('DeleteScheduledReactionRequest', (_message.Message,), {'DESCRIPTOR': _DELETESCHEDULEDREACTIONREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(DeleteScheduledReactionRequest)
DeleteScheduledReactionResponse = _reflection.GeneratedProtocolMessageType('DeleteScheduledReactionResponse', (_message.Message,), {'DESCRIPTOR': _DELETESCHEDULEDREACTIONRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(DeleteScheduledReactionResponse)
Prevalidation = _reflection.GeneratedProtocolMessageType('Prevalidation', (_message.Message,), {'DESCRIPTOR': _PREVALIDATION, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(Prevalidation)
ConsideringContractPrevalidation = _reflection.GeneratedProtocolMessageType('ConsideringContractPrevalidation', (_message.Message,), {'ValidationCondition': _reflection.GeneratedProtocolMessageType('ValidationCondition', (_message.Message,), {'DESCRIPTOR': _CONSIDERINGCONTRACTPREVALIDATION_VALIDATIONCONDITION, '__module__': 'webhook_api.v1.webhook_pb2'}), 'DESCRIPTOR': _CONSIDERINGCONTRACTPREVALIDATION, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ConsideringContractPrevalidation)
_sym_db.RegisterMessage(ConsideringContractPrevalidation.ValidationCondition)
CreatePrevalidationRequest = _reflection.GeneratedProtocolMessageType('CreatePrevalidationRequest', (_message.Message,), {'DESCRIPTOR': _CREATEPREVALIDATIONREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(CreatePrevalidationRequest)
CreatePrevalidationResponse = _reflection.GeneratedProtocolMessageType('CreatePrevalidationResponse', (_message.Message,), {'DESCRIPTOR': _CREATEPREVALIDATIONRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(CreatePrevalidationResponse)
ListPrevalidationsRequest = _reflection.GeneratedProtocolMessageType('ListPrevalidationsRequest', (_message.Message,), {'DESCRIPTOR': _LISTPREVALIDATIONSREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ListPrevalidationsRequest)
ListPrevalidationsResponse = _reflection.GeneratedProtocolMessageType('ListPrevalidationsResponse', (_message.Message,), {'DESCRIPTOR': _LISTPREVALIDATIONSRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(ListPrevalidationsResponse)
DeletePrevalidationRequest = _reflection.GeneratedProtocolMessageType('DeletePrevalidationRequest', (_message.Message,), {'DESCRIPTOR': _DELETEPREVALIDATIONREQUEST, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(DeletePrevalidationRequest)
DeletePrevalidationResponse = _reflection.GeneratedProtocolMessageType('DeletePrevalidationResponse', (_message.Message,), {'DESCRIPTOR': _DELETEPREVALIDATIONRESPONSE, '__module__': 'webhook_api.v1.webhook_pb2'})
_sym_db.RegisterMessage(DeletePrevalidationResponse)
_WEBHOOKMANAGERSERVICE = DESCRIPTOR.services_by_name['WebhookManagerService']
_EVENTCONSUMERSERVICE = DESCRIPTOR.services_by_name['EventConsumerService']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z8github.com/knox-networks/grpc-sdks/sdk/go/webhook_api/v1'
    _DELIVERYSTAGE._serialized_start = 5407
    _DELIVERYSTAGE._serialized_end = 5671
    _EVENTTYPE._serialized_start = 5674
    _EVENTTYPE._serialized_end = 5888
    _DELIVERYMETHODTYPE._serialized_start = 5890
    _DELIVERYMETHODTYPE._serialized_end = 6010
    _SCHEDULEREACTIONTYPE._serialized_start = 6012
    _SCHEDULEREACTIONTYPE._serialized_end = 6118
    _VALIDATIONOPERATOR._serialized_start = 6121
    _VALIDATIONOPERATOR._serialized_end = 6373
    _WEBHOOK._serialized_start = 97
    _WEBHOOK._serialized_end = 285
    _CREATEWALLETPAYLOAD._serialized_start = 287
    _CREATEWALLETPAYLOAD._serialized_end = 352
    _PROMISSORYRECEIVEDPAYLOAD._serialized_start = 354
    _PROMISSORYRECEIVEDPAYLOAD._serialized_end = 400
    _PINGPAYLOAD._serialized_start = 402
    _PINGPAYLOAD._serialized_end = 432
    _CONTRACTPROPOSALCOMPLETEDPAYLOAD._serialized_start = 435
    _CONTRACTPROPOSALCOMPLETEDPAYLOAD._serialized_end = 568
    _CONTRACTPAYMENTCOMPLETEDPAYLOAD._serialized_start = 571
    _CONTRACTPAYMENTCOMPLETEDPAYLOAD._serialized_end = 703
    _EVENT._serialized_start = 706
    _EVENT._serialized_end = 1269
    _CREATEWEBHOOKREQUEST._serialized_start = 1272
    _CREATEWEBHOOKREQUEST._serialized_end = 1460
    _CREATEWEBHOOKRESPONSE._serialized_start = 1462
    _CREATEWEBHOOKRESPONSE._serialized_end = 1555
    _LISTWEBHOOKSREQUEST._serialized_start = 1557
    _LISTWEBHOOKSREQUEST._serialized_end = 1628
    _LISTWEBHOOKSRESPONSE._serialized_start = 1630
    _LISTWEBHOOKSRESPONSE._serialized_end = 1725
    _DELETEWEBHOOKREQUEST._serialized_start = 1727
    _DELETEWEBHOOKREQUEST._serialized_end = 1786
    _DELETEWEBHOOKRESPONSE._serialized_start = 1788
    _DELETEWEBHOOKRESPONSE._serialized_end = 1811
    _EVENTREQUEST._serialized_start = 1813
    _EVENTREQUEST._serialized_end = 1865
    _EVENTRESPONSE._serialized_start = 1867
    _EVENTRESPONSE._serialized_end = 1882
    _LISTEVENTSREQUEST._serialized_start = 1885
    _LISTEVENTSREQUEST._serialized_end = 2020
    _LISTEVENTSRESPONSE._serialized_start = 2022
    _LISTEVENTSRESPONSE._serialized_end = 2109
    _RETRYEVENTREQUEST._serialized_start = 2111
    _RETRYEVENTREQUEST._serialized_end = 2185
    _RETRYEVENTRESPONSE._serialized_start = 2187
    _RETRYEVENTRESPONSE._serialized_end = 2207
    _PINGREQUEST._serialized_start = 2209
    _PINGREQUEST._serialized_end = 2256
    _PINGRESPONSE._serialized_start = 2258
    _PINGRESPONSE._serialized_end = 2289
    _LISTDELIVERYHISTORYREQUEST._serialized_start = 2292
    _LISTDELIVERYHISTORYREQUEST._serialized_end = 2436
    _LISTDELIVERYHISTORYRESPONSE._serialized_start = 2439
    _LISTDELIVERYHISTORYRESPONSE._serialized_end = 2753
    _LISTDELIVERYHISTORYRESPONSE_DELIVERYHISTORY._serialized_start = 2596
    _LISTDELIVERYHISTORYRESPONSE_DELIVERYHISTORY._serialized_end = 2753
    _SCHEDULEDREACTION._serialized_start = 2756
    _SCHEDULEDREACTION._serialized_end = 2982
    _CREATECONTRACTREACTIONPAYLOAD._serialized_start = 2985
    _CREATECONTRACTREACTIONPAYLOAD._serialized_end = 3158
    _CREATESCHEDULEDREACTIONREQUEST._serialized_start = 3161
    _CREATESCHEDULEDREACTIONREQUEST._serialized_end = 3405
    _CREATESCHEDULEDREACTIONRESPONSE._serialized_start = 3407
    _CREATESCHEDULEDREACTIONRESPONSE._serialized_end = 3503
    _LISTSCHEDULEDREACTIONSREQUEST._serialized_start = 3506
    _LISTSCHEDULEDREACTIONSREQUEST._serialized_end = 3653
    _LISTSCHEDULEDREACTIONSRESPONSE._serialized_start = 3656
    _LISTSCHEDULEDREACTIONSRESPONSE._serialized_end = 3793
    _DELETESCHEDULEDREACTIONREQUEST._serialized_start = 3795
    _DELETESCHEDULEDREACTIONREQUEST._serialized_end = 3875
    _DELETESCHEDULEDREACTIONRESPONSE._serialized_start = 3877
    _DELETESCHEDULEDREACTIONRESPONSE._serialized_end = 3910
    _PREVALIDATION._serialized_start = 3913
    _PREVALIDATION._serialized_end = 4061
    _CONSIDERINGCONTRACTPREVALIDATION._serialized_start = 4064
    _CONSIDERINGCONTRACTPREVALIDATION._serialized_end = 4779
    _CONSIDERINGCONTRACTPREVALIDATION_VALIDATIONCONDITION._serialized_start = 4322
    _CONSIDERINGCONTRACTPREVALIDATION_VALIDATIONCONDITION._serialized_end = 4529
    _CONSIDERINGCONTRACTPREVALIDATION_VALIDATIONTARGET._serialized_start = 4531
    _CONSIDERINGCONTRACTPREVALIDATION_VALIDATIONTARGET._serialized_end = 4625
    _CONSIDERINGCONTRACTPREVALIDATION_VALIDATIONOUTCOME._serialized_start = 4628
    _CONSIDERINGCONTRACTPREVALIDATION_VALIDATIONOUTCOME._serialized_end = 4779
    _CREATEPREVALIDATIONREQUEST._serialized_start = 4782
    _CREATEPREVALIDATIONREQUEST._serialized_end = 4948
    _CREATEPREVALIDATIONRESPONSE._serialized_start = 4950
    _CREATEPREVALIDATIONRESPONSE._serialized_end = 5033
    _LISTPREVALIDATIONSREQUEST._serialized_start = 5036
    _LISTPREVALIDATIONSREQUEST._serialized_end = 5179
    _LISTPREVALIDATIONSRESPONSE._serialized_start = 5181
    _LISTPREVALIDATIONSRESPONSE._serialized_end = 5300
    _DELETEPREVALIDATIONREQUEST._serialized_start = 5302
    _DELETEPREVALIDATIONREQUEST._serialized_end = 5373
    _DELETEPREVALIDATIONRESPONSE._serialized_start = 5375
    _DELETEPREVALIDATIONRESPONSE._serialized_end = 5404
    _WEBHOOKMANAGERSERVICE._serialized_start = 6376
    _WEBHOOKMANAGERSERVICE._serialized_end = 7729
    _EVENTCONSUMERSERVICE._serialized_start = 7731
    _EVENTCONSUMERSERVICE._serialized_end = 7823