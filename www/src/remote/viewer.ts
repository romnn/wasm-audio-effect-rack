import {ClientReadableStream, Error, Metadata, Status} from "grpc-web";
import {
  SubscriptionRequest,
  UnsubscriptionRequest,
  Update
} from "../generated/proto/grpc/remote_pb";
import {
  RemoteViewerClient,
} from "../generated/proto/grpc/RemoteServiceClientPb";
import RemoteClient from "./index";

type SubMessageHandler = (message: Update) => void;
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

  protected isSubscribed = false;
  protected updateStream?: ClientReadableStream<unknown>;
  // protected interceptors: {
  //   stream: StreamAuthInterceptor,
  //   unary: UnaryAuthInterceptor,
  // };

  
  constructor(userToken: string, options?: RemoteViewerConfig) {
    super(RemoteViewerClient, userToken);
    // this.client = new RemoteViewerClient(RemoteClient.endpoint, null, {
    //   unaryInterceptors : [ this.interceptors.unary ],
    //   streamInterceptors : [ this.interceptors.stream ]
    // });
    this.onUpdate = options?.onUpdate;
    this.onStatus = options?.onStatus;
    this.onMetadata = options?.onMetadata;
    this.onError = options?.onError;
  }

  public unsubscribe =
      () => {
        const req = new UnsubscriptionRequest();
        this.client.unsubscribe(req, null)
            .then(() => { console.log("unsubscribed"); })
            .catch((err) => { console.log("failed to unsubscribe", err); });
      }

  
  public subscribe = (callback?: () => void) => {
    const req = new SubscriptionRequest();
    // req.setUserId(this.userID);
    this.updateStream = this.client.subscribe(req, undefined);
    this.updateStream.on("error", (err: Error) => {
      if (this.onError) {
        this.onError(err);
      } else {
        console.log("error while subscribing", err);
      }
    });
    this.updateStream.on('data', (msg: unknown) => {
      if (!this.isSubscribed) {
        // todo: set when the oneof of the update is a status update?
        this.isSubscribed = true;
        if (callback)
          callback();
        console.log("subscribed");
      }
      if (msg instanceof Update) {
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
      this.isSubscribed = false;
      console.log("unsubscribed");
    });
  }
}
