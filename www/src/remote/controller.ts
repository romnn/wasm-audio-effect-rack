// import {ClientReadableStream, Error, Metadata, Status} from "grpc-web";

import {
  // NewInstanceIdRequest,
  AddAudioInputStreamRequest,
  InstanceId,
  AudioInputStream,
} from "../generated/proto/grpc/remote_pb";
import {
  RemoteControllerClient,
} from "../generated/proto/grpc/RemoteServiceClientPb";

import RemoteClient from "./index";

export interface RemoteControllerConfig {}

export default class RemoteController extends
    RemoteClient<RemoteControllerClient> {

  constructor(session: string|undefined, instance: string|undefined, options?: RemoteControllerConfig) {
    super(RemoteControllerClient, session, instance);
  }

  // public newInstanceId = async (): Promise<InstanceId> => {
  //   const req = new NewInstanceIdRequest();
  //   const test = await this.client.newInstanceId(req, null);
  //   return test;
  //       // .then(
  //       //     (stream) => { console.log("added new audio input stream", input); })
  //       // .catch((err) => { console.log("failed to start analysis", err); });
  // }

  public addAudioInputStream = () => {
    const req = new AddAudioInputStreamRequest();
    this.client.addAudioInputStream(req, null)
        .then(
            (stream) => { console.log("added new audio input stream", stream); })
        .catch((err) => { console.log("failed to start analysis", err); });
  }
}
