/* eslint-disable */
/*Generated by GenDocu.com*/
// package: vc_api.v1
// file: vc_api/v1/vc.proto

import * as jspb from "google-protobuf";

export class CredentialProof extends jspb.Message {
  getType(): string;
  setType(value: string): void;

  getCreated(): string;
  setCreated(value: string): void;

  getVerificationMethod(): string;
  setVerificationMethod(value: string): void;

  getProofPurpose(): string;
  setProofPurpose(value: string): void;

  getProofValue(): string;
  setProofValue(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CredentialProof.AsObject;
  static toObject(includeInstance: boolean, msg: CredentialProof): CredentialProof.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CredentialProof, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CredentialProof;
  static deserializeBinaryFromReader(message: CredentialProof, reader: jspb.BinaryReader): CredentialProof;
}

export namespace CredentialProof {
  export type AsObject = {
    type: string,
    created: string,
    verificationMethod: string,
    proofPurpose: string,
    proofValue: string,
  }
}

export class CreateIssuanceChallengeRequest extends jspb.Message {
  getCredentialType(): CredentialTypeMap[keyof CredentialTypeMap];
  setCredentialType(value: CredentialTypeMap[keyof CredentialTypeMap]): void;

  getDid(): string;
  setDid(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateIssuanceChallengeRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateIssuanceChallengeRequest): CreateIssuanceChallengeRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateIssuanceChallengeRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateIssuanceChallengeRequest;
  static deserializeBinaryFromReader(message: CreateIssuanceChallengeRequest, reader: jspb.BinaryReader): CreateIssuanceChallengeRequest;
}

export namespace CreateIssuanceChallengeRequest {
  export type AsObject = {
    credentialType: CredentialTypeMap[keyof CredentialTypeMap],
    did: string,
  }
}

export class CreateIssuanceChallengeResponse extends jspb.Message {
  getNonce(): string;
  setNonce(value: string): void;

  getEndpoint(): string;
  setEndpoint(value: string): void;

  getCredentialType(): CredentialTypeMap[keyof CredentialTypeMap];
  setCredentialType(value: CredentialTypeMap[keyof CredentialTypeMap]): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateIssuanceChallengeResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateIssuanceChallengeResponse): CreateIssuanceChallengeResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreateIssuanceChallengeResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateIssuanceChallengeResponse;
  static deserializeBinaryFromReader(message: CreateIssuanceChallengeResponse, reader: jspb.BinaryReader): CreateIssuanceChallengeResponse;
}

export namespace CreateIssuanceChallengeResponse {
  export type AsObject = {
    nonce: string,
    endpoint: string,
    credentialType: CredentialTypeMap[keyof CredentialTypeMap],
  }
}

export class IssueVerifiableCredentialRequest extends jspb.Message {
  getCredentialType(): CredentialTypeMap[keyof CredentialTypeMap];
  setCredentialType(value: CredentialTypeMap[keyof CredentialTypeMap]): void;

  getNonce(): string;
  setNonce(value: string): void;

  getSignature(): Uint8Array | string;
  getSignature_asU8(): Uint8Array;
  getSignature_asB64(): string;
  setSignature(value: Uint8Array | string): void;

  getDid(): string;
  setDid(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IssueVerifiableCredentialRequest.AsObject;
  static toObject(includeInstance: boolean, msg: IssueVerifiableCredentialRequest): IssueVerifiableCredentialRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: IssueVerifiableCredentialRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IssueVerifiableCredentialRequest;
  static deserializeBinaryFromReader(message: IssueVerifiableCredentialRequest, reader: jspb.BinaryReader): IssueVerifiableCredentialRequest;
}

export namespace IssueVerifiableCredentialRequest {
  export type AsObject = {
    credentialType: CredentialTypeMap[keyof CredentialTypeMap],
    nonce: string,
    signature: Uint8Array | string,
    did: string,
  }
}

export class IssueVerifiableCredentialResponse extends jspb.Message {
  getCredential(): string;
  setCredential(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IssueVerifiableCredentialResponse.AsObject;
  static toObject(includeInstance: boolean, msg: IssueVerifiableCredentialResponse): IssueVerifiableCredentialResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: IssueVerifiableCredentialResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IssueVerifiableCredentialResponse;
  static deserializeBinaryFromReader(message: IssueVerifiableCredentialResponse, reader: jspb.BinaryReader): IssueVerifiableCredentialResponse;
}

export namespace IssueVerifiableCredentialResponse {
  export type AsObject = {
    credential: string,
  }
}

export class CreatePresentationChallengeRequest extends jspb.Message {
  clearCredentialTypesList(): void;
  getCredentialTypesList(): Array<CredentialTypeMap[keyof CredentialTypeMap]>;
  setCredentialTypesList(value: Array<CredentialTypeMap[keyof CredentialTypeMap]>): void;
  addCredentialTypes(value: CredentialTypeMap[keyof CredentialTypeMap], index?: number): CredentialTypeMap[keyof CredentialTypeMap];

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreatePresentationChallengeRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreatePresentationChallengeRequest): CreatePresentationChallengeRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreatePresentationChallengeRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreatePresentationChallengeRequest;
  static deserializeBinaryFromReader(message: CreatePresentationChallengeRequest, reader: jspb.BinaryReader): CreatePresentationChallengeRequest;
}

export namespace CreatePresentationChallengeRequest {
  export type AsObject = {
    credentialTypesList: Array<CredentialTypeMap[keyof CredentialTypeMap]>,
  }
}

export class CreatePresentationChallengeResponse extends jspb.Message {
  getNonce(): string;
  setNonce(value: string): void;

