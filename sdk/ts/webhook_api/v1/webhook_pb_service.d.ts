/* eslint-disable */
/*Generated by GenDocu.com*/
// package: webhook_api.v1
// file: webhook_api/v1/webhook.proto

import * as webhook_api_v1_webhook_pb from "../../webhook_api/v1/webhook_pb";
import {grpc} from "@improbable-eng/grpc-web";

type WebhookManagerServiceCreateWebhook = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.CreateWebhookRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.CreateWebhookResponse;
};

type WebhookManagerServiceListWebhooks = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.ListWebhooksRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.ListWebhooksResponse;
};

type WebhookManagerServiceDeleteWebhook = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.DeleteWebhookRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.DeleteWebhookResponse;
};

type WebhookManagerServiceListEvents = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.ListEventsRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.ListEventsResponse;
};

type WebhookManagerServiceRetryEvent = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.RetryEventRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.RetryEventResponse;
};

type WebhookManagerServicePing = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.PingRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.PingResponse;
};

type WebhookManagerServiceListDeliveryHistory = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.ListDeliveryHistoryRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.ListDeliveryHistoryResponse;
};

type WebhookManagerServiceCreateScheduledReaction = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.CreateScheduledReactionRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.CreateScheduledReactionResponse;
};

type WebhookManagerServiceListScheduledReactions = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.ListScheduledReactionsRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.ListScheduledReactionsResponse;
};

type WebhookManagerServiceDeleteScheduledReaction = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.DeleteScheduledReactionRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.DeleteScheduledReactionResponse;
};

type WebhookManagerServiceCreatePrevalidation = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.CreatePrevalidationRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.CreatePrevalidationResponse;
};

type WebhookManagerServiceListPrevalidations = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.ListPrevalidationsRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.ListPrevalidationsResponse;
};

type WebhookManagerServiceDeletePrevalidation = {
  readonly methodName: string;
  readonly service: typeof WebhookManagerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.DeletePrevalidationRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.DeletePrevalidationResponse;
};

export class WebhookManagerService {
  static readonly serviceName: string;
  static readonly CreateWebhook: WebhookManagerServiceCreateWebhook;
  static readonly ListWebhooks: WebhookManagerServiceListWebhooks;
  static readonly DeleteWebhook: WebhookManagerServiceDeleteWebhook;
  static readonly ListEvents: WebhookManagerServiceListEvents;
  static readonly RetryEvent: WebhookManagerServiceRetryEvent;
  static readonly Ping: WebhookManagerServicePing;
  static readonly ListDeliveryHistory: WebhookManagerServiceListDeliveryHistory;
  static readonly CreateScheduledReaction: WebhookManagerServiceCreateScheduledReaction;
  static readonly ListScheduledReactions: WebhookManagerServiceListScheduledReactions;
  static readonly DeleteScheduledReaction: WebhookManagerServiceDeleteScheduledReaction;
  static readonly CreatePrevalidation: WebhookManagerServiceCreatePrevalidation;
  static readonly ListPrevalidations: WebhookManagerServiceListPrevalidations;
  static readonly DeletePrevalidation: WebhookManagerServiceDeletePrevalidation;
}

type EventConsumerServiceEvent = {
  readonly methodName: string;
  readonly service: typeof EventConsumerService;
  readonly requestStream: false;
  readonly responseStream: false;
  readonly requestType: typeof webhook_api_v1_webhook_pb.EventRequest;
  readonly responseType: typeof webhook_api_v1_webhook_pb.EventResponse;
};

export class EventConsumerService {
  static readonly serviceName: string;
  static readonly Event: EventConsumerServiceEvent;
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

export class WebhookManagerServiceClient {
  readonly serviceHost: string;

