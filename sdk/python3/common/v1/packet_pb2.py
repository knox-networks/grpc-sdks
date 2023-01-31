"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x16common/v1/packet.proto\x12\x06common\x1a\x16common/v1/common.proto"\xeb\x01\n\x06Packet\x12\'\n\x0bpacket_type\x18\x01 \x01(\x0e2\x12.common.PacketType\x12\x0c\n\x04data\x18\x02 \x01(\x0c\x12\x0f\n\x07channel\x18\x03 \x01(\t\x12\'\n\x06sender\x18\x04 \x01(\x0b2\x17.common.DynamicVerifier\x12*\n\trecipient\x18\x05 \x01(\x0b2\x17.common.DynamicVerifier\x12\r\n\x05nonce\x18\x06 \x01(\x0c\x125\n\x12transaction_header\x18\x07 \x01(\x0b2\x19.common.TransactionHeader"\xcc\x01\n\x11TransactionHeader\x12(\n\x07starter\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x12*\n\tresponder\x18\x02 \x01(\x0b2\x17.common.DynamicVerifier\x12\x0c\n\x04uetr\x18\x03 \x01(\t\x12\x11\n\ttimestamp\x18\x04 \x01(\x04\x12\x11\n\tsignature\x18\x05 \x01(\x0c\x12\x16\n\x0euser_reference\x18\x06 \x01(\t\x12\x15\n\rrelated_uetrs\x18\x07 \x03(\t"6\n\x14SimplePaymentRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount"\x12\n\x10SimplePaymentAck"\xe4\x01\n\x11SimplePaymentNack\x12C\n\x10rejection_reason\x18\x01 \x01(\x0e2).common.SimplePaymentNack.RejectionReason\x12\r\n\x05other\x18\x02 \x01(\t"{\n\x0fRejectionReason\x12\x12\n\x0eREASON_UNKNOWN\x10\x00\x12\x10\n\x0cREASON_OTHER\x10\x01\x12\x17\n\x13REASON_AMOUNT_EMPTY\x10\x02\x12)\n%REASON_CHANGE_REQUEST_AMOUNT_MISMATCH\x10\x03"_\n\x15SimplePaymentResponse\x12\'\n\x0freceived_amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x1d\n\x15received_promissories\x18\x02 \x03(\t"~\n\x12FundsChangeRequest\x12$\n\x0cgross_amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12$\n\x0cexact_amount\x18\x02 \x01(\x0b2\x0e.common.Amount\x12\x1c\n\x04fees\x18\x03 \x01(\x0b2\x0e.common.Amount"\x10\n\x0eFundsChangeAck"\xd2\x01\n\x0fFundsChangeNack\x12A\n\x10rejection_reason\x18\x01 \x01(\x0e2\'.common.FundsChangeNack.RejectionReason\x12\r\n\x05other\x18\x02 \x01(\t"m\n\x0fRejectionReason\x12\x12\n\x0eREASON_UNKNOWN\x10\x00\x12\x10\n\x0cREASON_OTHER\x10\x01\x12\x17\n\x13REASON_AMOUNT_EMPTY\x10\x02\x12\x1b\n\x17REASON_AMOUNTS_MISMATCH\x10\x03"\xc4\x01\n\x11CancelTransaction\x12C\n\x10rejection_reason\x18\x01 \x01(\x0e2).common.CancelTransaction.RejectionReason\x12\r\n\x05other\x18\x02 \x01(\t"[\n\x0fRejectionReason\x12\x12\n\x0eREASON_UNKNOWN\x10\x00\x12\x10\n\x0cREASON_OTHER\x10\x01\x12"\n\x1eREASON_MISMATCHED_TRANSACTIONS\x10\x02*\xbb\x05\n\nPacketType\x12\x0b\n\x07Unknown\x10\x00\x12\x0f\n\x0bUnsupported\x10\x01\x12\x10\n\x0cAuthenticate\x10\x02\x12\x12\n\x0eReturnToSender\x10\x03\x12\x13\n\x0fTerminateStream\x10\x04\x12\r\n\tKeepAlive\x10\x05\x12\x13\n\x0fAcknowledgement\x10\x06\x12\x17\n\x13AuthenticateRequest\x10\x07\x12\x18\n\x14AuthenticateResponse\x10\x08\x12\x1b\n\x17AddAssociatedConnection\x10\t\x12\x1c\n\x18DropAssociatedConnection\x10\n\x12\x0e\n\nPromissory\x10d\x12\x17\n\x13AuthorizePromissory\x10e\x12!\n\x1dAuthorizeAndForwardPromissory\x10f\x12\x15\n\x11ArchivePromissory\x10g\x12\x15\n\x11PromissoryReceipt\x10h\x12\x14\n\x0fIdentityRequest\x10\xc8\x01\x12\x15\n\x10IdentityResponse\x10\xc9\x01\x12\x12\n\rIdentityProof\x10\xca\x01\x12\x1d\n\x18NotaryNotarizePromissory\x10\xac\x02\x12\'\n"NotaryNotarizeAndForwardPromissory\x10\xad\x02\x12\x1f\n\x1aEmissaryConnectionIdentity\x10\x90\x03\x12\x16\n\x11RemittanceRequest\x10\xbc\x05\x12\x17\n\x12RemittanceResponse\x10\xbd\x05\x12\x0e\n\tSPRequest\x10\xe8\x07\x12\n\n\x05SPAck\x10\xe9\x07\x12\x0b\n\x06SPNack\x10\xea\x07\x12\r\n\x08SPStatus\x10\xeb\x07\x12\x0c\n\x07SPStart\x10\xec\x07\x12\x0e\n\tFCRequest\x10\xcc\x08\x12\n\n\x05FCAck\x10\xcd\x08\x12\x0b\n\x06FCNack\x10\xce\x08B5Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1b\x06proto3')
_PACKETTYPE = DESCRIPTOR.enum_types_by_name['PacketType']
PacketType = enum_type_wrapper.EnumTypeWrapper(_PACKETTYPE)
Unknown = 0
Unsupported = 1
Authenticate = 2
ReturnToSender = 3
TerminateStream = 4
KeepAlive = 5
Acknowledgement = 6
AuthenticateRequest = 7
AuthenticateResponse = 8
AddAssociatedConnection = 9
DropAssociatedConnection = 10
Promissory = 100
AuthorizePromissory = 101
AuthorizeAndForwardPromissory = 102
ArchivePromissory = 103
PromissoryReceipt = 104
IdentityRequest = 200
IdentityResponse = 201
IdentityProof = 202
NotaryNotarizePromissory = 300
NotaryNotarizeAndForwardPromissory = 301
EmissaryConnectionIdentity = 400
RemittanceRequest = 700
RemittanceResponse = 701
SPRequest = 1000
SPAck = 1001
SPNack = 1002
SPStatus = 1003
SPStart = 1004
FCRequest = 1100
FCAck = 1101
FCNack = 1102
_PACKET = DESCRIPTOR.message_types_by_name['Packet']
_TRANSACTIONHEADER = DESCRIPTOR.message_types_by_name['TransactionHeader']
_SIMPLEPAYMENTREQUEST = DESCRIPTOR.message_types_by_name['SimplePaymentRequest']
_SIMPLEPAYMENTACK = DESCRIPTOR.message_types_by_name['SimplePaymentAck']
_SIMPLEPAYMENTNACK = DESCRIPTOR.message_types_by_name['SimplePaymentNack']
_SIMPLEPAYMENTRESPONSE = DESCRIPTOR.message_types_by_name['SimplePaymentResponse']
_FUNDSCHANGEREQUEST = DESCRIPTOR.message_types_by_name['FundsChangeRequest']
_FUNDSCHANGEACK = DESCRIPTOR.message_types_by_name['FundsChangeAck']
_FUNDSCHANGENACK = DESCRIPTOR.message_types_by_name['FundsChangeNack']
_CANCELTRANSACTION = DESCRIPTOR.message_types_by_name['CancelTransaction']
_SIMPLEPAYMENTNACK_REJECTIONREASON = _SIMPLEPAYMENTNACK.enum_types_by_name['RejectionReason']
_FUNDSCHANGENACK_REJECTIONREASON = _FUNDSCHANGENACK.enum_types_by_name['RejectionReason']
_CANCELTRANSACTION_REJECTIONREASON = _CANCELTRANSACTION.enum_types_by_name['RejectionReason']
Packet = _reflection.GeneratedProtocolMessageType('Packet', (_message.Message,), {'DESCRIPTOR': _PACKET, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(Packet)
TransactionHeader = _reflection.GeneratedProtocolMessageType('TransactionHeader', (_message.Message,), {'DESCRIPTOR': _TRANSACTIONHEADER, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(TransactionHeader)
SimplePaymentRequest = _reflection.GeneratedProtocolMessageType('SimplePaymentRequest', (_message.Message,), {'DESCRIPTOR': _SIMPLEPAYMENTREQUEST, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(SimplePaymentRequest)
SimplePaymentAck = _reflection.GeneratedProtocolMessageType('SimplePaymentAck', (_message.Message,), {'DESCRIPTOR': _SIMPLEPAYMENTACK, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(SimplePaymentAck)
SimplePaymentNack = _reflection.GeneratedProtocolMessageType('SimplePaymentNack', (_message.Message,), {'DESCRIPTOR': _SIMPLEPAYMENTNACK, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(SimplePaymentNack)
SimplePaymentResponse = _reflection.GeneratedProtocolMessageType('SimplePaymentResponse', (_message.Message,), {'DESCRIPTOR': _SIMPLEPAYMENTRESPONSE, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(SimplePaymentResponse)
FundsChangeRequest = _reflection.GeneratedProtocolMessageType('FundsChangeRequest', (_message.Message,), {'DESCRIPTOR': _FUNDSCHANGEREQUEST, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(FundsChangeRequest)
FundsChangeAck = _reflection.GeneratedProtocolMessageType('FundsChangeAck', (_message.Message,), {'DESCRIPTOR': _FUNDSCHANGEACK, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(FundsChangeAck)
FundsChangeNack = _reflection.GeneratedProtocolMessageType('FundsChangeNack', (_message.Message,), {'DESCRIPTOR': _FUNDSCHANGENACK, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(FundsChangeNack)
CancelTransaction = _reflection.GeneratedProtocolMessageType('CancelTransaction', (_message.Message,), {'DESCRIPTOR': _CANCELTRANSACTION, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(CancelTransaction)
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1'
    _PACKETTYPE._serialized_start = 1466
    _PACKETTYPE._serialized_end = 2165
    _PACKET._serialized_start = 59
    _PACKET._serialized_end = 294
    _TRANSACTIONHEADER._serialized_start = 297
    _TRANSACTIONHEADER._serialized_end = 501
    _SIMPLEPAYMENTREQUEST._serialized_start = 503
    _SIMPLEPAYMENTREQUEST._serialized_end = 557
    _SIMPLEPAYMENTACK._serialized_start = 559
    _SIMPLEPAYMENTACK._serialized_end = 577
    _SIMPLEPAYMENTNACK._serialized_start = 580
    _SIMPLEPAYMENTNACK._serialized_end = 808
    _SIMPLEPAYMENTNACK_REJECTIONREASON._serialized_start = 685
    _SIMPLEPAYMENTNACK_REJECTIONREASON._serialized_end = 808
    _SIMPLEPAYMENTRESPONSE._serialized_start = 810
    _SIMPLEPAYMENTRESPONSE._serialized_end = 905
    _FUNDSCHANGEREQUEST._serialized_start = 907
    _FUNDSCHANGEREQUEST._serialized_end = 1033
    _FUNDSCHANGEACK._serialized_start = 1035
    _FUNDSCHANGEACK._serialized_end = 1051
    _FUNDSCHANGENACK._serialized_start = 1054
    _FUNDSCHANGENACK._serialized_end = 1264
    _FUNDSCHANGENACK_REJECTIONREASON._serialized_start = 1155
    _FUNDSCHANGENACK_REJECTIONREASON._serialized_end = 1264
    _CANCELTRANSACTION._serialized_start = 1267
    _CANCELTRANSACTION._serialized_end = 1463
    _CANCELTRANSACTION_REJECTIONREASON._serialized_start = 1372
    _CANCELTRANSACTION_REJECTIONREASON._serialized_end = 1463