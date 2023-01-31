"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1awallet_api/v1/wallet.proto\x12\rwallet_api.v1\x1a\x16common/v1/common.proto"o\n\x0fTransferRequest\x12\x16\n\x0efrom_wallet_id\x18\x01 \x01(\t\x12\x1c\n\x14recipient_public_key\x18\x02 \x01(\t\x12\x15\n\rpromissory_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t"\x12\n\x10TransferResponse"(\n\x12CreateOwnerRequest\x12\x12\n\nowner_name\x18\x01 \x01(\t"L\n\x13CreateOwnerResponse\x12\x10\n\x08owner_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\x12\n\nowner_name\x18\x03 \x01(\t";\n\x13CreateWalletRequest\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t"_\n\x14CreateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x13\n\x0bwallet_name\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03did\x18\x04 \x01(\t"F\n\x12ListWalletsRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x0f\n\x07api_key\x18\x03 \x01(\t"\xc1\x01\n\x13ListWalletsResponse\x12:\n\x07wallets\x18\x01 \x03(\x0b2).wallet_api.v1.ListWalletsResponse.Wallet\x12\x1b\n\x13total_owned_wallets\x18\x02 \x01(\r\x1aQ\n\x06Wallet\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03did\x18\x04 \x01(\t"?\n\x19ListWalletBalancesRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t"=\n\x1aListWalletBalancesResponse\x12\x1f\n\x07balance\x18\x01 \x03(\x0b2\x0e.common.Amount"W\n\x10ListFundsRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x11\n\twallet_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t"\x8d\x03\n\x11ListFundsResponse\x124\n\x05funds\x18\x01 \x03(\x0b2%.wallet_api.v1.ListFundsResponse.Fund\x12\x19\n\x11total_owned_funds\x18\x02 \x01(\r\x1a\xca\x01\n\nPromissory\x12\x0f\n\x07version\x18\x01 \x01(\t\x12\n\n\x02id\x18\x03 \x01(\t\x12\x10\n\x08currency\x18\x04 \x01(\t\x12\x0e\n\x06amount\x18\x05 \x01(\x03\x12\x10\n\x08decimals\x18\x06 \x01(\r\x12\x11\n\tauthority\x18\x07 \x01(\t\x12\x0e\n\x06issuer\x18\x08 \x01(\t\x12\x15\n\rissuance_date\x18\t \x01(\t\x12\x13\n\x0bblock_depth\x18\n \x01(\r\x12\x1c\n\x14authorized_signature\x18\x0b \x01(\x0c\x1aZ\n\x04Fund\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12?\n\npromissory\x18\x02 \x01(\x0b2+.wallet_api.v1.ListFundsResponse.Promissory"j\n\rRedeemRequest\x12\x16\n\x0efrom_wallet_id\x18\x01 \x01(\t\x12\x19\n\x11issuer_public_key\x18\x02 \x01(\t\x12\x15\n\rpromissory_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t"3\n\x0eRedeemResponse\x12!\n\tnew_limit\x18\x01 \x01(\x0b2\x0e.common.Amount"]\n\x16ListCredentialsRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x11\n\twallet_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t"O\n\x17ListCredentialsResponse\x12\x13\n\x0bcredentials\x18\x01 \x03(\t\x12\x1f\n\x17total_owned_credentials\x18\x02 \x01(\r"S\n\x14AddCredentialRequest\x12\x17\n\x0fcredential_type\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"+\n\x15AddCredentialResponse\x12\x12\n\ncredential\x18\x01 \x01(\t"<\n\x16AssociateWalletRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t"M\n\x17AssociateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0b\n\x03did\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t"V\n\x1fCreateAndAssociateWalletRequest\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\r\n\x05token\x18\x03 \x01(\t"k\n CreateAndAssociateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x13\n\x0bwallet_name\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03did\x18\x04 \x01(\t"S\n\x1aRequestPresentationRequest\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x17\n\x0fcredential_type\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"3\n\x1bRequestPresentationResponse\x12\x14\n\x0cpresentation\x18\x01 \x01(\t"\x8c\x01\n\x1bPrepareSimplePaymentRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x11\n\trecipient\x18\x02 \x01(\t\x12\x16\n\x0euser_reference\x18\x03 \x01(\t\x12\x11\n\twallet_id\x18\x04 \x01(\t\x12\x0f\n\x07api_key\x18\x05 \x01(\t"\xdd\x01\n\x1cPrepareSimplePaymentResponse\x12B\n\x06status\x18\x01 \x01(\x0e22.wallet_api.v1.PrepareSimplePaymentResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t"k\n\x06Status\x12\x16\n\x12STATUS_UNSPECIFIED\x10\x00\x12\x12\n\x0eSTATUS_SUCCESS\x10\x01\x12\x16\n\x12STATUS_NEED_CHANGE\x10\x02\x12\x1d\n\x19STATUS_INSUFFICIENT_FUNDS\x10\x03"w\n\x19PrepareFundsChangeRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x16\n\x0euser_reference\x18\x04 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"\xe0\x01\n\x1aPrepareFundsChangeResponse\x12@\n\x06status\x18\x01 \x01(\x0e20.wallet_api.v1.PrepareFundsChangeResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t"r\n\x06Status\x12\x16\n\x12STATUS_UNSPECIFIED\x10\x00\x12\x12\n\x0eSTATUS_SUCCESS\x10\x01\x12\x1d\n\x19STATUS_INSUFFICIENT_FUNDS\x10\x02\x12\x1d\n\x19STATUS_FUNDS_PRESENT_NOOP\x10\x03"I\n\x15GetTransactionRequest\x12\x0c\n\x04uetr\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"B\n\x16GetTransactionResponse\x12(\n\x0btransaction\x18\x01 \x01(\x0b2\x13.common.Transaction"6\n\x17ListTransactionsRequest\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t"E\n\x18ListTransactionsResponse\x12)\n\x0ctransactions\x18\x01 \x03(\x0b2\x13.common.Transaction"Q\n\x1dGetTransactionSnapshotRequest\x12\x0c\n\x04uetr\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"O\n\x1eGetTransactionSnapshotResponse\x12-\n\x08snapshot\x18\x01 \x01(\x0b2\x1b.common.TransactionSnapshot"S\n\x1fListTransactionSnapshotsRequest\x12\x0c\n\x04uetr\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"R\n ListTransactionSnapshotsResponse\x12.\n\tsnapshots\x18\x01 \x03(\x0b2\x1b.common.TransactionSnapshot2\xfe\x0c\n\x06Wallet\x12K\n\x08Transfer\x12\x1e.wallet_api.v1.TransferRequest\x1a\x1f.wallet_api.v1.TransferResponse\x12T\n\x0bCreateOwner\x12!.wallet_api.v1.CreateOwnerRequest\x1a".wallet_api.v1.CreateOwnerResponse\x12W\n\x0cCreateWallet\x12".wallet_api.v1.CreateWalletRequest\x1a#.wallet_api.v1.CreateWalletResponse\x12T\n\x0bListWallets\x12!.wallet_api.v1.ListWalletsRequest\x1a".wallet_api.v1.ListWalletsResponse\x12N\n\tListFunds\x12\x1f.wallet_api.v1.ListFundsRequest\x1a .wallet_api.v1.ListFundsResponse\x12i\n\x12ListWalletBalances\x12(.wallet_api.v1.ListWalletBalancesRequest\x1a).wallet_api.v1.ListWalletBalancesResponse\x12E\n\x06Redeem\x12\x1c.wallet_api.v1.RedeemRequest\x1a\x1d.wallet_api.v1.RedeemResponse\x12`\n\x0fListCredentials\x12%.wallet_api.v1.ListCredentialsRequest\x1a&.wallet_api.v1.ListCredentialsResponse\x12Z\n\rAddCredential\x12#.wallet_api.v1.AddCredentialRequest\x1a$.wallet_api.v1.AddCredentialResponse\x12`\n\x0fAssociateWallet\x12%.wallet_api.v1.AssociateWalletRequest\x1a&.wallet_api.v1.AssociateWalletResponse\x12l\n\x13RequestPresentation\x12).wallet_api.v1.RequestPresentationRequest\x1a*.wallet_api.v1.RequestPresentationResponse\x12o\n\x14PrepareSimplePayment\x12*.wallet_api.v1.PrepareSimplePaymentRequest\x1a+.wallet_api.v1.PrepareSimplePaymentResponse\x12i\n\x12PrepareFundsChange\x12(.wallet_api.v1.PrepareFundsChangeRequest\x1a).wallet_api.v1.PrepareFundsChangeResponse\x12]\n\x0eGetTransaction\x12$.wallet_api.v1.GetTransactionRequest\x1a%.wallet_api.v1.GetTransactionResponse\x12c\n\x10ListTransactions\x12&.wallet_api.v1.ListTransactionsRequest\x1a\'.wallet_api.v1.ListTransactionsResponse\x12u\n\x16GetTransactionSnapshot\x12,.wallet_api.v1.GetTransactionSnapshotRequest\x1a-.wallet_api.v1.GetTransactionSnapshotResponse\x12{\n\x18ListTransactionSnapshots\x12..wallet_api.v1.ListTransactionSnapshotsRequest\x1a/.wallet_api.v1.ListTransactionSnapshotsResponseB9Z7github.com/knox-networks/grpc-sdks/sdk/go/wallet_api/v1b\x06proto3')
_TRANSFERREQUEST = DESCRIPTOR.message_types_by_name['TransferRequest']
_TRANSFERRESPONSE = DESCRIPTOR.message_types_by_name['TransferResponse']
_CREATEOWNERREQUEST = DESCRIPTOR.message_types_by_name['CreateOwnerRequest']
_CREATEOWNERRESPONSE = DESCRIPTOR.message_types_by_name['CreateOwnerResponse']
_CREATEWALLETREQUEST = DESCRIPTOR.message_types_by_name['CreateWalletRequest']
_CREATEWALLETRESPONSE = DESCRIPTOR.message_types_by_name['CreateWalletResponse']
_LISTWALLETSREQUEST = DESCRIPTOR.message_types_by_name['ListWalletsRequest']
_LISTWALLETSRESPONSE = DESCRIPTOR.message_types_by_name['ListWalletsResponse']
_LISTWALLETSRESPONSE_WALLET = _LISTWALLETSRESPONSE.nested_types_by_name['Wallet']
_LISTWALLETBALANCESREQUEST = DESCRIPTOR.message_types_by_name['ListWalletBalancesRequest']
_LISTWALLETBALANCESRESPONSE = DESCRIPTOR.message_types_by_name['ListWalletBalancesResponse']
_LISTFUNDSREQUEST = DESCRIPTOR.message_types_by_name['ListFundsRequest']
_LISTFUNDSRESPONSE = DESCRIPTOR.message_types_by_name['ListFundsResponse']
_LISTFUNDSRESPONSE_PROMISSORY = _LISTFUNDSRESPONSE.nested_types_by_name['Promissory']
_LISTFUNDSRESPONSE_FUND = _LISTFUNDSRESPONSE.nested_types_by_name['Fund']
_REDEEMREQUEST = DESCRIPTOR.message_types_by_name['RedeemRequest']
_REDEEMRESPONSE = DESCRIPTOR.message_types_by_name['RedeemResponse']
_LISTCREDENTIALSREQUEST = DESCRIPTOR.message_types_by_name['ListCredentialsRequest']
_LISTCREDENTIALSRESPONSE = DESCRIPTOR.message_types_by_name['ListCredentialsResponse']
_ADDCREDENTIALREQUEST = DESCRIPTOR.message_types_by_name['AddCredentialRequest']
_ADDCREDENTIALRESPONSE = DESCRIPTOR.message_types_by_name['AddCredentialResponse']
_ASSOCIATEWALLETREQUEST = DESCRIPTOR.message_types_by_name['AssociateWalletRequest']
_ASSOCIATEWALLETRESPONSE = DESCRIPTOR.message_types_by_name['AssociateWalletResponse']
_CREATEANDASSOCIATEWALLETREQUEST = DESCRIPTOR.message_types_by_name['CreateAndAssociateWalletRequest']
_CREATEANDASSOCIATEWALLETRESPONSE = DESCRIPTOR.message_types_by_name['CreateAndAssociateWalletResponse']
_REQUESTPRESENTATIONREQUEST = DESCRIPTOR.message_types_by_name['RequestPresentationRequest']
_REQUESTPRESENTATIONRESPONSE = DESCRIPTOR.message_types_by_name['RequestPresentationResponse']
_PREPARESIMPLEPAYMENTREQUEST = DESCRIPTOR.message_types_by_name['PrepareSimplePaymentRequest']
_PREPARESIMPLEPAYMENTRESPONSE = DESCRIPTOR.message_types_by_name['PrepareSimplePaymentResponse']
_PREPAREFUNDSCHANGEREQUEST = DESCRIPTOR.message_types_by_name['PrepareFundsChangeRequest']
_PREPAREFUNDSCHANGERESPONSE = DESCRIPTOR.message_types_by_name['PrepareFundsChangeResponse']
_GETTRANSACTIONREQUEST = DESCRIPTOR.message_types_by_name['GetTransactionRequest']
_GETTRANSACTIONRESPONSE = DESCRIPTOR.message_types_by_name['GetTransactionResponse']
_LISTTRANSACTIONSREQUEST = DESCRIPTOR.message_types_by_name['ListTransactionsRequest']
_LISTTRANSACTIONSRESPONSE = DESCRIPTOR.message_types_by_name['ListTransactionsResponse']
_GETTRANSACTIONSNAPSHOTREQUEST = DESCRIPTOR.message_types_by_name['GetTransactionSnapshotRequest']
_GETTRANSACTIONSNAPSHOTRESPONSE = DESCRIPTOR.message_types_by_name['GetTransactionSnapshotResponse']
_LISTTRANSACTIONSNAPSHOTSREQUEST = DESCRIPTOR.message_types_by_name['ListTransactionSnapshotsRequest']
_LISTTRANSACTIONSNAPSHOTSRESPONSE = DESCRIPTOR.message_types_by_name['ListTransactionSnapshotsResponse']
_PREPARESIMPLEPAYMENTRESPONSE_STATUS = _PREPARESIMPLEPAYMENTRESPONSE.enum_types_by_name['Status']
_PREPAREFUNDSCHANGERESPONSE_STATUS = _PREPAREFUNDSCHANGERESPONSE.enum_types_by_name['Status']
TransferRequest = _reflection.GeneratedProtocolMessageType('TransferRequest', (_message.Message,), {'DESCRIPTOR': _TRANSFERREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(TransferRequest)
TransferResponse = _reflection.GeneratedProtocolMessageType('TransferResponse', (_message.Message,), {'DESCRIPTOR': _TRANSFERRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(TransferResponse)
CreateOwnerRequest = _reflection.GeneratedProtocolMessageType('CreateOwnerRequest', (_message.Message,), {'DESCRIPTOR': _CREATEOWNERREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(CreateOwnerRequest)
CreateOwnerResponse = _reflection.GeneratedProtocolMessageType('CreateOwnerResponse', (_message.Message,), {'DESCRIPTOR': _CREATEOWNERRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(CreateOwnerResponse)
CreateWalletRequest = _reflection.GeneratedProtocolMessageType('CreateWalletRequest', (_message.Message,), {'DESCRIPTOR': _CREATEWALLETREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(CreateWalletRequest)
CreateWalletResponse = _reflection.GeneratedProtocolMessageType('CreateWalletResponse', (_message.Message,), {'DESCRIPTOR': _CREATEWALLETRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(CreateWalletResponse)
ListWalletsRequest = _reflection.GeneratedProtocolMessageType('ListWalletsRequest', (_message.Message,), {'DESCRIPTOR': _LISTWALLETSREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListWalletsRequest)
ListWalletsResponse = _reflection.GeneratedProtocolMessageType('ListWalletsResponse', (_message.Message,), {'Wallet': _reflection.GeneratedProtocolMessageType('Wallet', (_message.Message,), {'DESCRIPTOR': _LISTWALLETSRESPONSE_WALLET, '__module__': 'wallet_api.v1.wallet_pb2'}), 'DESCRIPTOR': _LISTWALLETSRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListWalletsResponse)
_sym_db.RegisterMessage(ListWalletsResponse.Wallet)
ListWalletBalancesRequest = _reflection.GeneratedProtocolMessageType('ListWalletBalancesRequest', (_message.Message,), {'DESCRIPTOR': _LISTWALLETBALANCESREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListWalletBalancesRequest)
ListWalletBalancesResponse = _reflection.GeneratedProtocolMessageType('ListWalletBalancesResponse', (_message.Message,), {'DESCRIPTOR': _LISTWALLETBALANCESRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListWalletBalancesResponse)
ListFundsRequest = _reflection.GeneratedProtocolMessageType('ListFundsRequest', (_message.Message,), {'DESCRIPTOR': _LISTFUNDSREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListFundsRequest)
ListFundsResponse = _reflection.GeneratedProtocolMessageType('ListFundsResponse', (_message.Message,), {'Promissory': _reflection.GeneratedProtocolMessageType('Promissory', (_message.Message,), {'DESCRIPTOR': _LISTFUNDSRESPONSE_PROMISSORY, '__module__': 'wallet_api.v1.wallet_pb2'}), 'Fund': _reflection.GeneratedProtocolMessageType('Fund', (_message.Message,), {'DESCRIPTOR': _LISTFUNDSRESPONSE_FUND, '__module__': 'wallet_api.v1.wallet_pb2'}), 'DESCRIPTOR': _LISTFUNDSRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListFundsResponse)
_sym_db.RegisterMessage(ListFundsResponse.Promissory)
_sym_db.RegisterMessage(ListFundsResponse.Fund)
RedeemRequest = _reflection.GeneratedProtocolMessageType('RedeemRequest', (_message.Message,), {'DESCRIPTOR': _REDEEMREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(RedeemRequest)
RedeemResponse = _reflection.GeneratedProtocolMessageType('RedeemResponse', (_message.Message,), {'DESCRIPTOR': _REDEEMRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(RedeemResponse)
ListCredentialsRequest = _reflection.GeneratedProtocolMessageType('ListCredentialsRequest', (_message.Message,), {'DESCRIPTOR': _LISTCREDENTIALSREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListCredentialsRequest)
ListCredentialsResponse = _reflection.GeneratedProtocolMessageType('ListCredentialsResponse', (_message.Message,), {'DESCRIPTOR': _LISTCREDENTIALSRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListCredentialsResponse)
AddCredentialRequest = _reflection.GeneratedProtocolMessageType('AddCredentialRequest', (_message.Message,), {'DESCRIPTOR': _ADDCREDENTIALREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(AddCredentialRequest)
AddCredentialResponse = _reflection.GeneratedProtocolMessageType('AddCredentialResponse', (_message.Message,), {'DESCRIPTOR': _ADDCREDENTIALRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(AddCredentialResponse)
AssociateWalletRequest = _reflection.GeneratedProtocolMessageType('AssociateWalletRequest', (_message.Message,), {'DESCRIPTOR': _ASSOCIATEWALLETREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(AssociateWalletRequest)
AssociateWalletResponse = _reflection.GeneratedProtocolMessageType('AssociateWalletResponse', (_message.Message,), {'DESCRIPTOR': _ASSOCIATEWALLETRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(AssociateWalletResponse)
CreateAndAssociateWalletRequest = _reflection.GeneratedProtocolMessageType('CreateAndAssociateWalletRequest', (_message.Message,), {'DESCRIPTOR': _CREATEANDASSOCIATEWALLETREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(CreateAndAssociateWalletRequest)
CreateAndAssociateWalletResponse = _reflection.GeneratedProtocolMessageType('CreateAndAssociateWalletResponse', (_message.Message,), {'DESCRIPTOR': _CREATEANDASSOCIATEWALLETRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(CreateAndAssociateWalletResponse)
RequestPresentationRequest = _reflection.GeneratedProtocolMessageType('RequestPresentationRequest', (_message.Message,), {'DESCRIPTOR': _REQUESTPRESENTATIONREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(RequestPresentationRequest)
RequestPresentationResponse = _reflection.GeneratedProtocolMessageType('RequestPresentationResponse', (_message.Message,), {'DESCRIPTOR': _REQUESTPRESENTATIONRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(RequestPresentationResponse)
PrepareSimplePaymentRequest = _reflection.GeneratedProtocolMessageType('PrepareSimplePaymentRequest', (_message.Message,), {'DESCRIPTOR': _PREPARESIMPLEPAYMENTREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareSimplePaymentRequest)
PrepareSimplePaymentResponse = _reflection.GeneratedProtocolMessageType('PrepareSimplePaymentResponse', (_message.Message,), {'DESCRIPTOR': _PREPARESIMPLEPAYMENTRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareSimplePaymentResponse)
PrepareFundsChangeRequest = _reflection.GeneratedProtocolMessageType('PrepareFundsChangeRequest', (_message.Message,), {'DESCRIPTOR': _PREPAREFUNDSCHANGEREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareFundsChangeRequest)
PrepareFundsChangeResponse = _reflection.GeneratedProtocolMessageType('PrepareFundsChangeResponse', (_message.Message,), {'DESCRIPTOR': _PREPAREFUNDSCHANGERESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareFundsChangeResponse)
GetTransactionRequest = _reflection.GeneratedProtocolMessageType('GetTransactionRequest', (_message.Message,), {'DESCRIPTOR': _GETTRANSACTIONREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetTransactionRequest)
GetTransactionResponse = _reflection.GeneratedProtocolMessageType('GetTransactionResponse', (_message.Message,), {'DESCRIPTOR': _GETTRANSACTIONRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetTransactionResponse)
ListTransactionsRequest = _reflection.GeneratedProtocolMessageType('ListTransactionsRequest', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListTransactionsRequest)
ListTransactionsResponse = _reflection.GeneratedProtocolMessageType('ListTransactionsResponse', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListTransactionsResponse)
GetTransactionSnapshotRequest = _reflection.GeneratedProtocolMessageType('GetTransactionSnapshotRequest', (_message.Message,), {'DESCRIPTOR': _GETTRANSACTIONSNAPSHOTREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetTransactionSnapshotRequest)
GetTransactionSnapshotResponse = _reflection.GeneratedProtocolMessageType('GetTransactionSnapshotResponse', (_message.Message,), {'DESCRIPTOR': _GETTRANSACTIONSNAPSHOTRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetTransactionSnapshotResponse)
ListTransactionSnapshotsRequest = _reflection.GeneratedProtocolMessageType('ListTransactionSnapshotsRequest', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSNAPSHOTSREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListTransactionSnapshotsRequest)
ListTransactionSnapshotsResponse = _reflection.GeneratedProtocolMessageType('ListTransactionSnapshotsResponse', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSNAPSHOTSRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListTransactionSnapshotsResponse)
_WALLET = DESCRIPTOR.services_by_name['Wallet']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z7github.com/knox-networks/grpc-sdks/sdk/go/wallet_api/v1'
    _TRANSFERREQUEST._serialized_start = 69
    _TRANSFERREQUEST._serialized_end = 180
    _TRANSFERRESPONSE._serialized_start = 182
    _TRANSFERRESPONSE._serialized_end = 200
    _CREATEOWNERREQUEST._serialized_start = 202
    _CREATEOWNERREQUEST._serialized_end = 242
    _CREATEOWNERRESPONSE._serialized_start = 244
    _CREATEOWNERRESPONSE._serialized_end = 320
    _CREATEWALLETREQUEST._serialized_start = 322
    _CREATEWALLETREQUEST._serialized_end = 381
    _CREATEWALLETRESPONSE._serialized_start = 383
    _CREATEWALLETRESPONSE._serialized_end = 478
    _LISTWALLETSREQUEST._serialized_start = 480
    _LISTWALLETSREQUEST._serialized_end = 550
    _LISTWALLETSRESPONSE._serialized_start = 553
    _LISTWALLETSRESPONSE._serialized_end = 746
    _LISTWALLETSRESPONSE_WALLET._serialized_start = 665
    _LISTWALLETSRESPONSE_WALLET._serialized_end = 746
    _LISTWALLETBALANCESREQUEST._serialized_start = 748
    _LISTWALLETBALANCESREQUEST._serialized_end = 811
    _LISTWALLETBALANCESRESPONSE._serialized_start = 813
    _LISTWALLETBALANCESRESPONSE._serialized_end = 874
    _LISTFUNDSREQUEST._serialized_start = 876
    _LISTFUNDSREQUEST._serialized_end = 963
    _LISTFUNDSRESPONSE._serialized_start = 966
    _LISTFUNDSRESPONSE._serialized_end = 1363
    _LISTFUNDSRESPONSE_PROMISSORY._serialized_start = 1069
    _LISTFUNDSRESPONSE_PROMISSORY._serialized_end = 1271
    _LISTFUNDSRESPONSE_FUND._serialized_start = 1273
    _LISTFUNDSRESPONSE_FUND._serialized_end = 1363
    _REDEEMREQUEST._serialized_start = 1365
    _REDEEMREQUEST._serialized_end = 1471
    _REDEEMRESPONSE._serialized_start = 1473
    _REDEEMRESPONSE._serialized_end = 1524
    _LISTCREDENTIALSREQUEST._serialized_start = 1526
    _LISTCREDENTIALSREQUEST._serialized_end = 1619
    _LISTCREDENTIALSRESPONSE._serialized_start = 1621
    _LISTCREDENTIALSRESPONSE._serialized_end = 1700
    _ADDCREDENTIALREQUEST._serialized_start = 1702
    _ADDCREDENTIALREQUEST._serialized_end = 1785
    _ADDCREDENTIALRESPONSE._serialized_start = 1787
    _ADDCREDENTIALRESPONSE._serialized_end = 1830
    _ASSOCIATEWALLETREQUEST._serialized_start = 1832
    _ASSOCIATEWALLETREQUEST._serialized_end = 1892
    _ASSOCIATEWALLETRESPONSE._serialized_start = 1894
    _ASSOCIATEWALLETRESPONSE._serialized_end = 1971
    _CREATEANDASSOCIATEWALLETREQUEST._serialized_start = 1973
    _CREATEANDASSOCIATEWALLETREQUEST._serialized_end = 2059
    _CREATEANDASSOCIATEWALLETRESPONSE._serialized_start = 2061
    _CREATEANDASSOCIATEWALLETRESPONSE._serialized_end = 2168
    _REQUESTPRESENTATIONREQUEST._serialized_start = 2170
    _REQUESTPRESENTATIONREQUEST._serialized_end = 2253
    _REQUESTPRESENTATIONRESPONSE._serialized_start = 2255
    _REQUESTPRESENTATIONRESPONSE._serialized_end = 2306
    _PREPARESIMPLEPAYMENTREQUEST._serialized_start = 2309
    _PREPARESIMPLEPAYMENTREQUEST._serialized_end = 2449
    _PREPARESIMPLEPAYMENTRESPONSE._serialized_start = 2452
    _PREPARESIMPLEPAYMENTRESPONSE._serialized_end = 2673
    _PREPARESIMPLEPAYMENTRESPONSE_STATUS._serialized_start = 2566
    _PREPARESIMPLEPAYMENTRESPONSE_STATUS._serialized_end = 2673
    _PREPAREFUNDSCHANGEREQUEST._serialized_start = 2675
    _PREPAREFUNDSCHANGEREQUEST._serialized_end = 2794
    _PREPAREFUNDSCHANGERESPONSE._serialized_start = 2797
    _PREPAREFUNDSCHANGERESPONSE._serialized_end = 3021
    _PREPAREFUNDSCHANGERESPONSE_STATUS._serialized_start = 2907
    _PREPAREFUNDSCHANGERESPONSE_STATUS._serialized_end = 3021
    _GETTRANSACTIONREQUEST._serialized_start = 3023
    _GETTRANSACTIONREQUEST._serialized_end = 3096
    _GETTRANSACTIONRESPONSE._serialized_start = 3098
    _GETTRANSACTIONRESPONSE._serialized_end = 3164
    _LISTTRANSACTIONSREQUEST._serialized_start = 3166
    _LISTTRANSACTIONSREQUEST._serialized_end = 3220
    _LISTTRANSACTIONSRESPONSE._serialized_start = 3222
    _LISTTRANSACTIONSRESPONSE._serialized_end = 3291
    _GETTRANSACTIONSNAPSHOTREQUEST._serialized_start = 3293
    _GETTRANSACTIONSNAPSHOTREQUEST._serialized_end = 3374
    _GETTRANSACTIONSNAPSHOTRESPONSE._serialized_start = 3376
    _GETTRANSACTIONSNAPSHOTRESPONSE._serialized_end = 3455
    _LISTTRANSACTIONSNAPSHOTSREQUEST._serialized_start = 3457
    _LISTTRANSACTIONSNAPSHOTSREQUEST._serialized_end = 3540
    _LISTTRANSACTIONSNAPSHOTSRESPONSE._serialized_start = 3542
    _LISTTRANSACTIONSNAPSHOTSRESPONSE._serialized_end = 3624
    _WALLET._serialized_start = 3627
    _WALLET._serialized_end = 5289