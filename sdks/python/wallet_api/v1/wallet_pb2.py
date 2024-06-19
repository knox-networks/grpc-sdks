# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# NO CHECKED-IN PROTOBUF GENCODE
# source: wallet_api/v1/wallet.proto
# Protobuf Python Version: 5.27.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import runtime_version as _runtime_version
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
_runtime_version.ValidateProtobufRuntimeVersion(
    _runtime_version.Domain.PUBLIC,
    5,
    27,
    1,
    '',
    'wallet_api/v1/wallet.proto'
)
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from common.v1 import common_pb2 as common_dot_v1_dot_common__pb2
from common.v1 import packet_pb2 as common_dot_v1_dot_packet__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1awallet_api/v1/wallet.proto\x12\rwallet_api.v1\x1a\x16\x63ommon/v1/common.proto\x1a\x16\x63ommon/v1/packet.proto\"(\n\x12\x43reateOwnerRequest\x12\x12\n\nowner_name\x18\x01 \x01(\t\"L\n\x13\x43reateOwnerResponse\x12\x10\n\x08owner_id\x18\x01 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x02 \x01(\t\x12\x12\n\nowner_name\x18\x03 \x01(\t\";\n\x13\x43reateWalletRequest\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x02 \x01(\t\"_\n\x14\x43reateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x13\n\x0bwallet_name\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03\x64id\x18\x04 \x01(\t\"\x88\x01\n\x12ListWalletsRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x0f\n\x07\x61pi_key\x18\x03 \x01(\t\x12\x1f\n\x07\x66ilters\x18\x04 \x01(\x0b\x32\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x05 \x01(\x0b\x32\x0e.common.SortBy\"\xc1\x01\n\x13ListWalletsResponse\x12:\n\x07wallets\x18\x01 \x03(\x0b\x32).wallet_api.v1.ListWalletsResponse.Wallet\x12\x1b\n\x13total_owned_wallets\x18\x02 \x01(\r\x1aQ\n\x06Wallet\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03\x64id\x18\x04 \x01(\t\"`\n\x19ListWalletBalancesRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x02 \x01(\t\x12\x1f\n\x07sort_by\x18\x03 \x01(\x0b\x32\x0e.common.SortBy\"C\n\rWalletBalance\x12\x1e\n\x06\x61mount\x18\x01 \x01(\x0b\x32\x0e.common.Amount\x12\x12\n\nfile_count\x18\x02 \x01(\x03\"L\n\x1aListWalletBalancesResponse\x12.\n\x08\x62\x61lances\x18\x01 \x03(\x0b\x32\x1c.wallet_api.v1.WalletBalance\"\x9e\x01\n\x15ListAssetFilesRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x11\n\twallet_id\x18\x03 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x04 \x01(\t\x12\x1f\n\x07\x66ilters\x18\x05 \x01(\x0b\x32\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x06 \x01(\x0b\x32\x0e.common.SortBy\"\xb7\x04\n\x16ListAssetFilesResponse\x12\x39\n\x05\x66unds\x18\x01 \x03(\x0b\x32*.wallet_api.v1.ListAssetFilesResponse.Fund\x12\x19\n\x11total_owned_funds\x18\x02 \x01(\r\x1a(\n\x06Notary\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\x12\x11\n\tsignature\x18\x02 \x01(\x0c\x1a\xa9\x02\n\nPromissory\x12\x0f\n\x07version\x18\x01 \x01(\t\x12\n\n\x02id\x18\x03 \x01(\t\x12\x10\n\x08\x63urrency\x18\x04 \x01(\t\x12\x0e\n\x06\x61mount\x18\x05 \x01(\x03\x12\x10\n\x08\x64\x65\x63imals\x18\x06 \x01(\r\x12\x11\n\tauthority\x18\x07 \x01(\t\x12\x0e\n\x06issuer\x18\x08 \x01(\t\x12\x15\n\rissuance_date\x18\t \x01(\t\x12\x13\n\x0b\x62lock_depth\x18\n \x01(\r\x12\x1f\n\x17\x61uthorization_signature\x18\x0b \x01(\x0c\x12\x1a\n\x12transfer_signature\x18\x0c \x01(\x0c\x12>\n\x08notaries\x18\r \x03(\x0b\x32,.wallet_api.v1.ListAssetFilesResponse.Notary\x1aq\n\x04\x46und\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x44\n\npromissory\x18\x02 \x01(\x0b\x32\x30.wallet_api.v1.ListAssetFilesResponse.Promissory\x12\x10\n\x08reserved\x18\x03 \x01(\x08\"\x9f\x01\n\x16ListCredentialsRequest\x12\x0c\n\x04page\x18\x01 \x01(\x05\x12\x11\n\tpage_size\x18\x02 \x01(\x05\x12\x11\n\twallet_id\x18\x03 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x04 \x01(\t\x12\x1f\n\x07\x66ilters\x18\x05 \x01(\x0b\x32\x0e.common.Filter\x12\x1f\n\x07sort_by\x18\x06 \x01(\x0b\x32\x0e.common.SortBy\"O\n\x17ListCredentialsResponse\x12\x13\n\x0b\x63redentials\x18\x01 \x03(\t\x12\x1f\n\x17total_owned_credentials\x18\x02 \x01(\r\"S\n\x14\x41\x64\x64\x43redentialRequest\x12\x17\n\x0f\x63redential_type\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x03 \x01(\t\"+\n\x15\x41\x64\x64\x43redentialResponse\x12\x12\n\ncredential\x18\x01 \x01(\t\"<\n\x16\x41ssociateWalletRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x02 \x01(\t\"M\n\x17\x41ssociateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0b\n\x03\x64id\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\"V\n\x1f\x43reateAndAssociateWalletRequest\x12\x13\n\x0bwallet_name\x18\x01 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x02 \x01(\t\x12\r\n\x05token\x18\x03 \x01(\t\"k\n CreateAndAssociateWalletResponse\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x13\n\x0bwallet_name\x18\x02 \x01(\t\x12\x12\n\npublic_key\x18\x03 \x01(\t\x12\x0b\n\x03\x64id\x18\x04 \x01(\t\"S\n\x1aRequestPresentationRequest\x12\x0b\n\x03\x64id\x18\x01 \x01(\t\x12\x17\n\x0f\x63redential_type\x18\x02 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x03 \x01(\t\"3\n\x1bRequestPresentationResponse\x12\x14\n\x0cpresentation\x18\x01 \x01(\t\"w\n\x19PrepareFundsChangeRequest\x12\x1e\n\x06\x61mount\x18\x01 \x01(\x0b\x32\x0e.common.Amount\x12\x16\n\x0euser_reference\x18\x04 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x03 \x01(\t\"\xe0\x01\n\x1aPrepareFundsChangeResponse\x12@\n\x06status\x18\x01 \x01(\x0e\x32\x30.wallet_api.v1.PrepareFundsChangeResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t\"r\n\x06Status\x12\x16\n\x12STATUS_UNSPECIFIED\x10\x00\x12\x12\n\x0eSTATUS_SUCCESS\x10\x01\x12\x1d\n\x19STATUS_INSUFFICIENT_FUNDS\x10\x02\x12\x1d\n\x19STATUS_FUNDS_PRESENT_NOOP\x10\x03\"\x9d\x01\n\x19PrepareTransactionRequest\x12\x0f\n\x07\x61pi_key\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\'\n\x0b\x63ommitments\x18\x03 \x03(\x0b\x32\x12.common.Commitment\x12%\n\nconditions\x18\x04 \x03(\x0b\x32\x11.common.Condition\x12\x0c\n\x04memo\x18\x05 \x01(\t\"\xa1\x01\n\x1aPrepareTransactionResponse\x12@\n\x06status\x18\x01 \x01(\x0e\x32\x30.wallet_api.v1.PrepareTransactionResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t\"3\n\x06Status\x12\x0f\n\x0bUNSPECIFIED\x10\x00\x12\x0b\n\x07SUCCESS\x10\x01\x12\x0b\n\x07\x46\x41ILURE\x10\x02\"\xa0\x01\n\x17ListTransactionsRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x02 \x01(\t\x12\x1f\n\x07\x66ilters\x18\x03 \x01(\x0b\x32\x0e.common.Filter\x12\x0c\n\x04page\x18\x04 \x01(\x05\x12\x11\n\tpage_size\x18\x05 \x01(\x05\x12\x1f\n\x07sort_by\x18\x06 \x01(\x0b\x32\x0e.common.SortBy\"\x84\x01\n\x0f\x43ontractSummary\x12\x13\n\x0b\x63ontract_id\x18\x01 \x01(\t\x12\x15\n\rcontract_type\x18\x02 \x01(\t\x12\x0f\n\x07\x63reated\x18\x03 \x01(\t\x12\x16\n\x0e\x63ounterparties\x18\x04 \x03(\t\x12\x0e\n\x06status\x18\x05 \x01(\t\x12\x0c\n\x04memo\x18\x06 \x01(\t\"e\n\x18ListTransactionsResponse\x12\x31\n\tcontracts\x18\x01 \x03(\x0b\x32\x1e.wallet_api.v1.ContractSummary\x12\x16\n\x0etotalContracts\x18\x02 \x01(\r\"\x9e\x01\n\rBalanceDetail\x12-\n\x07\x62\x61lance\x18\x01 \x01(\x0b\x32\x1c.wallet_api.v1.WalletBalance\x12/\n\x04kind\x18\x02 \x01(\x0e\x32!.wallet_api.v1.BalanceDetail.Kind\"-\n\x04Kind\x12\x12\n\x0eKIND_AVAILABLE\x10\x00\x12\x11\n\rKIND_RESERVED\x10\x01\"E\n\x1fListWalletBalanceDetailsRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x02 \x01(\t\"R\n ListWalletBalanceDetailsResponse\x12.\n\x08\x62\x61lances\x18\x01 \x03(\x0b\x32\x1c.wallet_api.v1.BalanceDetail\"O\n\x1bGetTransactionDetailRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x02 \x01(\t\x12\x0c\n\x04uetr\x18\x03 \x01(\t\"\x9f\x03\n\x1cGetTransactionDetailResponse\x12\x15\n\rcontract_type\x18\x01 \x01(\t\x12\'\n\x0b\x63ommitments\x18\x02 \x03(\x0b\x32\x12.common.Commitment\x12\x0f\n\x07\x63reated\x18\x04 \x01(\t\x12\x16\n\x0ehash_presenter\x18\x05 \x01(\t\x12\x0c\n\x04hash\x18\x06 \x01(\x0c\x12\x12\n\nexpiration\x18\x07 \x01(\t\x12\x0c\n\x04memo\x18\x08 \x01(\t\x12\x41\n\x03log\x18\t \x03(\x0b\x32\x34.wallet_api.v1.GetTransactionDetailResponse.LogEntry\x1a\xa2\x01\n\x08LogEntry\x12\x0e\n\x06sender\x18\x01 \x01(\t\x12\x10\n\x08receiver\x18\x02 \x01(\t\x12\x1e\n\x06\x61mount\x18\x03 \x01(\x0b\x32\x0e.common.Amount\x12\x11\n\ttimestamp\x18\x04 \x01(\t\x12\x0c\n\x04kind\x18\x05 \x01(\t\x12\r\n\x05state\x18\x06 \x01(\t\x12\r\n\x05\x65vent\x18\x07 \x01(\t\x12\x15\n\rpromissory_id\x18\x08 \x01(\t\"\xc1\x01\n\x1dPrepareAtomicAssetSwapRequest\x12\x0f\n\x07\x61pi_key\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x1e\n\x06\x61mount\x18\x03 \x01(\x0b\x32\x0e.common.Amount\x12\x12\n\nrespondent\x18\x04 \x01(\t\x12)\n\x11respondent_amount\x18\x05 \x01(\x0b\x32\x0e.common.Amount\x12\x0c\n\x04memo\x18\x06 \x01(\t\x12\x0f\n\x07timeout\x18\x07 \x01(\r\"\xa9\x01\n\x1ePrepareAtomicAssetSwapResponse\x12\x44\n\x06status\x18\x01 \x01(\x0e\x32\x34.wallet_api.v1.PrepareAtomicAssetSwapResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t\"3\n\x06Status\x12\x0f\n\x0bUNSPECIFIED\x10\x00\x12\x0b\n\x07SUCCESS\x10\x01\x12\x0b\n\x07\x46\x41ILURE\x10\x02\"\xd6\x01\n PrepareCrossBorderPaymentRequest\x12\x0f\n\x07\x61pi_key\x18\x01 \x01(\t\x12\x11\n\twallet_id\x18\x02 \x01(\t\x12\x10\n\x08treasury\x18\x03 \x01(\t\x12\x1e\n\x06\x61mount\x18\x04 \x01(\x0b\x32\x0e.common.Amount\x12\x12\n\nrespondent\x18\x05 \x01(\t\x12)\n\x11respondent_amount\x18\x06 \x01(\x0b\x32\x0e.common.Amount\x12\x0c\n\x04memo\x18\x07 \x01(\t\x12\x0f\n\x07timeout\x18\x08 \x01(\r\"\xaf\x01\n!PrepareCrossBorderPaymentResponse\x12G\n\x06status\x18\x01 \x01(\x0e\x32\x37.wallet_api.v1.PrepareCrossBorderPaymentResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t\"3\n\x06Status\x12\x0f\n\x0bUNSPECIFIED\x10\x00\x12\x0b\n\x07SUCCESS\x10\x01\x12\x0b\n\x07\x46\x41ILURE\x10\x02\"\x9d\x01\n,PreparePaymentWithImplicitFundsChangeRequest\x12\x11\n\trecipient\x18\x01 \x01(\t\x12\x1e\n\x06\x61mount\x18\x02 \x01(\x0b\x32\x0e.common.Amount\x12\x16\n\x0euser_reference\x18\x03 \x01(\t\x12\x11\n\twallet_id\x18\x04 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x05 \x01(\t\"\xdf\x01\n-PreparePaymentWithImplicitFundsChangeResponse\x12S\n\x06status\x18\x01 \x01(\x0e\x32\x43.wallet_api.v1.PreparePaymentWithImplicitFundsChangeResponse.Status\x12\x0c\n\x04uetr\x18\x02 \x01(\t\"K\n\x06Status\x12\x0f\n\x0bUNSPECIFIED\x10\x00\x12\x0b\n\x07SUCCESS\x10\x01\x12\x0b\n\x07\x46\x41ILURE\x10\x02\x12\x16\n\x12INSUFFICIENT_FUNDS\x10\x03\"b\n\x1eResumePausedTransactionRequest\x12\x11\n\twallet_id\x18\x01 \x01(\t\x12\x0f\n\x07\x61pi_key\x18\x02 \x01(\t\x12\x0c\n\x04uetr\x18\x03 \x01(\t\x12\x0e\n\x06\x61\x63\x63\x65pt\x18\x04 \x01(\x08\"!\n\x1fResumePausedTransactionResponse2\xab\x0f\n\rWalletService\x12T\n\x0b\x43reateOwner\x12!.wallet_api.v1.CreateOwnerRequest\x1a\".wallet_api.v1.CreateOwnerResponse\x12W\n\x0c\x43reateWallet\x12\".wallet_api.v1.CreateWalletRequest\x1a#.wallet_api.v1.CreateWalletResponse\x12T\n\x0bListWallets\x12!.wallet_api.v1.ListWalletsRequest\x1a\".wallet_api.v1.ListWalletsResponse\x12]\n\x0eListAssetFiles\x12$.wallet_api.v1.ListAssetFilesRequest\x1a%.wallet_api.v1.ListAssetFilesResponse\x12i\n\x12ListWalletBalances\x12(.wallet_api.v1.ListWalletBalancesRequest\x1a).wallet_api.v1.ListWalletBalancesResponse\x12`\n\x0fListCredentials\x12%.wallet_api.v1.ListCredentialsRequest\x1a&.wallet_api.v1.ListCredentialsResponse\x12Z\n\rAddCredential\x12#.wallet_api.v1.AddCredentialRequest\x1a$.wallet_api.v1.AddCredentialResponse\x12`\n\x0f\x41ssociateWallet\x12%.wallet_api.v1.AssociateWalletRequest\x1a&.wallet_api.v1.AssociateWalletResponse\x12l\n\x13RequestPresentation\x12).wallet_api.v1.RequestPresentationRequest\x1a*.wallet_api.v1.RequestPresentationResponse\x12{\n\x18ListWalletBalanceDetails\x12..wallet_api.v1.ListWalletBalanceDetailsRequest\x1a/.wallet_api.v1.ListWalletBalanceDetailsResponse\x12\x63\n\x10ListTransactions\x12&.wallet_api.v1.ListTransactionsRequest\x1a\'.wallet_api.v1.ListTransactionsResponse\x12o\n\x14GetTransactionDetail\x12*.wallet_api.v1.GetTransactionDetailRequest\x1a+.wallet_api.v1.GetTransactionDetailResponse\x12i\n\x12PrepareTransaction\x12(.wallet_api.v1.PrepareTransactionRequest\x1a).wallet_api.v1.PrepareTransactionResponse\x12u\n\x16PrepareAtomicAssetSwap\x12,.wallet_api.v1.PrepareAtomicAssetSwapRequest\x1a-.wallet_api.v1.PrepareAtomicAssetSwapResponse\x12~\n\x19PrepareCrossBorderPayment\x12/.wallet_api.v1.PrepareCrossBorderPaymentRequest\x1a\x30.wallet_api.v1.PrepareCrossBorderPaymentResponse\x12i\n\x12PrepareFundsChange\x12(.wallet_api.v1.PrepareFundsChangeRequest\x1a).wallet_api.v1.PrepareFundsChangeResponse\x12\xa2\x01\n%PreparePaymentWithImplicitFundsChange\x12;.wallet_api.v1.PreparePaymentWithImplicitFundsChangeRequest\x1a<.wallet_api.v1.PreparePaymentWithImplicitFundsChangeResponse\x12x\n\x17ResumePausedTransaction\x12-.wallet_api.v1.ResumePausedTransactionRequest\x1a..wallet_api.v1.ResumePausedTransactionResponseB\x04Z\x02./b\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'wallet_api.v1.wallet_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  _globals['DESCRIPTOR']._loaded_options = None
  _globals['DESCRIPTOR']._serialized_options = b'Z\002./'
  _globals['_CREATEOWNERREQUEST']._serialized_start=93
  _globals['_CREATEOWNERREQUEST']._serialized_end=133
  _globals['_CREATEOWNERRESPONSE']._serialized_start=135
  _globals['_CREATEOWNERRESPONSE']._serialized_end=211
  _globals['_CREATEWALLETREQUEST']._serialized_start=213
  _globals['_CREATEWALLETREQUEST']._serialized_end=272
  _globals['_CREATEWALLETRESPONSE']._serialized_start=274
  _globals['_CREATEWALLETRESPONSE']._serialized_end=369
  _globals['_LISTWALLETSREQUEST']._serialized_start=372
  _globals['_LISTWALLETSREQUEST']._serialized_end=508
  _globals['_LISTWALLETSRESPONSE']._serialized_start=511
  _globals['_LISTWALLETSRESPONSE']._serialized_end=704
  _globals['_LISTWALLETSRESPONSE_WALLET']._serialized_start=623
  _globals['_LISTWALLETSRESPONSE_WALLET']._serialized_end=704
  _globals['_LISTWALLETBALANCESREQUEST']._serialized_start=706
  _globals['_LISTWALLETBALANCESREQUEST']._serialized_end=802
  _globals['_WALLETBALANCE']._serialized_start=804
  _globals['_WALLETBALANCE']._serialized_end=871
  _globals['_LISTWALLETBALANCESRESPONSE']._serialized_start=873
  _globals['_LISTWALLETBALANCESRESPONSE']._serialized_end=949
  _globals['_LISTASSETFILESREQUEST']._serialized_start=952
  _globals['_LISTASSETFILESREQUEST']._serialized_end=1110
  _globals['_LISTASSETFILESRESPONSE']._serialized_start=1113
  _globals['_LISTASSETFILESRESPONSE']._serialized_end=1680
  _globals['_LISTASSETFILESRESPONSE_NOTARY']._serialized_start=1225
  _globals['_LISTASSETFILESRESPONSE_NOTARY']._serialized_end=1265
  _globals['_LISTASSETFILESRESPONSE_PROMISSORY']._serialized_start=1268
  _globals['_LISTASSETFILESRESPONSE_PROMISSORY']._serialized_end=1565
  _globals['_LISTASSETFILESRESPONSE_FUND']._serialized_start=1567
  _globals['_LISTASSETFILESRESPONSE_FUND']._serialized_end=1680
  _globals['_LISTCREDENTIALSREQUEST']._serialized_start=1683
  _globals['_LISTCREDENTIALSREQUEST']._serialized_end=1842
  _globals['_LISTCREDENTIALSRESPONSE']._serialized_start=1844
  _globals['_LISTCREDENTIALSRESPONSE']._serialized_end=1923
  _globals['_ADDCREDENTIALREQUEST']._serialized_start=1925
  _globals['_ADDCREDENTIALREQUEST']._serialized_end=2008
  _globals['_ADDCREDENTIALRESPONSE']._serialized_start=2010
  _globals['_ADDCREDENTIALRESPONSE']._serialized_end=2053
  _globals['_ASSOCIATEWALLETREQUEST']._serialized_start=2055
  _globals['_ASSOCIATEWALLETREQUEST']._serialized_end=2115
  _globals['_ASSOCIATEWALLETRESPONSE']._serialized_start=2117
  _globals['_ASSOCIATEWALLETRESPONSE']._serialized_end=2194
  _globals['_CREATEANDASSOCIATEWALLETREQUEST']._serialized_start=2196
  _globals['_CREATEANDASSOCIATEWALLETREQUEST']._serialized_end=2282
  _globals['_CREATEANDASSOCIATEWALLETRESPONSE']._serialized_start=2284
  _globals['_CREATEANDASSOCIATEWALLETRESPONSE']._serialized_end=2391
  _globals['_REQUESTPRESENTATIONREQUEST']._serialized_start=2393
  _globals['_REQUESTPRESENTATIONREQUEST']._serialized_end=2476
  _globals['_REQUESTPRESENTATIONRESPONSE']._serialized_start=2478
  _globals['_REQUESTPRESENTATIONRESPONSE']._serialized_end=2529
  _globals['_PREPAREFUNDSCHANGEREQUEST']._serialized_start=2531
  _globals['_PREPAREFUNDSCHANGEREQUEST']._serialized_end=2650
  _globals['_PREPAREFUNDSCHANGERESPONSE']._serialized_start=2653
  _globals['_PREPAREFUNDSCHANGERESPONSE']._serialized_end=2877
  _globals['_PREPAREFUNDSCHANGERESPONSE_STATUS']._serialized_start=2763
  _globals['_PREPAREFUNDSCHANGERESPONSE_STATUS']._serialized_end=2877
  _globals['_PREPARETRANSACTIONREQUEST']._serialized_start=2880
  _globals['_PREPARETRANSACTIONREQUEST']._serialized_end=3037
  _globals['_PREPARETRANSACTIONRESPONSE']._serialized_start=3040
  _globals['_PREPARETRANSACTIONRESPONSE']._serialized_end=3201
  _globals['_PREPARETRANSACTIONRESPONSE_STATUS']._serialized_start=3150
  _globals['_PREPARETRANSACTIONRESPONSE_STATUS']._serialized_end=3201
  _globals['_LISTTRANSACTIONSREQUEST']._serialized_start=3204
  _globals['_LISTTRANSACTIONSREQUEST']._serialized_end=3364
  _globals['_CONTRACTSUMMARY']._serialized_start=3367
  _globals['_CONTRACTSUMMARY']._serialized_end=3499
  _globals['_LISTTRANSACTIONSRESPONSE']._serialized_start=3501
  _globals['_LISTTRANSACTIONSRESPONSE']._serialized_end=3602
  _globals['_BALANCEDETAIL']._serialized_start=3605
  _globals['_BALANCEDETAIL']._serialized_end=3763
  _globals['_BALANCEDETAIL_KIND']._serialized_start=3718
  _globals['_BALANCEDETAIL_KIND']._serialized_end=3763
  _globals['_LISTWALLETBALANCEDETAILSREQUEST']._serialized_start=3765
  _globals['_LISTWALLETBALANCEDETAILSREQUEST']._serialized_end=3834
  _globals['_LISTWALLETBALANCEDETAILSRESPONSE']._serialized_start=3836
  _globals['_LISTWALLETBALANCEDETAILSRESPONSE']._serialized_end=3918
  _globals['_GETTRANSACTIONDETAILREQUEST']._serialized_start=3920
  _globals['_GETTRANSACTIONDETAILREQUEST']._serialized_end=3999
  _globals['_GETTRANSACTIONDETAILRESPONSE']._serialized_start=4002
  _globals['_GETTRANSACTIONDETAILRESPONSE']._serialized_end=4417
  _globals['_GETTRANSACTIONDETAILRESPONSE_LOGENTRY']._serialized_start=4255
  _globals['_GETTRANSACTIONDETAILRESPONSE_LOGENTRY']._serialized_end=4417
  _globals['_PREPAREATOMICASSETSWAPREQUEST']._serialized_start=4420
  _globals['_PREPAREATOMICASSETSWAPREQUEST']._serialized_end=4613
  _globals['_PREPAREATOMICASSETSWAPRESPONSE']._serialized_start=4616
  _globals['_PREPAREATOMICASSETSWAPRESPONSE']._serialized_end=4785
  _globals['_PREPAREATOMICASSETSWAPRESPONSE_STATUS']._serialized_start=3150
  _globals['_PREPAREATOMICASSETSWAPRESPONSE_STATUS']._serialized_end=3201
  _globals['_PREPARECROSSBORDERPAYMENTREQUEST']._serialized_start=4788
  _globals['_PREPARECROSSBORDERPAYMENTREQUEST']._serialized_end=5002
  _globals['_PREPARECROSSBORDERPAYMENTRESPONSE']._serialized_start=5005
  _globals['_PREPARECROSSBORDERPAYMENTRESPONSE']._serialized_end=5180
  _globals['_PREPARECROSSBORDERPAYMENTRESPONSE_STATUS']._serialized_start=3150
  _globals['_PREPARECROSSBORDERPAYMENTRESPONSE_STATUS']._serialized_end=3201
  _globals['_PREPAREPAYMENTWITHIMPLICITFUNDSCHANGEREQUEST']._serialized_start=5183
  _globals['_PREPAREPAYMENTWITHIMPLICITFUNDSCHANGEREQUEST']._serialized_end=5340
  _globals['_PREPAREPAYMENTWITHIMPLICITFUNDSCHANGERESPONSE']._serialized_start=5343
  _globals['_PREPAREPAYMENTWITHIMPLICITFUNDSCHANGERESPONSE']._serialized_end=5566
  _globals['_PREPAREPAYMENTWITHIMPLICITFUNDSCHANGERESPONSE_STATUS']._serialized_start=5491
  _globals['_PREPAREPAYMENTWITHIMPLICITFUNDSCHANGERESPONSE_STATUS']._serialized_end=5566
  _globals['_RESUMEPAUSEDTRANSACTIONREQUEST']._serialized_start=5568
  _globals['_RESUMEPAUSEDTRANSACTIONREQUEST']._serialized_end=5666
  _globals['_RESUMEPAUSEDTRANSACTIONRESPONSE']._serialized_start=5668
  _globals['_RESUMEPAUSEDTRANSACTIONRESPONSE']._serialized_end=5701
  _globals['_WALLETSERVICE']._serialized_start=5704
  _globals['_WALLETSERVICE']._serialized_end=7667
# @@protoc_insertion_point(module_scope)