import * as flatbuffers from 'flatbuffers';
import { Account } from './account.js';
import { Input } from './input.js';
export declare class ExecutionRequestV1 {
    bb: flatbuffers.ByteBuffer | null;
    bb_pos: number;
    __init(i: number, bb: flatbuffers.ByteBuffer): ExecutionRequestV1;
    static getRootAsExecutionRequestV1(bb: flatbuffers.ByteBuffer, obj?: ExecutionRequestV1): ExecutionRequestV1;
    static getSizePrefixedRootAsExecutionRequestV1(bb: flatbuffers.ByteBuffer, obj?: ExecutionRequestV1): ExecutionRequestV1;
    tip(): bigint;
    mutate_tip(value: bigint): boolean;
    executionId(): string | null;
    executionId(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
    imageId(): string | null;
    imageId(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
    callbackProgramId(index: number): number | null;
    callbackProgramIdLength(): number;
    callbackProgramIdArray(): Uint8Array | null;
    callbackInstructionPrefix(index: number): number | null;
    callbackInstructionPrefixLength(): number;
    callbackInstructionPrefixArray(): Uint8Array | null;
    forwardOutput(): boolean;
    mutate_forward_output(value: boolean): boolean;
    verifyInputHash(): boolean;
    mutate_verify_input_hash(value: boolean): boolean;
    input(index: number, obj?: Input): Input | null;
    inputLength(): number;
    inputDigest(index: number): number | null;
    inputDigestLength(): number;
    inputDigestArray(): Uint8Array | null;
    maxBlockHeight(): bigint;
    mutate_max_block_height(value: bigint): boolean;
    callbackExtraAccounts(index: number, obj?: Account): Account | null;
    callbackExtraAccountsLength(): number;
    static startExecutionRequestV1(builder: flatbuffers.Builder): void;
    static addTip(builder: flatbuffers.Builder, tip: bigint): void;
    static addExecutionId(builder: flatbuffers.Builder, executionIdOffset: flatbuffers.Offset): void;
    static addImageId(builder: flatbuffers.Builder, imageIdOffset: flatbuffers.Offset): void;
    static addCallbackProgramId(builder: flatbuffers.Builder, callbackProgramIdOffset: flatbuffers.Offset): void;
    static createCallbackProgramIdVector(builder: flatbuffers.Builder, data: number[] | Uint8Array): flatbuffers.Offset;
    static startCallbackProgramIdVector(builder: flatbuffers.Builder, numElems: number): void;
    static addCallbackInstructionPrefix(builder: flatbuffers.Builder, callbackInstructionPrefixOffset: flatbuffers.Offset): void;
    static createCallbackInstructionPrefixVector(builder: flatbuffers.Builder, data: number[] | Uint8Array): flatbuffers.Offset;
    static startCallbackInstructionPrefixVector(builder: flatbuffers.Builder, numElems: number): void;
    static addForwardOutput(builder: flatbuffers.Builder, forwardOutput: boolean): void;
    static addVerifyInputHash(builder: flatbuffers.Builder, verifyInputHash: boolean): void;
    static addInput(builder: flatbuffers.Builder, inputOffset: flatbuffers.Offset): void;
    static createInputVector(builder: flatbuffers.Builder, data: flatbuffers.Offset[]): flatbuffers.Offset;
    static startInputVector(builder: flatbuffers.Builder, numElems: number): void;
    static addInputDigest(builder: flatbuffers.Builder, inputDigestOffset: flatbuffers.Offset): void;
    static createInputDigestVector(builder: flatbuffers.Builder, data: number[] | Uint8Array): flatbuffers.Offset;
    static startInputDigestVector(builder: flatbuffers.Builder, numElems: number): void;
    static addMaxBlockHeight(builder: flatbuffers.Builder, maxBlockHeight: bigint): void;
    static addCallbackExtraAccounts(builder: flatbuffers.Builder, callbackExtraAccountsOffset: flatbuffers.Offset): void;
    static startCallbackExtraAccountsVector(builder: flatbuffers.Builder, numElems: number): void;
    static endExecutionRequestV1(builder: flatbuffers.Builder): flatbuffers.Offset;
    static finishExecutionRequestV1Buffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
    static finishSizePrefixedExecutionRequestV1Buffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
    static createExecutionRequestV1(builder: flatbuffers.Builder, tip: bigint, executionIdOffset: flatbuffers.Offset, imageIdOffset: flatbuffers.Offset, callbackProgramIdOffset: flatbuffers.Offset, callbackInstructionPrefixOffset: flatbuffers.Offset, forwardOutput: boolean, verifyInputHash: boolean, inputOffset: flatbuffers.Offset, inputDigestOffset: flatbuffers.Offset, maxBlockHeight: bigint, callbackExtraAccountsOffset: flatbuffers.Offset): flatbuffers.Offset;
}
