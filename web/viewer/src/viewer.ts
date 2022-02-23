import {
  InstanceId,
  SessionToken,
} from "@disco/core/grpc";
import {
  RemoteViewerClient,
  ViewerSubscribeRequest,
  ViewerDisconnectRequest,
  ViewerUpdate,
} from "@disco/core/grpc/viewer";
import RemoteClient from "@disco/core/remote";
import {ClientReadableStream, Error, Metadata, Status} from "grpc-web";

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
  public onUpdate?: SubMessageHandler;
  public onStatus?: SubStatusHandler;
  public onError?: SubErrorHandler;
  public onMetadata?: SubMetadataHandler;

  protected isConnected = false;
  protected updateStream?: ClientReadableStream<unknown>;

  constructor(session: string|undefined, instance: string|undefined,
              options?: RemoteViewerConfig) {
    super(RemoteViewerClient, session, instance);
    this.onUpdate = options?.onUpdate;
    this.onStatus = options?.onStatus;
    this.onMetadata = options?.onMetadata;
    this.onError = options?.onError;
  }

  public disconnect =
      () => {
        const req = new ViewerDisconnectRequest();
        this.client.disconnect(req, null)
            .then(() => { console.log("disconnected"); })
            .catch((err) => { console.log("failed to disconnect", err); });
      }

  public connect = async(callback?: () => void): Promise<void> => {
    const req = new ViewerSubscribeRequest();
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
