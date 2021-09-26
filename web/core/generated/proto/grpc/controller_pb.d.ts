import * as jspb from "google-protobuf"

import * as proto_audio_analysis_analysis_pb from '../../proto/audio/analysis/analysis_pb';
import * as proto_grpc_connection_pb from '../../proto/grpc/connection_pb';
import * as proto_grpc_session_pb from '../../proto/grpc/session_pb';
import * as proto_grpc_descriptors_pb from '../../proto/grpc/descriptors_pb';
import * as proto_grpc_remote_pb from '../../proto/grpc/remote_pb';

export class GetSessionsRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSessionsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetSessionsRequest): GetSessionsRequest.AsObject;
  static serializeBinaryToWriter(message: GetSessionsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSessionsRequest;
  static deserializeBinaryFromReader(message: GetSessionsRequest, reader: jspb.BinaryReader): GetSessionsRequest;
}

export namespace GetSessionsRequest {
  export type AsObject = {
  }
}

export class ControllerUpdate extends jspb.Message {
  getHeartbeat(): proto_grpc_connection_pb.Heartbeat | undefined;
  setHeartbeat(value?: proto_grpc_connection_pb.Heartbeat): ControllerUpdate;
  hasHeartbeat(): boolean;
  clearHeartbeat(): ControllerUpdate;

  getAssignment(): proto_grpc_session_pb.Assignment | undefined;
  setAssignment(value?: proto_grpc_session_pb.Assignment): ControllerUpdate;
  hasAssignment(): boolean;
  clearAssignment(): ControllerUpdate;

  getUpdateCase(): ControllerUpdate.UpdateCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ControllerUpdate.AsObject;
  static toObject(includeInstance: boolean, msg: ControllerUpdate): ControllerUpdate.AsObject;
  static serializeBinaryToWriter(message: ControllerUpdate, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ControllerUpdate;
  static deserializeBinaryFromReader(message: ControllerUpdate, reader: jspb.BinaryReader): ControllerUpdate;
}

export namespace ControllerUpdate {
  export type AsObject = {
    heartbeat?: proto_grpc_connection_pb.Heartbeat.AsObject,
    assignment?: proto_grpc_session_pb.Assignment.AsObject,
  }

  export enum UpdateCase { 
    UPDATE_NOT_SET = 0,
    HEARTBEAT = 1,
    ASSIGNMENT = 2,
  }
}

export class SubscribeToAudioAnalyzerRequest extends jspb.Message {
  getInstanceId(): proto_grpc_session_pb.InstanceId | undefined;
  setInstanceId(value?: proto_grpc_session_pb.InstanceId): SubscribeToAudioAnalyzerRequest;
  hasInstanceId(): boolean;
  clearInstanceId(): SubscribeToAudioAnalyzerRequest;

  getAudioAnalyzerDescriptor(): proto_grpc_descriptors_pb.AudioAnalyzerDescriptor | undefined;
  setAudioAnalyzerDescriptor(value?: proto_grpc_descriptors_pb.AudioAnalyzerDescriptor): SubscribeToAudioAnalyzerRequest;
  hasAudioAnalyzerDescriptor(): boolean;
  clearAudioAnalyzerDescriptor(): SubscribeToAudioAnalyzerRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SubscribeToAudioAnalyzerRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SubscribeToAudioAnalyzerRequest): SubscribeToAudioAnalyzerRequest.AsObject;
  static serializeBinaryToWriter(message: SubscribeToAudioAnalyzerRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SubscribeToAudioAnalyzerRequest;
  static deserializeBinaryFromReader(message: SubscribeToAudioAnalyzerRequest, reader: jspb.BinaryReader): SubscribeToAudioAnalyzerRequest;
}

export namespace SubscribeToAudioAnalyzerRequest {
  export type AsObject = {
    instanceId?: proto_grpc_session_pb.InstanceId.AsObject,
    audioAnalyzerDescriptor?: proto_grpc_descriptors_pb.AudioAnalyzerDescriptor.AsObject,
  }
}

export class ConnectLightsToAudioAnalyzerRequest extends jspb.Message {
  getLights(): proto_grpc_remote_pb.Lights | undefined;
  setLights(value?: proto_grpc_remote_pb.Lights): ConnectLightsToAudioAnalyzerRequest;
  hasLights(): boolean;
  clearLights(): ConnectLightsToAudioAnalyzerRequest;

