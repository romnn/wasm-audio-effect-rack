import React from "react";
import {
  AudioAnalyzer,
  AudioAnalysisResult,
} from "./generated/proto/audio/analysis/analysis_pb";
import { SpectralAudioAnalyzer } from "./generated/proto/audio/analysis/spectral_pb";
import { BpmDetectionAudioAnalyzer } from "./generated/proto/audio/analysis/bpm_pb";

import { RouteComponentProps } from "react-router-dom";
import { Error, Metadata, Status } from "grpc-web";
import Remote, { RemoteState, RemoteURLQueryProps } from "./remote";
import RemoteViewer from "./remote/viewer";
import RemoteController from "./remote/controller";
import { ViewerUpdate } from "./generated/proto/grpc/remote_pb";
// import TextTransformVisualization from "./visualizations/text-transform";
import { TTFStartConfig } from "./visualizations/text-transform/parameterizer";
import VisualizationGallery from "./visualizations/gallery";
import {
  VisualizationController,
  // Visualization
} from "./visualizations/visualization";
// import { Parameterizer } from "./visualizations/parameterizer";

type ViewerState = {};
type ViewerProps = {};

type ViewerRouteProps = ViewerProps & RouteComponentProps<RemoteURLQueryProps>;

export default class Viewer extends React.Component<
  ViewerRouteProps,
  ViewerState & RemoteState
> {
  protected remote: RemoteViewer;
  protected controller: RemoteController;

  // protected visualization?: VisualizationController<AudioAnalysisResult>;
  protected visualization: VisualizationController<
    TTFStartConfig,
    AudioAnalysisResult
  > = new VisualizationGallery.visualizations[0]();

  constructor(props: ViewerRouteProps) {
    super(props);
    let { session, instance } = Remote.getSessionInstance(
      this.props.match,
      this.props.location
    );
    // token = token ?? Remote.generateToken();
    this.remote = new RemoteViewer(session, instance, {});
    // the controller here is just for testing
    this.controller = new RemoteController(session, instance, {});
    console.log(`viewer instance "${instance}" from session "${session}"`);
    // if (!instance) {
    //   console.log("querying a free viewer token");
    //   this.remote.newInstanceId().then((id) => {
    //     this.setState({instance: id});
    //     props.history.replace({
    //       pathname: `/viewer/${this.state.token}/${this.state.instance}`,
    //       // search: params.toString(),
    //     });
    //   });
    // }
    this.remote.onUpdate = this.handleUpdate;
    this.remote.onError = this.handleError;
    this.remote.onStatus = this.handleStatus;
    this.remote.onMetadata = this.handleMetadata;
    this.state = {
      session,
      instance,
    };
  }

  handleStatus = (status: Status) => {
    console.log("status", status);
  };

  handleMetadata = (metadata: Metadata) => {
    console.log("we got metadata:", metadata);
  };

  handleError = (error: Error) => {
    console.log("stream error", error);
  };

  handleUpdate = (update: ViewerUpdate) => {
    let audioAnalysisResult = update.getAudioAnalysisResult();
    let heartbeat = update.getHeartbeat();
    let assignment = update.getAssignment();
    if (audioAnalysisResult) {
      try {
        this.visualization.parameterize(audioAnalysisResult);
      } catch (err) {
        console.log(err);
      }
    } else if (assignment) {
      let session = assignment.getSessionToken()?.getToken();
      let instance = assignment.getInstanceId()?.getId();
      this.setState({
        session,
        instance,
      });
      console.log(`assigned to ${instance} in session ${session}`);
      this.props.history.replace({
        pathname: `/viewer/${session}/${instance}`,
        search: "",
      });
    } else if (heartbeat) {
      console.log("heartbeat", heartbeat.toObject());
    }
    // todo: add commands like use parameterizer
  };

  setup = async (): Promise<void> => {
    try {
      console.log("connecting...");
      await this.remote.connect();
      console.log("adding an audio input stream...");
      const inputStream = await this.controller.addAudioInputStream();
      console.log("connecting the audio input stream to an output stream...");
      const inputDescriptor = inputStream.getDescriptor();
      if (inputDescriptor) {
        const outputStream = await this.controller.addAudioOutputStream(
          inputDescriptor
        );
      }
      console.log("connecting the spectral analyzer to the audio input stream");
      if (inputDescriptor) {
        const audioAnalyzer = new AudioAnalyzer();
        const spectralAnalyzer = new SpectralAudioAnalyzer();
        audioAnalyzer.setSpectral(spectralAnalyzer);

        const audioAnalyzerDescriptor = (
          await this.controller.addAudioAnalyzer(audioAnalyzer, inputDescriptor)
        ).getDescriptor();
        console.log("subscribe this viewer instance to the analyzer");
        const instance = this.state.instance;
        if (audioAnalyzerDescriptor && instance) {
          await this.controller.subscribeToAudioAnalyzer(
            audioAnalyzerDescriptor,
            instance
          );
        }
        console.log("connect lights to the analyzer");
        if (audioAnalyzerDescriptor) {
          await this.controller.connectLightsToAudioAnalyzer(
            audioAnalyzerDescriptor,
            "/dev/ttyACM0",
            [{ numLights: 300, pin: 5 }]
            // [{numLights: 300, pin: 1 }, {numLights: 300, pin: 1 }],
          );
        }
      }
      console.log("connecting the bpm analyzer to the audio input stream");
      if (inputDescriptor) {
        const audioAnalyzer = new AudioAnalyzer();
        const bpmAnalyzer = new BpmDetectionAudioAnalyzer();
        audioAnalyzer.setBpm(bpmAnalyzer);
        // const bpmAnalyzer = new AudioAnalyzer();
        const audioAnalyzerDescriptor = (
          await this.controller.addAudioAnalyzer(audioAnalyzer, inputDescriptor)
        ).getDescriptor();
        console.log("subscribe this viewer instance to the analyzer");
        const instance = this.state.instance;
        if (audioAnalyzerDescriptor && instance) {
          const subscriptions = await this.controller.subscribeToAudioAnalyzer(
            audioAnalyzerDescriptor,
            instance
          );
        }
      }
    } catch (err) {
      // console.log("viewer failed to connect:", err);
      console.log(err);
      return;
    }
  };

  componentDidMount = () => {
    this.setup()
      .then(() => console.log("setup completed"))
      .catch((err) => console.log("setup failed", err));
    const container = document.getElementById("Viewer");
    if (container) {
      this.visualization.init(container);
      const config = this.visualization.visualization.getConfig();
      this.visualization.visualization.toggleStats(true);
      this.visualization.visualization.toggleControls(true);
      this.visualization.configure(config);
      this.visualization.start();
    }
  };

  // TODO: functions
  // remove visualization
  // update parameterizer
  // update visualization
  // show / hide debug view

  render = () => {
    return <div id="Viewer"></div>;
  };
}
