import * as jspb from "google-protobuf"

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';

export class Assignment extends jspb.Message {
  getSessionToken(): SessionToken | undefined;
  setSessionToken(value?: SessionToken): Assignment;
  hasSessionToken(): boolean;
  clearSessionToken(): Assignment;

  getInstanceId(): InstanceId | undefined;
  setInstanceId(value?: InstanceId): Assignment;
  hasInstanceId(): boolean;
  clearInstanceId(): Assignment;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Assignment.AsObject;
  static toObject(includeInstance: boolean, msg: Assignment): Assignment.AsObject;
  static serializeBinaryToWriter(message: Assignment, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Assignment;
  static deserializeBinaryFromReader(message: Assignment, reader: jspb.BinaryReader): Assignment;
}

export namespace Assignment {
  export type AsObject = {
    sessionToken?: SessionToken.AsObject,
    instanceId?: InstanceId.AsObject,
  }
}

export class InstanceSubscriptions extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InstanceSubscriptions.AsObject;
  static toObject(includeInstance: boolean, msg: InstanceSubscriptions): InstanceSubscriptions.AsObject;
  static serializeBinaryToWriter(message: InstanceSubscriptions, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InstanceSubscriptions;
  static deserializeBinaryFromReader(message: InstanceSubscriptions, reader: jspb.BinaryReader): InstanceSubscriptions;
}

export namespace InstanceSubscriptions {
  export type AsObject = {
  }
}

export class InstanceId extends jspb.Message {
  getId(): string;
  setId(value: string): InstanceId;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InstanceId.AsObject;
  static toObject(includeInstance: boolean, msg: InstanceId): InstanceId.AsObject;
  static serializeBinaryToWriter(message: InstanceId, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InstanceId;
  static deserializeBinaryFromReader(message: InstanceId, reader: jspb.BinaryReader): InstanceId;
}

export namespace InstanceId {
  export type AsObject = {
    id: string,
  }
}

export class SessionToken extends jspb.Message {
  getToken(): string;
  setToken(value: string): SessionToken;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SessionToken.AsObject;
  static toObject(includeInstance: boolean, msg: SessionToken): SessionToken.AsObject;
  static serializeBinaryToWriter(message: SessionToken, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SessionToken;
  static deserializeBinaryFromReader(message: SessionToken, reader: jspb.BinaryReader): SessionToken;
}

export namespace SessionToken {
  export type AsObject = {
    token: string,
  }
}

export class InstanceInfo extends jspb.Message {
  getSessionToken(): SessionToken | undefined;
  setSessionToken(value?: SessionToken): InstanceInfo;
  hasSessionToken(): boolean;
  clearSessionToken(): InstanceInfo;

  getInstanceId(): InstanceId | undefined;
  setInstanceId(value?: InstanceId): InstanceInfo;
  hasInstanceId(): boolean;
  clearInstanceId(): InstanceInfo;

  getConnectedSince(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setConnectedSince(value?: google_protobuf_timestamp_pb.Timestamp): InstanceInfo;
  hasConnectedSince(): boolean;
  clearConnectedSince(): InstanceInfo;

  getState(): InstanceState;
  setState(value: InstanceState): InstanceInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InstanceInfo.AsObject;
  static toObject(includeInstance: boolean, msg: InstanceInfo): InstanceInfo.AsObject;
  static serializeBinaryToWriter(message: InstanceInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InstanceInfo;
  static deserializeBinaryFromReader(message: InstanceInfo, reader: jspb.BinaryReader): InstanceInfo;
}

export namespace InstanceInfo {
  export type AsObject = {
    sessionToken?: SessionToken.AsObject,
    instanceId?: InstanceId.AsObject,
    connectedSince?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    state: InstanceState,
  }
}

export class ViewerInstanceInfo extends jspb.Message {
  getInfo(): InstanceInfo | undefined;
  setInfo(value?: InstanceInfo): ViewerInstanceInfo;
  hasInfo(): boolean;
  clearInfo(): ViewerInstanceInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ViewerInstanceInfo.AsObject;
  static toObject(includeInstance: boolean, msg: ViewerInstanceInfo): ViewerInstanceInfo.AsObject;
  static serializeBinaryToWriter(message: ViewerInstanceInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ViewerInstanceInfo;
  static deserializeBinaryFromReader(message: ViewerInstanceInfo, reader: jspb.BinaryReader): ViewerInstanceInfo;
}

export namespace ViewerInstanceInfo {
  export type AsObject = {
    info?: InstanceInfo.AsObject,
  }
}

export class ControllerInstanceInfo extends jspb.Message {
  getInfo(): InstanceInfo | undefined;
  setInfo(value?: InstanceInfo): ControllerInstanceInfo;
  hasInfo(): boolean;
  clearInfo(): ControllerInstanceInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ControllerInstanceInfo.AsObject;
  static toObject(includeInstance: boolean, msg: ControllerInstanceInfo): ControllerInstanceInfo.AsObject;
  static serializeBinaryToWriter(message: ControllerInstanceInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ControllerInstanceInfo;
  static deserializeBinaryFromReader(message: ControllerInstanceInfo, reader: jspb.BinaryReader): ControllerInstanceInfo;
}

export namespace ControllerInstanceInfo {
  export type AsObject = {
    info?: InstanceInfo.AsObject,
  }
}

export class SessionInfo extends jspb.Message {
  getToken(): SessionToken | undefined;
  setToken(value?: SessionToken): SessionInfo;
  hasToken(): boolean;
  clearToken(): SessionInfo;

  getViewersList(): Array<ViewerInstanceInfo>;
  setViewersList(value: Array<ViewerInstanceInfo>): SessionInfo;
  clearViewersList(): SessionInfo;
  addViewers(value?: ViewerInstanceInfo, index?: number): ViewerInstanceInfo;

  getControllersList(): Array<ControllerInstanceInfo>;
  setControllersList(value: Array<ControllerInstanceInfo>): SessionInfo;
  clearControllersList(): SessionInfo;
  addControllers(value?: ControllerInstanceInfo, index?: number): ControllerInstanceInfo;

  getStarted(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setStarted(value?: google_protobuf_timestamp_pb.Timestamp): SessionInfo;
  hasStarted(): boolean;
  clearStarted(): SessionInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SessionInfo.AsObject;
  static toObject(includeInstance: boolean, msg: SessionInfo): SessionInfo.AsObject;
  static serializeBinaryToWriter(message: SessionInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SessionInfo;
  static deserializeBinaryFromReader(message: SessionInfo, reader: jspb.BinaryReader): SessionInfo;
}

export namespace SessionInfo {
  export type AsObject = {
    token?: SessionToken.AsObject,
    viewersList: Array<ViewerInstanceInfo.AsObject>,
    controllersList: Array<ControllerInstanceInfo.AsObject>,
    started?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }
}

export class Sessions extends jspb.Message {
  getSessionsList(): Array<SessionInfo>;
  setSessionsList(value: Array<SessionInfo>): Sessions;
  clearSessionsList(): Sessions;
  addSessions(value?: SessionInfo, index?: number): SessionInfo;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Sessions.AsObject;
  static toObject(includeInstance: boolean, msg: Sessions): Sessions.AsObject;
  static serializeBinaryToWriter(message: Sessions, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Sessions;
  static deserializeBinaryFromReader(message: Sessions, reader: jspb.BinaryReader): Sessions;
}

export namespace Sessions {
  export type AsObject = {
    sessionsList: Array<SessionInfo.AsObject>,
  }
}

export enum InstanceState { 
  ONLINE = 0,
  OFFLINE = 1,
  FAILED = 2,
}
