// import {ClientReadableStream, Error, Metadata, Status} from "grpc-web";
import {
  AudioAnalyzer,
} from "../generated/proto/audio/analysis/analysis_pb";
import {
  AddAudioAnalyzerRequest,
  AddAudioInputStreamRequest,
  AddAudioOutputStreamRequest,
  AudioAnalyzerDescriptor,
  AudioInputDescriptor,
  AudioInputStream,
  ConnectLightsToAudioAnalyzerRequest,
  InstanceId,
  Lights,
  LightStrip,
  SubscribeToAudioAnalyzerRequest,
} from "../generated/proto/grpc/remote_pb";
import {
  RemoteControllerClient,
} from "../generated/proto/grpc/RemoteServiceClientPb";

import RemoteClient from "./index";

export interface RemoteControllerConfig {}

export default class RemoteController extends
    RemoteClient<RemoteControllerClient> {

  constructor(session: string|undefined, instance: string|undefined,
              options?: RemoteControllerConfig) {
    super(RemoteControllerClient, session, instance);
  }

  // public newInstanceId = async (): Promise<InstanceId> => {
  //   const req = new NewInstanceIdRequest();
  //   const test = await this.client.newInstanceId(req, null);
  //   return test;
  //       // .then(
  //       //     (stream) => { console.log("added new audio input stream",
  //       input); })
  //       // .catch((err) => { console.log("failed to start analysis", err);
  //       });
  // }

  public addAudioInputStream =
      () => {
        const req = new AddAudioInputStreamRequest();
        return this.client.addAudioInputStream(req, null);
        // .then((stream) => {
        //   console.log("added new audio input stream", stream);
        // })
        // .catch((err) => { console.log("failed to start analysis", err); });
      }

  public addAudioOutputStream =
      (descriptor: AudioInputDescriptor) => {
        const req = new AddAudioOutputStreamRequest();
        req.setInputDescriptor(descriptor);
        return this.client.addAudioOutputStream(req, null);
        // .then((stream) => {
        //   console.log("added new audio input stream", stream);
        // })
        // .catch((err) => { console.log("failed to start analysis", err); });
      }

  public addAudioAnalyzer =
      (analyzer: AudioAnalyzer, descriptor: AudioInputDescriptor) => {
        const req = new AddAudioAnalyzerRequest();
        req.setInputDescriptor(descriptor);
        req.setAnalyzer(analyzer);
        return this.client.addAudioAnalyzer(req, null);
        // .then((stream) => {
        //   console.log("added new audio input stream", stream);
        // })
        // .catch((err) => { console.log("failed to start analysis", err); });
      }

  public subscribeToAudioAnalyzer =
      (descriptor: AudioAnalyzerDescriptor, instance: string) => {
        const req = new SubscribeToAudioAnalyzerRequest();
        const instanceId = new InstanceId();
        instanceId.setId(instance);
        req.setAudioAnalyzerDescriptor(descriptor);
        req.setInstanceId(instanceId);
        return this.client.subscribeToAudioAnalyzer(req, null);
        // .then((stream) => {
        //   console.log("added new audio input stream", stream);
        // })
        // .catch((err) => { console.log("failed to start analysis", err); });
      }

  public connectLightsToAudioAnalyzer =
      (descriptor: AudioAnalyzerDescriptor, serialPort: string,
       config: {numLights: number, pin: number}[]) => {
        const req = new ConnectLightsToAudioAnalyzerRequest();
        const lights = new Lights();
        lights.setSerialPort(serialPort);
        lights.setStripsList(config.map((c) => {
          const strip = new LightStrip();
          strip.setNumLights(c.numLights);
          strip.setPin(c.pin);
          return strip
        }));
        req.setAudioAnalyzerDescriptor(descriptor);
        req.setLights(lights);
        return this.client.connectLightsToAudioAnalyzer(req, null);
      }
}
