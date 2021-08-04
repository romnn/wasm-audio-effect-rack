import {ClientReadableStream, Error, Metadata, Status} from "grpc-web";

// import {AnalysisResult} from "../generated/proto/audio/analysis_pb";
import {
  StartAnalysisRequest,
  SubscriptionRequest,
  UnsubscriptionRequest,
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

  protected userID: string;
  protected receiver: string;
  protected isSubscribed = false;
  protected updateStream?: ClientReadableStream<unknown>;
  protected client = new RemoteControllerClient(GRPC_ENDPOINT, null, null);

  constructor(receiver: string, options?: CommunicatorConfig) {
    console.log("constructor here");
    console.log(receiver);
    console.log(options);
    // todo: generate a random user id here
    this.userID = "roman";
    this.receiver = receiver;
    this.onMessage = options?.onMessage;
  }

  public unsubscribe =
      () => {
        const req = new UnsubscriptionRequest();
        req.setUserId(this.userID);
        this.client.unsubscribe(req, null)
            .then(() => { console.log("unsubscribed"); })
            .catch((err) => { console.log("failed to unsubscribe", err); });
      }

  public startAnalysis =
      () => {
        const req = new StartAnalysisRequest();
        req.setUserId(this.userID);
        this.client.startAnalysis(req, null)
            .then(() => { console.log("started analysis"); })
            .catch((err) => { console.log("failed to start analysis", err); });
      }

  public subscribe = () => {
    const req = new SubscriptionRequest();
    req.setUserId(this.userID);
    this.updateStream = this.client.subscribe(req, undefined);
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
        console.log("got update: ", msg.toObject());
        if (this.onMessage)
          this.onMessage(msg);
      }
    });
    this.updateStream.on(
        'status', (status: Status) => { console.log("got status", status); });
    this.updateStream.on(
        'metadata',
        (metadata: Metadata) => { console.log("got metadata", metadata); });
    this.updateStream.on('end', () => {
      this.isSubscribed = false;
      console.log("unsubscribed");
    });
  }
}
