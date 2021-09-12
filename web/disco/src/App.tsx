import React from "react";
import Viewer from "@disco/viewer";
import Controller from "@disco/controller";
import Landing from "./Landing";
import "./App.sass";
import { HashRouter as Router, Route } from "react-router-dom";

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
        <Router>
          <Route exact path="/" component={Landing} />
          <Route exact path="/viewer/:session?/:instance?" component={Viewer} />
          <Route
            exact
            path="/controller/:session?/:instance?"
            component={Controller}
          />
        </Router>
      </div>
    );
  };
}
