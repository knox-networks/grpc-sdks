/* eslint-disable */
/*Generated by GenDocu.com*/
// package: authority_api.v1
// file: authority_api/v1/authority.proto

import * as jspb from "google-protobuf";
import * as common_v1_common_pb from "../../common/v1/common_pb";

export class SetIssuerLimitRequest extends jspb.Message {
  hasLimit(): boolean;
  clearLimit(): void;
  getLimit(): common_v1_common_pb.Amount | undefined;
  setLimit(value?: common_v1_common_pb.Amount): void;

  getIssuerPublicKey(): string;
  setIssuerPublicKey(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetIssuerLimitRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SetIssuerLimitRequest): SetIssuerLimitRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SetIssuerLimitRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetIssuerLimitRequest;
  static deserializeBinaryFromReader(message: SetIssuerLimitRequest, reader: jspb.BinaryReader): SetIssuerLimitRequest;
}

export namespace SetIssuerLimitRequest {
  export type AsObject = {
    limit?: common_v1_common_pb.Amount.AsObject,
    issuerPublicKey: string,
  }
}

export class SetIssuerLimitResponse extends jspb.Message {
  hasLimit(): boolean;
  clearLimit(): void;
  getLimit(): common_v1_common_pb.Amount | undefined;
  setLimit(value?: common_v1_common_pb.Amount): void;

  getSignature(): Uint8Array | string;
  getSignature_asU8(): Uint8Array;
  getSignature_asB64(): string;
  setSignature(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetIssuerLimitResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SetIssuerLimitResponse): SetIssuerLimitResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SetIssuerLimitResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetIssuerLimitResponse;
  static deserializeBinaryFromReader(message: SetIssuerLimitResponse, reader: jspb.BinaryReader): SetIssuerLimitResponse;
}

export namespace SetIssuerLimitResponse {
  export type AsObject = {
    limit?: common_v1_common_pb.Amount.AsObject,
    signature: Uint8Array | string,
  }
}

export class GetIssuerLimitsRequest extends jspb.Message {
  getIssuerPublicKey(): string;
  setIssuerPublicKey(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIssuerLimitsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetIssuerLimitsRequest): GetIssuerLimitsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIssuerLimitsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIssuerLimitsRequest;
  static deserializeBinaryFromReader(message: GetIssuerLimitsRequest, reader: jspb.BinaryReader): GetIssuerLimitsRequest;
}

export namespace GetIssuerLimitsRequest {
  export type AsObject = {
    issuerPublicKey: string,
  }
}

export class GetIssuerLimitsResponse extends jspb.Message {
  clearLimitList(): void;
  getLimitList(): Array<common_v1_common_pb.Amount>;
  setLimitList(value: Array<common_v1_common_pb.Amount>): void;
  addLimit(value?: common_v1_common_pb.Amount, index?: number): common_v1_common_pb.Amount;

  getIssuerPublicKey(): string;
  setIssuerPublicKey(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIssuerLimitsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIssuerLimitsResponse): GetIssuerLimitsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetIssuerLimitsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIssuerLimitsResponse;
  static deserializeBinaryFromReader(message: GetIssuerLimitsResponse, reader: jspb.BinaryReader): GetIssuerLimitsResponse;
}

export namespace GetIssuerLimitsResponse {
  export type AsObject = {
    limitList: Array<common_v1_common_pb.Amount.AsObject>,
    issuerPublicKey: string,
  }
}

export class AuthorizeRequest extends jspb.Message {
  getPromissoryFile(): Uint8Array | string;
  getPromissoryFile_asU8(): Uint8Array;
  getPromissoryFile_asB64(): string;
  setPromissoryFile(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AuthorizeRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AuthorizeRequest): AuthorizeRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AuthorizeRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AuthorizeRequest;
  static deserializeBinaryFromReader(message: AuthorizeRequest, reader: jspb.BinaryReader): AuthorizeRequest;
}

