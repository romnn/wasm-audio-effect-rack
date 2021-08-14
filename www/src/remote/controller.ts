import {ClientReadableStream, Error, Metadata, Status} from "grpc-web";
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
  ControllerConnectRequest,
  ControllerDisconnectRequest,
  ControllerUpdate,
  InstanceId,
  Lights,
  LightStrip,
  SubscribeToAudioAnalyzerRequest,
} from "../generated/proto/grpc/remote_pb";
import {
  RemoteControllerClient,
} from "../generated/proto/grpc/RemoteServiceClientPb";

import RemoteClient from "./index";

type SubMessageHandler = (message: ControllerUpdate) => void;
type SubErrorHandler = (error: Error) => void;
type SubStatusHandler = (status: Status) => void;
type SubMetadataHandler = (metadata: Metadata) => void;

export interface RemoteControllerConfig {
  onUpdate?: SubMessageHandler;
  onStatus?: SubStatusHandler;
  onError?: SubErrorHandler;
  onMetadata?: SubMetadataHandler;
}

export default class RemoteController extends
    RemoteClient<RemoteControllerClient> {
  public onUpdate?: SubMessageHandler;
  public onStatus?: SubStatusHandler;
  public onError?: SubErrorHandler;
  public onMetadata?: SubMetadataHandler;

  protected isConnected = false;
  protected updateStream?: ClientReadableStream<unknown>;

  constructor(session: string|undefined, instance: string|undefined,
              options?: RemoteControllerConfig) {
    super(RemoteControllerClient, session, instance);
    this.onUpdate = options?.onUpdate;
    this.onStatus = options?.onStatus;
    this.onMetadata = options?.onMetadata;
    this.onError = options?.onError;
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

  public connect = async():
      Promise<void> => {
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
            console.log("connected");
          }
          if (msg instanceof ControllerUpdate) {
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

  public disconnect =
      () => {
        const req = new ControllerDisconnectRequest();
        this.client.disconnect(req, null)
            .then(() => { console.log("disconnected"); })
            .catch((err) => { console.log("failed to disconnect", err); });
      }

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
