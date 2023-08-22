/* eslint-disable */
/*Generated by GenDocu.com*/
// package: issuer_api.v1
// file: issuer_api/v1/issuer.proto

import * as jspb from "google-protobuf";
import * as common_v1_common_pb from "../../common/v1/common_pb";

export class IssueRequest extends jspb.Message {
  hasAuthenticationSignature(): boolean;
  clearAuthenticationSignature(): void;
  getAuthenticationSignature(): common_v1_common_pb.AuthenticationSignature | undefined;
  setAuthenticationSignature(value?: common_v1_common_pb.AuthenticationSignature): void;

  hasAmount(): boolean;
  clearAmount(): void;
  getAmount(): common_v1_common_pb.Amount | undefined;
  setAmount(value?: common_v1_common_pb.Amount): void;

  hasAuthority(): boolean;
  clearAuthority(): void;
  getAuthority(): common_v1_common_pb.DynamicVerifier | undefined;
  setAuthority(value?: common_v1_common_pb.DynamicVerifier): void;

  hasRecipient(): boolean;
  clearRecipient(): void;
  getRecipient(): common_v1_common_pb.DynamicVerifier | undefined;
  setRecipient(value?: common_v1_common_pb.DynamicVerifier): void;

  hasDistribution(): boolean;
  clearDistribution(): void;
  getDistribution(): common_v1_common_pb.Distribution | undefined;
  setDistribution(value?: common_v1_common_pb.Distribution): void;

  getSession(): string;
  setSession(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IssueRequest.AsObject;
  static toObject(includeInstance: boolean, msg: IssueRequest): IssueRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: IssueRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IssueRequest;
  static deserializeBinaryFromReader(message: IssueRequest, reader: jspb.BinaryReader): IssueRequest;
}

export namespace IssueRequest {
  export type AsObject = {
    authenticationSignature?: common_v1_common_pb.AuthenticationSignature.AsObject,
    amount?: common_v1_common_pb.Amount.AsObject,
    authority?: common_v1_common_pb.DynamicVerifier.AsObject,
    recipient?: common_v1_common_pb.DynamicVerifier.AsObject,
    distribution?: common_v1_common_pb.Distribution.AsObject,
    session: string,
  }
}

export class IssueResponse extends jspb.Message {
  getNumPromissoryFiles(): number;
  setNumPromissoryFiles(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): IssueResponse.AsObject;
  static toObject(includeInstance: boolean, msg: IssueResponse): IssueResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: IssueResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): IssueResponse;
  static deserializeBinaryFromReader(message: IssueResponse, reader: jspb.BinaryReader): IssueResponse;
}

export namespace IssueResponse {
  export type AsObject = {
    numPromissoryFiles: number,
  }
}

export class AuthenticateRequest extends jspb.Message {
  hasVerifier(): boolean;
  clearVerifier(): void;
  getVerifier(): common_v1_common_pb.DynamicVerifier | undefined;
  setVerifier(value?: common_v1_common_pb.DynamicVerifier): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AuthenticateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AuthenticateRequest): AuthenticateRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AuthenticateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AuthenticateRequest;
  static deserializeBinaryFromReader(message: AuthenticateRequest, reader: jspb.BinaryReader): AuthenticateRequest;
}

export namespace AuthenticateRequest {
  export type AsObject = {
    verifier?: common_v1_common_pb.DynamicVerifier.AsObject,
  }
}

export class AuthenticateResponse extends jspb.Message {
  getChallenge(): Uint8Array | string;
  getChallenge_asU8(): Uint8Array;
  getChallenge_asB64(): string;
  setChallenge(value: Uint8Array | string): void;

  getSession(): string;
  setSession(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AuthenticateResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AuthenticateResponse): AuthenticateResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AuthenticateResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AuthenticateResponse;
  static deserializeBinaryFromReader(message: AuthenticateResponse, reader: jspb.BinaryReader): AuthenticateResponse;
}

export namespace AuthenticateResponse {
  export type AsObject = {
    challenge: Uint8Array | string,
    session: string,
  }
}

export class GetRolesRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetRolesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetRolesRequest): GetRolesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetRolesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetRolesRequest;
  static deserializeBinaryFromReader(message: GetRolesRequest, reader: jspb.BinaryReader): GetRolesRequest;
}

export namespace GetRolesRequest {
  export type AsObject = {
  }
}

