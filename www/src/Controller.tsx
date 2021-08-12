import React from "react";
import { RouteComponentProps } from "react-router-dom";
import RemoteController from "./remote/controller";
import MidiController from "./controls/midi";
import Remote, { RemoteState, RemoteURLQueryProps } from "./remote";

type ControllerState = {};
type ControllerProps = {};

type ControllerRouteProps = ControllerProps &
  RouteComponentProps<RemoteURLQueryProps>;

export default class Controller extends React.Component<
  ControllerRouteProps,
  ControllerState & RemoteState
> {
  protected midiController?: MidiController;

  constructor(props: ControllerRouteProps) {
    super(props);
    let { session, instance } = Remote.getSessionInstance(
      this.props.match,
      this.props.location
    );
    this.state = {
      session,
      instance,
    };
  }

  setup = async (): Promise<void> => {
    try {
      this.midiController = new MidiController();
      await this.midiController.init();
    } catch (err) {
      console.log("midi controller not available:", err);
    }
  }

  componentDidMount = () => {
    this.setup()
      .then(() => console.log("controller setup completed"))
      .catch((err) => console.log("controller setup failed", err));
  };

  render = () => {
    return (
      <div className="Controller">
        {this.state.session}
        {this.state.instance}
      </div>
    );
  };
}
