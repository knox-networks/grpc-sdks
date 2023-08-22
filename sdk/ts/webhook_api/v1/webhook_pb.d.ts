/* eslint-disable */
/*Generated by GenDocu.com*/
// package: webhook_api.v1
// file: webhook_api/v1/webhook.proto

import * as jspb from "google-protobuf";
import * as common_v1_common_pb from "../../common/v1/common_pb";

export class Webhook extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getOwnerId(): string;
  setOwnerId(value: string): void;

  getName(): string;
  setName(value: string): void;

  getWebhookUrl(): string;
  setWebhookUrl(value: string): void;

  clearSupportedEventsList(): void;
  getSupportedEventsList(): Array<EventTypeMap[keyof EventTypeMap]>;
  setSupportedEventsList(value: Array<EventTypeMap[keyof EventTypeMap]>): void;
  addSupportedEvents(value: EventTypeMap[keyof EventTypeMap], index?: number): EventTypeMap[keyof EventTypeMap];

  getDeliveryMethod(): DeliveryMethodTypeMap[keyof DeliveryMethodTypeMap];
  setDeliveryMethod(value: DeliveryMethodTypeMap[keyof DeliveryMethodTypeMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Webhook.AsObject;
  static toObject(includeInstance: boolean, msg: Webhook): Webhook.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Webhook, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Webhook;
  static deserializeBinaryFromReader(message: Webhook, reader: jspb.BinaryReader): Webhook;
}

export namespace Webhook {
  export type AsObject = {
    id: string,
    ownerId: string,
    name: string,
    webhookUrl: string,
    supportedEventsList: Array<EventTypeMap[keyof EventTypeMap]>,
    deliveryMethod: DeliveryMethodTypeMap[keyof DeliveryMethodTypeMap],
  }
}

export class CreateWalletPayload extends jspb.Message {
  getName(): string;
  setName(value: string): void;

  getDid(): string;
  setDid(value: string): void;

  getCreated(): string;
  setCreated(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateWalletPayload.AsObject;
  static toObject(includeInstance: boolean, msg: CreateWalletPayload): CreateWalletPayload.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateWalletPayload, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateWalletPayload;
  static deserializeBinaryFromReader(message: CreateWalletPayload, reader: jspb.BinaryReader): CreateWalletPayload;
}

export namespace CreateWalletPayload {
  export type AsObject = {
    name: string,
    did: string,
    created: string,
  }
}

export class PromissoryReceivedPayload extends jspb.Message {
  getWalletId(): string;
  setWalletId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PromissoryReceivedPayload.AsObject;
  static toObject(includeInstance: boolean, msg: PromissoryReceivedPayload): PromissoryReceivedPayload.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PromissoryReceivedPayload, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PromissoryReceivedPayload;
  static deserializeBinaryFromReader(message: PromissoryReceivedPayload, reader: jspb.BinaryReader): PromissoryReceivedPayload;
}

export namespace PromissoryReceivedPayload {
  export type AsObject = {
    walletId: string,
  }
}

export class PingPayload extends jspb.Message {
  getMessage(): string;
  setMessage(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PingPayload.AsObject;
  static toObject(includeInstance: boolean, msg: PingPayload): PingPayload.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PingPayload, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PingPayload;
  static deserializeBinaryFromReader(message: PingPayload, reader: jspb.BinaryReader): PingPayload;
}

export namespace PingPayload {
  export type AsObject = {
    message: string,
  }
}

export class Event extends jspb.Message {
  getEventType(): EventTypeMap[keyof EventTypeMap];
  setEventType(value: EventTypeMap[keyof EventTypeMap]): void;

  getSignature(): string;
  setSignature(value: string): void;

  getEventId(): string;
  setEventId(value: string): void;

  getDeliveryDate(): string;
  setDeliveryDate(value: string): void;

  getOwnerId(): string;
  setOwnerId(value: string): void;

  hasWalletCreatedPayload(): boolean;
  clearWalletCreatedPayload(): void;
  getWalletCreatedPayload(): CreateWalletPayload | undefined;
  setWalletCreatedPayload(value?: CreateWalletPayload): void;

  hasPromissoryReceivedPayload(): boolean;
  clearPromissoryReceivedPayload(): void;
  getPromissoryReceivedPayload(): PromissoryReceivedPayload | undefined;
  setPromissoryReceivedPayload(value?: PromissoryReceivedPayload): void;

  hasPingPayload(): boolean;
  clearPingPayload(): void;
  getPingPayload(): PingPayload | undefined;
  setPingPayload(value?: PingPayload): void;

  getCreatedDate(): string;
  setCreatedDate(value: string): void;

  getPayloadCase(): Event.PayloadCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Event.AsObject;
  static toObject(includeInstance: boolean, msg: Event): Event.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Event, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Event;
  static deserializeBinaryFromReader(message: Event, reader: jspb.BinaryReader): Event;
}

export namespace Event {
  export type AsObject = {
    eventType: EventTypeMap[keyof EventTypeMap],
    signature: string,
    eventId: string,
    deliveryDate: string,
    ownerId: string,
    walletCreatedPayload?: CreateWalletPayload.AsObject,
    promissoryReceivedPayload?: PromissoryReceivedPayload.AsObject,
    pingPayload?: PingPayload.AsObject,
    createdDate: string,
  }

  export enum PayloadCase {
    PAYLOAD_NOT_SET = 0,
    WALLET_CREATED_PAYLOAD = 6,
    PROMISSORY_RECEIVED_PAYLOAD = 7,
    PING_PAYLOAD = 8,
  }
}

export class CreateWebhookRequest extends jspb.Message {
  getApiKey(): string;
  setApiKey(value: string): void;

  getName(): string;
  setName(value: string): void;

  clearSupportedEventsList(): void;
  getSupportedEventsList(): Array<EventTypeMap[keyof EventTypeMap]>;
  setSupportedEventsList(value: Array<EventTypeMap[keyof EventTypeMap]>): void;
  addSupportedEvents(value: EventTypeMap[keyof EventTypeMap], index?: number): EventTypeMap[keyof EventTypeMap];

  getWebhookUrl(): string;
  setWebhookUrl(value: string): void;

  getDeliveryMethod(): DeliveryMethodTypeMap[keyof DeliveryMethodTypeMap];
  setDeliveryMethod(value: DeliveryMethodTypeMap[keyof DeliveryMethodTypeMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateWebhookRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateWebhookRequest): CreateWebhookRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateWebhookRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateWebhookRequest;
  static deserializeBinaryFromReader(message: CreateWebhookRequest, reader: jspb.BinaryReader): CreateWebhookRequest;
}

export namespace CreateWebhookRequest {
  export type AsObject = {
    apiKey: string,
    name: string,
    supportedEventsList: Array<EventTypeMap[keyof EventTypeMap]>,
    webhookUrl: string,
    deliveryMethod: DeliveryMethodTypeMap[keyof DeliveryMethodTypeMap],
  }
}

export class CreateWebhookResponse extends jspb.Message {
  getWebhookPublicKey(): string;
  setWebhookPublicKey(value: string): void;

  hasWebhook(): boolean;
  clearWebhook(): void;
  getWebhook(): Webhook | undefined;
  setWebhook(value?: Webhook): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateWebhookResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateWebhookResponse): CreateWebhookResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateWebhookResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateWebhookResponse;
  static deserializeBinaryFromReader(message: CreateWebhookResponse, reader: jspb.BinaryReader): CreateWebhookResponse;
}

export namespace CreateWebhookResponse {
  export type AsObject = {
    webhookPublicKey: string,
    webhook?: Webhook.AsObject,
  }
}

export class ListWebhooksRequest extends jspb.Message {
  getApiKey(): string;
  setApiKey(value: string): void;

  getPage(): number;
  setPage(value: number): void;

  getPageSize(): number;
  setPageSize(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListWebhooksRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListWebhooksRequest): ListWebhooksRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListWebhooksRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListWebhooksRequest;
  static deserializeBinaryFromReader(message: ListWebhooksRequest, reader: jspb.BinaryReader): ListWebhooksRequest;
}

export namespace ListWebhooksRequest {
  export type AsObject = {
    apiKey: string,
    page: number,
    pageSize: number,
  }
}

export class ListWebhooksResponse extends jspb.Message {
  clearWebhooksList(): void;
  getWebhooksList(): Array<Webhook>;
  setWebhooksList(value: Array<Webhook>): void;
  addWebhooks(value?: Webhook, index?: number): Webhook;

  getTotalOwnedWebhooks(): number;
  setTotalOwnedWebhooks(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListWebhooksResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListWebhooksResponse): ListWebhooksResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListWebhooksResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListWebhooksResponse;
  static deserializeBinaryFromReader(message: ListWebhooksResponse, reader: jspb.BinaryReader): ListWebhooksResponse;
}

export namespace ListWebhooksResponse {
  export type AsObject = {
    webhooksList: Array<Webhook.AsObject>,
    totalOwnedWebhooks: number,
  }
}

export class DeleteWebhookRequest extends jspb.Message {
  getApiKey(): string;
  setApiKey(value: string): void;

  getWebhookId(): string;
  setWebhookId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteWebhookRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteWebhookRequest): DeleteWebhookRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteWebhookRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteWebhookRequest;
  static deserializeBinaryFromReader(message: DeleteWebhookRequest, reader: jspb.BinaryReader): DeleteWebhookRequest;
}

export namespace DeleteWebhookRequest {
  export type AsObject = {
    apiKey: string,
    webhookId: string,
  }
}

export class DeleteWebhookResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteWebhookResponse.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteWebhookResponse): DeleteWebhookResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: DeleteWebhookResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteWebhookResponse;
  static deserializeBinaryFromReader(message: DeleteWebhookResponse, reader: jspb.BinaryReader): DeleteWebhookResponse;
}

