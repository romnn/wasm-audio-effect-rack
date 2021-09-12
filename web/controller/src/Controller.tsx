import { ControllerUpdate } from "@disco/core/grpc/controller";
import { Error, Metadata, Status } from "grpc-web";
import React from "react";
import { RouteComponentProps } from "react-router-dom";
import { RemoteController } from "@disco/controller";
import MidiController from "@disco/controls/src/midi";
import Remote, { RemoteState, RemoteURLQueryProps } from "@disco/core/remote";

type ControllerState = {};
type ControllerProps = {};

type ControllerRouteProps = ControllerProps &
  RouteComponentProps<RemoteURLQueryProps>;

export default class Controller extends React.Component<
  ControllerRouteProps,
  ControllerState & RemoteState
> {
  protected midiController?: MidiController;
  protected remote: RemoteController;

  constructor(props: ControllerRouteProps) {
    super(props);
    let { session, instance } = Remote.getSessionInstance(
      this.props.match,
      this.props.location
    );
    this.remote = new RemoteController(session, instance, {});
    console.log(`controller instance "${instance}" from session "${session}"`);
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

  handleUpdate = (update: ControllerUpdate) => {
    let heartbeat = update.getHeartbeat();
    let assignment = update.getAssignment();
    if (assignment) {
      let session = assignment.getSessionToken()?.getToken();
      let instance = assignment.getInstanceId()?.getId();
      this.setState({
        session,
        instance,
      });
      console.log(`assigned to ${instance} in session ${session}`);
      this.props.history.replace({
        pathname: `/controller/${session}/${instance}`,
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
    } catch (err) {
      console.log(err);
    }

    try {
      this.midiController = new MidiController();
      await this.midiController.init();
    } catch (err) {
      console.log("midi controller not available:", err);
    }
  };

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
        <ul>
          <li></li>
        </ul>
      </div>
    );
  };
}
