"""Client and server classes corresponding to protobuf-defined services."""
import grpc
from ...wallet_api.v1 import wallet_pb2 as wallet__api_dot_v1_dot_wallet__pb2

class WalletServiceStub(object):
    """Transacts and holds digital banknotes and bank customer information required for financial compliance, in the form of
    a W3C Verifiable Credential (https://www.w3.org/TR/vc-data-model).  This is a server side wallet solution in lieu of
    a mobile wallet, hosted by the bank on behalf of the customer who opts in to do so. Can also be used by institutions,
    e.g. a financial institution vault.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Transfer = channel.unary_unary('/wallet_api.v1.WalletService/Transfer', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.TransferRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.TransferResponse.FromString)
        self.CreateOwner = channel.unary_unary('/wallet_api.v1.WalletService/CreateOwner', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.CreateOwnerRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.CreateOwnerResponse.FromString)
        self.CreateWallet = channel.unary_unary('/wallet_api.v1.WalletService/CreateWallet', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.CreateWalletRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.CreateWalletResponse.FromString)
        self.ListWallets = channel.unary_unary('/wallet_api.v1.WalletService/ListWallets', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListWalletsRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListWalletsResponse.FromString)
        self.ListAssetFiles = channel.unary_unary('/wallet_api.v1.WalletService/ListAssetFiles', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListAssetFilesRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListAssetFilesResponse.FromString)
        self.ListWalletBalances = channel.unary_unary('/wallet_api.v1.WalletService/ListWalletBalances', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListWalletBalancesRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListWalletBalancesResponse.FromString)
        self.Redeem = channel.unary_unary('/wallet_api.v1.WalletService/Redeem', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.RedeemRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.RedeemResponse.FromString)
        self.ListCredentials = channel.unary_unary('/wallet_api.v1.WalletService/ListCredentials', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListCredentialsRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListCredentialsResponse.FromString)
        self.AddCredential = channel.unary_unary('/wallet_api.v1.WalletService/AddCredential', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.AddCredentialRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.AddCredentialResponse.FromString)
        self.AssociateWallet = channel.unary_unary('/wallet_api.v1.WalletService/AssociateWallet', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.AssociateWalletRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.AssociateWalletResponse.FromString)
        self.RequestPresentation = channel.unary_unary('/wallet_api.v1.WalletService/RequestPresentation', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.RequestPresentationRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.RequestPresentationResponse.FromString)
        self.PrepareSimplePayment = channel.unary_unary('/wallet_api.v1.WalletService/PrepareSimplePayment', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareSimplePaymentRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareSimplePaymentResponse.FromString)
        self.PrepareFundsChange = channel.unary_unary('/wallet_api.v1.WalletService/PrepareFundsChange', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareFundsChangeRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareFundsChangeResponse.FromString)
        self.PrepareTwoStepPayment = channel.unary_unary('/wallet_api.v1.WalletService/PrepareTwoStepPayment', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareTwoStepPaymentRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareTwoStepPaymentResponse.FromString)
        self.GetTransaction = channel.unary_unary('/wallet_api.v1.WalletService/GetTransaction', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.GetTransactionRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.GetTransactionResponse.FromString)
        self.ListTransactions = channel.unary_unary('/wallet_api.v1.WalletService/ListTransactions', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListTransactionsRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListTransactionsResponse.FromString)
        self.GetTransactionSnapshot = channel.unary_unary('/wallet_api.v1.WalletService/GetTransactionSnapshot', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.GetTransactionSnapshotRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.GetTransactionSnapshotResponse.FromString)
        self.ListTransactionSnapshots = channel.unary_unary('/wallet_api.v1.WalletService/ListTransactionSnapshots', request_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListTransactionSnapshotsRequest.SerializeToString, response_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListTransactionSnapshotsResponse.FromString)

class WalletServiceServicer(object):
    """Transacts and holds digital banknotes and bank customer information required for financial compliance, in the form of
    a W3C Verifiable Credential (https://www.w3.org/TR/vc-data-model).  This is a server side wallet solution in lieu of
    a mobile wallet, hosted by the bank on behalf of the customer who opts in to do so. Can also be used by institutions,
    e.g. a financial institution vault.
    """

    def Transfer(self, request, context):
        """Transfers digital banknotes from a custodial wallet to a given recipient public key.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateOwner(self, request, context):
        """Creates an owned and API key for access to future custodial wallet interactions.  Ex Could be a bank that may
        manage many customer custodial wallets.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def CreateWallet(self, request, context):
        """Creates a wallet for a given owner, which can manage the wallet.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListWallets(self, request, context):
        """List all wallets for a given owner.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListAssetFiles(self, request, context):
        """List the digital banknotes for a wallet.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListWalletBalances(self, request, context):
        """List the balances of each currency in the wallet.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def Redeem(self, request, context):
        """Sends digital banknotes from the Issuer wallet to the Authority for redemption of digital banknotes and increase
        issuance limit.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListCredentials(self, request, context):
        """Lists the W3C Verifiable Credentials of the wallet subject.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def AddCredential(self, request, context):
        """Adds the specified W3C Verifiable Credential type for the wallet by calling out to the Credential Adapter. Based on
        the bearer JWT token provided by User Management to ensure the caller has been authenticated and authorized.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def AssociateWallet(self, request, context):
        """Associates the wallet with the account specified in the bearer JWT token provided by User Management.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def RequestPresentation(self, request, context):
        """Returns the JSON Verifiable Presentation containing the PII of the subject.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def PrepareSimplePayment(self, request, context):
        """Attempts to start a `StartSimplePaymentRequest`. The call immediately returns with a `status` and `uetr` that can
        be used to query more information about the transaction.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def PrepareFundsChange(self, request, context):
        """Attempts to start a funds change request. The call immediately returns with a `status` and `uetr` that can be used
        to query more information about the transaction.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def PrepareTwoStepPayment(self, request, context):
        """Request to start a new two-step payment. The call immediately returns with a `status` and `uetr` that can be used
        to query more information about the transaction.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetTransaction(self, request, context):
        """Obtains metadata about a transaction.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListTransactions(self, request, context):
        """List all transactions of a given wallet.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def GetTransactionSnapshot(self, request, context):
        """Obtains current snapshot of a transaction.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def ListTransactionSnapshots(self, request, context):
        """Obtains the history of a transaction.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

def add_WalletServiceServicer_to_server(servicer, server):
    rpc_method_handlers = {'Transfer': grpc.unary_unary_rpc_method_handler(servicer.Transfer, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.TransferRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.TransferResponse.SerializeToString), 'CreateOwner': grpc.unary_unary_rpc_method_handler(servicer.CreateOwner, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.CreateOwnerRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.CreateOwnerResponse.SerializeToString), 'CreateWallet': grpc.unary_unary_rpc_method_handler(servicer.CreateWallet, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.CreateWalletRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.CreateWalletResponse.SerializeToString), 'ListWallets': grpc.unary_unary_rpc_method_handler(servicer.ListWallets, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListWalletsRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListWalletsResponse.SerializeToString), 'ListAssetFiles': grpc.unary_unary_rpc_method_handler(servicer.ListAssetFiles, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListAssetFilesRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListAssetFilesResponse.SerializeToString), 'ListWalletBalances': grpc.unary_unary_rpc_method_handler(servicer.ListWalletBalances, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListWalletBalancesRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListWalletBalancesResponse.SerializeToString), 'Redeem': grpc.unary_unary_rpc_method_handler(servicer.Redeem, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.RedeemRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.RedeemResponse.SerializeToString), 'ListCredentials': grpc.unary_unary_rpc_method_handler(servicer.ListCredentials, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListCredentialsRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListCredentialsResponse.SerializeToString), 'AddCredential': grpc.unary_unary_rpc_method_handler(servicer.AddCredential, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.AddCredentialRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.AddCredentialResponse.SerializeToString), 'AssociateWallet': grpc.unary_unary_rpc_method_handler(servicer.AssociateWallet, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.AssociateWalletRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.AssociateWalletResponse.SerializeToString), 'RequestPresentation': grpc.unary_unary_rpc_method_handler(servicer.RequestPresentation, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.RequestPresentationRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.RequestPresentationResponse.SerializeToString), 'PrepareSimplePayment': grpc.unary_unary_rpc_method_handler(servicer.PrepareSimplePayment, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareSimplePaymentRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareSimplePaymentResponse.SerializeToString), 'PrepareFundsChange': grpc.unary_unary_rpc_method_handler(servicer.PrepareFundsChange, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareFundsChangeRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareFundsChangeResponse.SerializeToString), 'PrepareTwoStepPayment': grpc.unary_unary_rpc_method_handler(servicer.PrepareTwoStepPayment, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareTwoStepPaymentRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.PrepareTwoStepPaymentResponse.SerializeToString), 'GetTransaction': grpc.unary_unary_rpc_method_handler(servicer.GetTransaction, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.GetTransactionRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.GetTransactionResponse.SerializeToString), 'ListTransactions': grpc.unary_unary_rpc_method_handler(servicer.ListTransactions, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListTransactionsRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListTransactionsResponse.SerializeToString), 'GetTransactionSnapshot': grpc.unary_unary_rpc_method_handler(servicer.GetTransactionSnapshot, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.GetTransactionSnapshotRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.GetTransactionSnapshotResponse.SerializeToString), 'ListTransactionSnapshots': grpc.unary_unary_rpc_method_handler(servicer.ListTransactionSnapshots, request_deserializer=wallet__api_dot_v1_dot_wallet__pb2.ListTransactionSnapshotsRequest.FromString, response_serializer=wallet__api_dot_v1_dot_wallet__pb2.ListTransactionSnapshotsResponse.SerializeToString)}
    generic_handler = grpc.method_handlers_generic_handler('wallet_api.v1.WalletService', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))

class WalletService(object):
    """Transacts and holds digital banknotes and bank customer information required for financial compliance, in the form of
    a W3C Verifiable Credential (https://www.w3.org/TR/vc-data-model).  This is a server side wallet solution in lieu of
    a mobile wallet, hosted by the bank on behalf of the customer who opts in to do so. Can also be used by institutions,
    e.g. a financial institution vault.
    """

    @staticmethod
    def Transfer(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/Transfer', wallet__api_dot_v1_dot_wallet__pb2.TransferRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.TransferResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def CreateOwner(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/CreateOwner', wallet__api_dot_v1_dot_wallet__pb2.CreateOwnerRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.CreateOwnerResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def CreateWallet(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/CreateWallet', wallet__api_dot_v1_dot_wallet__pb2.CreateWalletRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.CreateWalletResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListWallets(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/ListWallets', wallet__api_dot_v1_dot_wallet__pb2.ListWalletsRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.ListWalletsResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListAssetFiles(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/ListAssetFiles', wallet__api_dot_v1_dot_wallet__pb2.ListAssetFilesRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.ListAssetFilesResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListWalletBalances(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/ListWalletBalances', wallet__api_dot_v1_dot_wallet__pb2.ListWalletBalancesRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.ListWalletBalancesResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def Redeem(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/Redeem', wallet__api_dot_v1_dot_wallet__pb2.RedeemRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.RedeemResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListCredentials(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/ListCredentials', wallet__api_dot_v1_dot_wallet__pb2.ListCredentialsRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.ListCredentialsResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def AddCredential(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/AddCredential', wallet__api_dot_v1_dot_wallet__pb2.AddCredentialRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.AddCredentialResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def AssociateWallet(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/AssociateWallet', wallet__api_dot_v1_dot_wallet__pb2.AssociateWalletRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.AssociateWalletResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def RequestPresentation(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/RequestPresentation', wallet__api_dot_v1_dot_wallet__pb2.RequestPresentationRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.RequestPresentationResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def PrepareSimplePayment(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/PrepareSimplePayment', wallet__api_dot_v1_dot_wallet__pb2.PrepareSimplePaymentRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.PrepareSimplePaymentResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def PrepareFundsChange(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/PrepareFundsChange', wallet__api_dot_v1_dot_wallet__pb2.PrepareFundsChangeRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.PrepareFundsChangeResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def PrepareTwoStepPayment(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/PrepareTwoStepPayment', wallet__api_dot_v1_dot_wallet__pb2.PrepareTwoStepPaymentRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.PrepareTwoStepPaymentResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetTransaction(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/GetTransaction', wallet__api_dot_v1_dot_wallet__pb2.GetTransactionRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.GetTransactionResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListTransactions(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/ListTransactions', wallet__api_dot_v1_dot_wallet__pb2.ListTransactionsRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.ListTransactionsResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def GetTransactionSnapshot(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/GetTransactionSnapshot', wallet__api_dot_v1_dot_wallet__pb2.GetTransactionSnapshotRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.GetTransactionSnapshotResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def ListTransactionSnapshots(request, target, options=(), channel_credentials=None, call_credentials=None, insecure=False, compression=None, wait_for_ready=None, timeout=None, metadata=None):
        return grpc.experimental.unary_unary(request, target, '/wallet_api.v1.WalletService/ListTransactionSnapshots', wallet__api_dot_v1_dot_wallet__pb2.ListTransactionSnapshotsRequest.SerializeToString, wallet__api_dot_v1_dot_wallet__pb2.ListTransactionSnapshotsResponse.FromString, options, channel_credentials, insecure, call_credentials, compression, wait_for_ready, timeout, metadata)