export namespace DeleteWebhookResponse {
  export type AsObject = {
  }
}

export class EventRequest extends jspb.Message {
  hasEvent(): boolean;
  clearEvent(): void;
  getEvent(): Event | undefined;
  setEvent(value?: Event): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EventRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EventRequest): EventRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: EventRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EventRequest;
  static deserializeBinaryFromReader(message: EventRequest, reader: jspb.BinaryReader): EventRequest;
}

export namespace EventRequest {
  export type AsObject = {
    event?: Event.AsObject,
  }
}

export class EventResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EventResponse.AsObject;
  static toObject(includeInstance: boolean, msg: EventResponse): EventResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: EventResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EventResponse;
  static deserializeBinaryFromReader(message: EventResponse, reader: jspb.BinaryReader): EventResponse;
}

export namespace EventResponse {
  export type AsObject = {
  }
}

export class ListEventsRequest extends jspb.Message {
  getApiKey(): string;
  setApiKey(value: string): void;

  getPage(): number;
  setPage(value: number): void;

  getPageSize(): number;
  setPageSize(value: number): void;

  hasFilters(): boolean;
  clearFilters(): void;
  getFilters(): common_v1_common_pb.Filter | undefined;
  setFilters(value?: common_v1_common_pb.Filter): void;

