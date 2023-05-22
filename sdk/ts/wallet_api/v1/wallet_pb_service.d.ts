/* eslint-disable */
/*Generated by GenDocu.com*/
// package: wallet_api.v1
// file: wallet_api/v1/wallet.proto

import * as wallet_api_v1_wallet_pb from "../../wallet_api/v1/wallet_pb";
import {grpc} from "@improbable-eng/grpc-web";

type WalletServiceTransfer = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.TransferRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.TransferResponse;
};

type WalletServiceCreateOwner = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.CreateOwnerRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.CreateOwnerResponse;
};

type WalletServiceCreateWallet = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.CreateWalletRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.CreateWalletResponse;
};

type WalletServiceListWallets = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.ListWalletsRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.ListWalletsResponse;
};

type WalletServiceListFunds = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.ListFundsRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.ListFundsResponse;
};

type WalletServiceListWalletBalances = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.ListWalletBalancesRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.ListWalletBalancesResponse;
};

type WalletServiceRedeem = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.RedeemRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.RedeemResponse;
};

type WalletServiceListCredentials = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.ListCredentialsRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.ListCredentialsResponse;
};

type WalletServiceAddCredential = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.AddCredentialRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.AddCredentialResponse;
};

type WalletServiceAssociateWallet = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.AssociateWalletRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.AssociateWalletResponse;
};

type WalletServiceRequestPresentation = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.RequestPresentationRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.RequestPresentationResponse;
};

type WalletServicePrepareSimplePayment = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.PrepareSimplePaymentRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.PrepareSimplePaymentResponse;
};

type WalletServicePrepareFundsChange = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.PrepareFundsChangeRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.PrepareFundsChangeResponse;
};

type WalletServiceGetTransaction = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.GetTransactionRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.GetTransactionResponse;
};

type WalletServiceListTransactions = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.ListTransactionsRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.ListTransactionsResponse;
};

type WalletServiceGetTransactionSnapshot = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.GetTransactionSnapshotRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.GetTransactionSnapshotResponse;
};

type WalletServiceListTransactionSnapshots = {
  readonly methodName: string;
  readonly service: typeof WalletService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof wallet_api_v1_wallet_pb.ListTransactionSnapshotsRequest;
  readonly responseType: typeof wallet_api_v1_wallet_pb.ListTransactionSnapshotsResponse;
};

export class WalletService {
  static readonly serviceName: string;
  static readonly Transfer: WalletServiceTransfer;
  static readonly CreateOwner: WalletServiceCreateOwner;
  static readonly CreateWallet: WalletServiceCreateWallet;
  static readonly ListWallets: WalletServiceListWallets;
  static readonly ListFunds: WalletServiceListFunds;
  static readonly ListWalletBalances: WalletServiceListWalletBalances;
  static readonly Redeem: WalletServiceRedeem;
  static readonly ListCredentials: WalletServiceListCredentials;
  static readonly AddCredential: WalletServiceAddCredential;
  static readonly AssociateWallet: WalletServiceAssociateWallet;
  static readonly RequestPresentation: WalletServiceRequestPresentation;
  static readonly PrepareSimplePayment: WalletServicePrepareSimplePayment;
  static readonly PrepareFundsChange: WalletServicePrepareFundsChange;
  static readonly GetTransaction: WalletServiceGetTransaction;
  static readonly ListTransactions: WalletServiceListTransactions;
  static readonly GetTransactionSnapshot: WalletServiceGetTransactionSnapshot;
  static readonly ListTransactionSnapshots: WalletServiceListTransactionSnapshots;
}

export type ServiceError = { message: string, code: number; metadata: grpc.Metadata }
export type Status = { details: string, code: number; metadata: grpc.Metadata }

interface UnaryResponse {
  cancel(): void;
}
interface ResponseStream<T> {
  cancel(): void;
  on(type: 'data', handler: (message: T) => void): ResponseStream<T>;
  on(type: 'end', handler: (status?: Status) => void): ResponseStream<T>;
  on(type: 'status', handler: (status: Status) => void): ResponseStream<T>;
}
interface RequestStream<T> {
  write(message: T): RequestStream<T>;
  end(): void;
  cancel(): void;
  on(type: 'end', handler: (status?: Status) => void): RequestStream<T>;
  on(type: 'status', handler: (status: Status) => void): RequestStream<T>;
}
interface BidirectionalStream<ReqT, ResT> {
  write(message: ReqT): BidirectionalStream<ReqT, ResT>;
  end(): void;
  cancel(): void;
  on(type: 'data', handler: (message: ResT) => void): BidirectionalStream<ReqT, ResT>;
  on(type: 'end', handler: (status?: Status) => void): BidirectionalStream<ReqT, ResT>;
  on(type: 'status', handler: (status: Status) => void): BidirectionalStream<ReqT, ResT>;
}

export class WalletServiceClient {
  readonly serviceHost: string;

