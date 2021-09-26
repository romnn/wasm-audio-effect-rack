import * as jspb from "google-protobuf"

export class LightStrip extends jspb.Message {
  getNumLights(): number;
  setNumLights(value: number): LightStrip;

  getPin(): number;
  setPin(value: number): LightStrip;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LightStrip.AsObject;
  static toObject(includeInstance: boolean, msg: LightStrip): LightStrip.AsObject;
  static serializeBinaryToWriter(message: LightStrip, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LightStrip;
  static deserializeBinaryFromReader(message: LightStrip, reader: jspb.BinaryReader): LightStrip;
}

export namespace LightStrip {
  export type AsObject = {
    numLights: number,
    pin: number,
  }
}

export class Lights extends jspb.Message {
  getSerialPort(): string;
  setSerialPort(value: string): Lights;

  getStripsList(): Array<LightStrip>;
  setStripsList(value: Array<LightStrip>): Lights;
  clearStripsList(): Lights;
  addStrips(value?: LightStrip, index?: number): LightStrip;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Lights.AsObject;
  static toObject(includeInstance: boolean, msg: Lights): Lights.AsObject;
  static serializeBinaryToWriter(message: Lights, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Lights;
  static deserializeBinaryFromReader(message: Lights, reader: jspb.BinaryReader): Lights;
}

export namespace Lights {
  export type AsObject = {
    serialPort: string,
    stripsList: Array<LightStrip.AsObject>,
  }
}

export class Visualization extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Visualization.AsObject;
  static toObject(includeInstance: boolean, msg: Visualization): Visualization.AsObject;
  static serializeBinaryToWriter(message: Visualization, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Visualization;
  static deserializeBinaryFromReader(message: Visualization, reader: jspb.BinaryReader): Visualization;
}

export namespace Visualization {
  export type AsObject = {
  }
}

export class QueryCurrentVisualizationRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): QueryCurrentVisualizationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: QueryCurrentVisualizationRequest): QueryCurrentVisualizationRequest.AsObject;
  static serializeBinaryToWriter(message: QueryCurrentVisualizationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): QueryCurrentVisualizationRequest;
  static deserializeBinaryFromReader(message: QueryCurrentVisualizationRequest, reader: jspb.BinaryReader): QueryCurrentVisualizationRequest;
}

export namespace QueryCurrentVisualizationRequest {
  export type AsObject = {
  }
}

export class RegisterVisualizationRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RegisterVisualizationRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RegisterVisualizationRequest): RegisterVisualizationRequest.AsObject;
  static serializeBinaryToWriter(message: RegisterVisualizationRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RegisterVisualizationRequest;
  static deserializeBinaryFromReader(message: RegisterVisualizationRequest, reader: jspb.BinaryReader): RegisterVisualizationRequest;
}

export namespace RegisterVisualizationRequest {
  export type AsObject = {
  }
}

export class Empty extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Empty.AsObject;
  static toObject(includeInstance: boolean, msg: Empty): Empty.AsObject;
  static serializeBinaryToWriter(message: Empty, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Empty;
  static deserializeBinaryFromReader(message: Empty, reader: jspb.BinaryReader): Empty;
}

export namespace Empty {
  export type AsObject = {
  }
}