  hasSortBy(): boolean;
  clearSortBy(): void;
  getSortBy(): common_v1_common_pb.SortBy | undefined;
  setSortBy(value?: common_v1_common_pb.SortBy): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListEventsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListEventsRequest): ListEventsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListEventsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListEventsRequest;
  static deserializeBinaryFromReader(message: ListEventsRequest, reader: jspb.BinaryReader): ListEventsRequest;
}

export namespace ListEventsRequest {
  export type AsObject = {
    apiKey: string,
    page: number,
    pageSize: number,
    filters?: common_v1_common_pb.Filter.AsObject,
    sortBy?: common_v1_common_pb.SortBy.AsObject,
  }
}

export class ListEventsResponse extends jspb.Message {
  clearEventsList(): void;
  getEventsList(): Array<Event>;
  setEventsList(value: Array<Event>): void;
  addEvents(value?: Event, index?: number): Event;

  getTotalOwnedEvents(): number;
  setTotalOwnedEvents(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListEventsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListEventsResponse): ListEventsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListEventsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListEventsResponse;
  static deserializeBinaryFromReader(message: ListEventsResponse, reader: jspb.BinaryReader): ListEventsResponse;
}

export namespace ListEventsResponse {
  export type AsObject = {
    eventsList: Array<Event.AsObject>,
    totalOwnedEvents: number,
  }
}

export class RetryEventRequest extends jspb.Message {
  getApiKey(): string;
  setApiKey(value: string): void;

  getEventId(): string;
  setEventId(value: string): void;

  getWebhookId(): string;
  setWebhookId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RetryEventRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RetryEventRequest): RetryEventRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: RetryEventRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RetryEventRequest;
  static deserializeBinaryFromReader(message: RetryEventRequest, reader: jspb.BinaryReader): RetryEventRequest;
}

export namespace RetryEventRequest {
  export type AsObject = {
    apiKey: string,
    eventId: string,
    webhookId: string,
  }
}

export class RetryEventResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RetryEventResponse.AsObject;
  static toObject(includeInstance: boolean, msg: RetryEventResponse): RetryEventResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: RetryEventResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RetryEventResponse;
  static deserializeBinaryFromReader(message: RetryEventResponse, reader: jspb.BinaryReader): RetryEventResponse;
}

export namespace RetryEventResponse {
  export type AsObject = {
  }
}

export class PingRequest extends jspb.Message {
  getApiKey(): string;
  setApiKey(value: string): void;

  getMessage(): string;
  setMessage(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PingRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PingRequest): PingRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PingRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PingRequest;
  static deserializeBinaryFromReader(message: PingRequest, reader: jspb.BinaryReader): PingRequest;
}

export namespace PingRequest {
  export type AsObject = {
    apiKey: string,
    message: string,
  }
}

export class PingResponse extends jspb.Message {
  getMessage(): string;
  setMessage(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PingResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PingResponse): PingResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PingResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PingResponse;
  static deserializeBinaryFromReader(message: PingResponse, reader: jspb.BinaryReader): PingResponse;
}

export namespace PingResponse {
  export type AsObject = {
    message: string,
  }
}

