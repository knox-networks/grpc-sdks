/* eslint-disable */
/*Generated by GenDocu.com*/
// package: wallet_api.v1
// file: wallet_api/v1/wallet.proto

var wallet_api_v1_wallet_pb = require("../../wallet_api/v1/wallet_pb");
var grpc = require("@improbable-eng/grpc-web").grpc;

var WalletService = (function () {
  function WalletService() {}
  WalletService.serviceName = "wallet_api.v1.WalletService";
  return WalletService;
}());

WalletService.Transfer = {
  methodName: "Transfer",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.TransferRequest,
  responseType: wallet_api_v1_wallet_pb.TransferResponse
};

WalletService.CreateOwner = {
  methodName: "CreateOwner",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.CreateOwnerRequest,
  responseType: wallet_api_v1_wallet_pb.CreateOwnerResponse
};

WalletService.CreateWallet = {
  methodName: "CreateWallet",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.CreateWalletRequest,
  responseType: wallet_api_v1_wallet_pb.CreateWalletResponse
};

WalletService.ListWallets = {
  methodName: "ListWallets",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.ListWalletsRequest,
  responseType: wallet_api_v1_wallet_pb.ListWalletsResponse
};

WalletService.ListAssetFiles = {
  methodName: "ListAssetFiles",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.ListAssetFilesRequest,
  responseType: wallet_api_v1_wallet_pb.ListAssetFilesResponse
};

WalletService.ListWalletBalances = {
  methodName: "ListWalletBalances",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.ListWalletBalancesRequest,
  responseType: wallet_api_v1_wallet_pb.ListWalletBalancesResponse
};

WalletService.Redeem = {
  methodName: "Redeem",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.RedeemRequest,
  responseType: wallet_api_v1_wallet_pb.RedeemResponse
};

WalletService.ListCredentials = {
  methodName: "ListCredentials",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.ListCredentialsRequest,
  responseType: wallet_api_v1_wallet_pb.ListCredentialsResponse
};

WalletService.AddCredential = {
  methodName: "AddCredential",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.AddCredentialRequest,
  responseType: wallet_api_v1_wallet_pb.AddCredentialResponse
};

WalletService.AssociateWallet = {
  methodName: "AssociateWallet",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.AssociateWalletRequest,
  responseType: wallet_api_v1_wallet_pb.AssociateWalletResponse
};

WalletService.RequestPresentation = {
  methodName: "RequestPresentation",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.RequestPresentationRequest,
  responseType: wallet_api_v1_wallet_pb.RequestPresentationResponse
};

WalletService.PrepareSimplePayment = {
  methodName: "PrepareSimplePayment",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.PrepareSimplePaymentRequest,
  responseType: wallet_api_v1_wallet_pb.PrepareSimplePaymentResponse
};

WalletService.PrepareFundsChange = {
  methodName: "PrepareFundsChange",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.PrepareFundsChangeRequest,
  responseType: wallet_api_v1_wallet_pb.PrepareFundsChangeResponse
};

WalletService.GetTransaction = {
  methodName: "GetTransaction",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.GetTransactionRequest,
  responseType: wallet_api_v1_wallet_pb.GetTransactionResponse
};

WalletService.ListTransactions = {
  methodName: "ListTransactions",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.ListTransactionsRequest,
  responseType: wallet_api_v1_wallet_pb.ListTransactionsResponse
};

WalletService.GetTransactionSnapshot = {
  methodName: "GetTransactionSnapshot",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.GetTransactionSnapshotRequest,
  responseType: wallet_api_v1_wallet_pb.GetTransactionSnapshotResponse
};

WalletService.ListTransactionSnapshots = {
  methodName: "ListTransactionSnapshots",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.ListTransactionSnapshotsRequest,
  responseType: wallet_api_v1_wallet_pb.ListTransactionSnapshotsResponse
};

WalletService.ListWalletBalanceDetails = {
  methodName: "ListWalletBalanceDetails",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.ListWalletBalanceDetailsRequest,
  responseType: wallet_api_v1_wallet_pb.ListWalletBalanceDetailsResponse
};

