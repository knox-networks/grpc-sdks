/* eslint-disable */
/*Generated by GenDocu.com*/
// package: authority_api.v1
// file: authority_api/v1/authority.proto

import * as authority_api_v1_authority_pb from "../../authority_api/v1/authority_pb";
import {grpc} from "@improbable-eng/grpc-web";

type AuthorityServiceSetIssuerLimit = {
  readonly methodName: string;
  readonly service: typeof AuthorityService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof authority_api_v1_authority_pb.SetIssuerLimitRequest;
  readonly responseType: typeof authority_api_v1_authority_pb.SetIssuerLimitResponse;
};

type AuthorityServiceGetIssuerLimits = {
  readonly methodName: string;
  readonly service: typeof AuthorityService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof authority_api_v1_authority_pb.GetIssuerLimitsRequest;
  readonly responseType: typeof authority_api_v1_authority_pb.GetIssuerLimitsResponse;
};

type AuthorityServiceAuthorize = {
  readonly methodName: string;
  readonly service: typeof AuthorityService;
  readonly requestStream: true;
  readonly responseStream: true;
  readonly requestType: typeof authority_api_v1_authority_pb.AuthorizeRequest;
  readonly responseType: typeof authority_api_v1_authority_pb.AuthorizeResponse;
};

type AuthorityServiceRedeem = {
  readonly methodName: string;
  readonly service: typeof AuthorityService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof authority_api_v1_authority_pb.RedeemRequest;
  readonly responseType: typeof authority_api_v1_authority_pb.RedeemResponse;
};

type AuthorityServiceGetNetworkInfo = {
  readonly methodName: string;
  readonly service: typeof AuthorityService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof authority_api_v1_authority_pb.GetNetworkInfoRequest;
  readonly responseType: typeof authority_api_v1_authority_pb.GetNetworkInfoResponse;
};

type AuthorityServiceGetNotary = {
  readonly methodName: string;
  readonly service: typeof AuthorityService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof authority_api_v1_authority_pb.GetNotaryRequest;
  readonly responseType: typeof authority_api_v1_authority_pb.GetNotaryResponse;
};

type AuthorityServiceListGovernedAssets = {
  readonly methodName: string;
  readonly service: typeof AuthorityService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof authority_api_v1_authority_pb.ListGovernedAssetsRequest;
  readonly responseType: typeof authority_api_v1_authority_pb.ListGovernedAssetsResponse;
};

type AuthorityServiceSetGovernedAssets = {
  readonly methodName: string;
  readonly service: typeof AuthorityService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof authority_api_v1_authority_pb.SetGovernedAssetsRequest;
  readonly responseType: typeof authority_api_v1_authority_pb.SetGovernedAssetsResponse;
};

export class AuthorityService {
  static readonly serviceName: string;
  static readonly SetIssuerLimit: AuthorityServiceSetIssuerLimit;
  static readonly GetIssuerLimits: AuthorityServiceGetIssuerLimits;
  static readonly Authorize: AuthorityServiceAuthorize;
  static readonly Redeem: AuthorityServiceRedeem;
  static readonly GetNetworkInfo: AuthorityServiceGetNetworkInfo;
  static readonly GetNotary: AuthorityServiceGetNotary;
  static readonly ListGovernedAssets: AuthorityServiceListGovernedAssets;
  static readonly SetGovernedAssets: AuthorityServiceSetGovernedAssets;
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

export class AuthorityServiceClient {
  readonly serviceHost: string;

  constructor(serviceHost: string, options?: grpc.RpcOptions);
  setIssuerLimit(
    requestMessage: authority_api_v1_authority_pb.SetIssuerLimitRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.SetIssuerLimitResponse|null) => void
  ): UnaryResponse;
  setIssuerLimit(
    requestMessage: authority_api_v1_authority_pb.SetIssuerLimitRequest,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.SetIssuerLimitResponse|null) => void
  ): UnaryResponse;
  getIssuerLimits(
    requestMessage: authority_api_v1_authority_pb.GetIssuerLimitsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.GetIssuerLimitsResponse|null) => void
  ): UnaryResponse;
  getIssuerLimits(
    requestMessage: authority_api_v1_authority_pb.GetIssuerLimitsRequest,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.GetIssuerLimitsResponse|null) => void
  ): UnaryResponse;
  authorize(metadata?: grpc.Metadata): BidirectionalStream<authority_api_v1_authority_pb.AuthorizeRequest, authority_api_v1_authority_pb.AuthorizeResponse>;
  redeem(
    requestMessage: authority_api_v1_authority_pb.RedeemRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.RedeemResponse|null) => void
  ): UnaryResponse;
  redeem(
    requestMessage: authority_api_v1_authority_pb.RedeemRequest,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.RedeemResponse|null) => void
  ): UnaryResponse;
  getNetworkInfo(
    requestMessage: authority_api_v1_authority_pb.GetNetworkInfoRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.GetNetworkInfoResponse|null) => void
  ): UnaryResponse;
  getNetworkInfo(
    requestMessage: authority_api_v1_authority_pb.GetNetworkInfoRequest,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.GetNetworkInfoResponse|null) => void
  ): UnaryResponse;
  getNotary(
    requestMessage: authority_api_v1_authority_pb.GetNotaryRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.GetNotaryResponse|null) => void
  ): UnaryResponse;
  getNotary(
    requestMessage: authority_api_v1_authority_pb.GetNotaryRequest,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.GetNotaryResponse|null) => void
  ): UnaryResponse;
  listGovernedAssets(
    requestMessage: authority_api_v1_authority_pb.ListGovernedAssetsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.ListGovernedAssetsResponse|null) => void
  ): UnaryResponse;
  listGovernedAssets(
    requestMessage: authority_api_v1_authority_pb.ListGovernedAssetsRequest,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.ListGovernedAssetsResponse|null) => void
  ): UnaryResponse;
  setGovernedAssets(
    requestMessage: authority_api_v1_authority_pb.SetGovernedAssetsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.SetGovernedAssetsResponse|null) => void
  ): UnaryResponse;
  setGovernedAssets(
    requestMessage: authority_api_v1_authority_pb.SetGovernedAssetsRequest,
    callback: (error: ServiceError|null, responseMessage: authority_api_v1_authority_pb.SetGovernedAssetsResponse|null) => void
  ): UnaryResponse;
}

