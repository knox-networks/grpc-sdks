"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
from ...common.v1 import packet_pb2 as common_dot_v1_dot_packet__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1awallet_api/v1/wallet.proto\x12\rwallet_api.v1\x1a\x16common/v1/common.proto\x1a\x16common/v1/packet.proto"o\n\x0fTransferRequest\x12\x16\n\x0efrom_wallet_id\x18\x01 \x01(\t\x12\x1c\n\x14recipient_public_key\x18\x02 \x01(\t\x12\x15\n\rpromissory_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t"\x12\n\x10TransferResponse"(\n\x12CreateOwnerRequest\x12\x12\n\nowner_name\x18\x01 \x01(\t"L\n\x13CreateOwnerResponse\x12\x10\n\x08owner_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\x12\n\nowner_name\x18\x03 \x01(\t";\n\x13CreateWalletRequest\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t"_\n\x14CreateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x13\n\x0bwallet_name\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03did\x18\x04 \x01(\t"\x88\x01\n\x12ListWalletsRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x0f\n\x07api_key\x18\x03 \x01(\t\x12\x1f\n\x07filters\x18\x04 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x05 \x01(\x0b2\x0e.common.SortBy"\xc1\x01\n\x13ListWalletsResponse\x12:\n\x07wallets\x18\x01 \x03(\x0b2).wallet_api.v1.ListWalletsResponse.Wallet\x12\x1b\n\x13total_owned_wallets\x18\x02 \x01(\r\x1aQ\n\x06Wallet\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03did\x18\x04 \x01(\t"`\n\x19ListWalletBalancesRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\x1f\n\x07sort_by\x18\x03 \x01(\x0b2\x0e.common.SortBy"C\n\rWalletBalance\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x12\n\nfile_count\x18\x02 \x01(\x03"L\n\x1aListWalletBalancesResponse\x12.\n\x08balances\x18\x01 \x03(\x0b2\x1c.wallet_api.v1.WalletBalance"\x9e\x01\n\x15ListAssetFilesRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x11\n\twallet_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t\x12\x1f\n\x07filters\x18\x05 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x06 \x01(\x0b2\x0e.common.SortBy"\xb7\x04\n\x16ListAssetFilesResponse\x129\n\x05funds\x18\x01 \x03(\x0b2*.wallet_api.v1.ListAssetFilesResponse.Fund\x12\x19\n\x11total_owned_funds\x18\x02 \x01(\r\x1a(\n\x06Notary\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x11\n\tsignature\x18\x02 \x01(\x0c\x1a\xa9\x02\n\nPromissory\x12\x0f\n\x07version\x18\x01 \x01(\t\x12\n\n\x02id\x18\x03 \x01(\t\x12\x10\n\x08currency\x18\x04 \x01(\t\x12\x0e\n\x06amount\x18\x05 \x01(\x03\x12\x10\n\x08decimals\x18\x06 \x01(\r\x12\x11\n\tauthority\x18\x07 \x01(\t\x12\x0e\n\x06issuer\x18\x08 \x01(\t\x12\x15\n\rissuance_date\x18\t \x01(\t\x12\x13\n\x0bblock_depth\x18\n \x01(\r\x12\x1f\n\x17authorization_signature\x18\x0b \x01(\x0c\x12\x1a\n\x12transfer_signature\x18\x0c \x01(\x0c\x12>\n\x08notaries\x18\r \x03(\x0b2,.wallet_api.v1.ListAssetFilesResponse.Notary\x1aq\n\x04Fund\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12D\n\npromissory\x18\x02 \x01(\x0b20.wallet_api.v1.ListAssetFilesResponse.Promissory\x12\x10\n\x08reserved\x18\x03 \x01(\x08"j\n\rRedeemRequest\x12\x16\n\x0efrom_wallet_id\x18\x01 \x01(\t\x12\x19\n\x11issuer_public_key\x18\x02 \x01(\t\x12\x15\n\rpromissory_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t"3\n\x0eRedeemResponse\x12!\n\tnew_limit\x18\x01 \x01(\x0b2\x0e.common.Amount"\x9f\x01\n\x16ListCredentialsRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x11\n\twallet_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t\x12\x1f\n\x07filters\x18\x05 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x06 \x01(\x0b2\x0e.common.SortBy"O\n\x17ListCredentialsResponse\x12\x13\n\x0bcredentials\x18\x01 \x03(\t\x12\x1f\n\x17total_owned_credentials\x18\x02 \x01(\r"S\n\x14AddCredentialRequest\x12\x17\n\x0fcredential_type\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"+\n\x15AddCredentialResponse\x12\x12\n\ncredential\x18\x01 \x01(\t"<\n\x16AssociateWalletRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t"M\n\x17AssociateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0b\n\x03did\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t"V\n\x1fCreateAndAssociateWalletRequest\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\r\n\x05token\x18\x03 \x01(\t"k\n CreateAndAssociateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x13\n\x0bwallet_name\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03did\x18\x04 \x01(\t"S\n\x1aRequestPresentationRequest\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x17\n\x0fcredential_type\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"3\n\x1bRequestPresentationResponse\x12\x14\n\x0cpresentation\x18\x01 \x01(\t"w\n\x19PrepareFundsChangeRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x16\n\x0euser_reference\x18\x04 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"\xe0\x01\n\x1aPrepareFundsChangeResponse\x12@\n\x06status\x18\x01 \x01(\x0e20.wallet_api.v1.PrepareFundsChangeResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t"r\n\x06Status\x12\x16\n\x12STATUS_UNSPECIFIED\x10\x00\x12\x12\n\x0eSTATUS_SUCCESS\x10\x01\x12\x1d\n\x19STATUS_INSUFFICIENT_FUNDS\x10\x02\x12\x1d\n\x19STATUS_FUNDS_PRESENT_NOOP\x10\x03"\x9d\x01\n\x19PrepareTransactionRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\'\n\x0bcommitments\x18\x03 \x03(\x0b2\x12.common.Commitment\x12%\n\nconditions\x18\x04 \x03(\x0b2\x11.common.Condition\x12\x0c\n\x04memo\x18\x05 \x01(\t"\xa1\x01\n\x1aPrepareTransactionResponse\x12@\n\x06status\x18\x01 \x01(\x0e20.wallet_api.v1.PrepareTransactionResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t"3\n\x06Status\x12\x0f\n\x0bUNSPECIFIED\x10\x00\x12\x0b\n\x07SUCCESS\x10\x01\x12\x0b\n\x07FAILURE\x10\x02"\xa0\x01\n\x17ListTransactionsRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\x1f\n\x07filters\x18\x03 \x01(\x0b2\x0e.common.Filter\x12\x0c\n\x04page\x18\x04 \x01(\x05\x12\x11\n\tpage_size\x18\x05 \x01(\x05\x12\x1f\n\x07sort_by\x18\x06 \x01(\x0b2\x0e.common.SortBy"\x84\x01\n\x0fContractSummary\x12\x13\n\x0bcontract_id\x18\x01 \x01(\t\x12\x15\n\rcontract_type\x18\x02 \x01(\t\x12\x0f\n\x07created\x18\x03 \x01(\t\x12\x16\n\x0ecounterparties\x18\x04 \x03(\t\x12\x0e\n\x06status\x18\x05 \x01(\t\x12\x0c\n\x04memo\x18\x06 \x01(\t"e\n\x18ListTransactionsResponse\x121\n\tcontracts\x18\x01 \x03(\x0b2\x1e.wallet_api.v1.ContractSummary\x12\x16\n\x0etotalContracts\x18\x02 \x01(\r"\x9e\x01\n\rBalanceDetail\x12-\n\x07balance\x18\x01 \x01(\x0b2\x1c.wallet_api.v1.WalletBalance\x12/\n\x04kind\x18\x02 \x01(\x0e2!.wallet_api.v1.BalanceDetail.Kind"-\n\x04Kind\x12\x12\n\x0eKIND_AVAILABLE\x10\x00\x12\x11\n\rKIND_RESERVED\x10\x01"E\n\x1fListWalletBalanceDetailsRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t"R\n ListWalletBalanceDetailsResponse\x12.\n\x08balances\x18\x01 \x03(\x0b2\x1c.wallet_api.v1.BalanceDetail"O\n\x1bGetTransactionDetailRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\x0c\n\x04uetr\x18\x03 \x01(\t"\x9f\x03\n\x1cGetTransactionDetailResponse\x12\x15\n\rcontract_type\x18\x01 \x01(\t\x12\'\n\x0bcommitments\x18\x02 \x03(\x0b2\x12.common.Commitment\x12\x0f\n\x07created\x18\x04 \x01(\t\x12\x16\n\x0ehash_presenter\x18\x05 \x01(\t\x12\x0c\n\x04hash\x18\x06 \x01(\x0c\x12\x12\n\nexpiration\x18\x07 \x01(\t\x12\x0c\n\x04memo\x18\x08 \x01(\t\x12A\n\x03log\x18\t \x03(\x0b24.wallet_api.v1.GetTransactionDetailResponse.LogEntry\x1a\xa2\x01\n\x08LogEntry\x12\x0e\n\x06sender\x18\x01 \x01(\t\x12\x10\n\x08receiver\x18\x02 \x01(\t\x12\x1e\n\x06amount\x18\x03 \x01(\x0b2\x0e.common.Amount\x12\x11\n\ttimestamp\x18\x04 \x01(\t\x12\x0c\n\x04kind\x18\x05 \x01(\t\x12\r\n\x05state\x18\x06 \x01(\t\x12\r\n\x05event\x18\x07 \x01(\t\x12\x15\n\rpromissory_id\x18\x08 \x01(\t"\xc1\x01\n\x1dPrepareAtomicAssetSwapRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x1e\n\x06amount\x18\x03 \x01(\x0b2\x0e.common.Amount\x12\x12\n\nrespondent\x18\x04 \x01(\t\x12)\n\x11respondent_amount\x18\x05 \x01(\x0b2\x0e.common.Amount\x12\x0c\n\x04memo\x18\x06 \x01(\t\x12\x0f\n\x07timeout\x18\x07 \x01(\r"\xa9\x01\n\x1ePrepareAtomicAssetSwapResponse\x12D\n\x06status\x18\x01 \x01(\x0e24.wallet_api.v1.PrepareAtomicAssetSwapResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t"3\n\x06Status\x12\x0f\n\x0bUNSPECIFIED\x10\x00\x12\x0b\n\x07SUCCESS\x10\x01\x12\x0b\n\x07FAILURE\x10\x02"\xd6\x01\n PrepareCrossBorderPaymentRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x10\n\x08treasury\x18\x03 \x01(\t\x12\x1e\n\x06amount\x18\x04 \x01(\x0b2\x0e.common.Amount\x12\x12\n\nrespondent\x18\x05 \x01(\t\x12)\n\x11respondent_amount\x18\x06 \x01(\x0b2\x0e.common.Amount\x12\x0c\n\x04memo\x18\x07 \x01(\t\x12\x0f\n\x07timeout\x18\x08 \x01(\r"\xaf\x01\n!PrepareCrossBorderPaymentResponse\x12G\n\x06status\x18\x01 \x01(\x0e27.wallet_api.v1.PrepareCrossBorderPaymentResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t"3\n\x06Status\x12\x0f\n\x0bUNSPECIFIED\x10\x00\x12\x0b\n\x07SUCCESS\x10\x01\x12\x0b\n\x07FAILURE\x10\x022\xa0\x0e\n\rWalletService\x12K\n\x08Transfer\x12\x1e.wallet_api.v1.TransferRequest\x1a\x1f.wallet_api.v1.TransferResponse\x12T\n\x0bCreateOwner\x12!.wallet_api.v1.CreateOwnerRequest\x1a".wallet_api.v1.CreateOwnerResponse\x12W\n\x0cCreateWallet\x12".wallet_api.v1.CreateWalletRequest\x1a#.wallet_api.v1.CreateWalletResponse\x12T\n\x0bListWallets\x12!.wallet_api.v1.ListWalletsRequest\x1a".wallet_api.v1.ListWalletsResponse\x12]\n\x0eListAssetFiles\x12$.wallet_api.v1.ListAssetFilesRequest\x1a%.wallet_api.v1.ListAssetFilesResponse\x12i\n\x12ListWalletBalances\x12(.wallet_api.v1.ListWalletBalancesRequest\x1a).wallet_api.v1.ListWalletBalancesResponse\x12E\n\x06Redeem\x12\x1c.wallet_api.v1.RedeemRequest\x1a\x1d.wallet_api.v1.RedeemResponse\x12`\n\x0fListCredentials\x12%.wallet_api.v1.ListCredentialsRequest\x1a&.wallet_api.v1.ListCredentialsResponse\x12Z\n\rAddCredential\x12#.wallet_api.v1.AddCredentialRequest\x1a$.wallet_api.v1.AddCredentialResponse\x12`\n\x0fAssociateWallet\x12%.wallet_api.v1.AssociateWalletRequest\x1a&.wallet_api.v1.AssociateWalletResponse\x12l\n\x13RequestPresentation\x12).wallet_api.v1.RequestPresentationRequest\x1a*.wallet_api.v1.RequestPresentationResponse\x12{\n\x18ListWalletBalanceDetails\x12..wallet_api.v1.ListWalletBalanceDetailsRequest\x1a/.wallet_api.v1.ListWalletBalanceDetailsResponse\x12c\n\x10ListTransactions\x12&.wallet_api.v1.ListTransactionsRequest\x1a\'.wallet_api.v1.ListTransactionsResponse\x12o\n\x14GetTransactionDetail\x12*.wallet_api.v1.GetTransactionDetailRequest\x1a+.wallet_api.v1.GetTransactionDetailResponse\x12i\n\x12PrepareTransaction\x12(.wallet_api.v1.PrepareTransactionRequest\x1a).wallet_api.v1.PrepareTransactionResponse\x12u\n\x16PrepareAtomicAssetSwap\x12,.wallet_api.v1.PrepareAtomicAssetSwapRequest\x1a-.wallet_api.v1.PrepareAtomicAssetSwapResponse\x12~\n\x19PrepareCrossBorderPayment\x12/.wallet_api.v1.PrepareCrossBorderPaymentRequest\x1a0.wallet_api.v1.PrepareCrossBorderPaymentResponse\x12i\n\x12PrepareFundsChange\x12(.wallet_api.v1.PrepareFundsChangeRequest\x1a).wallet_api.v1.PrepareFundsChangeResponseB9Z7github.com/knox-networks/grpc-sdks/sdk/go/wallet_api/v1b\x06proto3')
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
_WALLETBALANCE = DESCRIPTOR.message_types_by_name['WalletBalance']
_LISTWALLETBALANCESRESPONSE = DESCRIPTOR.message_types_by_name['ListWalletBalancesResponse']
_LISTASSETFILESREQUEST = DESCRIPTOR.message_types_by_name['ListAssetFilesRequest']
_LISTASSETFILESRESPONSE = DESCRIPTOR.message_types_by_name['ListAssetFilesResponse']
_LISTASSETFILESRESPONSE_NOTARY = _LISTASSETFILESRESPONSE.nested_types_by_name['Notary']
_LISTASSETFILESRESPONSE_PROMISSORY = _LISTASSETFILESRESPONSE.nested_types_by_name['Promissory']
_LISTASSETFILESRESPONSE_FUND = _LISTASSETFILESRESPONSE.nested_types_by_name['Fund']
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
_PREPAREFUNDSCHANGEREQUEST = DESCRIPTOR.message_types_by_name['PrepareFundsChangeRequest']
_PREPAREFUNDSCHANGERESPONSE = DESCRIPTOR.message_types_by_name['PrepareFundsChangeResponse']
_PREPARETRANSACTIONREQUEST = DESCRIPTOR.message_types_by_name['PrepareTransactionRequest']
_PREPARETRANSACTIONRESPONSE = DESCRIPTOR.message_types_by_name['PrepareTransactionResponse']
_LISTTRANSACTIONSREQUEST = DESCRIPTOR.message_types_by_name['ListTransactionsRequest']
_CONTRACTSUMMARY = DESCRIPTOR.message_types_by_name['ContractSummary']
_LISTTRANSACTIONSRESPONSE = DESCRIPTOR.message_types_by_name['ListTransactionsResponse']
_BALANCEDETAIL = DESCRIPTOR.message_types_by_name['BalanceDetail']
_LISTWALLETBALANCEDETAILSREQUEST = DESCRIPTOR.message_types_by_name['ListWalletBalanceDetailsRequest']
_LISTWALLETBALANCEDETAILSRESPONSE = DESCRIPTOR.message_types_by_name['ListWalletBalanceDetailsResponse']
_GETTRANSACTIONDETAILREQUEST = DESCRIPTOR.message_types_by_name['GetTransactionDetailRequest']
_GETTRANSACTIONDETAILRESPONSE = DESCRIPTOR.message_types_by_name['GetTransactionDetailResponse']
_GETTRANSACTIONDETAILRESPONSE_LOGENTRY = _GETTRANSACTIONDETAILRESPONSE.nested_types_by_name['LogEntry']
_PREPAREATOMICASSETSWAPREQUEST = DESCRIPTOR.message_types_by_name['PrepareAtomicAssetSwapRequest']
_PREPAREATOMICASSETSWAPRESPONSE = DESCRIPTOR.message_types_by_name['PrepareAtomicAssetSwapResponse']
_PREPARECROSSBORDERPAYMENTREQUEST = DESCRIPTOR.message_types_by_name['PrepareCrossBorderPaymentRequest']
_PREPARECROSSBORDERPAYMENTRESPONSE = DESCRIPTOR.message_types_by_name['PrepareCrossBorderPaymentResponse']
_PREPAREFUNDSCHANGERESPONSE_STATUS = _PREPAREFUNDSCHANGERESPONSE.enum_types_by_name['Status']
_PREPARETRANSACTIONRESPONSE_STATUS = _PREPARETRANSACTIONRESPONSE.enum_types_by_name['Status']
_BALANCEDETAIL_KIND = _BALANCEDETAIL.enum_types_by_name['Kind']
_PREPAREATOMICASSETSWAPRESPONSE_STATUS = _PREPAREATOMICASSETSWAPRESPONSE.enum_types_by_name['Status']
_PREPARECROSSBORDERPAYMENTRESPONSE_STATUS = _PREPARECROSSBORDERPAYMENTRESPONSE.enum_types_by_name['Status']
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
WalletBalance = _reflection.GeneratedProtocolMessageType('WalletBalance', (_message.Message,), {'DESCRIPTOR': _WALLETBALANCE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(WalletBalance)
ListWalletBalancesResponse = _reflection.GeneratedProtocolMessageType('ListWalletBalancesResponse', (_message.Message,), {'DESCRIPTOR': _LISTWALLETBALANCESRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListWalletBalancesResponse)
ListAssetFilesRequest = _reflection.GeneratedProtocolMessageType('ListAssetFilesRequest', (_message.Message,), {'DESCRIPTOR': _LISTASSETFILESREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListAssetFilesRequest)
ListAssetFilesResponse = _reflection.GeneratedProtocolMessageType('ListAssetFilesResponse', (_message.Message,), {'Notary': _reflection.GeneratedProtocolMessageType('Notary', (_message.Message,), {'DESCRIPTOR': _LISTASSETFILESRESPONSE_NOTARY, '__module__': 'wallet_api.v1.wallet_pb2'}), 'Promissory': _reflection.GeneratedProtocolMessageType('Promissory', (_message.Message,), {'DESCRIPTOR': _LISTASSETFILESRESPONSE_PROMISSORY, '__module__': 'wallet_api.v1.wallet_pb2'}), 'Fund': _reflection.GeneratedProtocolMessageType('Fund', (_message.Message,), {'DESCRIPTOR': _LISTASSETFILESRESPONSE_FUND, '__module__': 'wallet_api.v1.wallet_pb2'}), 'DESCRIPTOR': _LISTASSETFILESRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListAssetFilesResponse)
_sym_db.RegisterMessage(ListAssetFilesResponse.Notary)
_sym_db.RegisterMessage(ListAssetFilesResponse.Promissory)
_sym_db.RegisterMessage(ListAssetFilesResponse.Fund)
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
PrepareFundsChangeRequest = _reflection.GeneratedProtocolMessageType('PrepareFundsChangeRequest', (_message.Message,), {'DESCRIPTOR': _PREPAREFUNDSCHANGEREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareFundsChangeRequest)
PrepareFundsChangeResponse = _reflection.GeneratedProtocolMessageType('PrepareFundsChangeResponse', (_message.Message,), {'DESCRIPTOR': _PREPAREFUNDSCHANGERESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareFundsChangeResponse)
PrepareTransactionRequest = _reflection.GeneratedProtocolMessageType('PrepareTransactionRequest', (_message.Message,), {'DESCRIPTOR': _PREPARETRANSACTIONREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareTransactionRequest)
PrepareTransactionResponse = _reflection.GeneratedProtocolMessageType('PrepareTransactionResponse', (_message.Message,), {'DESCRIPTOR': _PREPARETRANSACTIONRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareTransactionResponse)
ListTransactionsRequest = _reflection.GeneratedProtocolMessageType('ListTransactionsRequest', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListTransactionsRequest)
ContractSummary = _reflection.GeneratedProtocolMessageType('ContractSummary', (_message.Message,), {'DESCRIPTOR': _CONTRACTSUMMARY, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ContractSummary)
ListTransactionsResponse = _reflection.GeneratedProtocolMessageType('ListTransactionsResponse', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListTransactionsResponse)
BalanceDetail = _reflection.GeneratedProtocolMessageType('BalanceDetail', (_message.Message,), {'DESCRIPTOR': _BALANCEDETAIL, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(BalanceDetail)
ListWalletBalanceDetailsRequest = _reflection.GeneratedProtocolMessageType('ListWalletBalanceDetailsRequest', (_message.Message,), {'DESCRIPTOR': _LISTWALLETBALANCEDETAILSREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListWalletBalanceDetailsRequest)
ListWalletBalanceDetailsResponse = _reflection.GeneratedProtocolMessageType('ListWalletBalanceDetailsResponse', (_message.Message,), {'DESCRIPTOR': _LISTWALLETBALANCEDETAILSRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListWalletBalanceDetailsResponse)
GetTransactionDetailRequest = _reflection.GeneratedProtocolMessageType('GetTransactionDetailRequest', (_message.Message,), {'DESCRIPTOR': _GETTRANSACTIONDETAILREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetTransactionDetailRequest)
GetTransactionDetailResponse = _reflection.GeneratedProtocolMessageType('GetTransactionDetailResponse', (_message.Message,), {'LogEntry': _reflection.GeneratedProtocolMessageType('LogEntry', (_message.Message,), {'DESCRIPTOR': _GETTRANSACTIONDETAILRESPONSE_LOGENTRY, '__module__': 'wallet_api.v1.wallet_pb2'}), 'DESCRIPTOR': _GETTRANSACTIONDETAILRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetTransactionDetailResponse)
_sym_db.RegisterMessage(GetTransactionDetailResponse.LogEntry)
PrepareAtomicAssetSwapRequest = _reflection.GeneratedProtocolMessageType('PrepareAtomicAssetSwapRequest', (_message.Message,), {'DESCRIPTOR': _PREPAREATOMICASSETSWAPREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareAtomicAssetSwapRequest)
PrepareAtomicAssetSwapResponse = _reflection.GeneratedProtocolMessageType('PrepareAtomicAssetSwapResponse', (_message.Message,), {'DESCRIPTOR': _PREPAREATOMICASSETSWAPRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareAtomicAssetSwapResponse)
PrepareCrossBorderPaymentRequest = _reflection.GeneratedProtocolMessageType('PrepareCrossBorderPaymentRequest', (_message.Message,), {'DESCRIPTOR': _PREPARECROSSBORDERPAYMENTREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareCrossBorderPaymentRequest)
PrepareCrossBorderPaymentResponse = _reflection.GeneratedProtocolMessageType('PrepareCrossBorderPaymentResponse', (_message.Message,), {'DESCRIPTOR': _PREPARECROSSBORDERPAYMENTRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareCrossBorderPaymentResponse)
_WALLETSERVICE = DESCRIPTOR.services_by_name['WalletService']
if _descriptor._USE_C_DESCRIPTORS == False:
    DESCRIPTOR._options = None
    DESCRIPTOR._serialized_options = b'Z7github.com/knox-networks/grpc-sdks/sdk/go/wallet_api/v1'
    _TRANSFERREQUEST._serialized_start = 93
    _TRANSFERREQUEST._serialized_end = 204
    _TRANSFERRESPONSE._serialized_start = 206
    _TRANSFERRESPONSE._serialized_end = 224
    _CREATEOWNERREQUEST._serialized_start = 226
    _CREATEOWNERREQUEST._serialized_end = 266
    _CREATEOWNERRESPONSE._serialized_start = 268
    _CREATEOWNERRESPONSE._serialized_end = 344
    _CREATEWALLETREQUEST._serialized_start = 346
    _CREATEWALLETREQUEST._serialized_end = 405
    _CREATEWALLETRESPONSE._serialized_start = 407
    _CREATEWALLETRESPONSE._serialized_end = 502
    _LISTWALLETSREQUEST._serialized_start = 505
    _LISTWALLETSREQUEST._serialized_end = 641
    _LISTWALLETSRESPONSE._serialized_start = 644
    _LISTWALLETSRESPONSE._serialized_end = 837
    _LISTWALLETSRESPONSE_WALLET._serialized_start = 756
    _LISTWALLETSRESPONSE_WALLET._serialized_end = 837
    _LISTWALLETBALANCESREQUEST._serialized_start = 839
    _LISTWALLETBALANCESREQUEST._serialized_end = 935
    _WALLETBALANCE._serialized_start = 937
    _WALLETBALANCE._serialized_end = 1004
    _LISTWALLETBALANCESRESPONSE._serialized_start = 1006
    _LISTWALLETBALANCESRESPONSE._serialized_end = 1082
    _LISTASSETFILESREQUEST._serialized_start = 1085
    _LISTASSETFILESREQUEST._serialized_end = 1243
    _LISTASSETFILESRESPONSE._serialized_start = 1246
    _LISTASSETFILESRESPONSE._serialized_end = 1813
    _LISTASSETFILESRESPONSE_NOTARY._serialized_start = 1358
    _LISTASSETFILESRESPONSE_NOTARY._serialized_end = 1398
    _LISTASSETFILESRESPONSE_PROMISSORY._serialized_start = 1401
    _LISTASSETFILESRESPONSE_PROMISSORY._serialized_end = 1698
    _LISTASSETFILESRESPONSE_FUND._serialized_start = 1700
    _LISTASSETFILESRESPONSE_FUND._serialized_end = 1813
    _REDEEMREQUEST._serialized_start = 1815
    _REDEEMREQUEST._serialized_end = 1921
    _REDEEMRESPONSE._serialized_start = 1923
    _REDEEMRESPONSE._serialized_end = 1974
    _LISTCREDENTIALSREQUEST._serialized_start = 1977
    _LISTCREDENTIALSREQUEST._serialized_end = 2136
    _LISTCREDENTIALSRESPONSE._serialized_start = 2138
    _LISTCREDENTIALSRESPONSE._serialized_end = 2217
    _ADDCREDENTIALREQUEST._serialized_start = 2219
    _ADDCREDENTIALREQUEST._serialized_end = 2302
    _ADDCREDENTIALRESPONSE._serialized_start = 2304
    _ADDCREDENTIALRESPONSE._serialized_end = 2347
    _ASSOCIATEWALLETREQUEST._serialized_start = 2349
    _ASSOCIATEWALLETREQUEST._serialized_end = 2409
    _ASSOCIATEWALLETRESPONSE._serialized_start = 2411
    _ASSOCIATEWALLETRESPONSE._serialized_end = 2488
    _CREATEANDASSOCIATEWALLETREQUEST._serialized_start = 2490
    _CREATEANDASSOCIATEWALLETREQUEST._serialized_end = 2576
    _CREATEANDASSOCIATEWALLETRESPONSE._serialized_start = 2578
    _CREATEANDASSOCIATEWALLETRESPONSE._serialized_end = 2685
    _REQUESTPRESENTATIONREQUEST._serialized_start = 2687
    _REQUESTPRESENTATIONREQUEST._serialized_end = 2770
    _REQUESTPRESENTATIONRESPONSE._serialized_start = 2772
    _REQUESTPRESENTATIONRESPONSE._serialized_end = 2823
    _PREPAREFUNDSCHANGEREQUEST._serialized_start = 2825
    _PREPAREFUNDSCHANGEREQUEST._serialized_end = 2944
    _PREPAREFUNDSCHANGERESPONSE._serialized_start = 2947
    _PREPAREFUNDSCHANGERESPONSE._serialized_end = 3171
    _PREPAREFUNDSCHANGERESPONSE_STATUS._serialized_start = 3057
    _PREPAREFUNDSCHANGERESPONSE_STATUS._serialized_end = 3171
    _PREPARETRANSACTIONREQUEST._serialized_start = 3174
    _PREPARETRANSACTIONREQUEST._serialized_end = 3331
    _PREPARETRANSACTIONRESPONSE._serialized_start = 3334
    _PREPARETRANSACTIONRESPONSE._serialized_end = 3495
    _PREPARETRANSACTIONRESPONSE_STATUS._serialized_start = 3444
    _PREPARETRANSACTIONRESPONSE_STATUS._serialized_end = 3495
    _LISTTRANSACTIONSREQUEST._serialized_start = 3498
    _LISTTRANSACTIONSREQUEST._serialized_end = 3658
    _CONTRACTSUMMARY._serialized_start = 3661
    _CONTRACTSUMMARY._serialized_end = 3793
    _LISTTRANSACTIONSRESPONSE._serialized_start = 3795
    _LISTTRANSACTIONSRESPONSE._serialized_end = 3896
    _BALANCEDETAIL._serialized_start = 3899
    _BALANCEDETAIL._serialized_end = 4057
    _BALANCEDETAIL_KIND._serialized_start = 4012
    _BALANCEDETAIL_KIND._serialized_end = 4057
    _LISTWALLETBALANCEDETAILSREQUEST._serialized_start = 4059
    _LISTWALLETBALANCEDETAILSREQUEST._serialized_end = 4128
    _LISTWALLETBALANCEDETAILSRESPONSE._serialized_start = 4130
    _LISTWALLETBALANCEDETAILSRESPONSE._serialized_end = 4212
    _GETTRANSACTIONDETAILREQUEST._serialized_start = 4214
    _GETTRANSACTIONDETAILREQUEST._serialized_end = 4293
    _GETTRANSACTIONDETAILRESPONSE._serialized_start = 4296
    _GETTRANSACTIONDETAILRESPONSE._serialized_end = 4711
    _GETTRANSACTIONDETAILRESPONSE_LOGENTRY._serialized_start = 4549
    _GETTRANSACTIONDETAILRESPONSE_LOGENTRY._serialized_end = 4711
    _PREPAREATOMICASSETSWAPREQUEST._serialized_start = 4714
    _PREPAREATOMICASSETSWAPREQUEST._serialized_end = 4907
    _PREPAREATOMICASSETSWAPRESPONSE._serialized_start = 4910
    _PREPAREATOMICASSETSWAPRESPONSE._serialized_end = 5079
    _PREPAREATOMICASSETSWAPRESPONSE_STATUS._serialized_start = 3444
    _PREPAREATOMICASSETSWAPRESPONSE_STATUS._serialized_end = 3495
    _PREPARECROSSBORDERPAYMENTREQUEST._serialized_start = 5082
    _PREPARECROSSBORDERPAYMENTREQUEST._serialized_end = 5296
    _PREPARECROSSBORDERPAYMENTRESPONSE._serialized_start = 5299
    _PREPARECROSSBORDERPAYMENTRESPONSE._serialized_end = 5474
    _PREPARECROSSBORDERPAYMENTRESPONSE_STATUS._serialized_start = 3444
    _PREPARECROSSBORDERPAYMENTRESPONSE_STATUS._serialized_end = 3495
    _WALLETSERVICE._serialized_start = 5477
    _WALLETSERVICE._serialized_end = 7301