export class GetRolesResponse extends jspb.Message {
  getRolesMap(): jspb.Map<string, string>;
  clearRolesMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetRolesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetRolesResponse): GetRolesResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GetRolesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetRolesResponse;
  static deserializeBinaryFromReader(message: GetRolesResponse, reader: jspb.BinaryReader): GetRolesResponse;
}

export namespace GetRolesResponse {
  export type AsObject = {
    rolesMap: Array<[string, string]>,
  }
}

export class SetRoleRequest extends jspb.Message {
  hasRole(): boolean;
  clearRole(): void;
  getRole(): common_v1_common_pb.Role | undefined;
  setRole(value?: common_v1_common_pb.Role): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetRoleRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SetRoleRequest): SetRoleRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SetRoleRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetRoleRequest;
  static deserializeBinaryFromReader(message: SetRoleRequest, reader: jspb.BinaryReader): SetRoleRequest;
}

export namespace SetRoleRequest {
  export type AsObject = {
    role?: common_v1_common_pb.Role.AsObject,
  }
}

export class SetRoleResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetRoleResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SetRoleResponse): SetRoleResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SetRoleResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetRoleResponse;
  static deserializeBinaryFromReader(message: SetRoleResponse, reader: jspb.BinaryReader): SetRoleResponse;
}

export namespace SetRoleResponse {
  export type AsObject = {
  }
}

export class AssetDenomination extends jspb.Message {
  clearDenominationsList(): void;
  getDenominationsList(): Array<number>;
  setDenominationsList(value: Array<number>): void;
  addDenominations(value: number, index?: number): number;

  getPrecision(): number;
  setPrecision(value: number): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AssetDenomination.AsObject;
  static toObject(includeInstance: boolean, msg: AssetDenomination): AssetDenomination.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AssetDenomination, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AssetDenomination;
  static deserializeBinaryFromReader(message: AssetDenomination, reader: jspb.BinaryReader): AssetDenomination;
}

export namespace AssetDenomination {
  export type AsObject = {
    denominationsList: Array<number>,
    precision: number,
  }
}

export class SetAssetDenominationsRequest extends jspb.Message {
  getDenominationsMap(): jspb.Map<string, AssetDenomination>;
  clearDenominationsMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetAssetDenominationsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SetAssetDenominationsRequest): SetAssetDenominationsRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SetAssetDenominationsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetAssetDenominationsRequest;
  static deserializeBinaryFromReader(message: SetAssetDenominationsRequest, reader: jspb.BinaryReader): SetAssetDenominationsRequest;
}

export namespace SetAssetDenominationsRequest {
  export type AsObject = {
    denominationsMap: Array<[string, AssetDenomination.AsObject]>,
  }
}

export class SetAssetDenominationsResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetAssetDenominationsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SetAssetDenominationsResponse): SetAssetDenominationsResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SetAssetDenominationsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetAssetDenominationsResponse;
  static deserializeBinaryFromReader(message: SetAssetDenominationsResponse, reader: jspb.BinaryReader): SetAssetDenominationsResponse;
}

export namespace SetAssetDenominationsResponse {
  export type AsObject = {
  }
}

export class SetAssetAuthoritiesRequest extends jspb.Message {
  getAuthoritiesMap(): jspb.Map<string, common_v1_common_pb.AssetAuthority>;
  clearAuthoritiesMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetAssetAuthoritiesRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SetAssetAuthoritiesRequest): SetAssetAuthoritiesRequest.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SetAssetAuthoritiesRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetAssetAuthoritiesRequest;
  static deserializeBinaryFromReader(message: SetAssetAuthoritiesRequest, reader: jspb.BinaryReader): SetAssetAuthoritiesRequest;
}

export namespace SetAssetAuthoritiesRequest {
  export type AsObject = {
    authoritiesMap: Array<[string, common_v1_common_pb.AssetAuthority.AsObject]>,
  }
}

export class SetAssetAuthoritiesResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetAssetAuthoritiesResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SetAssetAuthoritiesResponse): SetAssetAuthoritiesResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SetAssetAuthoritiesResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetAssetAuthoritiesResponse;
  static deserializeBinaryFromReader(message: SetAssetAuthoritiesResponse, reader: jspb.BinaryReader): SetAssetAuthoritiesResponse;
}

export namespace SetAssetAuthoritiesResponse {
  export type AsObject = {
  }
}

