import {ClientReadableStream, Error, Metadata, Status} from "grpc-web";

import {AnalysisResult} from "../generated/proto/audio/analysis_pb";
import {
  SubscriptionRequest,
  Update
} from "../generated/proto/grpc/remote_pb";
import {
  RemoteControllerClient,
} from "../generated/proto/grpc/RemoteServiceClientPb";

type MessageHandler = (message: Update) => void;

export interface CommunicatorConfig {
  token?: string;
  onMessage?: MessageHandler;
}

export const GRPC_ENDPOINT =
    window.location.protocol + "//" + window.location.hostname + ":9000";
// (window.location.port ? ":" + window.location.port : "");

export default class Communicator {
  public onMessage?: MessageHandler;

  protected receiver: string;
  protected isSubscribed = false;
  protected updateStream?: ClientReadableStream<unknown>;
  protected client = new RemoteControllerClient(GRPC_ENDPOINT, null, null);

  constructor(receiver: string, options?: CommunicatorConfig) {
    console.log("constructor here");
    console.log(receiver);
    console.log(options);
    this.receiver = receiver;
    this.onMessage = options?.onMessage;

    // subscribe to updates from the remote controller
    this.subscribe();
  }

  public unsubscribe = () => { 
    // todo: actually send an unsubscribe request to the backend
    console.log("unsubscribed"); 

  }

  // Promise<void>
  // return new Promise<void>((resolve, reject) => {

  public subscribe = () => {
    if (this.isSubscribed)
      return;
    const subRequest = new SubscriptionRequest();
    subRequest.setUserId("roman");
    this.updateStream = this.client.subscribe(subRequest, undefined);
    this.updateStream.on(
        "error",
        (err: Error) => { console.log("error while subscribing", err); });
    this.updateStream.on('data', (msg: unknown) => {
      if (!this.isSubscribed) {
        // todo: set when the oneof of the update is a status update?
        this.isSubscribed = true;
        console.log("subscribed");
      }
      if (msg instanceof Update) {
        // todo: get a user token for unsubscribe
        // console.log(response.getMessage());
        console.log("got update: ", msg);
        if (this.onMessage)
          this.onMessage(msg);
      }
    });
    this.updateStream.on('status', (status: Status) => {
      // todo: get a user token for unsubscribe
      console.log("got status", status);
    });
    this.updateStream.on('metadata', (metadata: Metadata) => {
      // todo: get a user token for unsubscribe
      console.log("got metadata", metadata);
    });
    this.updateStream.on('end', () => {
      // stream end signal
      this.isSubscribed = false;
      console.log("unsubscribed");
    });
  }
}