  constructor(serviceHost: string, options?: grpc.RpcOptions);
  createWebhook(
    requestMessage: webhook_api_v1_webhook_pb.CreateWebhookRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.CreateWebhookResponse|null) => void
  ): UnaryResponse;
  createWebhook(
    requestMessage: webhook_api_v1_webhook_pb.CreateWebhookRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.CreateWebhookResponse|null) => void
  ): UnaryResponse;
  listWebhooks(
    requestMessage: webhook_api_v1_webhook_pb.ListWebhooksRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.ListWebhooksResponse|null) => void
  ): UnaryResponse;
  listWebhooks(
    requestMessage: webhook_api_v1_webhook_pb.ListWebhooksRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.ListWebhooksResponse|null) => void
  ): UnaryResponse;
  deleteWebhook(
    requestMessage: webhook_api_v1_webhook_pb.DeleteWebhookRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.DeleteWebhookResponse|null) => void
  ): UnaryResponse;
  deleteWebhook(
    requestMessage: webhook_api_v1_webhook_pb.DeleteWebhookRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.DeleteWebhookResponse|null) => void
  ): UnaryResponse;
  listEvents(
    requestMessage: webhook_api_v1_webhook_pb.ListEventsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.ListEventsResponse|null) => void
  ): UnaryResponse;
  listEvents(
    requestMessage: webhook_api_v1_webhook_pb.ListEventsRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.ListEventsResponse|null) => void
  ): UnaryResponse;
  retryEvent(
    requestMessage: webhook_api_v1_webhook_pb.RetryEventRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.RetryEventResponse|null) => void
  ): UnaryResponse;
  retryEvent(
    requestMessage: webhook_api_v1_webhook_pb.RetryEventRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.RetryEventResponse|null) => void
  ): UnaryResponse;
  ping(
    requestMessage: webhook_api_v1_webhook_pb.PingRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.PingResponse|null) => void
  ): UnaryResponse;
  ping(
    requestMessage: webhook_api_v1_webhook_pb.PingRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.PingResponse|null) => void
  ): UnaryResponse;
  listDeliveryHistory(
    requestMessage: webhook_api_v1_webhook_pb.ListDeliveryHistoryRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.ListDeliveryHistoryResponse|null) => void
  ): UnaryResponse;
  listDeliveryHistory(
    requestMessage: webhook_api_v1_webhook_pb.ListDeliveryHistoryRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.ListDeliveryHistoryResponse|null) => void
  ): UnaryResponse;
  createScheduledReaction(
    requestMessage: webhook_api_v1_webhook_pb.CreateScheduledReactionRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.CreateScheduledReactionResponse|null) => void
  ): UnaryResponse;
  createScheduledReaction(
    requestMessage: webhook_api_v1_webhook_pb.CreateScheduledReactionRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.CreateScheduledReactionResponse|null) => void
  ): UnaryResponse;
  listScheduledReactions(
    requestMessage: webhook_api_v1_webhook_pb.ListScheduledReactionsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.ListScheduledReactionsResponse|null) => void
  ): UnaryResponse;
  listScheduledReactions(
    requestMessage: webhook_api_v1_webhook_pb.ListScheduledReactionsRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.ListScheduledReactionsResponse|null) => void
  ): UnaryResponse;
  deleteScheduledReaction(
    requestMessage: webhook_api_v1_webhook_pb.DeleteScheduledReactionRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.DeleteScheduledReactionResponse|null) => void
  ): UnaryResponse;
  deleteScheduledReaction(
    requestMessage: webhook_api_v1_webhook_pb.DeleteScheduledReactionRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.DeleteScheduledReactionResponse|null) => void
  ): UnaryResponse;
  createPrevalidation(
    requestMessage: webhook_api_v1_webhook_pb.CreatePrevalidationRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.CreatePrevalidationResponse|null) => void
  ): UnaryResponse;
  createPrevalidation(
    requestMessage: webhook_api_v1_webhook_pb.CreatePrevalidationRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.CreatePrevalidationResponse|null) => void
  ): UnaryResponse;
  listPrevalidations(
    requestMessage: webhook_api_v1_webhook_pb.ListPrevalidationsRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.ListPrevalidationsResponse|null) => void
  ): UnaryResponse;
  listPrevalidations(
    requestMessage: webhook_api_v1_webhook_pb.ListPrevalidationsRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.ListPrevalidationsResponse|null) => void
  ): UnaryResponse;
  deletePrevalidation(
    requestMessage: webhook_api_v1_webhook_pb.DeletePrevalidationRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.DeletePrevalidationResponse|null) => void
  ): UnaryResponse;
  deletePrevalidation(
    requestMessage: webhook_api_v1_webhook_pb.DeletePrevalidationRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.DeletePrevalidationResponse|null) => void
  ): UnaryResponse;
}

export class EventConsumerServiceClient {
  readonly serviceHost: string;

  constructor(serviceHost: string, options?: grpc.RpcOptions);
  event(
    requestMessage: webhook_api_v1_webhook_pb.EventRequest,
    metadata: grpc.Metadata,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.EventResponse|null) => void
  ): UnaryResponse;
  event(
    requestMessage: webhook_api_v1_webhook_pb.EventRequest,
    callback: (error: ServiceError|null, responseMessage: webhook_api_v1_webhook_pb.EventResponse|null) => void
  ): UnaryResponse;
}

