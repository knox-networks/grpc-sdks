"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import packet_pb2 as common_dot_v1_dot_packet__pb2
from ...common.v1 import amqp_pb2 as common_dot_v1_dot_amqp__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1ebusiness_api/v1/business.proto\x12\x0fbusiness_api.v1\x1a\x16common/v1/packet.proto\x1a\x14common/v1/amqp.proto"\r\n\x0bListRequest"\x1c\n\x0cListResponse\x12\x0c\n\x04list\x18\x01 \x03(\t"\x0e\n\x0cClearRequest"\x0f\n\rClearResponse"6\n\x12SendPayloadRequest\x12 \n\x07payload\x18\x01 \x01(\x0b2\x0f.common.Payload"\x15\n\x13SendPayloadResponse"\x88\x01\n\x19SendNewTransactionRequest\x12\r\n\x05owner\x18\x01 \x01(\t\x12\x0c\n\x04memo\x18\x02 \x01(\t\x12\'\n\x0bcommitments\x18\x03 \x03(\x0b2\x12.common.Commitment\x12%\n\nconditions\x18\x04 \x03(\x0b2\x11.common.Condition"1\n\x1aSendNewTransactionResponse\x12\x13\n\x0bcontract_id\x18\x01 \x01(\t"x\n\x18SendProposalAckedRequest\x12\r\n\x05owner\x18\x01 \x01(\t\x12\x13\n\x0bcontract_id\x18\x02 \x01(\t\x12\x0e\n\x06secret\x18\x03 \x01(\t\x12(\n\talgorithm\x18\x04 \x01(\x0e2\x15.common.HashAlgorithm"\x1b\n\x19SendProposalAckedResponse"?\n\x19SendProposalNackedRequest\x12\r\n\x05owner\x18\x01 \x01(\t\x12\x13\n\x0bcontract_id\x18\x02 \x01(\t"\x1c\n\x1aSendProposalNackedResponse"O\n\x19SendSecretRevealedRequest\x12\r\n\x05owner\x18\x01 \x01(\t\x12\x13\n\x0bcontract_id\x18\x02 \x01(\t\x12\x0e\n\x06secret\x18\x03 \x01(\t"\x1c\n\x1aSendSecretRevealedResponse2\xb1\x05\n\x0fBusinessService\x12C\n\x04List\x12\x1c.business_api.v1.ListRequest\x1a\x1d.business_api.v1.ListResponse\x12F\n\x05Clear\x12\x1d.business_api.v1.ClearRequest\x1a\x1e.business_api.v1.ClearResponse\x12X\n\x0bSendPayload\x12#.business_api.v1.SendPayloadRequest\x1a$.business_api.v1.SendPayloadResponse\x12m\n\x12SendNewTransaction\x12*.business_api.v1.SendNewTransactionRequest\x1a+.business_api.v1.SendNewTransactionResponse\x12j\n\x11SendProposalAcked\x12).business_api.v1.SendProposalAckedRequest\x1a*.business_api.v1.SendProposalAckedResponse\x12m\n\x12SendProposalNacked\x12*.business_api.v1.SendProposalNackedRequest\x1a+.business_api.v1.SendProposalNackedResponse\x12m\n\x12SendSecretRevealed\x12*.business_api.v1.SendSecretRevealedRequest\x1a+.business_api.v1.SendSecretRevealedResponseB;Z9github.com/knox-networks/grpc-sdks/sdk/go/business_api/v1b\x06proto3')
_LISTREQUEST = DESCRIPTOR.message_types_by_name['ListRequest']
_LISTRESPONSE = DESCRIPTOR.message_types_by_name['ListResponse']
_CLEARREQUEST = DESCRIPTOR.message_types_by_name['ClearRequest']
_CLEARRESPONSE = DESCRIPTOR.message_types_by_name['ClearResponse']
_SENDPAYLOADREQUEST = DESCRIPTOR.message_types_by_name['SendPayloadRequest']
_SENDPAYLOADRESPONSE = DESCRIPTOR.message_types_by_name['SendPayloadResponse']
_SENDNEWTRANSACTIONREQUEST = DESCRIPTOR.message_types_by_name['SendNewTransactionRequest']
_SENDNEWTRANSACTIONRESPONSE = DESCRIPTOR.message_types_by_name['SendNewTransactionResponse']
_SENDPROPOSALACKEDREQUEST = DESCRIPTOR.message_types_by_name['SendProposalAckedRequest']
_SENDPROPOSALACKEDRESPONSE = DESCRIPTOR.message_types_by_name['SendProposalAckedResponse']
_SENDPROPOSALNACKEDREQUEST = DESCRIPTOR.message_types_by_name['SendProposalNackedRequest']
_SENDPROPOSALNACKEDRESPONSE = DESCRIPTOR.message_types_by_name['SendProposalNackedResponse']
_SENDSECRETREVEALEDREQUEST = DESCRIPTOR.message_types_by_name['SendSecretRevealedRequest']
_SENDSECRETREVEALEDRESPONSE = DESCRIPTOR.message_types_by_name['SendSecretRevealedResponse']
ListRequest = _reflection.GeneratedProtocolMessageType('ListRequest', (_message.Message,), {'DESCRIPTOR': _LISTREQUEST, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(ListRequest)
ListResponse = _reflection.GeneratedProtocolMessageType('ListResponse', (_message.Message,), {'DESCRIPTOR': _LISTRESPONSE, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(ListResponse)
ClearRequest = _reflection.GeneratedProtocolMessageType('ClearRequest', (_message.Message,), {'DESCRIPTOR': _CLEARREQUEST, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(ClearRequest)
ClearResponse = _reflection.GeneratedProtocolMessageType('ClearResponse', (_message.Message,), {'DESCRIPTOR': _CLEARRESPONSE, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(ClearResponse)
SendPayloadRequest = _reflection.GeneratedProtocolMessageType('SendPayloadRequest', (_message.Message,), {'DESCRIPTOR': _SENDPAYLOADREQUEST, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(SendPayloadRequest)
SendPayloadResponse = _reflection.GeneratedProtocolMessageType('SendPayloadResponse', (_message.Message,), {'DESCRIPTOR': _SENDPAYLOADRESPONSE, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(SendPayloadResponse)
SendNewTransactionRequest = _reflection.GeneratedProtocolMessageType('SendNewTransactionRequest', (_message.Message,), {'DESCRIPTOR': _SENDNEWTRANSACTIONREQUEST, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(SendNewTransactionRequest)
SendNewTransactionResponse = _reflection.GeneratedProtocolMessageType('SendNewTransactionResponse', (_message.Message,), {'DESCRIPTOR': _SENDNEWTRANSACTIONRESPONSE, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(SendNewTransactionResponse)
SendProposalAckedRequest = _reflection.GeneratedProtocolMessageType('SendProposalAckedRequest', (_message.Message,), {'DESCRIPTOR': _SENDPROPOSALACKEDREQUEST, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(SendProposalAckedRequest)
SendProposalAckedResponse = _reflection.GeneratedProtocolMessageType('SendProposalAckedResponse', (_message.Message,), {'DESCRIPTOR': _SENDPROPOSALACKEDRESPONSE, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(SendProposalAckedResponse)
SendProposalNackedRequest = _reflection.GeneratedProtocolMessageType('SendProposalNackedRequest', (_message.Message,), {'DESCRIPTOR': _SENDPROPOSALNACKEDREQUEST, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(SendProposalNackedRequest)
SendProposalNackedResponse = _reflection.GeneratedProtocolMessageType('SendProposalNackedResponse', (_message.Message,), {'DESCRIPTOR': _SENDPROPOSALNACKEDRESPONSE, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(SendProposalNackedResponse)
SendSecretRevealedRequest = _reflection.GeneratedProtocolMessageType('SendSecretRevealedRequest', (_message.Message,), {'DESCRIPTOR': _SENDSECRETREVEALEDREQUEST, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(SendSecretRevealedRequest)
SendSecretRevealedResponse = _reflection.GeneratedProtocolMessageType('SendSecretRevealedResponse', (_message.Message,), {'DESCRIPTOR': _SENDSECRETREVEALEDRESPONSE, '__module__': 'business_api.v1.business_pb2'})
_sym_db.RegisterMessage(SendSecretRevealedResponse)
_BUSINESSSERVICE = DESCRIPTOR.services_by_name['BusinessService']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z9github.com/knox-networks/grpc-sdks/sdk/go/business_api/v1'
    _LISTREQUEST._serialized_start = 97
    _LISTREQUEST._serialized_end = 110
    _LISTRESPONSE._serialized_start = 112
    _LISTRESPONSE._serialized_end = 140
    _CLEARREQUEST._serialized_start = 142
    _CLEARREQUEST._serialized_end = 156
    _CLEARRESPONSE._serialized_start = 158
    _CLEARRESPONSE._serialized_end = 173
    _SENDPAYLOADREQUEST._serialized_start = 175
    _SENDPAYLOADREQUEST._serialized_end = 229
    _SENDPAYLOADRESPONSE._serialized_start = 231
    _SENDPAYLOADRESPONSE._serialized_end = 252
    _SENDNEWTRANSACTIONREQUEST._serialized_start = 255
    _SENDNEWTRANSACTIONREQUEST._serialized_end = 391
    _SENDNEWTRANSACTIONRESPONSE._serialized_start = 393
    _SENDNEWTRANSACTIONRESPONSE._serialized_end = 442
    _SENDPROPOSALACKEDREQUEST._serialized_start = 444
    _SENDPROPOSALACKEDREQUEST._serialized_end = 564
    _SENDPROPOSALACKEDRESPONSE._serialized_start = 566
    _SENDPROPOSALACKEDRESPONSE._serialized_end = 593
    _SENDPROPOSALNACKEDREQUEST._serialized_start = 595
    _SENDPROPOSALNACKEDREQUEST._serialized_end = 658
    _SENDPROPOSALNACKEDRESPONSE._serialized_start = 660
    _SENDPROPOSALNACKEDRESPONSE._serialized_end = 688
    _SENDSECRETREVEALEDREQUEST._serialized_start = 690
    _SENDSECRETREVEALEDREQUEST._serialized_end = 769
    _SENDSECRETREVEALEDRESPONSE._serialized_start = 771
    _SENDSECRETREVEALEDRESPONSE._serialized_end = 799
    _BUSINESSSERVICE._serialized_start = 802
    _BUSINESSSERVICE._serialized_end = 1491