WalletService.ListContractTransactions = {
  methodName: "ListContractTransactions",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.ListContractTransactionsRequest,
  responseType: wallet_api_v1_wallet_pb.ListContractTransactionsResponse
};

WalletService.GetContractTransactionDetail = {
  methodName: "GetContractTransactionDetail",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.GetContractTransactionDetailRequest,
  responseType: wallet_api_v1_wallet_pb.GetContractTransactionDetailResponse
};

WalletService.PrepareContractTransaction = {
  methodName: "PrepareContractTransaction",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.PrepareContractTransactionRequest,
  responseType: wallet_api_v1_wallet_pb.PrepareContractTransactionResponse
};

WalletService.PrepareAtomicAssetSwap = {
  methodName: "PrepareAtomicAssetSwap",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.PrepareAtomicAssetSwapRequest,
  responseType: wallet_api_v1_wallet_pb.PrepareAtomicAssetSwapResponse
};

WalletService.PrepareCrossBorderPayment = {
  methodName: "PrepareCrossBorderPayment",
  service: WalletService,
  requestStream: false,
  responseStream: false,
  requestType: wallet_api_v1_wallet_pb.PrepareCrossBorderPaymentRequest,
  responseType: wallet_api_v1_wallet_pb.PrepareCrossBorderPaymentResponse
};

exports.WalletService = WalletService;

function WalletServiceClient(serviceHost, options) {
  this.serviceHost = serviceHost;
  this.options = options || {};
}

WalletServiceClient.prototype.transfer = function transfer(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.Transfer, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.createOwner = function createOwner(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.CreateOwner, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.createWallet = function createWallet(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.CreateWallet, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.listWallets = function listWallets(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.ListWallets, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.listAssetFiles = function listAssetFiles(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.ListAssetFiles, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.listWalletBalances = function listWalletBalances(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.ListWalletBalances, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.redeem = function redeem(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.Redeem, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.listCredentials = function listCredentials(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.ListCredentials, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.addCredential = function addCredential(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.AddCredential, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.associateWallet = function associateWallet(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.AssociateWallet, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.requestPresentation = function requestPresentation(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.RequestPresentation, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.prepareSimplePayment = function prepareSimplePayment(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.PrepareSimplePayment, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.prepareFundsChange = function prepareFundsChange(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.PrepareFundsChange, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.getTransaction = function getTransaction(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.GetTransaction, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.listTransactions = function listTransactions(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.ListTransactions, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.getTransactionSnapshot = function getTransactionSnapshot(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.GetTransactionSnapshot, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.listTransactionSnapshots = function listTransactionSnapshots(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.ListTransactionSnapshots, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.listWalletBalanceDetails = function listWalletBalanceDetails(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.ListWalletBalanceDetails, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.listContractTransactions = function listContractTransactions(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.ListContractTransactions, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.getContractTransactionDetail = function getContractTransactionDetail(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.GetContractTransactionDetail, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.prepareContractTransaction = function prepareContractTransaction(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.PrepareContractTransaction, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.prepareAtomicAssetSwap = function prepareAtomicAssetSwap(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.PrepareAtomicAssetSwap, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

WalletServiceClient.prototype.prepareCrossBorderPayment = function prepareCrossBorderPayment(requestMessage, metadata, callback) {
  if (arguments.length === 2) {
    callback = arguments[1];
  }
  var client = grpc.unary(WalletService.PrepareCrossBorderPayment, {
    request: requestMessage,
    host: this.serviceHost,
    metadata: metadata,
    transport: this.options.transport,
    debug: this.options.debug,
    onEnd: function (response) {
      if (callback) {
        if (response.status !== grpc.Code.OK) {
          var err = new Error(response.statusMessage);
          err.code = response.status;
          err.metadata = response.trailers;
          callback(err, null);
        } else {
          callback(null, response.message);
        }
      }
    }
  });
  return {
    cancel: function () {
      callback = null;
      client.close();
    }
  };
};

exports.WalletServiceClient = WalletServiceClient;

