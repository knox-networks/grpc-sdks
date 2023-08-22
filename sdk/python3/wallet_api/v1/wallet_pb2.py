"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
_sym_db = _symbol_database.Default()
from ...common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
from ...common.v1 import packet_pb2 as common_dot_v1_dot_packet__pb2
DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1awallet_api/v1/wallet.proto\x12\rwallet_api.v1\x1a\x16common/v1/common.proto\x1a\x16common/v1/packet.proto"o\n\x0fTransferRequest\x12\x16\n\x0efrom_wallet_id\x18\x01 \x01(\t\x12\x1c\n\x14recipient_public_key\x18\x02 \x01(\t\x12\x15\n\rpromissory_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t"\x12\n\x10TransferResponse"(\n\x12CreateOwnerRequest\x12\x12\n\nowner_name\x18\x01 \x01(\t"L\n\x13CreateOwnerResponse\x12\x10\n\x08owner_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\x12\n\nowner_name\x18\x03 \x01(\t";\n\x13CreateWalletRequest\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t"_\n\x14CreateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x13\n\x0bwallet_name\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03did\x18\x04 \x01(\t"\x88\x01\n\x12ListWalletsRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x0f\n\x07api_key\x18\x03 \x01(\t\x12\x1f\n\x07filters\x18\x04 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x05 \x01(\x0b2\x0e.common.SortBy"\xc1\x01\n\x13ListWalletsResponse\x12:\n\x07wallets\x18\x01 \x03(\x0b2).wallet_api.v1.ListWalletsResponse.Wallet\x12\x1b\n\x13total_owned_wallets\x18\x02 \x01(\r\x1aQ\n\x06Wallet\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03did\x18\x04 \x01(\t"`\n\x19ListWalletBalancesRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\x1f\n\x07sort_by\x18\x03 \x01(\x0b2\x0e.common.SortBy"C\n\rWalletBalance\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x12\n\nfile_count\x18\x02 \x01(\x03"L\n\x1aListWalletBalancesResponse\x12.\n\x08balances\x18\x01 \x03(\x0b2\x1c.wallet_api.v1.WalletBalance"\x9e\x01\n\x15ListAssetFilesRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x11\n\twallet_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t\x12\x1f\n\x07filters\x18\x05 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x06 \x01(\x0b2\x0e.common.SortBy"\xb7\x04\n\x16ListAssetFilesResponse\x129\n\x05funds\x18\x01 \x03(\x0b2*.wallet_api.v1.ListAssetFilesResponse.Fund\x12\x19\n\x11total_owned_funds\x18\x02 \x01(\r\x1a(\n\x06Notary\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x11\n\tsignature\x18\x02 \x01(\x0c\x1a\xa9\x02\n\nPromissory\x12\x0f\n\x07version\x18\x01 \x01(\t\x12\n\n\x02id\x18\x03 \x01(\t\x12\x10\n\x08currency\x18\x04 \x01(\t\x12\x0e\n\x06amount\x18\x05 \x01(\x03\x12\x10\n\x08decimals\x18\x06 \x01(\r\x12\x11\n\tauthority\x18\x07 \x01(\t\x12\x0e\n\x06issuer\x18\x08 \x01(\t\x12\x15\n\rissuance_date\x18\t \x01(\t\x12\x13\n\x0bblock_depth\x18\n \x01(\r\x12\x1f\n\x17authorization_signature\x18\x0b \x01(\x0c\x12\x1a\n\x12transfer_signature\x18\x0c \x01(\x0c\x12>\n\x08notaries\x18\r \x03(\x0b2,.wallet_api.v1.ListAssetFilesResponse.Notary\x1aq\n\x04Fund\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12D\n\npromissory\x18\x02 \x01(\x0b20.wallet_api.v1.ListAssetFilesResponse.Promissory\x12\x10\n\x08reserved\x18\x03 \x01(\x08"j\n\rRedeemRequest\x12\x16\n\x0efrom_wallet_id\x18\x01 \x01(\t\x12\x19\n\x11issuer_public_key\x18\x02 \x01(\t\x12\x15\n\rpromissory_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t"3\n\x0eRedeemResponse\x12!\n\tnew_limit\x18\x01 \x01(\x0b2\x0e.common.Amount"\x9f\x01\n\x16ListCredentialsRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x11\n\twallet_id\x18\x03 \x01(\t\x12\x0f\n\x07api_key\x18\x04 \x01(\t\x12\x1f\n\x07filters\x18\x05 \x01(\x0b2\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x06 \x01(\x0b2\x0e.common.SortBy"O\n\x17ListCredentialsResponse\x12\x13\n\x0bcredentials\x18\x01 \x03(\t\x12\x1f\n\x17total_owned_credentials\x18\x02 \x01(\r"S\n\x14AddCredentialRequest\x12\x17\n\x0fcredential_type\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"+\n\x15AddCredentialResponse\x12\x12\n\ncredential\x18\x01 \x01(\t"<\n\x16AssociateWalletRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t"M\n\x17AssociateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0b\n\x03did\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t"V\n\x1fCreateAndAssociateWalletRequest\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\r\n\x05token\x18\x03 \x01(\t"k\n CreateAndAssociateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x13\n\x0bwallet_name\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03did\x18\x04 \x01(\t"S\n\x1aRequestPresentationRequest\x12\x0b\n\x03did\x18\x01 \x01(\t\x12\x17\n\x0fcredential_type\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"3\n\x1bRequestPresentationResponse\x12\x14\n\x0cpresentation\x18\x01 \x01(\t"\x8c\x01\n\x1bPrepareSimplePaymentRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x11\n\trecipient\x18\x02 \x01(\t\x12\x16\n\x0euser_reference\x18\x03 \x01(\t\x12\x11\n\twallet_id\x18\x04 \x01(\t\x12\x0f\n\x07api_key\x18\x05 \x01(\t"\xdd\x01\n\x1cPrepareSimplePaymentResponse\x12B\n\x06status\x18\x01 \x01(\x0e22.wallet_api.v1.PrepareSimplePaymentResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t"k\n\x06Status\x12\x16\n\x12STATUS_UNSPECIFIED\x10\x00\x12\x12\n\x0eSTATUS_SUCCESS\x10\x01\x12\x16\n\x12STATUS_NEED_CHANGE\x10\x02\x12\x1d\n\x19STATUS_INSUFFICIENT_FUNDS\x10\x03"w\n\x19PrepareFundsChangeRequest\x12\x1e\n\x06amount\x18\x01 \x01(\x0b2\x0e.common.Amount\x12\x16\n\x0euser_reference\x18\x04 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"\xe0\x01\n\x1aPrepareFundsChangeResponse\x12@\n\x06status\x18\x01 \x01(\x0e20.wallet_api.v1.PrepareFundsChangeResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t"r\n\x06Status\x12\x16\n\x12STATUS_UNSPECIFIED\x10\x00\x12\x12\n\x0eSTATUS_SUCCESS\x10\x01\x12\x1d\n\x19STATUS_INSUFFICIENT_FUNDS\x10\x02\x12\x1d\n\x19STATUS_FUNDS_PRESENT_NOOP\x10\x03"\xa5\x01\n!PrepareContractTransactionRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\'\n\x0bcommitments\x18\x03 \x03(\x0b2\x12.common.Commitment\x12%\n\nconditions\x18\x04 \x03(\x0b2\x11.common.Condition\x12\x0c\n\x04memo\x18\x05 \x01(\t"\xb1\x01\n"PrepareContractTransactionResponse\x12H\n\x06status\x18\x01 \x01(\x0e28.wallet_api.v1.PrepareContractTransactionResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t"3\n\x06Status\x12\x0f\n\x0bUNSPECIFIED\x10\x00\x12\x0b\n\x07SUCCESS\x10\x01\x12\x0b\n\x07FAILURE\x10\x02"I\n\x15GetTransactionRequest\x12\x0c\n\x04uetr\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"B\n\x16GetTransactionResponse\x12(\n\x0btransaction\x18\x01 \x01(\x0b2\x13.common.Transaction"\x99\x01\n\x17ListTransactionsRequest\x12\n\n\x02id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\x1f\n\x07filters\x18\x03 \x01(\x0b2\x0e.common.Filter\x12\x0c\n\x04page\x18\x04 \x01(\x05\x12\x11\n\tpage_size\x18\x05 \x01(\x05\x12\x1f\n\x07sort_by\x18\x06 \x01(\x0b2\x0e.common.SortBy"j\n\x18ListTransactionsResponse\x12)\n\x0ctransactions\x18\x01 \x03(\x0b2\x13.common.Transaction\x12#\n\x1btotal_matching_transactions\x18\x02 \x01(\r"\xa8\x01\n\x1fListContractTransactionsRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\x1f\n\x07filters\x18\x03 \x01(\x0b2\x0e.common.Filter\x12\x0c\n\x04page\x18\x04 \x01(\x05\x12\x11\n\tpage_size\x18\x05 \x01(\x05\x12\x1f\n\x07sort_by\x18\x06 \x01(\x0b2\x0e.common.SortBy"\x84\x01\n\x0fContractSummary\x12\x13\n\x0bcontract_id\x18\x01 \x01(\t\x12\x15\n\rcontract_type\x18\x02 \x01(\t\x12\x0f\n\x07created\x18\x03 \x01(\t\x12\x16\n\x0ecounterparties\x18\x04 \x03(\t\x12\x0e\n\x06status\x18\x05 \x01(\t\x12\x0c\n\x04memo\x18\x06 \x01(\t"m\n ListContractTransactionsResponse\x121\n\tcontracts\x18\x01 \x03(\x0b2\x1e.wallet_api.v1.ContractSummary\x12\x16\n\x0etotalContracts\x18\x02 \x01(\r"Q\n\x1dGetTransactionSnapshotRequest\x12\x0c\n\x04uetr\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"O\n\x1eGetTransactionSnapshotResponse\x12-\n\x08snapshot\x18\x01 \x01(\x0b2\x1b.common.TransactionSnapshot"S\n\x1fListTransactionSnapshotsRequest\x12\x0c\n\x04uetr\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07api_key\x18\x03 \x01(\t"R\n ListTransactionSnapshotsResponse\x12.\n\tsnapshots\x18\x01 \x03(\x0b2\x1b.common.TransactionSnapshot"\x9e\x01\n\rBalanceDetail\x12-\n\x07balance\x18\x01 \x01(\x0b2\x1c.wallet_api.v1.WalletBalance\x12/\n\x04kind\x18\x02 \x01(\x0e2!.wallet_api.v1.BalanceDetail.Kind"-\n\x04Kind\x12\x12\n\x0eKIND_AVAILABLE\x10\x00\x12\x11\n\rKIND_RESERVED\x10\x01"E\n\x1fListWalletBalanceDetailsRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t"R\n ListWalletBalanceDetailsResponse\x12.\n\x08balances\x18\x01 \x03(\x0b2\x1c.wallet_api.v1.BalanceDetail"W\n#GetContractTransactionDetailRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07api_key\x18\x02 \x01(\t\x12\x0c\n\x04uetr\x18\x03 \x01(\t"\xc3\x03\n$GetContractTransactionDetailResponse\x12\x15\n\rcontract_type\x18\x01 \x01(\t\x12\'\n\x0bcommitments\x18\x02 \x03(\x0b2\x12.common.Commitment\x12\x12\n\noriginator\x18\x03 \x01(\t\x12\x0f\n\x07created\x18\x04 \x01(\t\x12\x16\n\x0ehash_presenter\x18\x05 \x01(\t\x12\x0c\n\x04hash\x18\x06 \x01(\x0c\x12\x12\n\nexpiration\x18\x07 \x01(\t\x12\x0c\n\x04memo\x18\x08 \x01(\t\x12I\n\x03log\x18\t \x03(\x0b2<.wallet_api.v1.GetContractTransactionDetailResponse.LogEntry\x1a\xa2\x01\n\x08LogEntry\x12\x0e\n\x06sender\x18\x01 \x01(\t\x12\x10\n\x08receiver\x18\x02 \x01(\t\x12\x1e\n\x06amount\x18\x03 \x01(\x0b2\x0e.common.Amount\x12\x11\n\ttimestamp\x18\x04 \x01(\t\x12\x0c\n\x04kind\x18\x05 \x01(\t\x12\r\n\x05state\x18\x06 \x01(\t\x12\r\n\x05event\x18\x07 \x01(\t\x12\x15\n\rpromissory_id\x18\x08 \x01(\t"\xc1\x01\n\x1dPrepareAtomicAssetSwapRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x1e\n\x06amount\x18\x03 \x01(\x0b2\x0e.common.Amount\x12\x12\n\nrespondent\x18\x04 \x01(\t\x12)\n\x11respondent_amount\x18\x05 \x01(\x0b2\x0e.common.Amount\x12\x0c\n\x04memo\x18\x06 \x01(\t\x12\x0f\n\x07timeout\x18\x07 \x01(\r"\xa9\x01\n\x1ePrepareAtomicAssetSwapResponse\x12D\n\x06status\x18\x01 \x01(\x0e24.wallet_api.v1.PrepareAtomicAssetSwapResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t"3\n\x06Status\x12\x0f\n\x0bUNSPECIFIED\x10\x00\x12\x0b\n\x07SUCCESS\x10\x01\x12\x0b\n\x07FAILURE\x10\x02"\xd6\x01\n PrepareCrossBorderPaymentRequest\x12\x0f\n\x07api_key\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x10\n\x08treasury\x18\x03 \x01(\t\x12\x1e\n\x06amount\x18\x04 \x01(\x0b2\x0e.common.Amount\x12\x12\n\nrespondent\x18\x05 \x01(\t\x12)\n\x11respondent_amount\x18\x06 \x01(\x0b2\x0e.common.Amount\x12\x0c\n\x04memo\x18\x07 \x01(\t\x12\x0f\n\x07timeout\x18\x08 \x01(\r"\xaf\x01\n!PrepareCrossBorderPaymentResponse\x12G\n\x06status\x18\x01 \x01(\x0e27.wallet_api.v1.PrepareCrossBorderPaymentResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t"3\n\x06Status\x12\x0f\n\x0bUNSPECIFIED\x10\x00\x12\x0b\n\x07SUCCESS\x10\x01\x12\x0b\n\x07FAILURE\x10\x022\x93\x13\n\rWalletService\x12K\n\x08Transfer\x12\x1e.wallet_api.v1.TransferRequest\x1a\x1f.wallet_api.v1.TransferResponse\x12T\n\x0bCreateOwner\x12!.wallet_api.v1.CreateOwnerRequest\x1a".wallet_api.v1.CreateOwnerResponse\x12W\n\x0cCreateWallet\x12".wallet_api.v1.CreateWalletRequest\x1a#.wallet_api.v1.CreateWalletResponse\x12T\n\x0bListWallets\x12!.wallet_api.v1.ListWalletsRequest\x1a".wallet_api.v1.ListWalletsResponse\x12]\n\x0eListAssetFiles\x12$.wallet_api.v1.ListAssetFilesRequest\x1a%.wallet_api.v1.ListAssetFilesResponse\x12i\n\x12ListWalletBalances\x12(.wallet_api.v1.ListWalletBalancesRequest\x1a).wallet_api.v1.ListWalletBalancesResponse\x12E\n\x06Redeem\x12\x1c.wallet_api.v1.RedeemRequest\x1a\x1d.wallet_api.v1.RedeemResponse\x12`\n\x0fListCredentials\x12%.wallet_api.v1.ListCredentialsRequest\x1a&.wallet_api.v1.ListCredentialsResponse\x12Z\n\rAddCredential\x12#.wallet_api.v1.AddCredentialRequest\x1a$.wallet_api.v1.AddCredentialResponse\x12`\n\x0fAssociateWallet\x12%.wallet_api.v1.AssociateWalletRequest\x1a&.wallet_api.v1.AssociateWalletResponse\x12l\n\x13RequestPresentation\x12).wallet_api.v1.RequestPresentationRequest\x1a*.wallet_api.v1.RequestPresentationResponse\x12o\n\x14PrepareSimplePayment\x12*.wallet_api.v1.PrepareSimplePaymentRequest\x1a+.wallet_api.v1.PrepareSimplePaymentResponse\x12i\n\x12PrepareFundsChange\x12(.wallet_api.v1.PrepareFundsChangeRequest\x1a).wallet_api.v1.PrepareFundsChangeResponse\x12]\n\x0eGetTransaction\x12$.wallet_api.v1.GetTransactionRequest\x1a%.wallet_api.v1.GetTransactionResponse\x12c\n\x10ListTransactions\x12&.wallet_api.v1.ListTransactionsRequest\x1a\'.wallet_api.v1.ListTransactionsResponse\x12u\n\x16GetTransactionSnapshot\x12,.wallet_api.v1.GetTransactionSnapshotRequest\x1a-.wallet_api.v1.GetTransactionSnapshotResponse\x12{\n\x18ListTransactionSnapshots\x12..wallet_api.v1.ListTransactionSnapshotsRequest\x1a/.wallet_api.v1.ListTransactionSnapshotsResponse\x12{\n\x18ListWalletBalanceDetails\x12..wallet_api.v1.ListWalletBalanceDetailsRequest\x1a/.wallet_api.v1.ListWalletBalanceDetailsResponse\x12{\n\x18ListContractTransactions\x12..wallet_api.v1.ListContractTransactionsRequest\x1a/.wallet_api.v1.ListContractTransactionsResponse\x12\x87\x01\n\x1cGetContractTransactionDetail\x122.wallet_api.v1.GetContractTransactionDetailRequest\x1a3.wallet_api.v1.GetContractTransactionDetailResponse\x12\x81\x01\n\x1aPrepareContractTransaction\x120.wallet_api.v1.PrepareContractTransactionRequest\x1a1.wallet_api.v1.PrepareContractTransactionResponse\x12u\n\x16PrepareAtomicAssetSwap\x12,.wallet_api.v1.PrepareAtomicAssetSwapRequest\x1a-.wallet_api.v1.PrepareAtomicAssetSwapResponse\x12~\n\x19PrepareCrossBorderPayment\x12/.wallet_api.v1.PrepareCrossBorderPaymentRequest\x1a0.wallet_api.v1.PrepareCrossBorderPaymentResponseB9Z7github.com/knox-networks/grpc-sdks/sdk/go/wallet_api/v1b\x06proto3')
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
_PREPARESIMPLEPAYMENTREQUEST = DESCRIPTOR.message_types_by_name['PrepareSimplePaymentRequest']
_PREPARESIMPLEPAYMENTRESPONSE = DESCRIPTOR.message_types_by_name['PrepareSimplePaymentResponse']
_PREPAREFUNDSCHANGEREQUEST = DESCRIPTOR.message_types_by_name['PrepareFundsChangeRequest']
_PREPAREFUNDSCHANGERESPONSE = DESCRIPTOR.message_types_by_name['PrepareFundsChangeResponse']
_PREPARECONTRACTTRANSACTIONREQUEST = DESCRIPTOR.message_types_by_name['PrepareContractTransactionRequest']
_PREPARECONTRACTTRANSACTIONRESPONSE = DESCRIPTOR.message_types_by_name['PrepareContractTransactionResponse']
_GETTRANSACTIONREQUEST = DESCRIPTOR.message_types_by_name['GetTransactionRequest']
_GETTRANSACTIONRESPONSE = DESCRIPTOR.message_types_by_name['GetTransactionResponse']
_LISTTRANSACTIONSREQUEST = DESCRIPTOR.message_types_by_name['ListTransactionsRequest']
_LISTTRANSACTIONSRESPONSE = DESCRIPTOR.message_types_by_name['ListTransactionsResponse']
_LISTCONTRACTTRANSACTIONSREQUEST = DESCRIPTOR.message_types_by_name['ListContractTransactionsRequest']
_CONTRACTSUMMARY = DESCRIPTOR.message_types_by_name['ContractSummary']
_LISTCONTRACTTRANSACTIONSRESPONSE = DESCRIPTOR.message_types_by_name['ListContractTransactionsResponse']
_GETTRANSACTIONSNAPSHOTREQUEST = DESCRIPTOR.message_types_by_name['GetTransactionSnapshotRequest']
_GETTRANSACTIONSNAPSHOTRESPONSE = DESCRIPTOR.message_types_by_name['GetTransactionSnapshotResponse']
_LISTTRANSACTIONSNAPSHOTSREQUEST = DESCRIPTOR.message_types_by_name['ListTransactionSnapshotsRequest']
_LISTTRANSACTIONSNAPSHOTSRESPONSE = DESCRIPTOR.message_types_by_name['ListTransactionSnapshotsResponse']
_BALANCEDETAIL = DESCRIPTOR.message_types_by_name['BalanceDetail']
_LISTWALLETBALANCEDETAILSREQUEST = DESCRIPTOR.message_types_by_name['ListWalletBalanceDetailsRequest']
_LISTWALLETBALANCEDETAILSRESPONSE = DESCRIPTOR.message_types_by_name['ListWalletBalanceDetailsResponse']
_GETCONTRACTTRANSACTIONDETAILREQUEST = DESCRIPTOR.message_types_by_name['GetContractTransactionDetailRequest']
_GETCONTRACTTRANSACTIONDETAILRESPONSE = DESCRIPTOR.message_types_by_name['GetContractTransactionDetailResponse']
_GETCONTRACTTRANSACTIONDETAILRESPONSE_LOGENTRY = _GETCONTRACTTRANSACTIONDETAILRESPONSE.nested_types_by_name['LogEntry']
_PREPAREATOMICASSETSWAPREQUEST = DESCRIPTOR.message_types_by_name['PrepareAtomicAssetSwapRequest']
_PREPAREATOMICASSETSWAPRESPONSE = DESCRIPTOR.message_types_by_name['PrepareAtomicAssetSwapResponse']
_PREPARECROSSBORDERPAYMENTREQUEST = DESCRIPTOR.message_types_by_name['PrepareCrossBorderPaymentRequest']
_PREPARECROSSBORDERPAYMENTRESPONSE = DESCRIPTOR.message_types_by_name['PrepareCrossBorderPaymentResponse']
_PREPARESIMPLEPAYMENTRESPONSE_STATUS = _PREPARESIMPLEPAYMENTRESPONSE.enum_types_by_name['Status']
_PREPAREFUNDSCHANGERESPONSE_STATUS = _PREPAREFUNDSCHANGERESPONSE.enum_types_by_name['Status']
_PREPARECONTRACTTRANSACTIONRESPONSE_STATUS = _PREPARECONTRACTTRANSACTIONRESPONSE.enum_types_by_name['Status']
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
PrepareSimplePaymentRequest = _reflection.GeneratedProtocolMessageType('PrepareSimplePaymentRequest', (_message.Message,), {'DESCRIPTOR': _PREPARESIMPLEPAYMENTREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareSimplePaymentRequest)
PrepareSimplePaymentResponse = _reflection.GeneratedProtocolMessageType('PrepareSimplePaymentResponse', (_message.Message,), {'DESCRIPTOR': _PREPARESIMPLEPAYMENTRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareSimplePaymentResponse)
PrepareFundsChangeRequest = _reflection.GeneratedProtocolMessageType('PrepareFundsChangeRequest', (_message.Message,), {'DESCRIPTOR': _PREPAREFUNDSCHANGEREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareFundsChangeRequest)
PrepareFundsChangeResponse = _reflection.GeneratedProtocolMessageType('PrepareFundsChangeResponse', (_message.Message,), {'DESCRIPTOR': _PREPAREFUNDSCHANGERESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareFundsChangeResponse)
PrepareContractTransactionRequest = _reflection.GeneratedProtocolMessageType('PrepareContractTransactionRequest', (_message.Message,), {'DESCRIPTOR': _PREPARECONTRACTTRANSACTIONREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareContractTransactionRequest)
PrepareContractTransactionResponse = _reflection.GeneratedProtocolMessageType('PrepareContractTransactionResponse', (_message.Message,), {'DESCRIPTOR': _PREPARECONTRACTTRANSACTIONRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(PrepareContractTransactionResponse)
GetTransactionRequest = _reflection.GeneratedProtocolMessageType('GetTransactionRequest', (_message.Message,), {'DESCRIPTOR': _GETTRANSACTIONREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetTransactionRequest)
GetTransactionResponse = _reflection.GeneratedProtocolMessageType('GetTransactionResponse', (_message.Message,), {'DESCRIPTOR': _GETTRANSACTIONRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetTransactionResponse)
ListTransactionsRequest = _reflection.GeneratedProtocolMessageType('ListTransactionsRequest', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListTransactionsRequest)
ListTransactionsResponse = _reflection.GeneratedProtocolMessageType('ListTransactionsResponse', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListTransactionsResponse)
ListContractTransactionsRequest = _reflection.GeneratedProtocolMessageType('ListContractTransactionsRequest', (_message.Message,), {'DESCRIPTOR': _LISTCONTRACTTRANSACTIONSREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListContractTransactionsRequest)
ContractSummary = _reflection.GeneratedProtocolMessageType('ContractSummary', (_message.Message,), {'DESCRIPTOR': _CONTRACTSUMMARY, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ContractSummary)
ListContractTransactionsResponse = _reflection.GeneratedProtocolMessageType('ListContractTransactionsResponse', (_message.Message,), {'DESCRIPTOR': _LISTCONTRACTTRANSACTIONSRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListContractTransactionsResponse)
GetTransactionSnapshotRequest = _reflection.GeneratedProtocolMessageType('GetTransactionSnapshotRequest', (_message.Message,), {'DESCRIPTOR': _GETTRANSACTIONSNAPSHOTREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetTransactionSnapshotRequest)
GetTransactionSnapshotResponse = _reflection.GeneratedProtocolMessageType('GetTransactionSnapshotResponse', (_message.Message,), {'DESCRIPTOR': _GETTRANSACTIONSNAPSHOTRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetTransactionSnapshotResponse)
ListTransactionSnapshotsRequest = _reflection.GeneratedProtocolMessageType('ListTransactionSnapshotsRequest', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSNAPSHOTSREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListTransactionSnapshotsRequest)
ListTransactionSnapshotsResponse = _reflection.GeneratedProtocolMessageType('ListTransactionSnapshotsResponse', (_message.Message,), {'DESCRIPTOR': _LISTTRANSACTIONSNAPSHOTSRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListTransactionSnapshotsResponse)
BalanceDetail = _reflection.GeneratedProtocolMessageType('BalanceDetail', (_message.Message,), {'DESCRIPTOR': _BALANCEDETAIL, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(BalanceDetail)
ListWalletBalanceDetailsRequest = _reflection.GeneratedProtocolMessageType('ListWalletBalanceDetailsRequest', (_message.Message,), {'DESCRIPTOR': _LISTWALLETBALANCEDETAILSREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListWalletBalanceDetailsRequest)
ListWalletBalanceDetailsResponse = _reflection.GeneratedProtocolMessageType('ListWalletBalanceDetailsResponse', (_message.Message,), {'DESCRIPTOR': _LISTWALLETBALANCEDETAILSRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(ListWalletBalanceDetailsResponse)
GetContractTransactionDetailRequest = _reflection.GeneratedProtocolMessageType('GetContractTransactionDetailRequest', (_message.Message,), {'DESCRIPTOR': _GETCONTRACTTRANSACTIONDETAILREQUEST, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetContractTransactionDetailRequest)
GetContractTransactionDetailResponse = _reflection.GeneratedProtocolMessageType('GetContractTransactionDetailResponse', (_message.Message,), {'LogEntry': _reflection.GeneratedProtocolMessageType('LogEntry', (_message.Message,), {'DESCRIPTOR': _GETCONTRACTTRANSACTIONDETAILRESPONSE_LOGENTRY, '__module__': 'wallet_api.v1.wallet_pb2'}), 'DESCRIPTOR': _GETCONTRACTTRANSACTIONDETAILRESPONSE, '__module__': 'wallet_api.v1.wallet_pb2'})
_sym_db.RegisterMessage(GetContractTransactionDetailResponse)
_sym_db.RegisterMessage(GetContractTransactionDetailResponse.LogEntry)
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
    _PREPARESIMPLEPAYMENTREQUEST._serialized_start = 2826
    _PREPARESIMPLEPAYMENTREQUEST._serialized_end = 2966
    _PREPARESIMPLEPAYMENTRESPONSE._serialized_start = 2969
    _PREPARESIMPLEPAYMENTRESPONSE._serialized_end = 3190
    _PREPARESIMPLEPAYMENTRESPONSE_STATUS._serialized_start = 3083
    _PREPARESIMPLEPAYMENTRESPONSE_STATUS._serialized_end = 3190
    _PREPAREFUNDSCHANGEREQUEST._serialized_start = 3192
    _PREPAREFUNDSCHANGEREQUEST._serialized_end = 3311
    _PREPAREFUNDSCHANGERESPONSE._serialized_start = 3314
    _PREPAREFUNDSCHANGERESPONSE._serialized_end = 3538
    _PREPAREFUNDSCHANGERESPONSE_STATUS._serialized_start = 3424
    _PREPAREFUNDSCHANGERESPONSE_STATUS._serialized_end = 3538
    _PREPARECONTRACTTRANSACTIONREQUEST._serialized_start = 3541
    _PREPARECONTRACTTRANSACTIONREQUEST._serialized_end = 3706
    _PREPARECONTRACTTRANSACTIONRESPONSE._serialized_start = 3709
    _PREPARECONTRACTTRANSACTIONRESPONSE._serialized_end = 3886
    _PREPARECONTRACTTRANSACTIONRESPONSE_STATUS._serialized_start = 3835
    _PREPARECONTRACTTRANSACTIONRESPONSE_STATUS._serialized_end = 3886
    _GETTRANSACTIONREQUEST._serialized_start = 3888
    _GETTRANSACTIONREQUEST._serialized_end = 3961
    _GETTRANSACTIONRESPONSE._serialized_start = 3963
    _GETTRANSACTIONRESPONSE._serialized_end = 4029
    _LISTTRANSACTIONSREQUEST._serialized_start = 4032
    _LISTTRANSACTIONSREQUEST._serialized_end = 4185
    _LISTTRANSACTIONSRESPONSE._serialized_start = 4187
    _LISTTRANSACTIONSRESPONSE._serialized_end = 4293
    _LISTCONTRACTTRANSACTIONSREQUEST._serialized_start = 4296
    _LISTCONTRACTTRANSACTIONSREQUEST._serialized_end = 4464
    _CONTRACTSUMMARY._serialized_start = 4467
    _CONTRACTSUMMARY._serialized_end = 4599
    _LISTCONTRACTTRANSACTIONSRESPONSE._serialized_start = 4601
    _LISTCONTRACTTRANSACTIONSRESPONSE._serialized_end = 4710
    _GETTRANSACTIONSNAPSHOTREQUEST._serialized_start = 4712
    _GETTRANSACTIONSNAPSHOTREQUEST._serialized_end = 4793
    _GETTRANSACTIONSNAPSHOTRESPONSE._serialized_start = 4795
    _GETTRANSACTIONSNAPSHOTRESPONSE._serialized_end = 4874
    _LISTTRANSACTIONSNAPSHOTSREQUEST._serialized_start = 4876
    _LISTTRANSACTIONSNAPSHOTSREQUEST._serialized_end = 4959
    _LISTTRANSACTIONSNAPSHOTSRESPONSE._serialized_start = 4961
    _LISTTRANSACTIONSNAPSHOTSRESPONSE._serialized_end = 5043
    _BALANCEDETAIL._serialized_start = 5046
    _BALANCEDETAIL._serialized_end = 5204
    _BALANCEDETAIL_KIND._serialized_start = 5159
    _BALANCEDETAIL_KIND._serialized_end = 5204
    _LISTWALLETBALANCEDETAILSREQUEST._serialized_start = 5206
    _LISTWALLETBALANCEDETAILSREQUEST._serialized_end = 5275
    _LISTWALLETBALANCEDETAILSRESPONSE._serialized_start = 5277
    _LISTWALLETBALANCEDETAILSRESPONSE._serialized_end = 5359
    _GETCONTRACTTRANSACTIONDETAILREQUEST._serialized_start = 5361
    _GETCONTRACTTRANSACTIONDETAILREQUEST._serialized_end = 5448
    _GETCONTRACTTRANSACTIONDETAILRESPONSE._serialized_start = 5451
    _GETCONTRACTTRANSACTIONDETAILRESPONSE._serialized_end = 5902
    _GETCONTRACTTRANSACTIONDETAILRESPONSE_LOGENTRY._serialized_start = 5740
    _GETCONTRACTTRANSACTIONDETAILRESPONSE_LOGENTRY._serialized_end = 5902
    _PREPAREATOMICASSETSWAPREQUEST._serialized_start = 5905
    _PREPAREATOMICASSETSWAPREQUEST._serialized_end = 6098
    _PREPAREATOMICASSETSWAPRESPONSE._serialized_start = 6101
    _PREPAREATOMICASSETSWAPRESPONSE._serialized_end = 6270
    _PREPAREATOMICASSETSWAPRESPONSE_STATUS._serialized_start = 3835
    _PREPAREATOMICASSETSWAPRESPONSE_STATUS._serialized_end = 3886
    _PREPARECROSSBORDERPAYMENTREQUEST._serialized_start = 6273
    _PREPARECROSSBORDERPAYMENTREQUEST._serialized_end = 6487
    _PREPARECROSSBORDERPAYMENTRESPONSE._serialized_start = 6490
    _PREPARECROSSBORDERPAYMENTRESPONSE._serialized_end = 6665
    _PREPARECROSSBORDERPAYMENTRESPONSE_STATUS._serialized_start = 3835
    _PREPARECROSSBORDERPAYMENTRESPONSE_STATUS._serialized_end = 3886
    _WALLETSERVICE._serialized_start = 6668
    _WALLETSERVICE._serialized_end = 9119