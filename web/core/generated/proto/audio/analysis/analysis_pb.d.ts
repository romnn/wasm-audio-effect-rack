import * as jspb from "google-protobuf"

import * as proto_audio_analysis_spectral_pb from '../../../proto/audio/analysis/spectral_pb';
import * as proto_audio_analysis_bpm_pb from '../../../proto/audio/analysis/bpm_pb';

export class AudioAnalysisResult extends jspb.Message {
  getSeqNum(): number;
  setSeqNum(value: number): AudioAnalysisResult;

  getWindowSize(): number;
  setWindowSize(value: number): AudioAnalysisResult;

  getSpectral(): proto_audio_analysis_spectral_pb.SpectralAudioAnalysisResult | undefined;
  setSpectral(value?: proto_audio_analysis_spectral_pb.SpectralAudioAnalysisResult): AudioAnalysisResult;
  hasSpectral(): boolean;
  clearSpectral(): AudioAnalysisResult;

  getBpm(): proto_audio_analysis_bpm_pb.BpmDetectionAudioAnalysisResult | undefined;
  setBpm(value?: proto_audio_analysis_bpm_pb.BpmDetectionAudioAnalysisResult): AudioAnalysisResult;
  hasBpm(): boolean;
  clearBpm(): AudioAnalysisResult;

  getResultCase(): AudioAnalysisResult.ResultCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AudioAnalysisResult.AsObject;
  static toObject(includeInstance: boolean, msg: AudioAnalysisResult): AudioAnalysisResult.AsObject;
  static serializeBinaryToWriter(message: AudioAnalysisResult, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AudioAnalysisResult;
  static deserializeBinaryFromReader(message: AudioAnalysisResult, reader: jspb.BinaryReader): AudioAnalysisResult;
}

export namespace AudioAnalysisResult {
  export type AsObject = {
    seqNum: number,
    windowSize: number,
    spectral?: proto_audio_analysis_spectral_pb.SpectralAudioAnalysisResult.AsObject,
    bpm?: proto_audio_analysis_bpm_pb.BpmDetectionAudioAnalysisResult.AsObject,
  }

  export enum ResultCase { 
    RESULT_NOT_SET = 0,
    SPECTRAL = 100,
    BPM = 101,
  }
}

export class AudioAnalyzer extends jspb.Message {
  getSpectral(): proto_audio_analysis_spectral_pb.SpectralAudioAnalyzer | undefined;
  setSpectral(value?: proto_audio_analysis_spectral_pb.SpectralAudioAnalyzer): AudioAnalyzer;
  hasSpectral(): boolean;
  clearSpectral(): AudioAnalyzer;

  getBpm(): proto_audio_analysis_bpm_pb.BpmDetectionAudioAnalyzer | undefined;
  setBpm(value?: proto_audio_analysis_bpm_pb.BpmDetectionAudioAnalyzer): AudioAnalyzer;
  hasBpm(): boolean;
  clearBpm(): AudioAnalyzer;

  getAnalyzerCase(): AudioAnalyzer.AnalyzerCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AudioAnalyzer.AsObject;
  static toObject(includeInstance: boolean, msg: AudioAnalyzer): AudioAnalyzer.AsObject;
  static serializeBinaryToWriter(message: AudioAnalyzer, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AudioAnalyzer;
  static deserializeBinaryFromReader(message: AudioAnalyzer, reader: jspb.BinaryReader): AudioAnalyzer;
}

export namespace AudioAnalyzer {
  export type AsObject = {
    spectral?: proto_audio_analysis_spectral_pb.SpectralAudioAnalyzer.AsObject,
    bpm?: proto_audio_analysis_bpm_pb.BpmDetectionAudioAnalyzer.AsObject,
  }

  export enum AnalyzerCase { 
    ANALYZER_NOT_SET = 0,
    SPECTRAL = 1,
    BPM = 2,
  }
}

