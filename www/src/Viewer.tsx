import React from "react";
import { RouteComponentProps } from "react-router-dom";
import { getToken, P2PState, P2PURLQueryProps } from "./communication/p2p";
import Communicator from "./communication/communicator";
import {
  Visualization as TextTransformVisualization,
  Parameterizer as TextTransformParameterizer,
} from "./visualizations/text-transform";
import { Visualization } from "./visualizations/visualization";

type ViewerState = {};
type ViewerProps = {};

type ViewerRouteProps = ViewerProps & RouteComponentProps<P2PURLQueryProps>;

export default class Viewer extends React.Component<
  ViewerRouteProps,
  ViewerState & P2PState
> {
  protected comm: Communicator;
  protected visualization!: Visualization;

  constructor(props: ViewerRouteProps) {
    super(props);
    const token = getToken(this.props.match, this.props.location);
    this.comm = new Communicator("viewer", { token });
    this.state = {
      token,
    };
  }

  componentDidMount = () => {
    // subscribe to updates from the communicator
    this.comm.subscribe();
    this.comm.startAnalysis();
    this.visualization = new TextTransformVisualization();
    console.log(this.visualization, "lol");
    const container = document.getElementById("Viewer");
    if (container) {
      this.visualization.init(container);
      this.visualization.start();
    }
  };

  render = () => {
    return <div id="Viewer"></div>;
  };
}