  getEndpoint(): string;
  setEndpoint(value: string): void;

  clearCredentialTypesList(): void;
  getCredentialTypesList(): Array<CredentialTypeMap[keyof CredentialTypeMap]>;
  setCredentialTypesList(value: Array<CredentialTypeMap[keyof CredentialTypeMap]>): void;
  addCredentialTypes(value: CredentialTypeMap[keyof CredentialTypeMap], index?: number): CredentialTypeMap[keyof CredentialTypeMap];

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreatePresentationChallengeResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreatePresentationChallengeResponse): CreatePresentationChallengeResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CreatePresentationChallengeResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreatePresentationChallengeResponse;
  static deserializeBinaryFromReader(message: CreatePresentationChallengeResponse, reader: jspb.BinaryReader): CreatePresentationChallengeResponse;
}

export namespace CreatePresentationChallengeResponse {
  export type AsObject = {
    nonce: string,
    endpoint: string,
    credentialTypesList: Array<CredentialTypeMap[keyof CredentialTypeMap]>,
  }
}

export class PresentVerifiableCredentialRequest extends jspb.Message {
  getPresentation(): string;
  setPresentation(value: string): void;

  getNonce(): string;
  setNonce(value: string): void;

  getSignature(): Uint8Array | string;
  getSignature_asU8(): Uint8Array;
  getSignature_asB64(): string;
  setSignature(value: Uint8Array | string): void;

  getDid(): string;
  setDid(value: string): void;

  clearCredentialTypeList(): void;
  getCredentialTypeList(): Array<CredentialTypeMap[keyof CredentialTypeMap]>;
  setCredentialTypeList(value: Array<CredentialTypeMap[keyof CredentialTypeMap]>): void;
  addCredentialType(value: CredentialTypeMap[keyof CredentialTypeMap], index?: number): CredentialTypeMap[keyof CredentialTypeMap];

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PresentVerifiableCredentialRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PresentVerifiableCredentialRequest): PresentVerifiableCredentialRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PresentVerifiableCredentialRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PresentVerifiableCredentialRequest;
  static deserializeBinaryFromReader(message: PresentVerifiableCredentialRequest, reader: jspb.BinaryReader): PresentVerifiableCredentialRequest;
}

export namespace PresentVerifiableCredentialRequest {
  export type AsObject = {
    presentation: string,
    nonce: string,
    signature: Uint8Array | string,
    did: string,
    credentialTypeList: Array<CredentialTypeMap[keyof CredentialTypeMap]>,
  }
}

export class PresentVerifiableCredentialResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PresentVerifiableCredentialResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PresentVerifiableCredentialResponse): PresentVerifiableCredentialResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PresentVerifiableCredentialResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PresentVerifiableCredentialResponse;
  static deserializeBinaryFromReader(message: PresentVerifiableCredentialResponse, reader: jspb.BinaryReader): PresentVerifiableCredentialResponse;
}

export namespace PresentVerifiableCredentialResponse {
  export type AsObject = {
  }
}

export class WaitForCompletionRequest extends jspb.Message {
  getNonce(): string;
  setNonce(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): WaitForCompletionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: WaitForCompletionRequest): WaitForCompletionRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: WaitForCompletionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): WaitForCompletionRequest;
  static deserializeBinaryFromReader(message: WaitForCompletionRequest, reader: jspb.BinaryReader): WaitForCompletionRequest;
}

export namespace WaitForCompletionRequest {
  export type AsObject = {
    nonce: string,
  }
}

export class WaitForCompletionResponse extends jspb.Message {
  getDid(): string;
  setDid(value: string): void;

  getData(): string;
  setData(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): WaitForCompletionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: WaitForCompletionResponse): WaitForCompletionResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: WaitForCompletionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): WaitForCompletionResponse;
  static deserializeBinaryFromReader(message: WaitForCompletionResponse, reader: jspb.BinaryReader): WaitForCompletionResponse;
}

export namespace WaitForCompletionResponse {
  export type AsObject = {
    did: string,
    data: string,
  }
}

export interface CredentialTypeMap {
  CREDENTIAL_TYPE_UNSPECIFIED: 0;
  CREDENTIAL_TYPE_PERMANENT_RESIDENT_CARD: 1;
  CREDENTIAL_TYPE_BANK_CARD: 2;
  CREDENTIAL_TYPE_BANK_ACCOUNT: 3;
}

export const CredentialType: CredentialTypeMap;

