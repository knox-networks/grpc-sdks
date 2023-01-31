/* eslint-disable */
/*Generated by GenDocu.com*/
// package: common
// file: common/v1/packet.proto

import * as jspb from "google-protobuf";
import * as common_v1_common_pb from "../../common/v1/common_pb";

export class Packet extends jspb.Message {
  getPacketType(): PacketTypeMap[keyof PacketTypeMap];
  setPacketType(value: PacketTypeMap[keyof PacketTypeMap]): void;

  getData(): Uint8Array | string;
  getData_asU8(): Uint8Array;
  getData_asB64(): string;
  setData(value: Uint8Array | string): void;

  getChannel(): string;
  setChannel(value: string): void;

  hasSender(): boolean;
  clearSender(): void;
  getSender(): common_v1_common_pb.DynamicVerifier | undefined;
  setSender(value?: common_v1_common_pb.DynamicVerifier): void;

  hasRecipient(): boolean;
  clearRecipient(): void;
  getRecipient(): common_v1_common_pb.DynamicVerifier | undefined;
  setRecipient(value?: common_v1_common_pb.DynamicVerifier): void;

  getNonce(): Uint8Array | string;
  getNonce_asU8(): Uint8Array;
  getNonce_asB64(): string;
  setNonce(value: Uint8Array | string): void;

  hasTransactionHeader(): boolean;
  clearTransactionHeader(): void;
  getTransactionHeader(): TransactionHeader | undefined;
  setTransactionHeader(value?: TransactionHeader): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Packet.AsObject;
  static toObject(includeInstance: boolean, msg: Packet): Packet.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Packet, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Packet;
  static deserializeBinaryFromReader(message: Packet, reader: jspb.BinaryReader): Packet;
}

export namespace Packet {
  export type AsObject = {
    packetType: PacketTypeMap[keyof PacketTypeMap],
    data: Uint8Array | string,
    channel: string,
    sender?: common_v1_common_pb.DynamicVerifier.AsObject,
    recipient?: common_v1_common_pb.DynamicVerifier.AsObject,
    nonce: Uint8Array | string,
    transactionHeader?: TransactionHeader.AsObject,
  }
}

export class TransactionHeader extends jspb.Message {
  hasStarter(): boolean;
  clearStarter(): void;
  getStarter(): common_v1_common_pb.DynamicVerifier | undefined;
  setStarter(value?: common_v1_common_pb.DynamicVerifier): void;

  hasResponder(): boolean;
  clearResponder(): void;
  getResponder(): common_v1_common_pb.DynamicVerifier | undefined;
  setResponder(value?: common_v1_common_pb.DynamicVerifier): void;

  getUetr(): string;
  setUetr(value: string): void;

  getTimestamp(): number;
  setTimestamp(value: number): void;

  getSignature(): Uint8Array | string;
  getSignature_asU8(): Uint8Array;
  getSignature_asB64(): string;
  setSignature(value: Uint8Array | string): void;

  getUserReference(): string;
  setUserReference(value: string): void;

  clearRelatedUetrsList(): void;
  getRelatedUetrsList(): Array<string>;
  setRelatedUetrsList(value: Array<string>): void;
  addRelatedUetrs(value: string, index?: number): string;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): TransactionHeader.AsObject;
  static toObject(includeInstance: boolean, msg: TransactionHeader): TransactionHeader.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: TransactionHeader, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): TransactionHeader;
  static deserializeBinaryFromReader(message: TransactionHeader, reader: jspb.BinaryReader): TransactionHeader;
}

export namespace TransactionHeader {
  export type AsObject = {
    starter?: common_v1_common_pb.DynamicVerifier.AsObject,
    responder?: common_v1_common_pb.DynamicVerifier.AsObject,
    uetr: string,
    timestamp: number,
    signature: Uint8Array | string,
    userReference: string,
    relatedUetrsList: Array<string>,
  }
}

export class SimplePaymentRequest extends jspb.Message {
  hasAmount(): boolean;
  clearAmount(): void;
  getAmount(): common_v1_common_pb.Amount | undefined;
  setAmount(value?: common_v1_common_pb.Amount): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SimplePaymentRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SimplePaymentRequest): SimplePaymentRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SimplePaymentRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SimplePaymentRequest;
  static deserializeBinaryFromReader(message: SimplePaymentRequest, reader: jspb.BinaryReader): SimplePaymentRequest;
}