  getAudioAnalyzerDescriptor(): proto_grpc_descriptors_pb.AudioAnalyzerDescriptor | undefined;
  setAudioAnalyzerDescriptor(value?: proto_grpc_descriptors_pb.AudioAnalyzerDescriptor): ConnectLightsToAudioAnalyzerRequest;
  hasAudioAnalyzerDescriptor(): boolean;
  clearAudioAnalyzerDescriptor(): ConnectLightsToAudioAnalyzerRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ConnectLightsToAudioAnalyzerRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ConnectLightsToAudioAnalyzerRequest): ConnectLightsToAudioAnalyzerRequest.AsObject;
  static serializeBinaryToWriter(message: ConnectLightsToAudioAnalyzerRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ConnectLightsToAudioAnalyzerRequest;
  static deserializeBinaryFromReader(message: ConnectLightsToAudioAnalyzerRequest, reader: jspb.BinaryReader): ConnectLightsToAudioAnalyzerRequest;
}

export namespace ConnectLightsToAudioAnalyzerRequest {
  export type AsObject = {
    lights?: proto_grpc_remote_pb.Lights.AsObject,
    audioAnalyzerDescriptor?: proto_grpc_descriptors_pb.AudioAnalyzerDescriptor.AsObject,
  }
}

export class AddAudioInputStreamRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AddAudioInputStreamRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AddAudioInputStreamRequest): AddAudioInputStreamRequest.AsObject;
  static serializeBinaryToWriter(message: AddAudioInputStreamRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AddAudioInputStreamRequest;
  static deserializeBinaryFromReader(message: AddAudioInputStreamRequest, reader: jspb.BinaryReader): AddAudioInputStreamRequest;
}

export namespace AddAudioInputStreamRequest {
  export type AsObject = {
  }
}

export class AddAudioAnalyzerRequest extends jspb.Message {
  getAnalyzer(): proto_audio_analysis_analysis_pb.AudioAnalyzer | undefined;
  setAnalyzer(value?: proto_audio_analysis_analysis_pb.AudioAnalyzer): AddAudioAnalyzerRequest;
  hasAnalyzer(): boolean;
  clearAnalyzer(): AddAudioAnalyzerRequest;

  getInputDescriptor(): proto_grpc_descriptors_pb.AudioInputDescriptor | undefined;
  setInputDescriptor(value?: proto_grpc_descriptors_pb.AudioInputDescriptor): AddAudioAnalyzerRequest;
  hasInputDescriptor(): boolean;
  clearInputDescriptor(): AddAudioAnalyzerRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AddAudioAnalyzerRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AddAudioAnalyzerRequest): AddAudioAnalyzerRequest.AsObject;
  static serializeBinaryToWriter(message: AddAudioAnalyzerRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AddAudioAnalyzerRequest;
  static deserializeBinaryFromReader(message: AddAudioAnalyzerRequest, reader: jspb.BinaryReader): AddAudioAnalyzerRequest;
}

export namespace AddAudioAnalyzerRequest {
  export type AsObject = {
    analyzer?: proto_audio_analysis_analysis_pb.AudioAnalyzer.AsObject,
    inputDescriptor?: proto_grpc_descriptors_pb.AudioInputDescriptor.AsObject,
  }
}

export class AddAudioOutputStreamRequest extends jspb.Message {
  getInputDescriptor(): proto_grpc_descriptors_pb.AudioInputDescriptor | undefined;
  setInputDescriptor(value?: proto_grpc_descriptors_pb.AudioInputDescriptor): AddAudioOutputStreamRequest;
  hasInputDescriptor(): boolean;
  clearInputDescriptor(): AddAudioOutputStreamRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AddAudioOutputStreamRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AddAudioOutputStreamRequest): AddAudioOutputStreamRequest.AsObject;
  static serializeBinaryToWriter(message: AddAudioOutputStreamRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AddAudioOutputStreamRequest;
  static deserializeBinaryFromReader(message: AddAudioOutputStreamRequest, reader: jspb.BinaryReader): AddAudioOutputStreamRequest;
}

export namespace AddAudioOutputStreamRequest {
  export type AsObject = {
    inputDescriptor?: proto_grpc_descriptors_pb.AudioInputDescriptor.AsObject,
  }
}

export class ControllerConnectRequest extends jspb.Message {
  getInstance(): proto_grpc_session_pb.InstanceId | undefined;
  setInstance(value?: proto_grpc_session_pb.InstanceId): ControllerConnectRequest;
  hasInstance(): boolean;
  clearInstance(): ControllerConnectRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ControllerConnectRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ControllerConnectRequest): ControllerConnectRequest.AsObject;
  static serializeBinaryToWriter(message: ControllerConnectRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ControllerConnectRequest;
  static deserializeBinaryFromReader(message: ControllerConnectRequest, reader: jspb.BinaryReader): ControllerConnectRequest;
}

export namespace ControllerConnectRequest {
  export type AsObject = {
    instance?: proto_grpc_session_pb.InstanceId.AsObject,
  }
}

export class ControllerDisconnectRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ControllerDisconnectRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ControllerDisconnectRequest): ControllerDisconnectRequest.AsObject;
  static serializeBinaryToWriter(message: ControllerDisconnectRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ControllerDisconnectRequest;
  static deserializeBinaryFromReader(message: ControllerDisconnectRequest, reader: jspb.BinaryReader): ControllerDisconnectRequest;
}

export namespace ControllerDisconnectRequest {
  export type AsObject = {
  }
}

