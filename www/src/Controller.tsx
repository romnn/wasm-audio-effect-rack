import React from "react";
import { RouteComponentProps } from "react-router-dom";
import RemoteController from "./remote/controller";
import Remote, {RemoteState, RemoteURLQueryProps } from "./remote";

type ControllerState = {};
type ControllerProps = {};

type ControllerRouteProps = ControllerProps &
  RouteComponentProps<RemoteURLQueryProps>;

export default class Controller extends React.Component<
  ControllerRouteProps,
  ControllerState & RemoteState
> {
  constructor(props: ControllerRouteProps) {
    super(props);
    let { token, instance } = Remote.getUser(
      this.props.match,
      this.props.location
    );
    token = token ?? Remote.generateToken();
    this.state = {
      token,
      instance: instance ?? "todo",
    };
  }

  componentDidMount = () => {};

  render = () => {
    return <div className="Controller">{this.state.token} </div>;
  };
}
