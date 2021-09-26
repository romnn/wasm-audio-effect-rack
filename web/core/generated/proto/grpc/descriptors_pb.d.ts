import * as jspb from "google-protobuf"

export class AudioInputDescriptor extends jspb.Message {
  getBackend(): string;
  setBackend(value: string): AudioInputDescriptor;

  getDevice(): string;
  setDevice(value: string): AudioInputDescriptor;

  getHost(): string;
  setHost(value: string): AudioInputDescriptor;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AudioInputDescriptor.AsObject;
  static toObject(includeInstance: boolean, msg: AudioInputDescriptor): AudioInputDescriptor.AsObject;
  static serializeBinaryToWriter(message: AudioInputDescriptor, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AudioInputDescriptor;
  static deserializeBinaryFromReader(message: AudioInputDescriptor, reader: jspb.BinaryReader): AudioInputDescriptor;
}

export namespace AudioInputDescriptor {
  export type AsObject = {
    backend: string,
    device: string,
    host: string,
  }
}

export class AudioAnalyzerDescriptor extends jspb.Message {
  getName(): string;
  setName(value: string): AudioAnalyzerDescriptor;

  getInput(): AudioInputDescriptor | undefined;
  setInput(value?: AudioInputDescriptor): AudioAnalyzerDescriptor;
  hasInput(): boolean;
  clearInput(): AudioAnalyzerDescriptor;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AudioAnalyzerDescriptor.AsObject;
  static toObject(includeInstance: boolean, msg: AudioAnalyzerDescriptor): AudioAnalyzerDescriptor.AsObject;
  static serializeBinaryToWriter(message: AudioAnalyzerDescriptor, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AudioAnalyzerDescriptor;
  static deserializeBinaryFromReader(message: AudioAnalyzerDescriptor, reader: jspb.BinaryReader): AudioAnalyzerDescriptor;
}

export namespace AudioAnalyzerDescriptor {
  export type AsObject = {
    name: string,
    input?: AudioInputDescriptor.AsObject,
  }
}

export class AudioOutputDescriptor extends jspb.Message {
  getBackend(): string;
  setBackend(value: string): AudioOutputDescriptor;

  getDevice(): string;
  setDevice(value: string): AudioOutputDescriptor;

  getHost(): string;
  setHost(value: string): AudioOutputDescriptor;

  getInput(): AudioInputDescriptor | undefined;
  setInput(value?: AudioInputDescriptor): AudioOutputDescriptor;
  hasInput(): boolean;
  clearInput(): AudioOutputDescriptor;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AudioOutputDescriptor.AsObject;
  static toObject(includeInstance: boolean, msg: AudioOutputDescriptor): AudioOutputDescriptor.AsObject;
  static serializeBinaryToWriter(message: AudioOutputDescriptor, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AudioOutputDescriptor;
  static deserializeBinaryFromReader(message: AudioOutputDescriptor, reader: jspb.BinaryReader): AudioOutputDescriptor;
}

export namespace AudioOutputDescriptor {
  export type AsObject = {
    backend: string,
    device: string,
    host: string,
    input?: AudioInputDescriptor.AsObject,
  }
}

export class AudioInputStream extends jspb.Message {
  getDescriptor(): AudioInputDescriptor | undefined;
  setDescriptor(value?: AudioInputDescriptor): AudioInputStream;
  hasDescriptor(): boolean;
  clearDescriptor(): AudioInputStream;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AudioInputStream.AsObject;
  static toObject(includeInstance: boolean, msg: AudioInputStream): AudioInputStream.AsObject;
  static serializeBinaryToWriter(message: AudioInputStream, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AudioInputStream;
  static deserializeBinaryFromReader(message: AudioInputStream, reader: jspb.BinaryReader): AudioInputStream;
}

export namespace AudioInputStream {
  export type AsObject = {
    descriptor?: AudioInputDescriptor.AsObject,
  }
}

export class AudioAnalyzer extends jspb.Message {
  getDescriptor(): AudioAnalyzerDescriptor | undefined;
  setDescriptor(value?: AudioAnalyzerDescriptor): AudioAnalyzer;
  hasDescriptor(): boolean;
  clearDescriptor(): AudioAnalyzer;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AudioAnalyzer.AsObject;
  static toObject(includeInstance: boolean, msg: AudioAnalyzer): AudioAnalyzer.AsObject;
  static serializeBinaryToWriter(message: AudioAnalyzer, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AudioAnalyzer;
  static deserializeBinaryFromReader(message: AudioAnalyzer, reader: jspb.BinaryReader): AudioAnalyzer;
}

export namespace AudioAnalyzer {
  export type AsObject = {
    descriptor?: AudioAnalyzerDescriptor.AsObject,
  }
}

export class AudioOutputStream extends jspb.Message {
  getDescriptor(): AudioOutputDescriptor | undefined;
  setDescriptor(value?: AudioOutputDescriptor): AudioOutputStream;
  hasDescriptor(): boolean;
  clearDescriptor(): AudioOutputStream;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AudioOutputStream.AsObject;
  static toObject(includeInstance: boolean, msg: AudioOutputStream): AudioOutputStream.AsObject;
  static serializeBinaryToWriter(message: AudioOutputStream, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AudioOutputStream;
  static deserializeBinaryFromReader(message: AudioOutputStream, reader: jspb.BinaryReader): AudioOutputStream;
}

export namespace AudioOutputStream {
  export type AsObject = {
    descriptor?: AudioOutputDescriptor.AsObject,
  }
}