export namespace SimplePaymentRequest {
  export type AsObject = {
    amount?: common_v1_common_pb.Amount.AsObject,
  }
}

export class SimplePaymentAck extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SimplePaymentAck.AsObject;
  static toObject(includeInstance: boolean, msg: SimplePaymentAck): SimplePaymentAck.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SimplePaymentAck, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SimplePaymentAck;
  static deserializeBinaryFromReader(message: SimplePaymentAck, reader: jspb.BinaryReader): SimplePaymentAck;
}

export namespace SimplePaymentAck {
  export type AsObject = {
  }
}

export class SimplePaymentNack extends jspb.Message {
  getRejectionReason(): SimplePaymentNack.RejectionReasonMap[keyof SimplePaymentNack.RejectionReasonMap];
  setRejectionReason(value: SimplePaymentNack.RejectionReasonMap[keyof SimplePaymentNack.RejectionReasonMap]): void;

  getOther(): string;
  setOther(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SimplePaymentNack.AsObject;
  static toObject(includeInstance: boolean, msg: SimplePaymentNack): SimplePaymentNack.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SimplePaymentNack, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SimplePaymentNack;
  static deserializeBinaryFromReader(message: SimplePaymentNack, reader: jspb.BinaryReader): SimplePaymentNack;
}

export namespace SimplePaymentNack {
  export type AsObject = {
    rejectionReason: SimplePaymentNack.RejectionReasonMap[keyof SimplePaymentNack.RejectionReasonMap],
    other: string,
  }

  export interface RejectionReasonMap {
    REASON_UNKNOWN: 0;
    REASON_OTHER: 1;
    REASON_AMOUNT_EMPTY: 2;
    REASON_CHANGE_REQUEST_AMOUNT_MISMATCH: 3;
  }

  export const RejectionReason: RejectionReasonMap;
}

export class SimplePaymentResponse extends jspb.Message {
  hasReceivedAmount(): boolean;
  clearReceivedAmount(): void;
  getReceivedAmount(): common_v1_common_pb.Amount | undefined;
  setReceivedAmount(value?: common_v1_common_pb.Amount): void;

  clearReceivedPromissoriesList(): void;
  getReceivedPromissoriesList(): Array<string>;
  setReceivedPromissoriesList(value: Array<string>): void;
  addReceivedPromissories(value: string, index?: number): string;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SimplePaymentResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SimplePaymentResponse): SimplePaymentResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SimplePaymentResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SimplePaymentResponse;
  static deserializeBinaryFromReader(message: SimplePaymentResponse, reader: jspb.BinaryReader): SimplePaymentResponse;
}

export namespace SimplePaymentResponse {
  export type AsObject = {
    receivedAmount?: common_v1_common_pb.Amount.AsObject,
    receivedPromissoriesList: Array<string>,
  }
}

export class FundsChangeRequest extends jspb.Message {
  hasGrossAmount(): boolean;
  clearGrossAmount(): void;
  getGrossAmount(): common_v1_common_pb.Amount | undefined;
  setGrossAmount(value?: common_v1_common_pb.Amount): void;

  hasExactAmount(): boolean;
  clearExactAmount(): void;
  getExactAmount(): common_v1_common_pb.Amount | undefined;
  setExactAmount(value?: common_v1_common_pb.Amount): void;

  hasFees(): boolean;
  clearFees(): void;
  getFees(): common_v1_common_pb.Amount | undefined;
  setFees(value?: common_v1_common_pb.Amount): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FundsChangeRequest.AsObject;
  static toObject(includeInstance: boolean, msg: FundsChangeRequest): FundsChangeRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: FundsChangeRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FundsChangeRequest;
  static deserializeBinaryFromReader(message: FundsChangeRequest, reader: jspb.BinaryReader): FundsChangeRequest;
}

export namespace FundsChangeRequest {
  export type AsObject = {
    grossAmount?: common_v1_common_pb.Amount.AsObject,
    exactAmount?: common_v1_common_pb.Amount.AsObject,
    fees?: common_v1_common_pb.Amount.AsObject,
  }
}