export namespace AuthorizeRequest {
  export type AsObject = {
    promissoryFile: Uint8Array | string,
  }
}

export class AuthorizeResponse extends jspb.Message {
  getPromissoryFile(): Uint8Array | string;
  getPromissoryFile_asU8(): Uint8Array;
  getPromissoryFile_asB64(): string;
  setPromissoryFile(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AuthorizeResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AuthorizeResponse): AuthorizeResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AuthorizeResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AuthorizeResponse;
  static deserializeBinaryFromReader(message: AuthorizeResponse, reader: jspb.BinaryReader): AuthorizeResponse;
}

export namespace AuthorizeResponse {
  export type AsObject = {
    promissoryFile: Uint8Array | string,
  }
}

export class RedeemRequest extends jspb.Message {
  getPromissoryFile(): Uint8Array | string;
  getPromissoryFile_asU8(): Uint8Array;
  getPromissoryFile_asB64(): string;
  setPromissoryFile(value: Uint8Array | string): void;

  getVerifier(): string;
  setVerifier(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RedeemRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RedeemRequest): RedeemRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: RedeemRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RedeemRequest;
  static deserializeBinaryFromReader(message: RedeemRequest, reader: jspb.BinaryReader): RedeemRequest;
}

export namespace RedeemRequest {
  export type AsObject = {
    promissoryFile: Uint8Array | string,
    verifier: string,
  }
}

export class RedeemResponse extends jspb.Message {
  hasLimit(): boolean;
  clearLimit(): void;
  getLimit(): common_v1_common_pb.Amount | undefined;
  setLimit(value?: common_v1_common_pb.Amount): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RedeemResponse.AsObject;
  static toObject(includeInstance: boolean, msg: RedeemResponse): RedeemResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: RedeemResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RedeemResponse;
  static deserializeBinaryFromReader(message: RedeemResponse, reader: jspb.BinaryReader): RedeemResponse;
}

export namespace RedeemResponse {
  export type AsObject = {
    limit?: common_v1_common_pb.Amount.AsObject,
  }
}

export class GetEmissaryRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetEmissaryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetEmissaryRequest): GetEmissaryRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetEmissaryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetEmissaryRequest;
  static deserializeBinaryFromReader(message: GetEmissaryRequest, reader: jspb.BinaryReader): GetEmissaryRequest;
}

export namespace GetEmissaryRequest {
  export type AsObject = {
  }
}

export class GetEmissaryResponse extends jspb.Message {
  getVerifier(): string;
  setVerifier(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetEmissaryResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetEmissaryResponse): GetEmissaryResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetEmissaryResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetEmissaryResponse;
  static deserializeBinaryFromReader(message: GetEmissaryResponse, reader: jspb.BinaryReader): GetEmissaryResponse;
}

export namespace GetEmissaryResponse {
  export type AsObject = {
    verifier: string,
  }
}

export class GetNotaryRequest extends jspb.Message {
  getVerifier(): string;
  setVerifier(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetNotaryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetNotaryRequest): GetNotaryRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetNotaryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetNotaryRequest;
  static deserializeBinaryFromReader(message: GetNotaryRequest, reader: jspb.BinaryReader): GetNotaryRequest;
}

export namespace GetNotaryRequest {
  export type AsObject = {
    verifier: string,
  }
}

export class GetNotaryResponse extends jspb.Message {
  getSignature(): Uint8Array | string;
  getSignature_asU8(): Uint8Array;
  getSignature_asB64(): string;
  setSignature(value: Uint8Array | string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetNotaryResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetNotaryResponse): GetNotaryResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetNotaryResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetNotaryResponse;
  static deserializeBinaryFromReader(message: GetNotaryResponse, reader: jspb.BinaryReader): GetNotaryResponse;
}

export namespace GetNotaryResponse {
  export type AsObject = {
    signature: Uint8Array | string,
  }
}

