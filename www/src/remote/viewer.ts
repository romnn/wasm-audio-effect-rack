import {ClientReadableStream, Error, Metadata, Status} from "grpc-web";

import {
  InstanceId,
  // NewInstanceIdRequest,
  SessionToken,
  ViewerConnectRequest,
  ViewerDisconnectRequest,
  ViewerUpdate,
} from "../generated/proto/grpc/remote_pb";
import {
  RemoteViewerClient,
} from "../generated/proto/grpc/RemoteServiceClientPb";

import RemoteClient from "./index";

type SubMessageHandler = (message: ViewerUpdate) => void;
type SubErrorHandler = (error: Error) => void;
type SubStatusHandler = (status: Status) => void;
type SubMetadataHandler = (metadata: Metadata) => void;

export interface RemoteViewerConfig {
  onUpdate?: SubMessageHandler;
  onStatus?: SubStatusHandler;
  onError?: SubErrorHandler;
  onMetadata?: SubMetadataHandler;
}

export default class RemoteViewer extends RemoteClient<RemoteViewerClient> {
  // public userToken: string;

  public onUpdate?: SubMessageHandler;
  public onStatus?: SubStatusHandler;
  public onError?: SubErrorHandler;
  public onMetadata?: SubMetadataHandler;

  protected isConnected = false;
  protected updateStream?: ClientReadableStream<unknown>;
  // protected interceptors: {
  //   stream: StreamAuthInterceptor,
  //   unary: UnaryAuthInterceptor,
  // };

  // constructor(sessionToken: SessionToken|undefined,
  //             instanceId: InstanceId|undefined, options?:
  //             RemoteViewerConfig) {
  constructor(session: string|undefined, instance: string|undefined,
              options?: RemoteViewerConfig) {
    super(RemoteViewerClient, session, instance);
    // this.client = new RemoteViewerClient(RemoteClient.endpoint, null, {
    //   unaryInterceptors : [ this.interceptors.unary ],
    //   streamInterceptors : [ this.interceptors.stream ]
    // });
    this.onUpdate = options?.onUpdate;
    this.onStatus = options?.onStatus;
    this.onMetadata = options?.onMetadata;
    this.onError = options?.onError;
  }

  // public newInstanceId = async():
  //     Promise<InstanceId> => {
  //       const req = new NewInstanceIdRequest();
  //       return this.client.newInstanceId(req, null);
  //     }

  public disconnect =
      () => {
        const req = new ViewerDisconnectRequest();
        this.client.disconnect(req, null)
            .then(() => { console.log("disconnected"); })
            .catch((err) => { console.log("failed to disconnect", err); });
      }

  // public connect = async (instance: InstanceId|undefined, callback?: () =>
  // void): Promise<void> => {
  public connect = async(callback?: () => void): Promise<void> => {
    const req = new ViewerConnectRequest();
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
      console.log("DisConnected");
    });
  }
}
