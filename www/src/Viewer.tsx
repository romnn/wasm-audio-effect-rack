import React from "react";
import { AudioAnalysisResult } from "./generated/proto/audio/analysis/analysis_pb";

import { RouteComponentProps } from "react-router-dom";
import { Error, Metadata, Status } from "grpc-web";
import Remote, { RemoteState, RemoteURLQueryProps } from "./remote";
import RemoteViewer from "./remote/viewer";
import RemoteController from "./remote/controller";
import { Update } from "./generated/proto/grpc/remote_pb";
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
    let { token, instance } = Remote.getUser(
      this.props.match,
      this.props.location
    );
    token = token ?? Remote.generateToken();
    console.log(token);
    this.remote = new RemoteViewer(token, {});
    if (!instance) {
      console.log("querying a free viewer token");
      props.history.replace({
        pathname: `/viewer/${token}/${instance}`,
        // search: params.toString(),
      });
    }
    // the controller here is just for testing
    this.controller = new RemoteController(token, {});
    this.remote.onUpdate = this.handleUpdate;
    this.remote.onError = this.handleError;
    this.remote.onStatus = this.handleStatus;
    this.remote.onMetadata = this.handleMetadata;
    this.state = {
      token,
      instance: instance ?? "todo",
    };
  }

  handleStatus = (status: Status) => {
    console.log(status);
  };

  handleMetadata = (metadata: Metadata) => {
    console.log(metadata);
  };

  handleError = (error: Error) => {
    console.log(error);
  };

  handleUpdate = (update: Update) => {
    let audioAnalysisResult = update.getAudioAnalysisResult();
    if (audioAnalysisResult) {
      try {
        this.visualization.parameterize(audioAnalysisResult);
      } catch (err) {
        console.log(err);
      }
    }
    // todo: add commands like use parameterizer
  };

  componentDidMount = () => {
    // subscribe to updates from the remote
    this.remote.subscribe(() => {
      console.log("starting analysis");
      this.controller.startAnalysis();
    });
    // this.visualization = new TextTransformVisualization();
    const container = document.getElementById("Viewer");
    if (container) {
      this.visualization.init(container);
      const config = this.visualization.visualization.getConfig();
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
