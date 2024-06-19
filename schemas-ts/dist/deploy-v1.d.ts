import * as flatbuffers from 'flatbuffers';
import { ProgramInputType } from './program-input-type.js';
export declare class DeployV1 {
    bb: flatbuffers.ByteBuffer | null;
    bb_pos: number;
    __init(i: number, bb: flatbuffers.ByteBuffer): DeployV1;
    static getRootAsDeployV1(bb: flatbuffers.ByteBuffer, obj?: DeployV1): DeployV1;
    static getSizePrefixedRootAsDeployV1(bb: flatbuffers.ByteBuffer, obj?: DeployV1): DeployV1;
    owner(index: number): number | null;
    ownerLength(): number;
    ownerArray(): Uint8Array | null;
    imageId(): string | null;
    imageId(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
    programName(): string | null;
    programName(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
    url(): string | null;
    url(optionalEncoding: flatbuffers.Encoding): string | Uint8Array | null;
    size(): bigint;
    mutate_size(value: bigint): boolean;
    inputs(index: number): ProgramInputType | null;
    inputsLength(): number;
    inputsArray(): Uint8Array | null;
    static startDeployV1(builder: flatbuffers.Builder): void;
    static addOwner(builder: flatbuffers.Builder, ownerOffset: flatbuffers.Offset): void;
    static createOwnerVector(builder: flatbuffers.Builder, data: number[] | ReadonlyUint8Array): flatbuffers.Offset;
    static startOwnerVector(builder: flatbuffers.Builder, numElems: number): void;
    static addImageId(builder: flatbuffers.Builder, imageIdOffset: flatbuffers.Offset): void;
    static addProgramName(builder: flatbuffers.Builder, programNameOffset: flatbuffers.Offset): void;
    static addUrl(builder: flatbuffers.Builder, urlOffset: flatbuffers.Offset): void;
    static addSize(builder: flatbuffers.Builder, size: bigint): void;
    static addInputs(builder: flatbuffers.Builder, inputsOffset: flatbuffers.Offset): void;
    static createInputsVector(builder: flatbuffers.Builder, data: ProgramInputType[]): flatbuffers.Offset;
    static startInputsVector(builder: flatbuffers.Builder, numElems: number): void;
    static endDeployV1(builder: flatbuffers.Builder): flatbuffers.Offset;
    static finishDeployV1Buffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
    static finishSizePrefixedDeployV1Buffer(builder: flatbuffers.Builder, offset: flatbuffers.Offset): void;
    static createDeployV1(builder: flatbuffers.Builder, ownerOffset: flatbuffers.Offset, imageIdOffset: flatbuffers.Offset, programNameOffset: flatbuffers.Offset, urlOffset: flatbuffers.Offset, size: bigint, inputsOffset: flatbuffers.Offset): flatbuffers.Offset;
}
