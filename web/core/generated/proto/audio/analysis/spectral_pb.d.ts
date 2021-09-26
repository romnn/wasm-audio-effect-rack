import * as jspb from "google-protobuf"

export class SpectralAudioAnalyzerConfig extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SpectralAudioAnalyzerConfig.AsObject;
  static toObject(includeInstance: boolean, msg: SpectralAudioAnalyzerConfig): SpectralAudioAnalyzerConfig.AsObject;
  static serializeBinaryToWriter(message: SpectralAudioAnalyzerConfig, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SpectralAudioAnalyzerConfig;
  static deserializeBinaryFromReader(message: SpectralAudioAnalyzerConfig, reader: jspb.BinaryReader): SpectralAudioAnalyzerConfig;
}

export namespace SpectralAudioAnalyzerConfig {
  export type AsObject = {
  }
}

export class SpectralAudioAnalyzer extends jspb.Message {
  getConfig(): SpectralAudioAnalyzerConfig | undefined;
  setConfig(value?: SpectralAudioAnalyzerConfig): SpectralAudioAnalyzer;
  hasConfig(): boolean;
  clearConfig(): SpectralAudioAnalyzer;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SpectralAudioAnalyzer.AsObject;
  static toObject(includeInstance: boolean, msg: SpectralAudioAnalyzer): SpectralAudioAnalyzer.AsObject;
  static serializeBinaryToWriter(message: SpectralAudioAnalyzer, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SpectralAudioAnalyzer;
  static deserializeBinaryFromReader(message: SpectralAudioAnalyzer, reader: jspb.BinaryReader): SpectralAudioAnalyzer;
}

export namespace SpectralAudioAnalyzer {
  export type AsObject = {
    config?: SpectralAudioAnalyzerConfig.AsObject,
  }
}

export class SpectralAudioAnalysisResult extends jspb.Message {
  getNumMelBands(): number;
  setNumMelBands(value: number): SpectralAudioAnalysisResult;

  getVolume(): number;
  setVolume(value: number): SpectralAudioAnalysisResult;

  getMelBandsList(): Array<number>;
  setMelBandsList(value: Array<number>): SpectralAudioAnalysisResult;
  clearMelBandsList(): SpectralAudioAnalysisResult;
  addMelBands(value: number, index?: number): SpectralAudioAnalysisResult;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SpectralAudioAnalysisResult.AsObject;
  static toObject(includeInstance: boolean, msg: SpectralAudioAnalysisResult): SpectralAudioAnalysisResult.AsObject;
  static serializeBinaryToWriter(message: SpectralAudioAnalysisResult, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SpectralAudioAnalysisResult;
  static deserializeBinaryFromReader(message: SpectralAudioAnalysisResult, reader: jspb.BinaryReader): SpectralAudioAnalysisResult;
}

export namespace SpectralAudioAnalysisResult {
  export type AsObject = {
    numMelBands: number,
    volume: number,
    melBandsList: Array<number>,
  }
}

