import {ClientReadableStream, Error, Metadata, Status} from "grpc-web";

import {
  ControllerConnectRequest,
  ControllerDisconnectRequest,
  ControllerUpdate,
} from "./generated/proto/grpc/controller_pb";
import {
  RemoteControllerClient,
} from "./generated/proto/grpc/ControllerServiceClientPb";
import {
  InstanceId,
  SessionToken,
} from "./generated/proto/grpc/session_pb";

type SubMessageHandler = (message: ControllerUpdate) => void;
type SubErrorHandler = (error: Error) => void;
type SubStatusHandler = (status: Status) => void;
type SubMetadataHandler = (metadata: Metadata) => void;

export interface RemoteState {}

export interface RemoteControllerConfig {
  onUpdate?: SubMessageHandler;
  onStatus?: SubStatusHandler;
  onError?: SubErrorHandler;
  onMetadata?: SubMetadataHandler;
}

export default class RemoteController {
  protected isConnected = false;
  protected client: RemoteControllerClient;
  protected updateStream?: ClientReadableStream<unknown>;
  public endpoint =
      window.location.protocol + "//" + window.location.hostname + ":9000";

  constructor(session: string|undefined, instance: string|undefined,
              options?: RemoteControllerConfig) {
    this.client = new RemoteControllerClient(this.endpoint, null, {
      unaryInterceptors : [ this.interceptors.unary ],
      streamInterceptors : [ this.interceptors.stream ]
    });
    this.onUpdate = options?.onUpdate;
    this.onStatus = options?.onStatus;
    this.onMetadata = options?.onMetadata;
    this.onError = options?.onError;
  }

  public disconnect =
      () => {
        const req = new ControllerDisconnectRequest();
        this.client.disconnect(req, null)
            .then(() => { console.log("disconnected"); })
            .catch((err) => { console.log("failed to disconnect", err); });
      }

  public connect = async(callback?: () => void): Promise<void> => {
    const req = new ControllerConnectRequest();
    this.updateStream = this.client.connect(req, undefined);
    this.updateStream.on("error", (err: Error) => {
      if (this.onError) {
        this.onError(err);
      } else {
        console.log("error while subscribing", err);
      }
    });

    this.updateStream.on('data', (msg: unknown) => {
      if (!this.isConnected) {
        // todo: set when the oneof of the update is a status update?
        this.isConnected = true;
        if (callback)
          callback();
        console.log("connected");
      }
      if (msg instanceof ViewerUpdate) {
        if (this.onUpdate) {
          this.onUpdate(msg);
        } else {
          console.log("got update: ", msg.toObject());
        }
      } else {
        console.log("here be dragons");
      }
    });

    this.updateStream.on('status', (status: Status) => {
      if (this.onStatus) {
        this.onStatus(status);
      } else {
        console.log("got status", status);
      }
    });

    this.updateStream.on('metadata', (metadata: Metadata) => {
      if (this.onMetadata) {
        this.onMetadata(metadata)
      } else {
        console.log("got metadata", metadata);
      }
    });

    this.updateStream.on('end', () => {
      this.isConnected = false;
      console.log("disconnected");
    });
  }
}
