import React from "react";

import { test } from "@disco/core";
import {
  AudioAnalyzer,
  AudioAnalysisResult,
  SpectralAudioAnalyzer,
  BpmDetectionAudioAnalyzer,
} from "@disco/core/audio/analysis";

import { RouteComponentProps } from "react-router-dom";
import { Error, Metadata, Status } from "grpc-web";
import Remote, { RemoteState, RemoteURLQueryProps } from "@disco/core/remote";
import { RemoteViewer } from "@disco/viewer";
import { RemoteController } from "@disco/controller";
import { ViewerUpdate } from "@disco/core/grpc/viewer";

import VisualizationGallery from "@disco/visuals/gallery";
import { VisualizationController } from "@disco/visuals";

type ViewerState = {};
type ViewerProps = {};

type ViewerRouteProps = ViewerProps & RouteComponentProps<RemoteURLQueryProps>;

export default class Viewer extends React.Component<
  ViewerRouteProps,
  ViewerState & RemoteState
> {
  protected remote: RemoteViewer;
  protected controller: RemoteController;

  protected visualization: VisualizationController = new VisualizationGallery.visualizations[0]();

  constructor(props: ViewerRouteProps) {
    super(props);
    let { session, instance } = Remote.getSessionInstance(
      this.props.match,
      this.props.location
    );
    this.remote = new RemoteViewer(session, instance, {});
    // the controller here is just for testing
    this.controller = new RemoteController(session, instance, {});
    console.log(`viewer instance "${instance}" from session "${session}"`);
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
      // console.log("analysis result", audioAnalysisResult.toObject());
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
      // console.log("connecting the bpm analyzer to the audio input stream");
      // if (inputDescriptor) {
      //   const audioAnalyzer = new AudioAnalyzer();
      //   const bpmAnalyzer = new BpmDetectionAudioAnalyzer();
      //   audioAnalyzer.setBpm(bpmAnalyzer);
      //   // const bpmAnalyzer = new AudioAnalyzer();
      //   const audioAnalyzerDescriptor = (
      //     await this.controller.addAudioAnalyzer(audioAnalyzer, inputDescriptor)
      //   ).getDescriptor();
      //   console.log("subscribe this viewer instance to the analyzer");
      //   const instance = this.state.instance;
      //   if (audioAnalyzerDescriptor && instance) {
      //     const subscriptions = await this.controller.subscribeToAudioAnalyzer(
      //       audioAnalyzerDescriptor,
      //       instance
      //     );
      //   }
      // }
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
      const config = this.visualization.getConfig();
      this.visualization.toggleStats(true);
      this.visualization.toggleControls(true);
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