"""Generated protocol buffer code."""
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x16common/v1/packet.proto\x12\x06common\x1a\x16common/v1/common.proto"\xeb\x01\n\x06Packet\x12\'\n\x0bpacket_type\x18\x01 \x01(\x0e2\x12.common.PacketType\x12\x0c\n\x04data\x18\x02 \x01(\x0c\x12\x0f\n\x07channel\x18\x03 \x01(\t\x12\'\n\x06sender\x18\x04 \x01(\x0b2\x17.common.DynamicVerifier\x12*\n\trecipient\x18\x05 \x01(\x0b2\x17.common.DynamicVerifier\x12\r\n\x05nonce\x18\x06 \x01(\x0c\x125\n\x12transaction_header\x18\x07 \x01(\x0b2\x19.common.TransactionHeader"\xcc\x01\n\x11TransactionHeader\x12(\n\x07starter\x18\x01 \x01(\x0b2\x17.common.DynamicVerifier\x12*\n\tresponder\x18\x02 \x01(\x0b2\x17.common.DynamicVerifier\x12\x0c\n\x04uetr\x18\x03 \x01(\t\x12\x11\n\ttimestamp\x18\x04 \x01(\x04\x12\x11\n\tsignature\x18\x05 \x01(\x0c\x12\x16\n\x0euser_reference\x18\x06 \x01(\t\x12\x15\n\rrelated_uetrs\x18\x07 \x03(\t"7\n\x10LockedPromissory\x12\x12\n\npromissory\x18\x01 \x01(\x0c\x12\x0f\n\x07lock_id\x18\x02 \x01(\t"D\n\x17LockPromissoryBatchData\x12\x0f\n\x07lock_id\x18\x01 \x01(\t\x12\x18\n\x10promissory_batch\x18\x02 \x03(\x0c"X\n\x1aLockPromissoryBatchAckData\x12\x0f\n\x07lock_id\x18\x01 \x01(\t\x12)\n\x04acks\x18\x02 \x03(\x0b2\x1b.common.LockedPromissoryAck"\\\n\x13LockedPromissoryAck\x12\x16\n\x0eprevious_owner\x18\x02 \x01(\t\x12\r\n\x05owner\x18\x03 \x01(\t\x12\x1e\n\x06amount\x18\x04 \x01(\x0b2\x0e.common.Amount"-\n\x11BatchedPromissory\x12\x18\n\x10promissory_batch\x18\x01 \x03(\x0c"6\n\x14SimplePaymentRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount"\x12\n\x10SimplePaymentAck"\xe4\x01\n\x11SimplePaymentNack\x12C\n\x10rejection_reason\x18\x01 \x01(\x0e2).common.SimplePaymentNack.RejectionReason\x12\r\n\x05other\x18\x02 \x01(\t"{\n\x0fRejectionReason\x12\x12\n\x0eREASON_UNKNOWN\x10\x00\x12\x10\n\x0cREASON_OTHER\x10\x01\x12\x17\n\x13REASON_AMOUNT_EMPTY\x10\x02\x12)\n%REASON_CHANGE_REQUEST_AMOUNT_MISMATCH\x10\x03"_\n\x15SimplePaymentResponse\x12\'\n\x0freceived_amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x1d\n\x15received_promissories\x18\x02 \x03(\t"~\n\x12FundsChangeRequest\x12$\n\x0cgross_amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12$\n\x0cexact_amount\x18\x02 \x01(\x0b2\x0e.common.Amount\x12\x1c\n\x04fees\x18\x03 \x01(\x0b2\x0e.common.Amount"\x10\n\x0eFundsChangeAck"\xd2\x01\n\x0fFundsChangeNack\x12A\n\x10rejection_reason\x18\x01 \x01(\x0e2\'.common.FundsChangeNack.RejectionReason\x12\r\n\x05other\x18\x02 \x01(\t"m\n\x0fRejectionReason\x12\x12\n\x0eREASON_UNKNOWN\x10\x00\x12\x10\n\x0cREASON_OTHER\x10\x01\x12\x17\n\x13REASON_AMOUNT_EMPTY\x10\x02\x12\x1b\n\x17REASON_AMOUNTS_MISMATCH\x10\x03"\xc4\x01\n\x11CancelTransaction\x12C\n\x10rejection_reason\x18\x01 \x01(\x0e2).common.CancelTransaction.RejectionReason\x12\r\n\x05other\x18\x02 \x01(\t"[\n\x0fRejectionReason\x12\x12\n\x0eREASON_UNKNOWN\x10\x00\x12\x10\n\x0cREASON_OTHER\x10\x01\x12"\n\x1eREASON_MISMATCHED_TRANSACTIONS\x10\x02"O\n\nCommitment\x12\x0e\n\x06sender\x18\x01 \x01(\t\x12\x11\n\trecipient\x18\x02 \x01(\t\x12\x1e\n\x06amount\x18\x03 \x01(\x0b2\x0e.common.Amount"#\n\x10TimeoutCondition\x12\x0f\n\x07timeout\x18\x01 \x01(\x04"0\n\rHashCondition\x12\x11\n\tpresenter\x18\x01 \x01(\t\x12\x0c\n\x04hash\x18\x02 \x01(\x0c"l\n\tCondition\x12+\n\x07timeout\x18\x01 \x01(\x0b2\x18.common.TimeoutConditionH\x00\x12%\n\x04hash\x18\x02 \x01(\x0b2\x15.common.HashConditionH\x00B\x0b\n\tcondition"\xed\x01\n\x08Contract\x12\x0b\n\x03urn\x18\x01 \x01(\t\x12\n\n\x02id\x18\x02 \x01(\t\x12\'\n\x0bcommitments\x18\x03 \x03(\x0b2\x12.common.Commitment\x12%\n\nconditions\x18\x04 \x03(\x0b2\x11.common.Condition\x126\n\x0bsignatories\x18\x05 \x03(\x0b2!.common.Contract.SignatoriesEntry\x12\x0c\n\x04memo\x18\x06 \x01(\t\x1a2\n\x10SignatoriesEntry\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\x0c:\x028\x01"$\n\x06Secret\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0e\n\x06secret\x18\x02 \x01(\t*\xc5\n\n\nPacketType\x12\x0b\n\x07Unknown\x10\x00\x12\x0f\n\x0bUnsupported\x10\x01\x12\x10\n\x0cAuthenticate\x10\x02\x12\x12\n\x0eReturnToSender\x10\x03\x12\x13\n\x0fTerminateStream\x10\x04\x12\r\n\tKeepAlive\x10\x05\x12\x13\n\x0fAcknowledgement\x10\x06\x12\x17\n\x13AuthenticateRequest\x10\x07\x12\x18\n\x14AuthenticateResponse\x10\x08\x12\x1b\n\x17AddAssociatedConnection\x10\t\x12\x1c\n\x18DropAssociatedConnection\x10\n\x12\x0e\n\nPromissory\x10d\x12\x17\n\x13AuthorizePromissory\x10e\x12!\n\x1dAuthorizeAndForwardPromissory\x10f\x12\x15\n\x11ArchivePromissory\x10g\x12\x15\n\x11PromissoryReceipt\x10h\x12\x12\n\x0eLockPromissory\x10i\x12\x15\n\x11LockPromissoryAck\x10j\x12\x1f\n\x1bAuthorizeLockedPromissories\x10k\x12\x17\n\x13AuthorizationStatus\x10l\x12\x12\n\x0eRetrieveAssets\x10m\x12\x13\n\x0fPromissoryBatch\x10n\x12\x17\n\x13LockPromissoryBatch\x10o\x12\x1a\n\x16LockPromissoryBatchAck\x10p\x12\x17\n\x13RetrieveAssetsBatch\x10q\x12\x1e\n\x1aRollbackLockedPromissories\x10r\x12!\n\x1dRollbackLockedPromissoriesAck\x10s\x12&\n"AuthorizeAndForwardPromissoryBatch\x10t\x12\x18\n\x14ValidationBlockBatch\x10u\x12\x18\n\x14AuthorizedBlockBatch\x10v\x12\x14\n\x0fIdentityRequest\x10\xc8\x01\x12\x15\n\x10IdentityResponse\x10\xc9\x01\x12\x12\n\rIdentityProof\x10\xca\x01\x12\x1d\n\x18NotaryNotarizePromissory\x10\xac\x02\x12\'\n"NotaryNotarizeAndForwardPromissory\x10\xad\x02\x12\x1f\n\x1aEmissaryConnectionIdentity\x10\x90\x03\x12\x15\n\x10ContractProposal\x10\xf4\x03\x12\x10\n\x0bContractAck\x10\xf5\x03\x12\x16\n\x11ContractSignature\x10\xf6\x03\x12\x15\n\x10ContractComplete\x10\xf7\x03\x12\x16\n\x11RemittanceRequest\x10\xbc\x05\x12\x17\n\x12RemittanceResponse\x10\xbd\x05\x12\x0e\n\tSPRequest\x10\xe8\x07\x12\n\n\x05SPAck\x10\xe9\x07\x12\x0b\n\x06SPNack\x10\xea\x07\x12\r\n\x08SPStatus\x10\xeb\x07\x12\x0c\n\x07SPStart\x10\xec\x07\x12\x0e\n\tFCRequest\x10\xcc\x08\x12\n\n\x05FCAck\x10\xcd\x08\x12\x0b\n\x06FCNack\x10\xce\x08\x12\x0e\n\tCPRequest\x10\x94\n\x12\n\n\x05CPAck\x10\x95\n\x12\x0b\n\x06CPNack\x10\x96\n\x12\x0f\n\nCPContract\x10\x97\n\x12\x12\n\rCPContractAck\x10\x98\n\x12\x17\n\x12CPAssetsAuthorized\x10\x99\n\x12\x0f\n\nCPComplete\x10\x9a\n\x12\x12\n\rRequestSecret\x10\x9c\n\x12\x12\n\rPresentSecret\x10\x9d\n\x12\x15\n\x10CPAssetsReturned\x10\x9e\n*\x93\x01\n\x0cContractType\x12\x1d\n\x19CONTRACT_TYPE_UNSPECIFIED\x10\x00\x12\x19\n\x15CONTRACT_TYPE_PAYMENT\x10\x01\x12\x16\n\x12CONTRACT_TYPE_SWAP\x10\x02\x12\x17\n\x13CONTRACT_TYPE_RELAY\x10\x03\x12\x18\n\x14CONTRACT_TYPE_CUSTOM\x10\x04B5Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1b\x06proto3')
_PACKETTYPE = DESCRIPTOR.enum_types_by_name['PacketType']
PacketType = enum_type_wrapper.EnumTypeWrapper(_PACKETTYPE)
_CONTRACTTYPE = DESCRIPTOR.enum_types_by_name['ContractType']
ContractType = enum_type_wrapper.EnumTypeWrapper(_CONTRACTTYPE)
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
LockPromissory = 105
LockPromissoryAck = 106
AuthorizeLockedPromissories = 107
AuthorizationStatus = 108
RetrieveAssets = 109
PromissoryBatch = 110
LockPromissoryBatch = 111
LockPromissoryBatchAck = 112
RetrieveAssetsBatch = 113
RollbackLockedPromissories = 114
RollbackLockedPromissoriesAck = 115
AuthorizeAndForwardPromissoryBatch = 116
ValidationBlockBatch = 117
AuthorizedBlockBatch = 118
IdentityRequest = 200
IdentityResponse = 201
IdentityProof = 202
NotaryNotarizePromissory = 300
NotaryNotarizeAndForwardPromissory = 301
EmissaryConnectionIdentity = 400
ContractProposal = 500
ContractAck = 501
ContractSignature = 502
ContractComplete = 503
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
CPRequest = 1300
CPAck = 1301
CPNack = 1302
CPContract = 1303
CPContractAck = 1304
CPAssetsAuthorized = 1305
CPComplete = 1306
RequestSecret = 1308
PresentSecret = 1309
CPAssetsReturned = 1310
CONTRACT_TYPE_UNSPECIFIED = 0
CONTRACT_TYPE_PAYMENT = 1
CONTRACT_TYPE_SWAP = 2
CONTRACT_TYPE_RELAY = 3
CONTRACT_TYPE_CUSTOM = 4
_PACKET = DESCRIPTOR.message_types_by_name['Packet']
_TRANSACTIONHEADER = DESCRIPTOR.message_types_by_name['TransactionHeader']
_LOCKEDPROMISSORY = DESCRIPTOR.message_types_by_name['LockedPromissory']
_LOCKPROMISSORYBATCHDATA = DESCRIPTOR.message_types_by_name['LockPromissoryBatchData']
_LOCKPROMISSORYBATCHACKDATA = DESCRIPTOR.message_types_by_name['LockPromissoryBatchAckData']
_LOCKEDPROMISSORYACK = DESCRIPTOR.message_types_by_name['LockedPromissoryAck']
_BATCHEDPROMISSORY = DESCRIPTOR.message_types_by_name['BatchedPromissory']
_SIMPLEPAYMENTREQUEST = DESCRIPTOR.message_types_by_name['SimplePaymentRequest']
_SIMPLEPAYMENTACK = DESCRIPTOR.message_types_by_name['SimplePaymentAck']
_SIMPLEPAYMENTNACK = DESCRIPTOR.message_types_by_name['SimplePaymentNack']
_SIMPLEPAYMENTRESPONSE = DESCRIPTOR.message_types_by_name['SimplePaymentResponse']
_FUNDSCHANGEREQUEST = DESCRIPTOR.message_types_by_name['FundsChangeRequest']
_FUNDSCHANGEACK = DESCRIPTOR.message_types_by_name['FundsChangeAck']
_FUNDSCHANGENACK = DESCRIPTOR.message_types_by_name['FundsChangeNack']
_CANCELTRANSACTION = DESCRIPTOR.message_types_by_name['CancelTransaction']
_COMMITMENT = DESCRIPTOR.message_types_by_name['Commitment']
_TIMEOUTCONDITION = DESCRIPTOR.message_types_by_name['TimeoutCondition']
_HASHCONDITION = DESCRIPTOR.message_types_by_name['HashCondition']
_CONDITION = DESCRIPTOR.message_types_by_name['Condition']
_CONTRACT = DESCRIPTOR.message_types_by_name['Contract']
_CONTRACT_SIGNATORIESENTRY = _CONTRACT.nested_types_by_name['SignatoriesEntry']
_SECRET = DESCRIPTOR.message_types_by_name['Secret']
_SIMPLEPAYMENTNACK_REJECTIONREASON = _SIMPLEPAYMENTNACK.enum_types_by_name['RejectionReason']
_FUNDSCHANGENACK_REJECTIONREASON = _FUNDSCHANGENACK.enum_types_by_name['RejectionReason']
_CANCELTRANSACTION_REJECTIONREASON = _CANCELTRANSACTION.enum_types_by_name['RejectionReason']
Packet = _reflection.GeneratedProtocolMessageType('Packet', (_message.Message,), {'DESCRIPTOR': _PACKET, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(Packet)
TransactionHeader = _reflection.GeneratedProtocolMessageType('TransactionHeader', (_message.Message,), {'DESCRIPTOR': _TRANSACTIONHEADER, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(TransactionHeader)
LockedPromissory = _reflection.GeneratedProtocolMessageType('LockedPromissory', (_message.Message,), {'DESCRIPTOR': _LOCKEDPROMISSORY, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(LockedPromissory)
LockPromissoryBatchData = _reflection.GeneratedProtocolMessageType('LockPromissoryBatchData', (_message.Message,), {'DESCRIPTOR': _LOCKPROMISSORYBATCHDATA, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(LockPromissoryBatchData)
LockPromissoryBatchAckData = _reflection.GeneratedProtocolMessageType('LockPromissoryBatchAckData', (_message.Message,), {'DESCRIPTOR': _LOCKPROMISSORYBATCHACKDATA, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(LockPromissoryBatchAckData)
LockedPromissoryAck = _reflection.GeneratedProtocolMessageType('LockedPromissoryAck', (_message.Message,), {'DESCRIPTOR': _LOCKEDPROMISSORYACK, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(LockedPromissoryAck)
BatchedPromissory = _reflection.GeneratedProtocolMessageType('BatchedPromissory', (_message.Message,), {'DESCRIPTOR': _BATCHEDPROMISSORY, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(BatchedPromissory)
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
Commitment = _reflection.GeneratedProtocolMessageType('Commitment', (_message.Message,), {'DESCRIPTOR': _COMMITMENT, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(Commitment)
TimeoutCondition = _reflection.GeneratedProtocolMessageType('TimeoutCondition', (_message.Message,), {'DESCRIPTOR': _TIMEOUTCONDITION, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(TimeoutCondition)
HashCondition = _reflection.GeneratedProtocolMessageType('HashCondition', (_message.Message,), {'DESCRIPTOR': _HASHCONDITION, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(HashCondition)
Condition = _reflection.GeneratedProtocolMessageType('Condition', (_message.Message,), {'DESCRIPTOR': _CONDITION, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(Condition)
Contract = _reflection.GeneratedProtocolMessageType('Contract', (_message.Message,), {'SignatoriesEntry': _reflection.GeneratedProtocolMessageType('SignatoriesEntry', (_message.Message,), {'DESCRIPTOR': _CONTRACT_SIGNATORIESENTRY, '__module__': 'common.v1.packet_pb2'}), 'DESCRIPTOR': _CONTRACT, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(Contract)
_sym_db.RegisterMessage(Contract.SignatoriesEntry)
Secret = _reflection.GeneratedProtocolMessageType('Secret', (_message.Message,), {'DESCRIPTOR': _SECRET, '__module__': 'common.v1.packet_pb2'})
_sym_db.RegisterMessage(Secret)
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z3github.com/knox-networks/grpc-sdks/sdk/go/common/v1'
    _CONTRACT_SIGNATORIESENTRY._options = None
    _CONTRACT_SIGNATORIESENTRY._serialized_options = b'8\x01'
    _PACKETTYPE._serialized_start = 2380
    _PACKETTYPE._serialized_end = 3729
    _CONTRACTTYPE._serialized_start = 3732
    _CONTRACTTYPE._serialized_end = 3879
    _PACKET._serialized_start = 59
    _PACKET._serialized_end = 294
    _TRANSACTIONHEADER._serialized_start = 297
    _TRANSACTIONHEADER._serialized_end = 501
    _LOCKEDPROMISSORY._serialized_start = 503
    _LOCKEDPROMISSORY._serialized_end = 558
    _LOCKPROMISSORYBATCHDATA._serialized_start = 560
    _LOCKPROMISSORYBATCHDATA._serialized_end = 628
    _LOCKPROMISSORYBATCHACKDATA._serialized_start = 630
    _LOCKPROMISSORYBATCHACKDATA._serialized_end = 718
    _LOCKEDPROMISSORYACK._serialized_start = 720
    _LOCKEDPROMISSORYACK._serialized_end = 812
    _BATCHEDPROMISSORY._serialized_start = 814
    _BATCHEDPROMISSORY._serialized_end = 859
    _SIMPLEPAYMENTREQUEST._serialized_start = 861
    _SIMPLEPAYMENTREQUEST._serialized_end = 915
    _SIMPLEPAYMENTACK._serialized_start = 917
    _SIMPLEPAYMENTACK._serialized_end = 935
    _SIMPLEPAYMENTNACK._serialized_start = 938
    _SIMPLEPAYMENTNACK._serialized_end = 1166
    _SIMPLEPAYMENTNACK_REJECTIONREASON._serialized_start = 1043
    _SIMPLEPAYMENTNACK_REJECTIONREASON._serialized_end = 1166
    _SIMPLEPAYMENTRESPONSE._serialized_start = 1168
    _SIMPLEPAYMENTRESPONSE._serialized_end = 1263
    _FUNDSCHANGEREQUEST._serialized_start = 1265
    _FUNDSCHANGEREQUEST._serialized_end = 1391
    _FUNDSCHANGEACK._serialized_start = 1393
    _FUNDSCHANGEACK._serialized_end = 1409
    _FUNDSCHANGENACK._serialized_start = 1412
    _FUNDSCHANGENACK._serialized_end = 1622
    _FUNDSCHANGENACK_REJECTIONREASON._serialized_start = 1513
    _FUNDSCHANGENACK_REJECTIONREASON._serialized_end = 1622
    _CANCELTRANSACTION._serialized_start = 1625
    _CANCELTRANSACTION._serialized_end = 1821
    _CANCELTRANSACTION_REJECTIONREASON._serialized_start = 1730
    _CANCELTRANSACTION_REJECTIONREASON._serialized_end = 1821
    _COMMITMENT._serialized_start = 1823
    _COMMITMENT._serialized_end = 1902
    _TIMEOUTCONDITION._serialized_start = 1904
    _TIMEOUTCONDITION._serialized_end = 1939
    _HASHCONDITION._serialized_start = 1941
    _HASHCONDITION._serialized_end = 1989
    _CONDITION._serialized_start = 1991
    _CONDITION._serialized_end = 2099
    _CONTRACT._serialized_start = 2102
    _CONTRACT._serialized_end = 2339
    _CONTRACT_SIGNATORIESENTRY._serialized_start = 2289
    _CONTRACT_SIGNATORIESENTRY._serialized_end = 2339
    _SECRET._serialized_start = 2341
    _SECRET._serialized_end = 2377