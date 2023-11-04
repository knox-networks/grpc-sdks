"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import packet_pb2 as common_dot_v1_dot_packet__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x14common/v1/amqp.proto\x12\x06common\x1a\x16common/v1/packet.proto"B\n\x07Payload\x12)\n\x0cpayload_type\x18\x01 \x01(\x0e2\x13.common.PayloadType\x12\x0c\n\x04data\x18\x02 \x01(\x0c"G\n\x12NewTransactionData\x12"\n\x08contract\x18\x01 \x01(\x0b2\x10.common.Contract\x12\r\n\x05owner\x18\x02 \x01(\t"X\n\x0fAckProposalData\x12"\n\x08contract\x18\x01 \x01(\x0b2\x10.common.Contract\x12\r\n\x05owner\x18\x02 \x01(\t\x12\x12\n\noriginator\x18\x03 \x01(\t"\\\n\x13ProposalAckNackData\x12\r\n\x05owner\x18\x01 \x01(\t\x12\x12\n\noriginator\x18\x02 \x01(\t\x12"\n\x08contract\x18\x03 \x01(\x0b2\x10.common.Contract"6\n\x10RevealSecretData\x12\x13\n\x0bcontract_id\x18\x01 \x01(\t\x12\r\n\x05owner\x18\x02 \x01(\t"H\n\x12SecretRevealedData\x12\r\n\x05owner\x18\x01 \x01(\t\x12\x13\n\x0bcontract_id\x18\x02 \x01(\t\x12\x0e\n\x06secret\x18\x03 \x01(\t"G\n\x11ReceiveSecretData\x12\x13\n\x0bcontract_id\x18\x01 \x01(\t\x12\r\n\x05owner\x18\x02 \x01(\t\x12\x0e\n\x06secret\x18\x03 \x01(\t"g\n\x16ReceiveTerminationData\x12\x13\n\x0bcontract_id\x18\x01 \x01(\t\x12\r\n\x05owner\x18\x02 \x01(\t\x12)\n\x06result\x18\x03 \x01(\x0e2\x19.common.TerminationResult*\xaa\x01\n\x0bPayloadType\x12\x12\n\x0eNewTransaction\x10\x00\x12\x0f\n\x0bAckProposal\x10\x01\x12\x11\n\rProposalAcked\x10\x02\x12\x12\n\x0eProposalNacked\x10\x03\x12\x10\n\x0cRevealSecret\x10\x04\x12\x12\n\x0eSecretRevealed\x10\x05\x12\x11\n\rReceiveSecret\x10\x06\x12\x16\n\x12ReceiveTermination\x10\x07*H\n\x11TerminationResult\x12\x12\n\x0eContractFailed\x10\x00\x12\r\n\tTxnFailed\x10\x01\x12\x10\n\x0cTxnCompleted\x10\x02B5Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1b\x06proto3')
_PAYLOADTYPE = DESCRIPTOR.enum_types_by_name['PayloadType']
PayloadType = enum_type_wrapper.EnumTypeWrapper(_PAYLOADTYPE)
_TERMINATIONRESULT = DESCRIPTOR.enum_types_by_name['TerminationResult']
TerminationResult = enum_type_wrapper.EnumTypeWrapper(_TERMINATIONRESULT)
NewTransaction = 0
AckProposal = 1
ProposalAcked = 2
ProposalNacked = 3
RevealSecret = 4
SecretRevealed = 5
ReceiveSecret = 6
ReceiveTermination = 7
ContractFailed = 0
TxnFailed = 1
TxnCompleted = 2
_PAYLOAD = DESCRIPTOR.message_types_by_name['Payload']
_NEWTRANSACTIONDATA = DESCRIPTOR.message_types_by_name['NewTransactionData']
_ACKPROPOSALDATA = DESCRIPTOR.message_types_by_name['AckProposalData']
_PROPOSALACKNACKDATA = DESCRIPTOR.message_types_by_name['ProposalAckNackData']
_REVEALSECRETDATA = DESCRIPTOR.message_types_by_name['RevealSecretData']
_SECRETREVEALEDDATA = DESCRIPTOR.message_types_by_name['SecretRevealedData']
_RECEIVESECRETDATA = DESCRIPTOR.message_types_by_name['ReceiveSecretData']
_RECEIVETERMINATIONDATA = DESCRIPTOR.message_types_by_name['ReceiveTerminationData']
Payload = _reflection.GeneratedProtocolMessageType('Payload', (_message.Message,), {'DESCRIPTOR': _PAYLOAD, '__module__': 'common.v1.amqp_pb2'})
_sym_db.RegisterMessage(Payload)
NewTransactionData = _reflection.GeneratedProtocolMessageType('NewTransactionData', (_message.Message,), {'DESCRIPTOR': _NEWTRANSACTIONDATA, '__module__': 'common.v1.amqp_pb2'})
_sym_db.RegisterMessage(NewTransactionData)
AckProposalData = _reflection.GeneratedProtocolMessageType('AckProposalData', (_message.Message,), {'DESCRIPTOR': _ACKPROPOSALDATA, '__module__': 'common.v1.amqp_pb2'})
_sym_db.RegisterMessage(AckProposalData)
ProposalAckNackData = _reflection.GeneratedProtocolMessageType('ProposalAckNackData', (_message.Message,), {'DESCRIPTOR': _PROPOSALACKNACKDATA, '__module__': 'common.v1.amqp_pb2'})
_sym_db.RegisterMessage(ProposalAckNackData)
RevealSecretData = _reflection.GeneratedProtocolMessageType('RevealSecretData', (_message.Message,), {'DESCRIPTOR': _REVEALSECRETDATA, '__module__': 'common.v1.amqp_pb2'})
_sym_db.RegisterMessage(RevealSecretData)
SecretRevealedData = _reflection.GeneratedProtocolMessageType('SecretRevealedData', (_message.Message,), {'DESCRIPTOR': _SECRETREVEALEDDATA, '__module__': 'common.v1.amqp_pb2'})
_sym_db.RegisterMessage(SecretRevealedData)
ReceiveSecretData = _reflection.GeneratedProtocolMessageType('ReceiveSecretData', (_message.Message,), {'DESCRIPTOR': _RECEIVESECRETDATA, '__module__': 'common.v1.amqp_pb2'})
_sym_db.RegisterMessage(ReceiveSecretData)
ReceiveTerminationData = _reflection.GeneratedProtocolMessageType('ReceiveTerminationData', (_message.Message,), {'DESCRIPTOR': _RECEIVETERMINATIONDATA, '__module__': 'common.v1.amqp_pb2'})
_sym_db.RegisterMessage(ReceiveTerminationData)
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1'
    _PAYLOADTYPE._serialized_start = 690
    _PAYLOADTYPE._serialized_end = 860
    _TERMINATIONRESULT._serialized_start = 862
    _TERMINATIONRESULT._serialized_end = 934
    _PAYLOAD._serialized_start = 56
    _PAYLOAD._serialized_end = 122
    _NEWTRANSACTIONDATA._serialized_start = 124
    _NEWTRANSACTIONDATA._serialized_end = 195
    _ACKPROPOSALDATA._serialized_start = 197
    _ACKPROPOSALDATA._serialized_end = 285
    _PROPOSALACKNACKDATA._serialized_start = 287
    _PROPOSALACKNACKDATA._serialized_end = 379
    _REVEALSECRETDATA._serialized_start = 381
    _REVEALSECRETDATA._serialized_end = 435
    _SECRETREVEALEDDATA._serialized_start = 437
    _SECRETREVEALEDDATA._serialized_end = 509
    _RECEIVESECRETDATA._serialized_start = 511
    _RECEIVESECRETDATA._serialized_end = 582
    _RECEIVETERMINATIONDATA._serialized_start = 584
    _RECEIVETERMINATIONDATA._serialized_end = 687