import React from "react";
import { AudioAnalysisResult } from "./generated/proto/audio/analysis/analysis_pb";

import { RouteComponentProps } from "react-router-dom";
import { Error, Metadata, Status } from "grpc-web";
import { getToken, P2PState, P2PURLQueryProps } from "./remote/p2p";
import { generateToken } from "./remote";
import RemoteViewer from "./remote/viewer";
import RemoteController from "./remote/controller";
import { Update } from "./generated/proto/grpc/remote_pb";
// import TextTransformVisualization from "./visualizations/text-transform";
import VisualizationGallery from "./visualizations/gallery";
import {
  VisualizationController,
  Visualization,
} from "./visualizations/visualization";
import { Parameterizer } from "./visualizations/parameterizer";

type ViewerState = {};
type ViewerProps = {};

type ViewerRouteProps = ViewerProps & RouteComponentProps<P2PURLQueryProps>;

export default class Viewer extends React.Component<
  ViewerRouteProps,
  ViewerState & P2PState
> {
  protected remote: RemoteViewer;
  protected controller: RemoteController;
  // protected visualization?: VisualizationController<AudioAnalysisResult>;
  protected visualization: VisualizationController<AudioAnalysisResult> = new VisualizationGallery.visualizations[0]();

  constructor(props: ViewerRouteProps) {
    super(props);
    const token =
      getToken(this.props.match, this.props.location) ?? generateToken();
    console.log(token);
    this.remote = new RemoteViewer(token, {});
    // the controller here is just for testing
    this.controller = new RemoteController(token, {});
    this.remote.onUpdate = this.handleUpdate;
    this.remote.onError = this.handleError;
    this.remote.onStatus = this.handleStatus;
    this.remote.onMetadata = this.handleMetadata;
    this.state = {
      token,
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
      this.visualization.parameterize(audioAnalysisResult);
    }
    // todo: add commands like use parameterizer
  };

  componentDidMount = () => {
    // subscribe to updates from the remote
    this.remote.subscribe(() => {
      this.controller.startAnalysis();
    });
    // this.visualization = new TextTransformVisualization();
    const container = document.getElementById("Viewer");
    if (container) {
      this.visualization.init(container);
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
