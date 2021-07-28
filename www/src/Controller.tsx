import React from "react";
import { RouteComponentProps } from "react-router-dom";
import { getToken, P2PState, P2PURLQueryProps } from "./communication/p2p";

type ControllerState = {};
type ControllerProps = {};

type ControllerRouteProps = ControllerProps &
  RouteComponentProps<P2PURLQueryProps>;

export default class Controller extends React.Component<
  ControllerRouteProps,
  ControllerState & P2PState
> {
  constructor(props: ControllerRouteProps) {
    super(props);
    const token = getToken(this.props.match, this.props.location);
    this.state = {
      token,
    };
  }

  componentDidMount = () => {};

  render = () => {
    return <div className="Controller">{this.state.token} </div>;
  };
}
