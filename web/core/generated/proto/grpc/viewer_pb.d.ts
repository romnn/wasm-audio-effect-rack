import * as jspb from "google-protobuf"

import * as proto_audio_analysis_analysis_pb from '../../proto/audio/analysis/analysis_pb';
import * as proto_grpc_connection_pb from '../../proto/grpc/connection_pb';
import * as proto_grpc_session_pb from '../../proto/grpc/session_pb';
import * as proto_grpc_remote_pb from '../../proto/grpc/remote_pb';

export class ViewerUpdate extends jspb.Message {
  getHeartbeat(): proto_grpc_connection_pb.Heartbeat | undefined;
  setHeartbeat(value?: proto_grpc_connection_pb.Heartbeat): ViewerUpdate;
  hasHeartbeat(): boolean;
  clearHeartbeat(): ViewerUpdate;

  getAssignment(): proto_grpc_session_pb.Assignment | undefined;
  setAssignment(value?: proto_grpc_session_pb.Assignment): ViewerUpdate;
  hasAssignment(): boolean;
  clearAssignment(): ViewerUpdate;

  getAudioAnalysisResult(): proto_audio_analysis_analysis_pb.AudioAnalysisResult | undefined;
  setAudioAnalysisResult(value?: proto_audio_analysis_analysis_pb.AudioAnalysisResult): ViewerUpdate;
  hasAudioAnalysisResult(): boolean;
  clearAudioAnalysisResult(): ViewerUpdate;

  getUpdateCase(): ViewerUpdate.UpdateCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ViewerUpdate.AsObject;
  static toObject(includeInstance: boolean, msg: ViewerUpdate): ViewerUpdate.AsObject;
  static serializeBinaryToWriter(message: ViewerUpdate, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ViewerUpdate;
  static deserializeBinaryFromReader(message: ViewerUpdate, reader: jspb.BinaryReader): ViewerUpdate;
}

export namespace ViewerUpdate {
  export type AsObject = {
    heartbeat?: proto_grpc_connection_pb.Heartbeat.AsObject,
    assignment?: proto_grpc_session_pb.Assignment.AsObject,
    audioAnalysisResult?: proto_audio_analysis_analysis_pb.AudioAnalysisResult.AsObject,
  }

  export enum UpdateCase { 
    UPDATE_NOT_SET = 0,
    HEARTBEAT = 1,
    ASSIGNMENT = 2,
    AUDIO_ANALYSIS_RESULT = 100,
  }
}

export class UpdateSubscriptionRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateSubscriptionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateSubscriptionRequest): UpdateSubscriptionRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateSubscriptionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateSubscriptionRequest;
  static deserializeBinaryFromReader(message: UpdateSubscriptionRequest, reader: jspb.BinaryReader): UpdateSubscriptionRequest;
}

export namespace UpdateSubscriptionRequest {
  export type AsObject = {
  }
}

export class ViewerConnectRequest extends jspb.Message {
  getInstance(): proto_grpc_session_pb.InstanceId | undefined;
  setInstance(value?: proto_grpc_session_pb.InstanceId): ViewerConnectRequest;
  hasInstance(): boolean;
  clearInstance(): ViewerConnectRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ViewerConnectRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ViewerConnectRequest): ViewerConnectRequest.AsObject;
  static serializeBinaryToWriter(message: ViewerConnectRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ViewerConnectRequest;
  static deserializeBinaryFromReader(message: ViewerConnectRequest, reader: jspb.BinaryReader): ViewerConnectRequest;
}

export namespace ViewerConnectRequest {
  export type AsObject = {
    instance?: proto_grpc_session_pb.InstanceId.AsObject,
  }
}

export class ViewerDisconnectRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ViewerDisconnectRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ViewerDisconnectRequest): ViewerDisconnectRequest.AsObject;
  static serializeBinaryToWriter(message: ViewerDisconnectRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ViewerDisconnectRequest;
  static deserializeBinaryFromReader(message: ViewerDisconnectRequest, reader: jspb.BinaryReader): ViewerDisconnectRequest;
}

export namespace ViewerDisconnectRequest {
  export type AsObject = {
  }
}

