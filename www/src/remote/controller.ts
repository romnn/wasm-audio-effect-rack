import {ClientReadableStream, Error, Metadata, Status} from "grpc-web";

import {
  StartAnalysisRequest,
} from "../generated/proto/grpc/remote_pb";
import {
  RemoteControllerClient,
} from "../generated/proto/grpc/RemoteServiceClientPb";

import RemoteClient from "./index";
import {StreamAuthInterceptor, UnaryAuthInterceptor} from "./interceptors";

export interface RemoteControllerConfig {}

export default class RemoteController extends
    RemoteClient<RemoteControllerClient> {
  // public userToken: string;
  // protected client: RemoteControllerClient;
  // protected interceptors: {
  //   stream: StreamAuthInterceptor,
  //   unary: UnaryAuthInterceptor,
  // };

  constructor(userToken: string, options?: RemoteControllerConfig) {
    super(RemoteControllerClient, userToken);
    // console.log(options);
    // this.userToken = userToken;
    // this.interceptors = {
    //   stream : new StreamAuthInterceptor(this.userToken),
    //   unary : new UnaryAuthInterceptor(this.userToken),
    // };
    // this.client = new RemoteControllerClient(RemoteClient.endpoint, null, {
    //   unaryInterceptors : [ this.interceptors.unary ],
    //   streamInterceptors : [ this.interceptors.stream ]
    // });
  }

  public startAnalysis = () => {
    const req = new StartAnalysisRequest();
    // req.setUserId(this.userID);
    this.client.startAnalysis(req, null)
        .then(() => { console.log("started analysis"); })
        .catch((err) => { console.log("failed to start analysis", err); });
  }
}
