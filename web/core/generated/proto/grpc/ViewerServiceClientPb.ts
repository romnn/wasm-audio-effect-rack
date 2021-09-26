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
import * as proto_grpc_remote_pb from '../../proto/grpc/remote_pb';

import {
  UpdateSubscriptionRequest,
  ViewerConnectRequest,
  ViewerDisconnectRequest,
  ViewerUpdate} from './viewer_pb';

export class RemoteViewerClient {
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
    ViewerUpdate,
    (request: ViewerConnectRequest) => {
      return request.serializeBinary();
    },
    ViewerUpdate.deserializeBinary
  );

  connect(
    request: ViewerConnectRequest,
    metadata?: grpcWeb.Metadata) {
    return this.client_.serverStreaming(
      new URL('/proto.grpc.RemoteViewer/Connect', this.hostname_).toString(),
      request,
      metadata || {},
      this.methodInfoConnect);
  }

  methodInfoDisconnect = new grpcWeb.AbstractClientBase.MethodInfo(
    proto_grpc_remote_pb.Empty,
    (request: ViewerDisconnectRequest) => {
      return request.serializeBinary();
    },
    proto_grpc_remote_pb.Empty.deserializeBinary
  );

  disconnect(
    request: ViewerDisconnectRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_grpc_remote_pb.Empty>;

  disconnect(
    request: ViewerDisconnectRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: proto_grpc_remote_pb.Empty) => void): grpcWeb.ClientReadableStream<proto_grpc_remote_pb.Empty>;

  disconnect(
    request: ViewerDisconnectRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.Error,
               response: proto_grpc_remote_pb.Empty) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        new URL('/proto.grpc.RemoteViewer/Disconnect', this.hostname_).toString(),
        request,
        metadata || {},
        this.methodInfoDisconnect,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/proto.grpc.RemoteViewer/Disconnect',
    request,
    metadata || {},
    this.methodInfoDisconnect);
  }

  methodInfoUpdateSubscription = new grpcWeb.AbstractClientBase.MethodInfo(
    proto_grpc_remote_pb.Empty,
    (request: UpdateSubscriptionRequest) => {
      return request.serializeBinary();
    },
    proto_grpc_remote_pb.Empty.deserializeBinary
  );

  updateSubscription(
    request: UpdateSubscriptionRequest,
    metadata: grpcWeb.Metadata | null): Promise<proto_grpc_remote_pb.Empty>;

  updateSubscription(
    request: UpdateSubscriptionRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.Error,
               response: proto_grpc_remote_pb.Empty) => void): grpcWeb.ClientReadableStream<proto_grpc_remote_pb.Empty>;

  updateSubscription(
    request: UpdateSubscriptionRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.Error,
               response: proto_grpc_remote_pb.Empty) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        new URL('/proto.grpc.RemoteViewer/UpdateSubscription', this.hostname_).toString(),
        request,
        metadata || {},
        this.methodInfoUpdateSubscription,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/proto.grpc.RemoteViewer/UpdateSubscription',
    request,
    metadata || {},
    this.methodInfoUpdateSubscription);
  }

}

