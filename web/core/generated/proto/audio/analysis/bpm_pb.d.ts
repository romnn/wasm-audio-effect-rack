import * as jspb from "google-protobuf"

export class BpmDetectionAudioAnalyzerConfig extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BpmDetectionAudioAnalyzerConfig.AsObject;
  static toObject(includeInstance: boolean, msg: BpmDetectionAudioAnalyzerConfig): BpmDetectionAudioAnalyzerConfig.AsObject;
  static serializeBinaryToWriter(message: BpmDetectionAudioAnalyzerConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BpmDetectionAudioAnalyzerConfig;
  static deserializeBinaryFromReader(message: BpmDetectionAudioAnalyzerConfig, reader: jspb.BinaryReader): BpmDetectionAudioAnalyzerConfig;
}

export namespace BpmDetectionAudioAnalyzerConfig {
  export type AsObject = {
  }
}

export class BpmDetectionAudioAnalyzer extends jspb.Message {
  getConfig(): BpmDetectionAudioAnalyzerConfig | undefined;
  setConfig(value?: BpmDetectionAudioAnalyzerConfig): BpmDetectionAudioAnalyzer;
  hasConfig(): boolean;
  clearConfig(): BpmDetectionAudioAnalyzer;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BpmDetectionAudioAnalyzer.AsObject;
  static toObject(includeInstance: boolean, msg: BpmDetectionAudioAnalyzer): BpmDetectionAudioAnalyzer.AsObject;
  static serializeBinaryToWriter(message: BpmDetectionAudioAnalyzer, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BpmDetectionAudioAnalyzer;
  static deserializeBinaryFromReader(message: BpmDetectionAudioAnalyzer, reader: jspb.BinaryReader): BpmDetectionAudioAnalyzer;
}

export namespace BpmDetectionAudioAnalyzer {
  export type AsObject = {
    config?: BpmDetectionAudioAnalyzerConfig.AsObject,
  }
}

export class BpmDetectionAudioAnalysisResult extends jspb.Message {
  getBpm(): number;
  setBpm(value: number): BpmDetectionAudioAnalysisResult;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): BpmDetectionAudioAnalysisResult.AsObject;
  static toObject(includeInstance: boolean, msg: BpmDetectionAudioAnalysisResult): BpmDetectionAudioAnalysisResult.AsObject;
  static serializeBinaryToWriter(message: BpmDetectionAudioAnalysisResult, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): BpmDetectionAudioAnalysisResult;
  static deserializeBinaryFromReader(message: BpmDetectionAudioAnalysisResult, reader: jspb.BinaryReader): BpmDetectionAudioAnalysisResult;
}

export namespace BpmDetectionAudioAnalysisResult {
  export type AsObject = {
    bpm: number,
  }
}