  constructor(serviceHost: string, options?: grpc.RpcOptions);
  transfer(
    requestMessage: wallet_api_v1_wallet_pb.TransferRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.TransferResponse|null) => void
  ): UnaryResponse;
  transfer(
    requestMessage: wallet_api_v1_wallet_pb.TransferRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.TransferResponse|null) => void
  ): UnaryResponse;
  createOwner(
    requestMessage: wallet_api_v1_wallet_pb.CreateOwnerRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.CreateOwnerResponse|null) => void
  ): UnaryResponse;
  createOwner(
    requestMessage: wallet_api_v1_wallet_pb.CreateOwnerRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.CreateOwnerResponse|null) => void
  ): UnaryResponse;
  createWallet(
    requestMessage: wallet_api_v1_wallet_pb.CreateWalletRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.CreateWalletResponse|null) => void
  ): UnaryResponse;
  createWallet(
    requestMessage: wallet_api_v1_wallet_pb.CreateWalletRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.CreateWalletResponse|null) => void
  ): UnaryResponse;
  listWallets(
    requestMessage: wallet_api_v1_wallet_pb.ListWalletsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListWalletsResponse|null) => void
  ): UnaryResponse;
  listWallets(
    requestMessage: wallet_api_v1_wallet_pb.ListWalletsRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListWalletsResponse|null) => void
  ): UnaryResponse;
  listFunds(
    requestMessage: wallet_api_v1_wallet_pb.ListFundsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListFundsResponse|null) => void
  ): UnaryResponse;
  listFunds(
    requestMessage: wallet_api_v1_wallet_pb.ListFundsRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListFundsResponse|null) => void
  ): UnaryResponse;
  listWalletBalances(
    requestMessage: wallet_api_v1_wallet_pb.ListWalletBalancesRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListWalletBalancesResponse|null) => void
  ): UnaryResponse;
  listWalletBalances(
    requestMessage: wallet_api_v1_wallet_pb.ListWalletBalancesRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListWalletBalancesResponse|null) => void
  ): UnaryResponse;
  redeem(
    requestMessage: wallet_api_v1_wallet_pb.RedeemRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.RedeemResponse|null) => void
  ): UnaryResponse;
  redeem(
    requestMessage: wallet_api_v1_wallet_pb.RedeemRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.RedeemResponse|null) => void
  ): UnaryResponse;
  listCredentials(
    requestMessage: wallet_api_v1_wallet_pb.ListCredentialsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListCredentialsResponse|null) => void
  ): UnaryResponse;
  listCredentials(
    requestMessage: wallet_api_v1_wallet_pb.ListCredentialsRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListCredentialsResponse|null) => void
  ): UnaryResponse;
  addCredential(
    requestMessage: wallet_api_v1_wallet_pb.AddCredentialRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.AddCredentialResponse|null) => void
  ): UnaryResponse;
  addCredential(
    requestMessage: wallet_api_v1_wallet_pb.AddCredentialRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.AddCredentialResponse|null) => void
  ): UnaryResponse;
  associateWallet(
    requestMessage: wallet_api_v1_wallet_pb.AssociateWalletRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.AssociateWalletResponse|null) => void
  ): UnaryResponse;
  associateWallet(
    requestMessage: wallet_api_v1_wallet_pb.AssociateWalletRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.AssociateWalletResponse|null) => void
  ): UnaryResponse;
  requestPresentation(
    requestMessage: wallet_api_v1_wallet_pb.RequestPresentationRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.RequestPresentationResponse|null) => void
  ): UnaryResponse;
  requestPresentation(
    requestMessage: wallet_api_v1_wallet_pb.RequestPresentationRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.RequestPresentationResponse|null) => void
  ): UnaryResponse;
  prepareSimplePayment(
    requestMessage: wallet_api_v1_wallet_pb.PrepareSimplePaymentRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.PrepareSimplePaymentResponse|null) => void
  ): UnaryResponse;
  prepareSimplePayment(
    requestMessage: wallet_api_v1_wallet_pb.PrepareSimplePaymentRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.PrepareSimplePaymentResponse|null) => void
  ): UnaryResponse;
  prepareFundsChange(
    requestMessage: wallet_api_v1_wallet_pb.PrepareFundsChangeRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.PrepareFundsChangeResponse|null) => void
  ): UnaryResponse;
  prepareFundsChange(
    requestMessage: wallet_api_v1_wallet_pb.PrepareFundsChangeRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.PrepareFundsChangeResponse|null) => void
  ): UnaryResponse;
  getTransaction(
    requestMessage: wallet_api_v1_wallet_pb.GetTransactionRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.GetTransactionResponse|null) => void
  ): UnaryResponse;
  getTransaction(
    requestMessage: wallet_api_v1_wallet_pb.GetTransactionRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.GetTransactionResponse|null) => void
  ): UnaryResponse;
  listTransactions(
    requestMessage: wallet_api_v1_wallet_pb.ListTransactionsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListTransactionsResponse|null) => void
  ): UnaryResponse;
  listTransactions(
    requestMessage: wallet_api_v1_wallet_pb.ListTransactionsRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListTransactionsResponse|null) => void
  ): UnaryResponse;
  getTransactionSnapshot(
    requestMessage: wallet_api_v1_wallet_pb.GetTransactionSnapshotRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.GetTransactionSnapshotResponse|null) => void
  ): UnaryResponse;
  getTransactionSnapshot(
    requestMessage: wallet_api_v1_wallet_pb.GetTransactionSnapshotRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.GetTransactionSnapshotResponse|null) => void
  ): UnaryResponse;
  listTransactionSnapshots(
    requestMessage: wallet_api_v1_wallet_pb.ListTransactionSnapshotsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListTransactionSnapshotsResponse|null) => void
  ): UnaryResponse;
  listTransactionSnapshots(
    requestMessage: wallet_api_v1_wallet_pb.ListTransactionSnapshotsRequest,
    callback: (error: ServiceError|null, responseMessage: wallet_api_v1_wallet_pb.ListTransactionSnapshotsResponse|null) => void
  ): UnaryResponse;
}

