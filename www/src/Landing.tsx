import React from "react";
import { Redirect, Link } from "react-router-dom";

type LandingState = {};
type LandingProps = {};

export default class Landing extends React.Component<
  LandingProps,
  LandingState
> {
  constructor(props: LandingProps) {
    super(props);
    this.state = {};
  }

  render = () => {
    return (
      <div id="Landing">
        <Redirect to="/viewer/roman" />
        <h1>Welcome</h1>
        <p>Here will be a cool landing page with instructions one day</p>
        also you should enter your name here
        <input id="name" type="text"></input>
        <Link to="/controller/">controller</Link>
        <Link to="/viewer/">viewer</Link>
      </div>
    );
  };
}
