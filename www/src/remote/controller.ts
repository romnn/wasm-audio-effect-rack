// import {ClientReadableStream, Error, Metadata, Status} from "grpc-web";

import {
  StartAnalysisRequest,
} from "../generated/proto/grpc/remote_pb";
import {
  RemoteControllerClient,
} from "../generated/proto/grpc/RemoteServiceClientPb";

import RemoteClient from "./index";

export interface RemoteControllerConfig {}

export default class RemoteController extends
    RemoteClient<RemoteControllerClient> {

  constructor(userToken: string, options?: RemoteControllerConfig) {
    super(RemoteControllerClient, userToken);
  }

  public startAnalysis = () => {
    const req = new StartAnalysisRequest();
    this.client.startAnalysis(req, null)
        .then(() => { console.log("started analysis"); })
        .catch((err) => { console.log("failed to start analysis", err); });
  }
}