export class FundsChangeAck extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FundsChangeAck.AsObject;
  static toObject(includeInstance: boolean, msg: FundsChangeAck): FundsChangeAck.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: FundsChangeAck, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FundsChangeAck;
  static deserializeBinaryFromReader(message: FundsChangeAck, reader: jspb.BinaryReader): FundsChangeAck;
}

export namespace FundsChangeAck {
  export type AsObject = {
  }
}

export class FundsChangeNack extends jspb.Message {
  getRejectionReason(): FundsChangeNack.RejectionReasonMap[keyof FundsChangeNack.RejectionReasonMap];
  setRejectionReason(value: FundsChangeNack.RejectionReasonMap[keyof FundsChangeNack.RejectionReasonMap]): void;

  getOther(): string;
  setOther(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FundsChangeNack.AsObject;
  static toObject(includeInstance: boolean, msg: FundsChangeNack): FundsChangeNack.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: FundsChangeNack, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FundsChangeNack;
  static deserializeBinaryFromReader(message: FundsChangeNack, reader: jspb.BinaryReader): FundsChangeNack;
}

export namespace FundsChangeNack {
  export type AsObject = {
    rejectionReason: FundsChangeNack.RejectionReasonMap[keyof FundsChangeNack.RejectionReasonMap],
    other: string,
  }

  export interface RejectionReasonMap {
    REASON_UNKNOWN: 0;
    REASON_OTHER: 1;
    REASON_AMOUNT_EMPTY: 2;
    REASON_AMOUNTS_MISMATCH: 3;
  }

  export const RejectionReason: RejectionReasonMap;
}

export class CancelTransaction extends jspb.Message {
  getRejectionReason(): CancelTransaction.RejectionReasonMap[keyof CancelTransaction.RejectionReasonMap];
  setRejectionReason(value: CancelTransaction.RejectionReasonMap[keyof CancelTransaction.RejectionReasonMap]): void;

  getOther(): string;
  setOther(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CancelTransaction.AsObject;
  static toObject(includeInstance: boolean, msg: CancelTransaction): CancelTransaction.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CancelTransaction, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CancelTransaction;
  static deserializeBinaryFromReader(message: CancelTransaction, reader: jspb.BinaryReader): CancelTransaction;
}

export namespace CancelTransaction {
  export type AsObject = {
    rejectionReason: CancelTransaction.RejectionReasonMap[keyof CancelTransaction.RejectionReasonMap],
    other: string,
  }

  export interface RejectionReasonMap {
    REASON_UNKNOWN: 0;
    REASON_OTHER: 1;
    REASON_MISMATCHED_TRANSACTIONS: 2;
  }

  export const RejectionReason: RejectionReasonMap;
}

export interface PacketTypeMap {
  UNKNOWN: 0;
  UNSUPPORTED: 1;
  AUTHENTICATE: 2;
  RETURNTOSENDER: 3;
  TERMINATESTREAM: 4;
  KEEPALIVE: 5;
  ACKNOWLEDGEMENT: 6;
  AUTHENTICATEREQUEST: 7;
  AUTHENTICATERESPONSE: 8;
  ADDASSOCIATEDCONNECTION: 9;
  DROPASSOCIATEDCONNECTION: 10;
  PROMISSORY: 100;
  AUTHORIZEPROMISSORY: 101;
  AUTHORIZEANDFORWARDPROMISSORY: 102;
  ARCHIVEPROMISSORY: 103;
  PROMISSORYRECEIPT: 104;
  IDENTITYREQUEST: 200;
  IDENTITYRESPONSE: 201;
  IDENTITYPROOF: 202;
  NOTARYNOTARIZEPROMISSORY: 300;
  NOTARYNOTARIZEANDFORWARDPROMISSORY: 301;
  EMISSARYCONNECTIONIDENTITY: 400;
  REMITTANCEREQUEST: 700;
  REMITTANCERESPONSE: 701;
  SPREQUEST: 1000;
  SPACK: 1001;
  SPNACK: 1002;
  SPSTATUS: 1003;
  SPSTART: 1004;
  FCREQUEST: 1100;
  FCACK: 1101;
  FCNACK: 1102;
}

export const PacketType: PacketTypeMap;

