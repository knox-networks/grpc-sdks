/**
 * Generated by the protoc-gen-ts.  DO NOT EDIT!
 * compiler version: 5.27.1
 * source: treasury/v1/treasury.proto
 * git: https://github.com/thesayyn/protoc-gen-ts */
import * as dependency_1 from "./../../common/v1/common";
import * as pb_1 from "google-protobuf";
import * as grpc_1 from "@grpc/grpc-js";
export namespace treasury {
    export class GetSupplyRequest extends pb_1.Message {
        #one_of_decls: number[][] = [];
        constructor(data?: any[] | {
            currency_code?: string;
        }) {
            super();
            pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [], this.#one_of_decls);
            if (!Array.isArray(data) && typeof data == "object") {
                if ("currency_code" in data && data.currency_code != undefined) {
                    this.currency_code = data.currency_code;
                }
            }
        }
        get currency_code() {
            return pb_1.Message.getFieldWithDefault(this, 1, "") as string;
        }
        set currency_code(value: string) {
            pb_1.Message.setField(this, 1, value);
        }
        static fromObject(data: {
            currency_code?: string;
        }): GetSupplyRequest {
            const message = new GetSupplyRequest({});
            if (data.currency_code != null) {
                message.currency_code = data.currency_code;
            }
            return message;
        }
        toObject() {
            const data: {
                currency_code?: string;
            } = {};
            if (this.currency_code != null) {
                data.currency_code = this.currency_code;
            }
            return data;
        }
        serialize(): Uint8Array;
        serialize(w: pb_1.BinaryWriter): void;
        serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
            const writer = w || new pb_1.BinaryWriter();
            if (this.currency_code.length)
                writer.writeString(1, this.currency_code);
            if (!w)
                return writer.getResultBuffer();
        }
        static deserialize(bytes: Uint8Array | pb_1.BinaryReader): GetSupplyRequest {
            const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new GetSupplyRequest();
            while (reader.nextField()) {
                if (reader.isEndGroup())
                    break;
                switch (reader.getFieldNumber()) {
                    case 1:
                        message.currency_code = reader.readString();
                        break;
                    default: reader.skipField();
                }
            }
            return message;
        }
        serializeBinary(): Uint8Array {
            return this.serialize();
        }
        static deserializeBinary(bytes: Uint8Array): GetSupplyRequest {
            return GetSupplyRequest.deserialize(bytes);
        }
    }
    export class DenominationCount extends pb_1.Message {
        #one_of_decls: number[][] = [];
        constructor(data?: any[] | {
            denomination?: dependency_1.common.Amount;
            count?: number;
            total?: dependency_1.common.Amount;
        }) {
            super();
            pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [], this.#one_of_decls);
            if (!Array.isArray(data) && typeof data == "object") {
                if ("denomination" in data && data.denomination != undefined) {
                    this.denomination = data.denomination;
                }
                if ("count" in data && data.count != undefined) {
                    this.count = data.count;
                }
                if ("total" in data && data.total != undefined) {
                    this.total = data.total;
                }
            }
        }
        get denomination() {
            return pb_1.Message.getWrapperField(this, dependency_1.common.Amount, 1) as dependency_1.common.Amount;
        }
        set denomination(value: dependency_1.common.Amount) {
            pb_1.Message.setWrapperField(this, 1, value);
        }
        get has_denomination() {
            return pb_1.Message.getField(this, 1) != null;
        }
        get count() {
            return pb_1.Message.getFieldWithDefault(this, 2, 0) as number;
        }
        set count(value: number) {
            pb_1.Message.setField(this, 2, value);
        }
        get total() {
            return pb_1.Message.getWrapperField(this, dependency_1.common.Amount, 3) as dependency_1.common.Amount;
        }
        set total(value: dependency_1.common.Amount) {
            pb_1.Message.setWrapperField(this, 3, value);
        }
        get has_total() {
            return pb_1.Message.getField(this, 3) != null;
        }
        static fromObject(data: {
            denomination?: ReturnType<typeof dependency_1.common.Amount.prototype.toObject>;
            count?: number;
            total?: ReturnType<typeof dependency_1.common.Amount.prototype.toObject>;
        }): DenominationCount {
            const message = new DenominationCount({});
            if (data.denomination != null) {
                message.denomination = dependency_1.common.Amount.fromObject(data.denomination);
            }
            if (data.count != null) {
                message.count = data.count;
            }
            if (data.total != null) {
                message.total = dependency_1.common.Amount.fromObject(data.total);
            }
            return message;
        }
        toObject() {
            const data: {
                denomination?: ReturnType<typeof dependency_1.common.Amount.prototype.toObject>;
                count?: number;
                total?: ReturnType<typeof dependency_1.common.Amount.prototype.toObject>;
            } = {};
            if (this.denomination != null) {
                data.denomination = this.denomination.toObject();
            }
            if (this.count != null) {
                data.count = this.count;
            }
            if (this.total != null) {
                data.total = this.total.toObject();
            }
            return data;
        }
        serialize(): Uint8Array;
        serialize(w: pb_1.BinaryWriter): void;
        serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
            const writer = w || new pb_1.BinaryWriter();
            if (this.has_denomination)
                writer.writeMessage(1, this.denomination, () => this.denomination.serialize(writer));
            if (this.count != 0)
                writer.writeUint32(2, this.count);
            if (this.has_total)
                writer.writeMessage(3, this.total, () => this.total.serialize(writer));
            if (!w)
                return writer.getResultBuffer();
        }
        static deserialize(bytes: Uint8Array | pb_1.BinaryReader): DenominationCount {
            const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new DenominationCount();
            while (reader.nextField()) {
                if (reader.isEndGroup())
                    break;
                switch (reader.getFieldNumber()) {
                    case 1:
                        reader.readMessage(message.denomination, () => message.denomination = dependency_1.common.Amount.deserialize(reader));
                        break;
                    case 2:
                        message.count = reader.readUint32();
                        break;
                    case 3:
                        reader.readMessage(message.total, () => message.total = dependency_1.common.Amount.deserialize(reader));
                        break;
                    default: reader.skipField();
                }
            }
            return message;
        }
        serializeBinary(): Uint8Array {
            return this.serialize();
        }
        static deserializeBinary(bytes: Uint8Array): DenominationCount {
            return DenominationCount.deserialize(bytes);
        }
    }
    export class GetSupplyResponse extends pb_1.Message {
        #one_of_decls: number[][] = [];
        constructor(data?: any[] | {
            supply?: DenominationCount[];
            total?: dependency_1.common.Amount;
        }) {
            super();
            pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [1], this.#one_of_decls);
            if (!Array.isArray(data) && typeof data == "object") {
                if ("supply" in data && data.supply != undefined) {
                    this.supply = data.supply;
                }
                if ("total" in data && data.total != undefined) {
                    this.total = data.total;
                }
            }
        }
        get supply() {
            return pb_1.Message.getRepeatedWrapperField(this, DenominationCount, 1) as DenominationCount[];
        }
        set supply(value: DenominationCount[]) {
            pb_1.Message.setRepeatedWrapperField(this, 1, value);
        }
        get total() {
            return pb_1.Message.getWrapperField(this, dependency_1.common.Amount, 2) as dependency_1.common.Amount;
        }
        set total(value: dependency_1.common.Amount) {
            pb_1.Message.setWrapperField(this, 2, value);
        }
        get has_total() {
            return pb_1.Message.getField(this, 2) != null;
        }
        static fromObject(data: {
            supply?: ReturnType<typeof DenominationCount.prototype.toObject>[];
            total?: ReturnType<typeof dependency_1.common.Amount.prototype.toObject>;
        }): GetSupplyResponse {
            const message = new GetSupplyResponse({});
            if (data.supply != null) {
                message.supply = data.supply.map(item => DenominationCount.fromObject(item));
            }
            if (data.total != null) {
                message.total = dependency_1.common.Amount.fromObject(data.total);
            }
            return message;
        }
        toObject() {
            const data: {
                supply?: ReturnType<typeof DenominationCount.prototype.toObject>[];
                total?: ReturnType<typeof dependency_1.common.Amount.prototype.toObject>;
            } = {};
            if (this.supply != null) {
                data.supply = this.supply.map((item: DenominationCount) => item.toObject());
            }
            if (this.total != null) {
                data.total = this.total.toObject();
            }
            return data;
        }
        serialize(): Uint8Array;
        serialize(w: pb_1.BinaryWriter): void;
        serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
            const writer = w || new pb_1.BinaryWriter();
            if (this.supply.length)
                writer.writeRepeatedMessage(1, this.supply, (item: DenominationCount) => item.serialize(writer));
            if (this.has_total)
                writer.writeMessage(2, this.total, () => this.total.serialize(writer));
            if (!w)
                return writer.getResultBuffer();
        }
        static deserialize(bytes: Uint8Array | pb_1.BinaryReader): GetSupplyResponse {
            const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new GetSupplyResponse();
            while (reader.nextField()) {
                if (reader.isEndGroup())
                    break;
                switch (reader.getFieldNumber()) {
                    case 1:
                        reader.readMessage(message.supply, () => pb_1.Message.addToRepeatedWrapperField(message, 1, DenominationCount.deserialize(reader), DenominationCount));
                        break;
                    case 2:
                        reader.readMessage(message.total, () => message.total = dependency_1.common.Amount.deserialize(reader));
                        break;
                    default: reader.skipField();
                }
            }
            return message;
        }
        serializeBinary(): Uint8Array {
            return this.serialize();
        }
        static deserializeBinary(bytes: Uint8Array): GetSupplyResponse {
            return GetSupplyResponse.deserialize(bytes);
        }
    }
    export class ListTransactionsRequest extends pb_1.Message {
        #one_of_decls: number[][] = [];
        constructor(data?: any[] | {
            page?: number;
            page_size?: number;
            sort_by?: dependency_1.common.SortBy;
            filters?: dependency_1.common.Filter;
        }) {
            super();
            pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [], this.#one_of_decls);
            if (!Array.isArray(data) && typeof data == "object") {
                if ("page" in data && data.page != undefined) {
                    this.page = data.page;
                }
                if ("page_size" in data && data.page_size != undefined) {
                    this.page_size = data.page_size;
                }
                if ("sort_by" in data && data.sort_by != undefined) {
                    this.sort_by = data.sort_by;
                }
                if ("filters" in data && data.filters != undefined) {
                    this.filters = data.filters;
                }
            }
        }
        get page() {
            return pb_1.Message.getFieldWithDefault(this, 1, 0) as number;
        }
        set page(value: number) {
            pb_1.Message.setField(this, 1, value);
        }
        get page_size() {
            return pb_1.Message.getFieldWithDefault(this, 2, 0) as number;
        }
        set page_size(value: number) {
            pb_1.Message.setField(this, 2, value);
        }
        get sort_by() {
            return pb_1.Message.getWrapperField(this, dependency_1.common.SortBy, 3) as dependency_1.common.SortBy;
        }
        set sort_by(value: dependency_1.common.SortBy) {
            pb_1.Message.setWrapperField(this, 3, value);
        }
        get has_sort_by() {
            return pb_1.Message.getField(this, 3) != null;
        }
        get filters() {
            return pb_1.Message.getWrapperField(this, dependency_1.common.Filter, 4) as dependency_1.common.Filter;
        }
        set filters(value: dependency_1.common.Filter) {
            pb_1.Message.setWrapperField(this, 4, value);
        }
        get has_filters() {
            return pb_1.Message.getField(this, 4) != null;
        }
        static fromObject(data: {
            page?: number;
            page_size?: number;
            sort_by?: ReturnType<typeof dependency_1.common.SortBy.prototype.toObject>;
            filters?: ReturnType<typeof dependency_1.common.Filter.prototype.toObject>;
        }): ListTransactionsRequest {
            const message = new ListTransactionsRequest({});
            if (data.page != null) {
                message.page = data.page;
            }
            if (data.page_size != null) {
                message.page_size = data.page_size;
            }
            if (data.sort_by != null) {
                message.sort_by = dependency_1.common.SortBy.fromObject(data.sort_by);
            }
            if (data.filters != null) {
                message.filters = dependency_1.common.Filter.fromObject(data.filters);
            }
            return message;
        }
        toObject() {
            const data: {
                page?: number;
                page_size?: number;
                sort_by?: ReturnType<typeof dependency_1.common.SortBy.prototype.toObject>;
                filters?: ReturnType<typeof dependency_1.common.Filter.prototype.toObject>;
            } = {};
            if (this.page != null) {
                data.page = this.page;
            }
            if (this.page_size != null) {
                data.page_size = this.page_size;
            }
            if (this.sort_by != null) {
                data.sort_by = this.sort_by.toObject();
            }
            if (this.filters != null) {
                data.filters = this.filters.toObject();
            }
            return data;
        }
        serialize(): Uint8Array;
        serialize(w: pb_1.BinaryWriter): void;
        serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
            const writer = w || new pb_1.BinaryWriter();
            if (this.page != 0)
                writer.writeInt32(1, this.page);
            if (this.page_size != 0)
                writer.writeInt32(2, this.page_size);
            if (this.has_sort_by)
                writer.writeMessage(3, this.sort_by, () => this.sort_by.serialize(writer));
            if (this.has_filters)
                writer.writeMessage(4, this.filters, () => this.filters.serialize(writer));
            if (!w)
                return writer.getResultBuffer();
        }
        static deserialize(bytes: Uint8Array | pb_1.BinaryReader): ListTransactionsRequest {
            const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new ListTransactionsRequest();
            while (reader.nextField()) {
                if (reader.isEndGroup())
                    break;
                switch (reader.getFieldNumber()) {
                    case 1:
                        message.page = reader.readInt32();
                        break;
                    case 2:
                        message.page_size = reader.readInt32();
                        break;
                    case 3:
                        reader.readMessage(message.sort_by, () => message.sort_by = dependency_1.common.SortBy.deserialize(reader));
                        break;
                    case 4:
                        reader.readMessage(message.filters, () => message.filters = dependency_1.common.Filter.deserialize(reader));
                        break;
                    default: reader.skipField();
                }
            }
            return message;
        }
        serializeBinary(): Uint8Array {
            return this.serialize();
        }
        static deserializeBinary(bytes: Uint8Array): ListTransactionsRequest {
            return ListTransactionsRequest.deserialize(bytes);
        }
    }
    export class ContractSummary extends pb_1.Message {
        #one_of_decls: number[][] = [];
        constructor(data?: any[] | {
            contract_id?: string;
            contract_type?: string;
            created?: string;
            counterparties?: string[];
            memo?: string;
        }) {
            super();
            pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [4], this.#one_of_decls);
            if (!Array.isArray(data) && typeof data == "object") {
                if ("contract_id" in data && data.contract_id != undefined) {
                    this.contract_id = data.contract_id;
                }
                if ("contract_type" in data && data.contract_type != undefined) {
                    this.contract_type = data.contract_type;
                }
                if ("created" in data && data.created != undefined) {
                    this.created = data.created;
                }
                if ("counterparties" in data && data.counterparties != undefined) {
                    this.counterparties = data.counterparties;
                }
                if ("memo" in data && data.memo != undefined) {
                    this.memo = data.memo;
                }
            }
        }
        get contract_id() {
            return pb_1.Message.getFieldWithDefault(this, 1, "") as string;
        }
        set contract_id(value: string) {
            pb_1.Message.setField(this, 1, value);
        }
        get contract_type() {
            return pb_1.Message.getFieldWithDefault(this, 2, "") as string;
        }
        set contract_type(value: string) {
            pb_1.Message.setField(this, 2, value);
        }
        get created() {
            return pb_1.Message.getFieldWithDefault(this, 3, "") as string;
        }
        set created(value: string) {
            pb_1.Message.setField(this, 3, value);
        }
        get counterparties() {
            return pb_1.Message.getFieldWithDefault(this, 4, []) as string[];
        }
        set counterparties(value: string[]) {
            pb_1.Message.setField(this, 4, value);
        }
        get memo() {
            return pb_1.Message.getFieldWithDefault(this, 6, "") as string;
        }
        set memo(value: string) {
            pb_1.Message.setField(this, 6, value);
        }
        static fromObject(data: {
            contract_id?: string;
            contract_type?: string;
            created?: string;
            counterparties?: string[];
            memo?: string;
        }): ContractSummary {
            const message = new ContractSummary({});
            if (data.contract_id != null) {
                message.contract_id = data.contract_id;
            }
            if (data.contract_type != null) {
                message.contract_type = data.contract_type;
            }
            if (data.created != null) {
                message.created = data.created;
            }
            if (data.counterparties != null) {
                message.counterparties = data.counterparties;
            }
            if (data.memo != null) {
                message.memo = data.memo;
            }
            return message;
        }
        toObject() {
            const data: {
                contract_id?: string;
                contract_type?: string;
                created?: string;
                counterparties?: string[];
                memo?: string;
            } = {};
            if (this.contract_id != null) {
                data.contract_id = this.contract_id;
            }
            if (this.contract_type != null) {
                data.contract_type = this.contract_type;
            }
            if (this.created != null) {
                data.created = this.created;
            }
            if (this.counterparties != null) {
                data.counterparties = this.counterparties;
            }
            if (this.memo != null) {
                data.memo = this.memo;
            }
            return data;
        }
        serialize(): Uint8Array;
        serialize(w: pb_1.BinaryWriter): void;
        serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
            const writer = w || new pb_1.BinaryWriter();
            if (this.contract_id.length)
                writer.writeString(1, this.contract_id);
            if (this.contract_type.length)
                writer.writeString(2, this.contract_type);
            if (this.created.length)
                writer.writeString(3, this.created);
            if (this.counterparties.length)
                writer.writeRepeatedString(4, this.counterparties);
            if (this.memo.length)
                writer.writeString(6, this.memo);
            if (!w)
                return writer.getResultBuffer();
        }
        static deserialize(bytes: Uint8Array | pb_1.BinaryReader): ContractSummary {
            const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new ContractSummary();
            while (reader.nextField()) {
                if (reader.isEndGroup())
                    break;
                switch (reader.getFieldNumber()) {
                    case 1:
                        message.contract_id = reader.readString();
                        break;
                    case 2:
                        message.contract_type = reader.readString();
                        break;
                    case 3:
                        message.created = reader.readString();
                        break;
                    case 4:
                        pb_1.Message.addToRepeatedField(message, 4, reader.readString());
                        break;
                    case 6:
                        message.memo = reader.readString();
                        break;
                    default: reader.skipField();
                }
            }
            return message;
        }
        serializeBinary(): Uint8Array {
            return this.serialize();
        }
        static deserializeBinary(bytes: Uint8Array): ContractSummary {
            return ContractSummary.deserialize(bytes);
        }
    }
    export class ListTransactionsResponse extends pb_1.Message {
        #one_of_decls: number[][] = [];
        constructor(data?: any[] | {
            contracts?: ContractSummary[];
            totalContracts?: number;
        }) {
            super();
            pb_1.Message.initialize(this, Array.isArray(data) ? data : [], 0, -1, [1], this.#one_of_decls);
            if (!Array.isArray(data) && typeof data == "object") {
                if ("contracts" in data && data.contracts != undefined) {
                    this.contracts = data.contracts;
                }
                if ("totalContracts" in data && data.totalContracts != undefined) {
                    this.totalContracts = data.totalContracts;
                }
            }
        }
        get contracts() {
            return pb_1.Message.getRepeatedWrapperField(this, ContractSummary, 1) as ContractSummary[];
        }
        set contracts(value: ContractSummary[]) {
            pb_1.Message.setRepeatedWrapperField(this, 1, value);
        }
        get totalContracts() {
            return pb_1.Message.getFieldWithDefault(this, 2, 0) as number;
        }
        set totalContracts(value: number) {
            pb_1.Message.setField(this, 2, value);
        }
        static fromObject(data: {
            contracts?: ReturnType<typeof ContractSummary.prototype.toObject>[];
            totalContracts?: number;
        }): ListTransactionsResponse {
            const message = new ListTransactionsResponse({});
            if (data.contracts != null) {
                message.contracts = data.contracts.map(item => ContractSummary.fromObject(item));
            }
            if (data.totalContracts != null) {
                message.totalContracts = data.totalContracts;
            }
            return message;
        }
        toObject() {
            const data: {
                contracts?: ReturnType<typeof ContractSummary.prototype.toObject>[];
                totalContracts?: number;
            } = {};
            if (this.contracts != null) {
                data.contracts = this.contracts.map((item: ContractSummary) => item.toObject());
            }
            if (this.totalContracts != null) {
                data.totalContracts = this.totalContracts;
            }
            return data;
        }
        serialize(): Uint8Array;
        serialize(w: pb_1.BinaryWriter): void;
        serialize(w?: pb_1.BinaryWriter): Uint8Array | void {
            const writer = w || new pb_1.BinaryWriter();
            if (this.contracts.length)
                writer.writeRepeatedMessage(1, this.contracts, (item: ContractSummary) => item.serialize(writer));
            if (this.totalContracts != 0)
                writer.writeUint32(2, this.totalContracts);
            if (!w)
                return writer.getResultBuffer();
        }
        static deserialize(bytes: Uint8Array | pb_1.BinaryReader): ListTransactionsResponse {
            const reader = bytes instanceof pb_1.BinaryReader ? bytes : new pb_1.BinaryReader(bytes), message = new ListTransactionsResponse();
            while (reader.nextField()) {
                if (reader.isEndGroup())
                    break;
                switch (reader.getFieldNumber()) {
                    case 1:
                        reader.readMessage(message.contracts, () => pb_1.Message.addToRepeatedWrapperField(message, 1, ContractSummary.deserialize(reader), ContractSummary));
                        break;
                    case 2:
                        message.totalContracts = reader.readUint32();
                        break;
                    default: reader.skipField();
                }
            }
            return message;
        }
        serializeBinary(): Uint8Array {
            return this.serialize();
        }
        static deserializeBinary(bytes: Uint8Array): ListTransactionsResponse {
            return ListTransactionsResponse.deserialize(bytes);
        }
    }
    interface GrpcUnaryServiceInterface<P, R> {
        (message: P, metadata: grpc_1.Metadata, options: grpc_1.CallOptions, callback: grpc_1.requestCallback<R>): grpc_1.ClientUnaryCall;
        (message: P, metadata: grpc_1.Metadata, callback: grpc_1.requestCallback<R>): grpc_1.ClientUnaryCall;
        (message: P, options: grpc_1.CallOptions, callback: grpc_1.requestCallback<R>): grpc_1.ClientUnaryCall;
        (message: P, callback: grpc_1.requestCallback<R>): grpc_1.ClientUnaryCall;
    }
    interface GrpcStreamServiceInterface<P, R> {
        (message: P, metadata: grpc_1.Metadata, options?: grpc_1.CallOptions): grpc_1.ClientReadableStream<R>;
        (message: P, options?: grpc_1.CallOptions): grpc_1.ClientReadableStream<R>;
    }
    interface GrpWritableServiceInterface<P, R> {
        (metadata: grpc_1.Metadata, options: grpc_1.CallOptions, callback: grpc_1.requestCallback<R>): grpc_1.ClientWritableStream<P>;
        (metadata: grpc_1.Metadata, callback: grpc_1.requestCallback<R>): grpc_1.ClientWritableStream<P>;
        (options: grpc_1.CallOptions, callback: grpc_1.requestCallback<R>): grpc_1.ClientWritableStream<P>;
        (callback: grpc_1.requestCallback<R>): grpc_1.ClientWritableStream<P>;
    }
    interface GrpcChunkServiceInterface<P, R> {
        (metadata: grpc_1.Metadata, options?: grpc_1.CallOptions): grpc_1.ClientDuplexStream<P, R>;
        (options?: grpc_1.CallOptions): grpc_1.ClientDuplexStream<P, R>;
    }
    interface GrpcPromiseServiceInterface<P, R> {
        (message: P, metadata: grpc_1.Metadata, options?: grpc_1.CallOptions): Promise<R>;
        (message: P, options?: grpc_1.CallOptions): Promise<R>;
    }
    export abstract class UnimplementedTreasuryService {
        static definition = {
            GetSupply: {
                path: "/treasury.Treasury/GetSupply",
                requestStream: false,
                responseStream: false,
                requestSerialize: (message: GetSupplyRequest) => Buffer.from(message.serialize()),
                requestDeserialize: (bytes: Buffer) => GetSupplyRequest.deserialize(new Uint8Array(bytes)),
                responseSerialize: (message: GetSupplyResponse) => Buffer.from(message.serialize()),
                responseDeserialize: (bytes: Buffer) => GetSupplyResponse.deserialize(new Uint8Array(bytes))
            },
            ListTransactions: {
                path: "/treasury.Treasury/ListTransactions",
                requestStream: false,
                responseStream: false,
                requestSerialize: (message: ListTransactionsRequest) => Buffer.from(message.serialize()),
                requestDeserialize: (bytes: Buffer) => ListTransactionsRequest.deserialize(new Uint8Array(bytes)),
                responseSerialize: (message: ListTransactionsResponse) => Buffer.from(message.serialize()),
                responseDeserialize: (bytes: Buffer) => ListTransactionsResponse.deserialize(new Uint8Array(bytes))
            }
        };
        [method: string]: grpc_1.UntypedHandleCall;
        abstract GetSupply(call: grpc_1.ServerUnaryCall<GetSupplyRequest, GetSupplyResponse>, callback: grpc_1.sendUnaryData<GetSupplyResponse>): void;
        abstract ListTransactions(call: grpc_1.ServerUnaryCall<ListTransactionsRequest, ListTransactionsResponse>, callback: grpc_1.sendUnaryData<ListTransactionsResponse>): void;
    }
    export class TreasuryClient extends grpc_1.makeGenericClientConstructor(UnimplementedTreasuryService.definition, "Treasury", {}) {
        constructor(address: string, credentials: grpc_1.ChannelCredentials, options?: Partial<grpc_1.ChannelOptions>) {
            super(address, credentials, options);
        }
        GetSupply: GrpcUnaryServiceInterface<GetSupplyRequest, GetSupplyResponse> = (message: GetSupplyRequest, metadata: grpc_1.Metadata | grpc_1.CallOptions | grpc_1.requestCallback<GetSupplyResponse>, options?: grpc_1.CallOptions | grpc_1.requestCallback<GetSupplyResponse>, callback?: grpc_1.requestCallback<GetSupplyResponse>): grpc_1.ClientUnaryCall => {
            return super.GetSupply(message, metadata, options, callback);
        };
        ListTransactions: GrpcUnaryServiceInterface<ListTransactionsRequest, ListTransactionsResponse> = (message: ListTransactionsRequest, metadata: grpc_1.Metadata | grpc_1.CallOptions | grpc_1.requestCallback<ListTransactionsResponse>, options?: grpc_1.CallOptions | grpc_1.requestCallback<ListTransactionsResponse>, callback?: grpc_1.requestCallback<ListTransactionsResponse>): grpc_1.ClientUnaryCall => {
            return super.ListTransactions(message, metadata, options, callback);
        };
    }
}