export class ListDeliveryHistoryRequest extends jspb.Message {
  getApiKey(): string;
  setApiKey(value: string): void;

  getPage(): number;
  setPage(value: number): void;

  getPageSize(): number;
  setPageSize(value: number): void;

  hasFilters(): boolean;
  clearFilters(): void;
  getFilters(): common_v1_common_pb.Filter | undefined;
  setFilters(value?: common_v1_common_pb.Filter): void;

  hasSortBy(): boolean;
  clearSortBy(): void;
  getSortBy(): common_v1_common_pb.SortBy | undefined;
  setSortBy(value?: common_v1_common_pb.SortBy): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeliveryHistoryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeliveryHistoryRequest): ListDeliveryHistoryRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListDeliveryHistoryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListDeliveryHistoryRequest;
  static deserializeBinaryFromReader(message: ListDeliveryHistoryRequest, reader: jspb.BinaryReader): ListDeliveryHistoryRequest;
}

export namespace ListDeliveryHistoryRequest {
  export type AsObject = {
    apiKey: string,
    page: number,
    pageSize: number,
    filters?: common_v1_common_pb.Filter.AsObject,
    sortBy?: common_v1_common_pb.SortBy.AsObject,
  }
}

export class ListDeliveryHistoryResponse extends jspb.Message {
  clearDeliveryHistoryList(): void;
  getDeliveryHistoryList(): Array<ListDeliveryHistoryResponse.DeliveryHistory>;
  setDeliveryHistoryList(value: Array<ListDeliveryHistoryResponse.DeliveryHistory>): void;
  addDeliveryHistory(value?: ListDeliveryHistoryResponse.DeliveryHistory, index?: number): ListDeliveryHistoryResponse.DeliveryHistory;

  getTotalOwnedDeliveryHistory(): number;
  setTotalOwnedDeliveryHistory(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListDeliveryHistoryResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ListDeliveryHistoryResponse): ListDeliveryHistoryResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ListDeliveryHistoryResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListDeliveryHistoryResponse;
  static deserializeBinaryFromReader(message: ListDeliveryHistoryResponse, reader: jspb.BinaryReader): ListDeliveryHistoryResponse;
}

export namespace ListDeliveryHistoryResponse {
  export type AsObject = {
    deliveryHistoryList: Array<ListDeliveryHistoryResponse.DeliveryHistory.AsObject>,
    totalOwnedDeliveryHistory: number,
  }

  export class DeliveryHistory extends jspb.Message {
    getId(): string;
    setId(value: string): void;

    getWebhookId(): string;
    setWebhookId(value: string): void;

    getEventId(): string;
    setEventId(value: string): void;

    getOwnerId(): string;
    setOwnerId(value: string): void;

    getDeliveryStage(): DeliveryStageMap[keyof DeliveryStageMap];
    setDeliveryStage(value: DeliveryStageMap[keyof DeliveryStageMap]): void;

    getCreated(): string;
    setCreated(value: string): void;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): DeliveryHistory.AsObject;
    static toObject(includeInstance: boolean, msg: DeliveryHistory): DeliveryHistory.AsObject;
    static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
    static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
    static serializeBinaryToWriter(message: DeliveryHistory, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): DeliveryHistory;
    static deserializeBinaryFromReader(message: DeliveryHistory, reader: jspb.BinaryReader): DeliveryHistory;
  }

  export namespace DeliveryHistory {
    export type AsObject = {
      id: string,
      webhookId: string,
      eventId: string,
      ownerId: string,
      deliveryStage: DeliveryStageMap[keyof DeliveryStageMap],
      created: string,
    }
  }
}

export interface DeliveryStageMap {
  DELIVERY_STAGE_UNSPECIFIED: 0;
  DELIVERY_STAGE_FIRST_RETRY: 1;
  DELIVERY_STAGE_SECOND_RETRY: 2;
  DELIVERY_STAGE_THIRD_RETRY: 3;
  DELIVERY_STAGE_FOURTH_RETRY: 4;
  DELIVERY_STAGE_FIFTH_RETRY: 5;
  DELIVERY_STAGE_FAILED: 6;
  DELIVERY_STAGE_SUCCESS: 7;
}

export const DeliveryStage: DeliveryStageMap;

export interface EventTypeMap {
  EVENT_TYPE_UNSPECIFIED: 0;
  EVENT_TYPE_WALLET_CREATED: 1;
  EVENT_TYPE_PROMISSORY_RECEIVED: 2;
  EVENT_TYPE_PING: 3;
}

export const EventType: EventTypeMap;

export interface DeliveryMethodTypeMap {
  DELIVERY_METHOD_TYPE_UNSPECIFIED: 0;
  DELIVERY_METHOD_TYPE_HTTP: 1;
  DELIVERY_METHOD_TYPE_GRPC: 2;
}

export const DeliveryMethodType: DeliveryMethodTypeMap;

