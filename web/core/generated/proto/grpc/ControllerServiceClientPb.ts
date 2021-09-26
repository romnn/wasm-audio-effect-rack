/**
 * @fileoverview gRPC-Web generated client stub for proto.grpc
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck


import * as grpcWeb from 'grpc-web';

import * as proto_audio_analysis_analysis_pb from '../../proto/audio/analysis/analysis_pb';
import * as proto_grpc_connection_pb from '../../proto/grpc/connection_pb';
import * as proto_grpc_session_pb from '../../proto/grpc/session_pb';
import * as proto_grpc_descriptors_pb from '../../proto/grpc/descriptors_pb';
import * as proto_grpc_remote_pb from '../../proto/grpc/remote_pb';

import {
  AddAudioAnalyzerRequest,
  AddAudioInputStreamRequest,
  AddAudioOutputStreamRequest,
  ConnectLightsToAudioAnalyzerRequest,
  ControllerConnectRequest,
  ControllerDisconnectRequest,
  ControllerUpdate,
  GetSessionsRequest,
  SubscribeToAudioAnalyzerRequest} from './controller_pb';

export class RemoteControllerClient {
  client_: grpcWeb.AbstractClientBase;
  hostname_: string;
  credentials_: null | { [index: string]: string; };
  options_: null | { [index: string]: string; };

  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: string; }) {
    if (!options) options = {};
    if (!credentials) credentials = {};
    options['format'] = 'text';

    this.client_ = new grpcWeb.GrpcWebClientBase(options);
    this.hostname_ = hostname;
    this.credentials_ = credentials;
    this.options_ = options;
  }

  methodInfoConnect = new grpcWeb.AbstractClientBase.MethodInfo(
    ControllerUpdate,
    (request: ControllerConnectRequest) => {
      return request.serializeBinary();
    },
    ControllerUpdate.deserializeBinary
  );

  connect(
    request: ControllerConnectRequest,
    metadata?: grpcWeb.Metadata) {
    return this.client_.serverStreaming(
      new URL('/proto.grpc.RemoteController/Connect', this.hostname_).toString(),
      request,
      metadata || {},
      this.methodInfoConnect);
  }

  methodInfoDisconnect = new grpcWeb.AbstractClientBase.MethodInfo(
    proto_grpc_remote_pb.Empty,
    (request: ControllerDisconnectRequest) => {
      return request.serializeBinary();
    },
    proto_grpc_remote_pb.Empty.deserializeBinary
  );

  disconnect(
    request: ControllerDisconnectRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_grpc_remote_pb.Empty>;

  disconnect(
    request: ControllerDisconnectRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: proto_grpc_remote_pb.Empty) => void): grpcWeb.ClientReadableStream<proto_grpc_remote_pb.Empty>;

  disconnect(
    request: ControllerDisconnectRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.Error,
               response: proto_grpc_remote_pb.Empty) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        new URL('/proto.grpc.RemoteController/Disconnect', this.hostname_).toString(),
        request,
        metadata || {},
        this.methodInfoDisconnect,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/proto.grpc.RemoteController/Disconnect',
    request,
    metadata || {},
    this.methodInfoDisconnect);
  }

  methodInfoAddAudioInputStream = new grpcWeb.AbstractClientBase.MethodInfo(
    proto_grpc_descriptors_pb.AudioInputStream,
    (request: AddAudioInputStreamRequest) => {
      return request.serializeBinary();
    },
    proto_grpc_descriptors_pb.AudioInputStream.deserializeBinary
  );

  addAudioInputStream(
    request: AddAudioInputStreamRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_grpc_descriptors_pb.AudioInputStream>;

  addAudioInputStream(
    request: AddAudioInputStreamRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: proto_grpc_descriptors_pb.AudioInputStream) => void): grpcWeb.ClientReadableStream<proto_grpc_descriptors_pb.AudioInputStream>;

  addAudioInputStream(
    request: AddAudioInputStreamRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.Error,
               response: proto_grpc_descriptors_pb.AudioInputStream) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        new URL('/proto.grpc.RemoteController/AddAudioInputStream', this.hostname_).toString(),
        request,
        metadata || {},
        this.methodInfoAddAudioInputStream,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/proto.grpc.RemoteController/AddAudioInputStream',
    request,
    metadata || {},
    this.methodInfoAddAudioInputStream);
  }

  methodInfoAddAudioAnalyzer = new grpcWeb.AbstractClientBase.MethodInfo(
    proto_grpc_descriptors_pb.AudioAnalyzer,
    (request: AddAudioAnalyzerRequest) => {
      return request.serializeBinary();
    },
    proto_grpc_descriptors_pb.AudioAnalyzer.deserializeBinary
  );

  addAudioAnalyzer(
    request: AddAudioAnalyzerRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_grpc_descriptors_pb.AudioAnalyzer>;

  addAudioAnalyzer(
    request: AddAudioAnalyzerRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: proto_grpc_descriptors_pb.AudioAnalyzer) => void): grpcWeb.ClientReadableStream<proto_grpc_descriptors_pb.AudioAnalyzer>;

  addAudioAnalyzer(
    request: AddAudioAnalyzerRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.Error,
               response: proto_grpc_descriptors_pb.AudioAnalyzer) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        new URL('/proto.grpc.RemoteController/AddAudioAnalyzer', this.hostname_).toString(),
        request,
        metadata || {},
        this.methodInfoAddAudioAnalyzer,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/proto.grpc.RemoteController/AddAudioAnalyzer',
    request,
    metadata || {},
    this.methodInfoAddAudioAnalyzer);
  }

  methodInfoAddAudioOutputStream = new grpcWeb.AbstractClientBase.MethodInfo(
    proto_grpc_descriptors_pb.AudioOutputStream,
    (request: AddAudioOutputStreamRequest) => {
      return request.serializeBinary();
    },
    proto_grpc_descriptors_pb.AudioOutputStream.deserializeBinary
  );

  addAudioOutputStream(
    request: AddAudioOutputStreamRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_grpc_descriptors_pb.AudioOutputStream>;

  addAudioOutputStream(
    request: AddAudioOutputStreamRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: proto_grpc_descriptors_pb.AudioOutputStream) => void): grpcWeb.ClientReadableStream<proto_grpc_descriptors_pb.AudioOutputStream>;

  addAudioOutputStream(
    request: AddAudioOutputStreamRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.Error,
               response: proto_grpc_descriptors_pb.AudioOutputStream) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        new URL('/proto.grpc.RemoteController/AddAudioOutputStream', this.hostname_).toString(),
        request,
        metadata || {},
        this.methodInfoAddAudioOutputStream,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/proto.grpc.RemoteController/AddAudioOutputStream',
    request,
    metadata || {},
    this.methodInfoAddAudioOutputStream);
  }

  methodInfoSubscribeToAudioAnalyzer = new grpcWeb.AbstractClientBase.MethodInfo(
    proto_grpc_session_pb.InstanceSubscriptions,
    (request: SubscribeToAudioAnalyzerRequest) => {
      return request.serializeBinary();
    },
    proto_grpc_session_pb.InstanceSubscriptions.deserializeBinary
  );

  subscribeToAudioAnalyzer(
    request: SubscribeToAudioAnalyzerRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_grpc_session_pb.InstanceSubscriptions>;

  subscribeToAudioAnalyzer(
    request: SubscribeToAudioAnalyzerRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: proto_grpc_session_pb.InstanceSubscriptions) => void): grpcWeb.ClientReadableStream<proto_grpc_session_pb.InstanceSubscriptions>;

  subscribeToAudioAnalyzer(
    request: SubscribeToAudioAnalyzerRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.Error,
               response: proto_grpc_session_pb.InstanceSubscriptions) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        new URL('/proto.grpc.RemoteController/SubscribeToAudioAnalyzer', this.hostname_).toString(),
        request,
        metadata || {},
        this.methodInfoSubscribeToAudioAnalyzer,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/proto.grpc.RemoteController/SubscribeToAudioAnalyzer',
    request,
    metadata || {},
    this.methodInfoSubscribeToAudioAnalyzer);
  }

  methodInfoConnectLightsToAudioAnalyzer = new grpcWeb.AbstractClientBase.MethodInfo(
    proto_grpc_session_pb.InstanceSubscriptions,
    (request: ConnectLightsToAudioAnalyzerRequest) => {
      return request.serializeBinary();
    },
    proto_grpc_session_pb.InstanceSubscriptions.deserializeBinary
  );

  connectLightsToAudioAnalyzer(
    request: ConnectLightsToAudioAnalyzerRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_grpc_session_pb.InstanceSubscriptions>;

  connectLightsToAudioAnalyzer(
    request: ConnectLightsToAudioAnalyzerRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: proto_grpc_session_pb.InstanceSubscriptions) => void): grpcWeb.ClientReadableStream<proto_grpc_session_pb.InstanceSubscriptions>;

  connectLightsToAudioAnalyzer(
    request: ConnectLightsToAudioAnalyzerRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.Error,
               response: proto_grpc_session_pb.InstanceSubscriptions) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        new URL('/proto.grpc.RemoteController/ConnectLightsToAudioAnalyzer', this.hostname_).toString(),
        request,
        metadata || {},
        this.methodInfoConnectLightsToAudioAnalyzer,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/proto.grpc.RemoteController/ConnectLightsToAudioAnalyzer',
    request,
    metadata || {},
    this.methodInfoConnectLightsToAudioAnalyzer);
  }

  methodInfoGetSessions = new grpcWeb.AbstractClientBase.MethodInfo(
    proto_grpc_session_pb.Sessions,
    (request: GetSessionsRequest) => {
      return request.serializeBinary();
    },
    proto_grpc_session_pb.Sessions.deserializeBinary
  );

  getSessions(
    request: GetSessionsRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_grpc_session_pb.Sessions>;

  getSessions(
    request: GetSessionsRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: proto_grpc_session_pb.Sessions) => void): grpcWeb.ClientReadableStream<proto_grpc_session_pb.Sessions>;

  getSessions(
    request: GetSessionsRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.Error,
               response: proto_grpc_session_pb.Sessions) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        new URL('/proto.grpc.RemoteController/GetSessions', this.hostname_).toString(),
        request,
        metadata || {},
        this.methodInfoGetSessions,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/proto.grpc.RemoteController/GetSessions',
    request,
    metadata || {},
    this.methodInfoGetSessions);
  }

}

