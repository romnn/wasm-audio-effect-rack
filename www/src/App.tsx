import React from "react";
import Fractal from "./Fractal1";
import "./App.css";

type AppState = {};
type AppProps = {};

export default class App extends React.Component<AppProps, AppState> {
  constructor(props: AppProps) {
    super(props);
    this.state = {};
  }

  componentDidMount = () => {};

  render = () => {
    return (
      <div className="App">
        <Fractal></Fractal>
      </div>
    );
  };
}
